//! Shared Core resource types and insertion helpers.
//!
//! Item metadata and NBT values share canonical identities across host
//! interfaces.

use std::collections::BTreeMap;

use crate::abi::{AEGILEX_INVALID_ARGUMENT, AEGILEX_LIMIT_EXCEEDED, AEGILEX_NOT_FOUND};
use crate::core_host::{ItemMetaItemMetaType, NbtTagType};
use crate::cxx_host_inventory::ffi as cxx_inventory;
use crate::host::endstone::inventory::resources::item_stack_resource_handle;
use crate::host::runtime::native::HostError;
use crate::runtime::PluginStoreState;

// Flattened endstone NBT node kind constants (see cxx_host_inventory::ffi).
pub(crate) const NBT_END: u8 = 0;
pub(crate) const NBT_BYTE: u8 = 1;
pub(crate) const NBT_SHORT: u8 = 2;
pub(crate) const NBT_INT: u8 = 3;
pub(crate) const NBT_LONG: u8 = 4;
pub(crate) const NBT_FLOAT: u8 = 5;
pub(crate) const NBT_DOUBLE: u8 = 6;
pub(crate) const NBT_BYTE_ARRAY: u8 = 7;
pub(crate) const NBT_STRING: u8 = 8;
pub(crate) const NBT_LIST: u8 = 9;
pub(crate) const NBT_COMPOUND: u8 = 10;
pub(crate) const NBT_INT_ARRAY: u8 = 11;

// Flattened endstone::ItemMeta kind constants (see cxx_host_inventory::ffi).
pub(crate) const ITEM_META_KIND_ITEM: u8 = 0;
pub(crate) const ITEM_META_KIND_BOOK: u8 = 1;
pub(crate) const ITEM_META_KIND_CROSSBOW: u8 = 2;
pub(crate) const ITEM_META_KIND_MAP: u8 = 3;
pub(crate) const ITEM_META_KIND_WRITABLE_BOOK: u8 = 4;

pub(crate) struct ItemMetaValue {
    pub(crate) meta: cxx_inventory::ItemMeta,
    pub(crate) projectiles: cxx::UniquePtr<cxx_inventory::ItemStackCollection>,
}

// Plugin stores are configured as non-Send and all CXX access remains on the
// invoking server thread. ResourceTable nevertheless requires Send entries.
unsafe impl Send for ItemMetaValue {}

pub(crate) type ItemMetaHandle = ItemMetaValue;

pub(crate) fn cxx_meta_defaults() -> cxx_inventory::ItemMeta {
    cxx_inventory::ItemMeta {
        kind: ITEM_META_KIND_ITEM,
        base: cxx_inventory::ItemMetaBase {
            has_display_name: false,
            display_name: String::new(),
            has_lore: false,
            lore: Vec::new(),
            enchants: Vec::new(),
            unbreakable: false,
            has_damage: false,
            damage: 0,
            has_repair_cost: false,
            repair_cost: 0,
        },
        pages: Vec::new(),
        has_title: false,
        title: String::new(),
        has_author: false,
        author: String::new(),
        has_generation: false,
        generation: 0,
        has_map_id: false,
        map_id: 0,
        has_map_view: false,
    }
}

pub(crate) fn insert_item_meta_resource(
    state: &mut PluginStoreState,
    meta: cxx_inventory::ItemMeta,
    projectiles: cxx::UniquePtr<cxx_inventory::ItemStackCollection>,
) -> Result<u32, HostError> {
    state.insert_owned_resource(ItemMetaValue { meta, projectiles })
}

pub(crate) fn item_meta_value(
    state: &PluginStoreState,
    rep: u32,
) -> Result<&ItemMetaHandle, HostError> {
    state.owned_resource::<ItemMetaHandle>(rep)
}

pub(crate) fn item_meta_type(meta: &cxx_inventory::ItemMeta) -> ItemMetaItemMetaType {
    match meta.kind {
        ITEM_META_KIND_BOOK => ItemMetaItemMetaType::Book,
        ITEM_META_KIND_CROSSBOW => ItemMetaItemMetaType::Crossbow,
        ITEM_META_KIND_MAP => ItemMetaItemMetaType::MapValue,
        ITEM_META_KIND_WRITABLE_BOOK => ItemMetaItemMetaType::WritableBook,
        _ => ItemMetaItemMetaType::Item,
    }
}

pub(crate) fn copy_item_meta(meta: &ItemMetaValue) -> Result<ItemMetaValue, HostError> {
    Ok(ItemMetaValue {
        meta: cxx_inventory::ItemMeta {
            kind: meta.meta.kind,
            base: cxx_inventory::ItemMetaBase {
                has_display_name: meta.meta.base.has_display_name,
                display_name: meta.meta.base.display_name.clone(),
                has_lore: meta.meta.base.has_lore,
                lore: meta.meta.base.lore.clone(),
                enchants: meta
                    .meta
                    .base
                    .enchants
                    .iter()
                    .map(|enchant| cxx_inventory::Enchantment {
                        type_id: enchant.type_id.clone(),
                        level: enchant.level,
                    })
                    .collect(),
                unbreakable: meta.meta.base.unbreakable,
                has_damage: meta.meta.base.has_damage,
                damage: meta.meta.base.damage,
                has_repair_cost: meta.meta.base.has_repair_cost,
                repair_cost: meta.meta.base.repair_cost,
            },
            pages: meta.meta.pages.clone(),
            has_title: meta.meta.has_title,
            title: meta.meta.title.clone(),
            has_author: meta.meta.has_author,
            author: meta.meta.author.clone(),
            has_generation: meta.meta.has_generation,
            generation: meta.meta.generation,
            has_map_id: meta.meta.has_map_id,
            map_id: meta.meta.map_id,
            has_map_view: meta.meta.has_map_view,
        },
        projectiles: copy_projectiles(&meta.projectiles)?,
    })
}

pub(crate) fn clone_item_meta_resource(
    state: &mut PluginStoreState,
    rep: u32,
) -> Result<u32, HostError> {
    let value = copy_item_meta(item_meta_value(state, rep)?)?;
    state.insert_owned_resource(value)
}

pub(crate) fn require_item_meta_kind(
    state: &PluginStoreState,
    rep: u32,
    kind: u8,
) -> Result<(), HostError> {
    (item_meta_value(state, rep)?.meta.kind == kind)
        .then_some(())
        .ok_or_else(not_found)
}

pub(crate) fn projectiles_from_resources(
    state: &PluginStoreState,
    projectiles: &[u32],
) -> Result<cxx::UniquePtr<cxx_inventory::ItemStackCollection>, HostError> {
    let mut collection = cxx_inventory::ItemStackCollection::create_item_stack_collection();
    if collection.is_null() {
        return Err(not_found());
    }
    for projectile in projectiles {
        let handle = item_stack_resource_handle(state, *projectile)?;
        let projectile = state
            .handles
            .item_stack(state.invocation_id, handle)
            .ok_or_else(not_found)?;
        collection.pin_mut().push(projectile);
    }
    Ok(collection)
}

pub(crate) fn copy_projectiles(
    projectiles: &cxx_inventory::ItemStackCollection,
) -> Result<cxx::UniquePtr<cxx_inventory::ItemStackCollection>, HostError> {
    let mut copy = cxx_inventory::ItemStackCollection::create_item_stack_collection();
    if copy.is_null() {
        return Err(not_found());
    }
    for index in 0..projectiles.len() {
        let projectile = projectiles.get(index);
        if projectile.is_null() {
            return Err(not_found());
        }
        copy.pin_mut().push(&projectile);
    }
    Ok(copy)
}

#[derive(Clone)]
pub(crate) enum NbtTag {
    End,
    Byte(u8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    StringValue(String),
    List(Vec<NbtTag>),
    Compound(BTreeMap<String, NbtTag>),
    IntArray(Vec<i32>),
}

pub(crate) fn not_found() -> HostError {
    HostError::from_status(AEGILEX_NOT_FOUND)
}

pub(crate) fn invalid_input() -> HostError {
    HostError::from_status(AEGILEX_INVALID_ARGUMENT)
}

pub(crate) fn limit_exceeded() -> HostError {
    HostError::from_status(AEGILEX_LIMIT_EXCEEDED)
}

pub(crate) fn exceeds_limit(value: usize, limit: u64) -> bool {
    limit != 0 && value > limit as usize
}

pub(crate) fn nbt_tag_kind(tag: &NbtTag) -> u8 {
    match tag {
        NbtTag::End => NBT_END,
        NbtTag::Byte(_) => NBT_BYTE,
        NbtTag::Short(_) => NBT_SHORT,
        NbtTag::Int(_) => NBT_INT,
        NbtTag::Long(_) => NBT_LONG,
        NbtTag::Float(_) => NBT_FLOAT,
        NbtTag::Double(_) => NBT_DOUBLE,
        NbtTag::ByteArray(_) => NBT_BYTE_ARRAY,
        NbtTag::StringValue(_) => NBT_STRING,
        NbtTag::List(_) => NBT_LIST,
        NbtTag::Compound(_) => NBT_COMPOUND,
        NbtTag::IntArray(_) => NBT_INT_ARRAY,
    }
}

pub(crate) fn nbt_tag_type(tag: &NbtTag) -> NbtTagType {
    match tag {
        NbtTag::End => NbtTagType::End,
        NbtTag::Byte(_) => NbtTagType::Byte,
        NbtTag::Short(_) => NbtTagType::Short,
        NbtTag::Int(_) => NbtTagType::Int,
        NbtTag::Long(_) => NbtTagType::Long,
        NbtTag::Float(_) => NbtTagType::Float,
        NbtTag::Double(_) => NbtTagType::Double,
        NbtTag::ByteArray(_) => NbtTagType::ByteArray,
        NbtTag::StringValue(_) => NbtTagType::StringValue,
        NbtTag::List(_) => NbtTagType::ListValue,
        NbtTag::Compound(_) => NbtTagType::Compound,
        NbtTag::IntArray(_) => NbtTagType::IntArray,
    }
}

pub(crate) fn validate_nbt_tag(
    tag: &NbtTag,
    depth: usize,
    nodes: &mut usize,
    state: &PluginStoreState,
) -> Result<(), HostError> {
    if exceeds_limit(depth, state.config.max_nbt_depth) {
        return Err(limit_exceeded());
    }
    *nodes += 1;
    if exceeds_limit(*nodes, state.config.max_nbt_nodes) {
        return Err(limit_exceeded());
    }
    match tag {
        NbtTag::ByteArray(values)
            if exceeds_limit(values.len(), state.config.max_nbt_array_bytes) =>
        {
            Err(limit_exceeded())
        }
        NbtTag::IntArray(values)
            if exceeds_limit(
                values.len().saturating_mul(std::mem::size_of::<i32>()),
                state.config.max_nbt_array_bytes,
            ) =>
        {
            Err(limit_exceeded())
        }
        NbtTag::StringValue(value)
            if exceeds_limit(value.len(), state.config.max_nbt_string_bytes) =>
        {
            Err(limit_exceeded())
        }
        NbtTag::List(values) => {
            let element_kind = values.first().map(nbt_tag_kind);
            if element_kind == Some(NBT_END)
                || values
                    .iter()
                    .any(|value| Some(nbt_tag_kind(value)) != element_kind)
            {
                return Err(invalid_input());
            }
            for value in values {
                validate_nbt_tag(value, depth + 1, nodes, state)?;
            }
            Ok(())
        }
        NbtTag::Compound(entries) => {
            if exceeds_limit(entries.len(), state.config.max_nbt_compound_entries) {
                return Err(limit_exceeded());
            }
            for (key, value) in entries {
                if exceeds_limit(key.len(), state.config.max_nbt_string_bytes) {
                    return Err(limit_exceeded());
                }
                validate_nbt_tag(value, depth + 1, nodes, state)?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

pub(crate) fn validate_nbt_tag_root(
    tag: &NbtTag,
    state: &PluginStoreState,
) -> Result<(), HostError> {
    validate_nbt_tag(tag, 0, &mut 0, state)
}

pub(crate) fn insert_nbt_tag_resource(
    state: &mut PluginStoreState,
    value: NbtTag,
) -> Result<u32, HostError> {
    validate_nbt_tag_root(&value, state)?;
    state.insert_owned_resource(value)
}

pub(crate) fn nbt_tag_value(state: &PluginStoreState, rep: u32) -> Result<&NbtTag, HostError> {
    state.owned_resource::<NbtTag>(rep)
}

pub(crate) fn nbt_tag_value_mut(
    state: &mut PluginStoreState,
    rep: u32,
) -> Result<&mut NbtTag, HostError> {
    state.owned_resource_mut::<NbtTag>(rep)
}

pub(crate) fn replace_nbt_tag_value(
    state: &mut PluginStoreState,
    rep: u32,
    value: NbtTag,
) -> Result<(), HostError> {
    validate_nbt_tag_root(&value, state)?;
    *nbt_tag_value_mut(state, rep)? = value;
    Ok(())
}

// Container setters copy the borrowed child so a child resource can never
// reference its own parent, which would otherwise allow a cycle.
pub(crate) fn clone_borrowed_nbt_tag(
    state: &PluginStoreState,
    tag: u32,
    child: u32,
) -> Result<NbtTag, HostError> {
    if tag == child {
        return Err(invalid_input());
    }
    nbt_tag_value(state, child).cloned()
}

pub(crate) fn nbt_list_set(
    state: &mut PluginStoreState,
    tag: u32,
    index: i32,
    child: u32,
) -> Result<(), HostError> {
    let child = clone_borrowed_nbt_tag(state, tag, child)?;
    if matches!(child, NbtTag::End) {
        return Err(invalid_input());
    }
    let index = usize::try_from(index).map_err(|_| invalid_input())?;
    {
        let list = nbt_tag_value_mut(state, tag)?;
        let NbtTag::List(values) = list else {
            return Err(invalid_input());
        };
        if index > values.len() {
            return Err(invalid_input());
        }
        let element_kind = values.first().map(nbt_tag_kind);
        if index == values.len() {
            if element_kind.is_some_and(|kind| kind != nbt_tag_kind(&child)) {
                return Err(invalid_input());
            }
            values.push(child);
        } else {
            if element_kind != Some(nbt_tag_kind(&child)) {
                return Err(invalid_input());
            }
            values[index] = child;
        }
    }
    validate_nbt_tag_root(nbt_tag_value(state, tag)?, state)?;
    Ok(())
}

pub(crate) fn nbt_list_remove(
    state: &mut PluginStoreState,
    tag: u32,
    index: i32,
) -> Result<(), HostError> {
    let index = usize::try_from(index).map_err(|_| invalid_input())?;
    let list = nbt_tag_value_mut(state, tag)?;
    let NbtTag::List(values) = list else {
        return Err(invalid_input());
    };
    if index >= values.len() {
        return Err(invalid_input());
    }
    values.remove(index);
    Ok(())
}

pub(crate) fn nbt_list_clear(state: &mut PluginStoreState, tag: u32) -> Result<(), HostError> {
    let list = nbt_tag_value_mut(state, tag)?;
    let NbtTag::List(values) = list else {
        return Err(invalid_input());
    };
    values.clear();
    Ok(())
}

pub(crate) fn nbt_compound_set(
    state: &mut PluginStoreState,
    tag: u32,
    key: String,
    child: u32,
) -> Result<(), HostError> {
    let max_compound_entries = state.config.max_nbt_compound_entries;
    if exceeds_limit(key.len(), state.config.max_nbt_string_bytes) {
        return Err(limit_exceeded());
    }
    let child = clone_borrowed_nbt_tag(state, tag, child)?;
    {
        let compound = nbt_tag_value_mut(state, tag)?;
        let NbtTag::Compound(entries) = compound else {
            return Err(invalid_input());
        };
        if !entries.contains_key(&key)
            && max_compound_entries != 0
            && entries.len() >= max_compound_entries as usize
        {
            return Err(limit_exceeded());
        }
        entries.insert(key, child);
    }
    validate_nbt_tag_root(nbt_tag_value(state, tag)?, state)?;
    Ok(())
}

pub(crate) fn nbt_compound_remove(
    state: &mut PluginStoreState,
    tag: u32,
    key: String,
) -> Result<bool, HostError> {
    let compound = nbt_tag_value_mut(state, tag)?;
    let NbtTag::Compound(entries) = compound else {
        return Err(invalid_input());
    };
    Ok(entries.remove(&key).is_some())
}

pub(crate) fn nbt_compound_clear(state: &mut PluginStoreState, tag: u32) -> Result<(), HostError> {
    let compound = nbt_tag_value_mut(state, tag)?;
    let NbtTag::Compound(entries) = compound else {
        return Err(invalid_input());
    };
    entries.clear();
    Ok(())
}
