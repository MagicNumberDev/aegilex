//! Rust-owned resource table for Core Wasm guests.
//!
//! Replaces the previous runtime resource table with Core-specific storage.
//! guest-visible representation stays an opaque canonical-ABI `u32` token,
//! but each token validates against:
//!
//! ```text
//! token generation + ResourceKind + plugin generation owner
//! + resource lifetime + current invocation frame (HostBorrowed)
//! ```
//!
//! Tokens are never pointers or native handles. A stale, wrong-kind,
//! cross-owner, wrong-generation, or expired-frame token resolves as
//! `not-found`. A slot whose generation counter exhausts is retired rather
//! than reused.

use std::any::Any;
use std::collections::BTreeSet;

use crate::host::runtime::handles::ResourceLifetime;

const TOKEN_GENERATION_BITS: u32 = 16;
const TOKEN_INDEX_MASK: u32 = (1 << TOKEN_GENERATION_BITS) - 1;
const MAX_GENERATION: u32 = TOKEN_INDEX_MASK;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResourceTableError {
    NotFound,
}

#[derive(Debug)]
struct Slot {
    generation: u32,
    kind: u32,
    owner: u64,
    lifetime: ResourceLifetime,
    frame: u64,
    value: Option<Box<dyn Any + Send>>,
    children: Vec<usize>,
    parent: Option<usize>,
}

/// Opaque guest-visible token: `(generation << 16) | slot index`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ResourceToken(u32);

impl ResourceToken {
    pub(crate) fn rep(self) -> u32 {
        self.0
    }

    pub(crate) fn from_rep(rep: u32) -> Self {
        Self(rep)
    }

    fn generation(self) -> u32 {
        self.0 >> TOKEN_GENERATION_BITS
    }

    fn index(self) -> usize {
        (self.0 & TOKEN_INDEX_MASK) as usize
    }
}

#[derive(Debug, Default)]
pub(crate) struct CoreResourceTable {
    slots: Vec<Option<Slot>>,
    /// Highest generation ever assigned to each slot index; survives removal
    /// so a reused index never validates an older token.
    generations: Vec<u32>,
    free: BTreeSet<usize>,
    retired: BTreeSet<usize>,
}

impl CoreResourceTable {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    #[cfg(test)]
    /// Allocates a slot and returns its token. `frame` is recorded for
    /// `HostBorrowed` lifetimes and checked on every `check`.
    pub(crate) fn insert(
        &mut self,
        kind: u32,
        owner: u64,
        lifetime: ResourceLifetime,
        frame: u64,
    ) -> Result<ResourceToken, ResourceTableError> {
        self.insert_inner(None, kind, owner, lifetime, frame, None)
    }

    /// Allocates a slot storing `value`; the value is retrievable through
    /// `get`/`get_mut`/`take` while the token is live.
    pub(crate) fn insert_value<T: Send + 'static>(
        &mut self,
        value: T,
        kind: u32,
        owner: u64,
        lifetime: ResourceLifetime,
        frame: u64,
    ) -> Result<ResourceToken, ResourceTableError> {
        self.insert_inner(Some(Box::new(value)), kind, owner, lifetime, frame, None)
    }

    /// Like `insert_value`, but links the slot as a child of `parent`; the
    /// child must be removed before the parent can be.
    pub(crate) fn insert_value_child<T: Send + 'static>(
        &mut self,
        value: T,
        kind: u32,
        owner: u64,
        lifetime: ResourceLifetime,
        frame: u64,
        parent: ResourceToken,
    ) -> Result<ResourceToken, ResourceTableError> {
        let _parent_slot = self
            .slots
            .get(parent.index())
            .and_then(|slot| slot.as_ref())
            .filter(|slot| slot.generation == parent.generation())
            .ok_or(ResourceTableError::NotFound)?;
        let parent_index = parent.index();
        let token = self.insert_inner(
            Some(Box::new(value)),
            kind,
            owner,
            lifetime,
            frame,
            Some(parent_index),
        )?;
        self.slots[parent_index]
            .as_mut()
            .expect("parent slot just validated")
            .children
            .push(token.index());
        Ok(token)
    }

    fn insert_inner(
        &mut self,
        value: Option<Box<dyn Any + Send>>,
        kind: u32,
        owner: u64,
        lifetime: ResourceLifetime,
        frame: u64,
        parent: Option<usize>,
    ) -> Result<ResourceToken, ResourceTableError> {
        let index = if let Some(index) = self.free.pop_first() {
            if self.retired.contains(&index) {
                return Err(ResourceTableError::NotFound);
            }
            index
        } else {
            self.slots.push(None);
            self.slots.len() - 1
        };
        if index >= self.generations.len() {
            self.generations.resize(index + 1, 0);
        }
        let generation = self.generations[index]
            .checked_add(1)
            .ok_or(ResourceTableError::NotFound)?;
        if generation > MAX_GENERATION {
            // Generation exhausted: retire the slot so its tokens can never
            // validate again.
            self.retired.insert(index);
            self.free.remove(&index);
            self.slots[index] = None;
            return Err(ResourceTableError::NotFound);
        }
        self.generations[index] = generation;
        self.slots[index] = Some(Slot {
            generation,
            kind,
            owner,
            lifetime,
            frame,
            value,
            children: Vec::new(),
            parent,
        });
        Ok(ResourceToken(
            (generation << TOKEN_GENERATION_BITS) | index as u32,
        ))
    }

    #[cfg(test)]
    /// Validates that `token` is live and matches kind, owner, and (for
    /// HostBorrowed) the current invocation frame. All failures collapse to
    /// `NotFound`, exactly as the guest-facing contract requires.
    pub(crate) fn check(
        &self,
        token: ResourceToken,
        kind: u32,
        owner: u64,
        current_frame: u64,
    ) -> Result<(), ResourceTableError> {
        self.live(token, kind, owner, current_frame).map(|_| ())
    }

    fn live(
        &self,
        token: ResourceToken,
        kind: u32,
        owner: u64,
        current_frame: u64,
    ) -> Result<&Slot, ResourceTableError> {
        let slot = self
            .slots
            .get(token.index())
            .and_then(|slot| slot.as_ref())
            .ok_or(ResourceTableError::NotFound)?;
        if slot.generation != token.generation()
            || slot.kind != kind
            || slot.owner != owner
            || (slot.lifetime == ResourceLifetime::HostBorrowed && slot.frame != current_frame)
        {
            return Err(ResourceTableError::NotFound);
        }
        Ok(slot)
    }

    /// Returns the stored value of a live token, or `NotFound` if the token
    /// is stale/mismatched or holds no value of type `T`.
    pub(crate) fn get<T: 'static>(
        &self,
        token: ResourceToken,
        kind: u32,
        owner: u64,
        current_frame: u64,
    ) -> Result<&T, ResourceTableError> {
        self.live(token, kind, owner, current_frame)?
            .value
            .as_ref()
            .and_then(|value| value.downcast_ref::<T>())
            .ok_or(ResourceTableError::NotFound)
    }

    pub(crate) fn get_mut<T: 'static>(
        &mut self,
        token: ResourceToken,
        kind: u32,
        owner: u64,
        current_frame: u64,
    ) -> Result<&mut T, ResourceTableError> {
        let index = token.index();
        let slot = self
            .slots
            .get_mut(index)
            .and_then(|slot| slot.as_mut())
            .ok_or(ResourceTableError::NotFound)?;
        if slot.generation != token.generation()
            || slot.kind != kind
            || slot.owner != owner
            || (slot.lifetime == ResourceLifetime::HostBorrowed && slot.frame != current_frame)
        {
            return Err(ResourceTableError::NotFound);
        }
        slot.value
            .as_mut()
            .and_then(|value| value.downcast_mut::<T>())
            .ok_or(ResourceTableError::NotFound)
    }

    /// Validated removal without returning a stored value.
    pub(crate) fn remove_checked(
        &mut self,
        token: ResourceToken,
        kind: u32,
        owner: u64,
        current_frame: u64,
    ) -> bool {
        let index = token.index();
        let parent = match self.slots.get(index).and_then(|slot| slot.as_ref()) {
            Some(slot)
                if slot.generation == token.generation()
                    && slot.kind == kind
                    && slot.owner == owner
                    && (slot.lifetime != ResourceLifetime::HostBorrowed
                        || slot.frame == current_frame)
                    && slot.children.is_empty() =>
            {
                slot.parent
            }
            _ => return false,
        };
        self.slots[index] = None;
        self.free.insert(index);
        if let Some(parent_index) = parent
            && let Some(parent_slot) = self.slots.get_mut(parent_index).and_then(|s| s.as_mut())
        {
            parent_slot.children.retain(|child| *child != index);
        }
        true
    }

    /// Generation-only lookup used by cleanup paths that already tracked the
    /// exact reps they inserted.
    pub(crate) fn get_raw<T: 'static>(
        &self,
        token: ResourceToken,
    ) -> Result<&T, ResourceTableError> {
        let slot = self
            .slots
            .get(token.index())
            .and_then(|slot| slot.as_ref())
            .filter(|slot| slot.generation == token.generation())
            .ok_or(ResourceTableError::NotFound)?;
        slot.value
            .as_ref()
            .and_then(|value| value.downcast_ref::<T>())
            .ok_or(ResourceTableError::NotFound)
    }

    /// Generation-only removal used by cleanup paths that already tracked the
    /// exact reps they inserted. Refuses parents with live children.
    pub(crate) fn remove_raw(&mut self, token: ResourceToken) -> bool {
        let index = token.index();
        let parent = match self.slots.get(index).and_then(|slot| slot.as_ref()) {
            Some(slot) if slot.generation == token.generation() && slot.children.is_empty() => {
                slot.parent
            }
            _ => return false,
        };
        self.slots[index] = None;
        self.free.insert(index);
        if let Some(parent_index) = parent
            && let Some(parent_slot) = self.slots.get_mut(parent_index).and_then(|s| s.as_mut())
        {
            parent_slot.children.retain(|child| *child != index);
        }
        true
    }

    #[cfg(test)]
    /// Removes the live slot for `token`. Returns true if a matching slot was
    /// removed; the token becomes permanently invalid.
    pub(crate) fn remove(&mut self, token: ResourceToken) -> bool {
        let Some(slot) = self.slots.get(token.index()).and_then(|slot| slot.as_ref()) else {
            return false;
        };
        if slot.generation != token.generation() {
            return false;
        }
        self.slots[token.index()] = None;
        self.free.insert(token.index());
        true
    }

    #[cfg(test)]
    /// Removes every live slot owned by `owner` (plugin disable/unload
    /// cleanup). Returns the number of slots removed.
    pub(crate) fn clear_owner(&mut self, owner: u64) -> usize {
        let mut removed = 0;
        for (index, slot) in self.slots.iter_mut().enumerate() {
            if slot.as_ref().is_some_and(|slot| slot.owner == owner) {
                *slot = None;
                self.free.insert(index);
                removed += 1;
            }
        }
        removed
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.slots.iter().filter(|slot| slot.is_some()).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const KIND_PLAYER: u32 = 1;
    const KIND_EVENT: u32 = 2;
    const OWNER_A: u64 = 10;
    const OWNER_B: u64 = 20;
    const FRAME_1: u64 = 100;
    const FRAME_2: u64 = 200;

    fn token_parts(token: ResourceToken) -> (u32, usize) {
        (token.generation(), token.index())
    }

    #[test]
    fn insert_and_check_match() {
        let mut table = CoreResourceTable::new();
        let token = table
            .insert(KIND_PLAYER, OWNER_A, ResourceLifetime::PluginOwned, FRAME_1)
            .unwrap();
        assert_eq!(table.len(), 1);
        assert!(table.check(token, KIND_PLAYER, OWNER_A, FRAME_1).is_ok());
        // PluginOwned ignores the frame.
        assert!(table.check(token, KIND_PLAYER, OWNER_A, FRAME_2).is_ok());
    }

    #[test]
    fn wrong_kind_is_not_found() {
        let mut table = CoreResourceTable::new();
        let token = table
            .insert(KIND_PLAYER, OWNER_A, ResourceLifetime::PluginOwned, FRAME_1)
            .unwrap();
        assert_eq!(
            table.check(token, KIND_EVENT, OWNER_A, FRAME_1),
            Err(ResourceTableError::NotFound)
        );
    }

    #[test]
    fn wrong_owner_is_not_found() {
        let mut table = CoreResourceTable::new();
        let token = table
            .insert(KIND_PLAYER, OWNER_A, ResourceLifetime::PluginOwned, FRAME_1)
            .unwrap();
        assert_eq!(
            table.check(token, KIND_PLAYER, OWNER_B, FRAME_1),
            Err(ResourceTableError::NotFound)
        );
    }

    #[test]
    fn host_borrowed_requires_current_frame() {
        let mut table = CoreResourceTable::new();
        let token = table
            .insert(KIND_EVENT, OWNER_A, ResourceLifetime::HostBorrowed, FRAME_1)
            .unwrap();
        assert!(table.check(token, KIND_EVENT, OWNER_A, FRAME_1).is_ok());
        assert_eq!(
            table.check(token, KIND_EVENT, OWNER_A, FRAME_2),
            Err(ResourceTableError::NotFound)
        );
    }

    #[test]
    fn removed_token_is_not_found_and_index_reuses_with_new_generation() {
        let mut table = CoreResourceTable::new();
        let token = table
            .insert(KIND_PLAYER, OWNER_A, ResourceLifetime::PluginOwned, FRAME_1)
            .unwrap();
        let (generation, index) = token_parts(token);
        assert!(table.remove(token));
        assert_eq!(
            table.check(token, KIND_PLAYER, OWNER_A, FRAME_1),
            Err(ResourceTableError::NotFound)
        );

        // The slot index is reused but the generation advances, so the old
        // token stays invalid.
        let fresh = table
            .insert(KIND_PLAYER, OWNER_A, ResourceLifetime::PluginOwned, FRAME_1)
            .unwrap();
        let (fresh_generation, fresh_index) = token_parts(fresh);
        assert_eq!(fresh_index, index);
        assert_eq!(fresh_generation, generation + 1);
        assert_eq!(
            table.check(token, KIND_PLAYER, OWNER_A, FRAME_1),
            Err(ResourceTableError::NotFound)
        );
        assert!(table.check(fresh, KIND_PLAYER, OWNER_A, FRAME_1).is_ok());
    }

    #[test]
    fn clear_owner_removes_only_that_owner() {
        let mut table = CoreResourceTable::new();
        let a = table
            .insert(KIND_PLAYER, OWNER_A, ResourceLifetime::PluginOwned, FRAME_1)
            .unwrap();
        let b = table
            .insert(KIND_EVENT, OWNER_B, ResourceLifetime::HostBorrowed, FRAME_1)
            .unwrap();
        assert_eq!(table.clear_owner(OWNER_A), 1);
        assert_eq!(table.len(), 1);
        assert!(table.check(b, KIND_EVENT, OWNER_B, FRAME_1).is_ok());
        assert_eq!(
            table.check(a, KIND_PLAYER, OWNER_A, FRAME_1),
            Err(ResourceTableError::NotFound)
        );
    }

    #[test]
    fn empty_and_double_remove() {
        let mut table = CoreResourceTable::new();
        let token = table
            .insert(KIND_PLAYER, OWNER_A, ResourceLifetime::PluginOwned, FRAME_1)
            .unwrap();
        assert!(table.remove(token));
        assert!(!table.remove(token));
        assert_eq!(table.len(), 0);
    }
}
