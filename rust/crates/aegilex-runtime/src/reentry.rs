//! Same-thread reentrant guest dispatch support.
//!
//! A generated host-import trampoline holds a `wasmtime::Caller` while it runs
//! native code. If that native code synchronously fires an Endstone callback
//! (event, command, task, form, map, service), the C++ bridge may re-enter the
//! runtime on the same thread *before* the outer import returns. The outer
//! trampoline therefore records the live `Caller` in a stack-only scope so the
//! nested dispatch can invoke the same guest instance without re-borrowing its
//! `Store` through the runtime table (which the outer call still owns).
//!
//! Safety proof:
//! 1. every entry is installed by `with_guard` around a live host import;
//! 2. native code can re-enter only synchronously before that import returns;
//! 3. the raw pointer is restored only while its guard is live and only after
//!    its exact store-state `TypeId` matches the requested caller type; and
//! 4. no C++ object or global retains the pointer after the guard is dropped.

use std::any::TypeId;
use std::cell::RefCell;
use wasmtime::{AsContext, AsContextMut};

#[derive(Clone, Copy)]
struct ActiveCaller {
    raw: *mut (),
    state_type: TypeId,
}

thread_local! {
    static ACTIVE_CALLER: RefCell<Vec<ActiveCaller>> = const { RefCell::new(Vec::new()) };
}

/// RAII guard installed by every generated host-import trampoline. It records
/// the live `Caller` so a synchronous nested callback can re-enter the same
/// guest instance.
struct HostImportGuard(ActiveCaller);

impl HostImportGuard {
    fn enter(raw: *mut (), state_type: TypeId) -> Self {
        let active = ActiveCaller { raw, state_type };
        ACTIVE_CALLER.with(|callers| callers.borrow_mut().push(active));
        Self(active)
    }
}

impl Drop for HostImportGuard {
    fn drop(&mut self) {
        ACTIVE_CALLER.with(|callers| {
            let mut callers = callers.borrow_mut();
            if callers.last().is_some_and(|active| {
                active.raw == self.0.raw && active.state_type == self.0.state_type
            }) {
                callers.pop();
            }
        });
    }
}

/// Installs a stack-only reentry scope around a generated host import.
pub(crate) fn with_guard<T: 'static, R>(
    caller: &mut wasmtime::Caller<'_, T>,
    call: impl FnOnce(&mut wasmtime::Caller<'_, T>) -> R,
) -> R {
    let _guard = HostImportGuard::enter(caller as *mut _ as *mut (), TypeId::of::<T>());
    call(caller)
}

#[cfg(test)]
/// Re-derives the active caller of type `T` for the ABI probe's synchronous
/// reentry assertion. Only valid while a matching host-import scope is live.
pub(crate) unsafe fn with_active_caller<T: 'static, R>(
    call: impl FnOnce(&mut wasmtime::Caller<'_, T>) -> R,
) -> Option<R> {
    let active = ACTIVE_CALLER.with(|callers| callers.borrow().last().copied())?;
    if active.state_type != TypeId::of::<T>() {
        return None;
    }
    // SAFETY: `with_guard` installs `raw` from a live `Caller<T>` and removes
    // it before that caller can be dropped. The `TypeId` check above prevents
    // restoring the pointer as any other store-state type.
    let caller = unsafe { &mut *active.raw.cast::<wasmtime::Caller<'_, T>>() };
    Some(call(caller))
}

/// Executes a nested callback inside a fresh invocation frame. The active
/// store keeps its remaining fuel: only a root dispatch may reset it.
///
/// The guard is intentionally constructed before guest work and dropped after
/// it, so every exit path—including a canonical ABI trap—clears only the
/// nested frame and restores the exact outer frame.
struct InvocationFrameGuard<'caller, 'store> {
    caller: &'caller mut wasmtime::Caller<'store, crate::runtime::PluginStoreState>,
    invocation_id: u64,
}

impl<'caller, 'store> InvocationFrameGuard<'caller, 'store> {
    fn enter(
        caller: &'caller mut wasmtime::Caller<'store, crate::runtime::PluginStoreState>,
        invocation_id: u64,
    ) -> Result<Self, u32> {
        let depth = caller.as_context().data().invocation_frames.len() as u64;
        let limit = caller.as_context().data().config.max_nested_dispatch_depth;
        if limit != 0 && depth >= limit {
            return Err(crate::abi::AEGILEX_LIMIT_EXCEEDED);
        }
        caller
            .as_context_mut()
            .data_mut()
            .push_invocation(invocation_id);
        Ok(Self {
            caller,
            invocation_id,
        })
    }

    fn caller(&mut self) -> &mut wasmtime::Caller<'store, crate::runtime::PluginStoreState> {
        self.caller
    }
}

impl Drop for InvocationFrameGuard<'_, '_> {
    fn drop(&mut self) {
        let mut context = self.caller.as_context_mut();
        let state = context.data_mut();
        state.clear_invocation_resources(self.invocation_id);
        state.pop_invocation();
    }
}

/// Dispatches an event to the currently-executing guest through the live
/// `Caller` (nested synchronous callback). Returns `false` when the plugin is
/// not subscribed, the instance is unavailable, or the guest rejects/traps.
pub(crate) fn nested_dispatch_event(
    caller: &mut wasmtime::Caller<'_, crate::runtime::PluginStoreState>,
    invocation_id: u64,
    subscription: &str,
    insert: impl FnOnce(
        &mut crate::runtime::PluginStoreState,
    ) -> Result<u32, crate::host::runtime::native::HostError>,
    make_event: impl FnOnce(u32) -> crate::core_host::EventsEvent,
) -> bool {
    if !caller
        .as_context()
        .data()
        .subscriptions
        .iter()
        .any(|item| item == subscription)
    {
        return false;
    }
    let instance = match caller.as_context().data().instance {
        Some(instance) => instance,
        None => return false,
    };
    let mut frame = match InvocationFrameGuard::enter(caller, invocation_id) {
        Ok(frame) => frame,
        Err(_) => return false,
    };
    let event_rep = match insert(frame.caller().as_context_mut().data_mut()) {
        Ok(rep) => rep,
        Err(_) => return false,
    };
    let exports = match crate::core_host::CoreExports::new(&instance, &mut *frame.caller()) {
        Ok(exports) => exports,
        Err(_) => return false,
    };
    match exports.call_events_on_event_with_caller(&instance, frame.caller(), make_event(event_rep))
    {
        Ok(Ok(())) => true,
        Ok(Err(_)) | Err(_) => false,
    }
}

/// Routes a nested callback to any currently executing guest instance with the
/// requested plugin identity. The matching scope may be below another active
/// plugin (for example `A -> host -> B -> host -> A`), so lookup walks the
/// stack from newest to oldest instead of consulting only its top entry.
pub(crate) fn route_nested<F, R>(target_plugin: &str, route: F) -> Option<R>
where
    F: FnOnce(&mut wasmtime::Caller<'_, crate::runtime::PluginStoreState>) -> R,
{
    let candidates = ACTIVE_CALLER.with(|callers| {
        callers
            .borrow()
            .iter()
            .rev()
            .filter(|active| active.state_type == TypeId::of::<crate::runtime::PluginStoreState>())
            .map(|active| active.raw)
            .collect::<Vec<_>>()
    });

    for raw in candidates {
        // SAFETY: candidates are copied only while their RAII guards are live;
        // route_nested is called synchronously from native code before the
        // corresponding generated host import returns. The TypeId filter
        // ensures this pointer is re-derived as the original store state type.
        let caller =
            unsafe { &mut *raw.cast::<wasmtime::Caller<'_, crate::runtime::PluginStoreState>>() };
        if caller.data().plugin_id == target_plugin {
            return Some(route(caller));
        }
    }
    None
}
/// Dispatches a task through an active plugin's live caller. A task callback
/// has no borrowed native facade, but it still owns an invocation frame so
/// resource cleanup precisely restores the outer callback state.
pub(crate) fn nested_dispatch_task(
    caller: &mut wasmtime::Caller<'_, crate::runtime::PluginStoreState>,
    invocation_id: u64,
    task_id: u64,
) -> Result<(), u32> {
    let instance = caller
        .as_context()
        .data()
        .instance
        .ok_or(crate::abi::AEGILEX_NOT_FOUND)?;
    let mut frame = InvocationFrameGuard::enter(caller, invocation_id)?;
    let outcome = crate::core_host::CoreExports::new(&instance, &mut *frame.caller())
        .map_err(|_| crate::abi::AEGILEX_INTERNAL_ERROR)
        .and_then(|exports| {
            exports
                .call_tasks_on_task_with_caller(&instance, frame.caller(), task_id)
                .map_err(|_| crate::abi::AEGILEX_TRAP)
        });
    match outcome {
        Ok(Ok(())) | Ok(Err(_)) => Ok(()),
        Err(status) => Err(status),
    }
}

/// Dispatches a form submission through an active plugin's live caller. The
/// caller owns the form resource; the player facade is created inside this
/// fresh invocation frame and is always cleared before the outer host import
/// resumes.
pub(crate) fn nested_dispatch_form_submit(
    caller: &mut wasmtime::Caller<'_, crate::runtime::PluginStoreState>,
    invocation_id: u64,
    form_id: u32,
    has_player: bool,
    player_uuid: &[u8],
    response: crate::cxx_runtime::ffi::FormResponseData,
) -> bool {
    if !caller.as_context().data().forms.contains_key(&form_id) {
        return false;
    }
    let instance = match caller.as_context().data().instance {
        Some(instance) => instance,
        None => return false,
    };
    let mut frame = match InvocationFrameGuard::enter(caller, invocation_id) {
        Ok(frame) => frame,
        Err(_) => return false,
    };
    let outcome = (|| {
        let player = crate::runtime::resolve_player_for_caller_dispatch(
            frame.caller(),
            has_player,
            player_uuid,
        )
        .ok_or_else(|| anyhow::anyhow!("form player is not online"))?;
        let response = match response.kind {
            crate::host::runtime::forms::FORM_ACTION => {
                crate::core_host::FormCallbacksFormResponse::Action(
                    crate::core_host::ActionFormActionResponse {
                        selected_index: response.selected_index,
                    },
                )
            }
            crate::host::runtime::forms::FORM_MESSAGE => {
                crate::core_host::FormCallbacksFormResponse::Message(
                    match response.message_button {
                        1 => crate::core_host::MessageFormMessageResponse::Button2,
                        _ => crate::core_host::MessageFormMessageResponse::Button1,
                    },
                )
            }
            crate::host::runtime::forms::FORM_MODAL => {
                crate::core_host::FormCallbacksFormResponse::Modal(
                    crate::core_host::ModalFormModalResponse {
                        json: response.modal_json,
                    },
                )
            }
            _ => return Err(anyhow::anyhow!("unknown form kind")),
        };
        let exports = crate::core_host::CoreExports::new(&instance, &mut *frame.caller())
            .map_err(anyhow::Error::msg)?;
        exports.call_formcallbacks_on_form_submit_with_caller(
            &instance,
            frame.caller(),
            form_id,
            player,
            response,
        )
    })();
    matches!(outcome, Ok(Ok(())))
}

/// Dispatches a form close through an active plugin's live caller.
pub(crate) fn nested_dispatch_form_close(
    caller: &mut wasmtime::Caller<'_, crate::runtime::PluginStoreState>,
    invocation_id: u64,
    form_id: u32,
    has_player: bool,
    player_uuid: &[u8],
) -> bool {
    if !caller.as_context().data().forms.contains_key(&form_id) {
        return false;
    }
    let instance = match caller.as_context().data().instance {
        Some(instance) => instance,
        None => return false,
    };
    let mut frame = match InvocationFrameGuard::enter(caller, invocation_id) {
        Ok(frame) => frame,
        Err(_) => return false,
    };
    let outcome = (|| {
        let player = crate::runtime::resolve_player_for_caller_dispatch(
            frame.caller(),
            has_player,
            player_uuid,
        )
        .ok_or_else(|| anyhow::anyhow!("form player is not online"))?;
        let exports = crate::core_host::CoreExports::new(&instance, &mut *frame.caller())
            .map_err(anyhow::Error::msg)?;
        exports.call_formcallbacks_on_form_close_with_caller(
            &instance,
            frame.caller(),
            form_id,
            player,
        )
    })();
    matches!(outcome, Ok(Ok(())))
}

/// Dispatches map initialization through an active plugin's live caller.
pub(crate) fn nested_dispatch_map_initialize(
    caller: &mut wasmtime::Caller<'_, crate::runtime::PluginStoreState>,
    invocation_id: u64,
    renderer: u32,
    map_id: i64,
) -> bool {
    if !caller
        .as_context()
        .data()
        .map_renderers
        .contains_key(&renderer)
    {
        return false;
    }
    let instance = match caller.as_context().data().instance {
        Some(instance) => instance,
        None => return false,
    };
    let mut frame = match InvocationFrameGuard::enter(caller, invocation_id) {
        Ok(frame) => frame,
        Err(_) => return false,
    };
    let outcome = crate::core_host::CoreExports::new(&instance, &mut *frame.caller())
        .map_err(anyhow::Error::msg)
        .and_then(|exports| {
            exports.call_maprenderercallbacks_on_map_initialize_with_caller(
                &instance,
                frame.caller(),
                renderer,
                map_id,
            )
        });
    matches!(outcome, Ok(Ok(())))
}

/// Dispatches map rendering through an active plugin's live caller. Returned
/// commands are owned Rust values; no guest-memory reference crosses back into
/// native code.
pub(crate) fn nested_dispatch_map_render(
    caller: &mut wasmtime::Caller<'_, crate::runtime::PluginStoreState>,
    invocation_id: u64,
    renderer: u32,
    map_id: i64,
    has_player: bool,
    player_uuid: &[u8],
) -> Vec<crate::core_host::MapCanvasMapDrawCommand> {
    if !caller
        .as_context()
        .data()
        .map_renderers
        .contains_key(&renderer)
    {
        return Vec::new();
    }
    let instance = match caller.as_context().data().instance {
        Some(instance) => instance,
        None => return Vec::new(),
    };
    let mut frame = match InvocationFrameGuard::enter(caller, invocation_id) {
        Ok(frame) => frame,
        Err(_) => return Vec::new(),
    };
    let outcome = (|| {
        let player = crate::runtime::resolve_player_for_caller_dispatch(
            frame.caller(),
            has_player,
            player_uuid,
        )
        .ok_or_else(|| anyhow::anyhow!("map player is not online"))?;
        let exports = crate::core_host::CoreExports::new(&instance, &mut *frame.caller())
            .map_err(anyhow::Error::msg)?;
        exports.call_maprenderercallbacks_on_map_render_with_caller(
            &instance,
            frame.caller(),
            renderer,
            map_id,
            player,
        )
    })();
    match outcome {
        Ok(Ok(commands)) => commands,
        Ok(Err(_)) | Err(_) => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guard_stack_tracks_nested_scopes() {
        assert!(ACTIVE_CALLER.with(|callers| callers.borrow().is_empty()));

        let mut outer = 0u32;
        let outer_raw = &mut outer as *mut u32 as *mut ();
        let outer = HostImportGuard::enter(outer_raw, TypeId::of::<u32>());
        assert_eq!(ACTIVE_CALLER.with(|callers| callers.borrow().len()), 1);

        let mut inner = 5u64;
        let inner_raw = &mut inner as *mut u64 as *mut ();
        {
            let _inner = HostImportGuard::enter(inner_raw, TypeId::of::<u64>());
            assert_eq!(ACTIVE_CALLER.with(|callers| callers.borrow().len()), 2);
            assert_eq!(
                ACTIVE_CALLER
                    .with(|callers| callers.borrow().last().map(|active| active.state_type)),
                Some(TypeId::of::<u64>())
            );
        }

        assert_eq!(ACTIVE_CALLER.with(|callers| callers.borrow().len()), 1);
        drop(outer);
        assert!(ACTIVE_CALLER.with(|callers| callers.borrow().is_empty()));
    }
}
