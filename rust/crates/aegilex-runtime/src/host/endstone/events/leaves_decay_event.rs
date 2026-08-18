//! Core ABI implementation for `native/bindings/endstone/events/leaves_decay_event_facade.h`.

use super::support::*;



fn resolve_leaves_decay_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::LeavesDecayEventFacade, HostError> {
    let handle = event_handle(state, event, ResourceKind::LeavesDecayEvent)?;
    state
        .handles
        .leaves_decay_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_leaves_decay_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::LeavesDecayEventFacade>, HostError> {
    let handle = event_handle(state, event, ResourceKind::LeavesDecayEvent)?;
    let invocation_id = state.invocation_id;
    state
        .handles
        .leaves_decay_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl crate::core_host::imports::HostLeavesDecayEvent for PluginStoreState {
    fn leaves_decay_event_get_block(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "leaves-decay-event.leaves-decay-event.get-block")?;
            let block = resolve_leaves_decay_event(self, self_)
                .and_then(|event| {
                    let block = event.getBlock();
                    (!block.is_null())
                        .then_some(block)
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            self.insert_block_resource(block)
                .map_err(map_core_host_error)
        })())
    }

    fn leaves_decay_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "leaves-decay-event.leaves-decay-event.is-cancelled")?;
            resolve_leaves_decay_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn leaves_decay_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "leaves-decay-event.leaves-decay-event.set-cancelled")?;
            resolve_leaves_decay_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setCancelled(cancelled)
                        .then_some(())
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }
}
