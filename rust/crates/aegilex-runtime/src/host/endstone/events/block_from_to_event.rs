//! Core ABI implementation for `native/bindings/endstone/events/block_from_to_event_facade.h`.

use super::support::*;



fn resolve_block_from_to_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::BlockFromToEventFacade, HostError> {
    let handle = event_handle(state, event, ResourceKind::BlockFromToEvent)?;
    state
        .handles
        .block_from_to_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_block_from_to_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::BlockFromToEventFacade>, HostError> {
    let handle = event_handle(state, event, ResourceKind::BlockFromToEvent)?;
    let invocation_id = state.invocation_id;
    state
        .handles
        .block_from_to_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl crate::core_host::imports::HostBlockFromToEvent for PluginStoreState {
    fn block_from_to_event_get_block(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-from-to-event.block-from-to-event.get-block")?;
            let block = resolve_block_from_to_event(self, self_)
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

    fn block_from_to_event_get_to_block(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-from-to-event.block-from-to-event.get-to-block")?;
            let block = resolve_block_from_to_event(self, self_)
                .and_then(|event| {
                    let block = event.getToBlock();
                    (!block.is_null())
                        .then_some(block)
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            self.insert_block_resource(block)
                .map_err(map_core_host_error)
        })())
    }

    fn block_from_to_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-from-to-event.block-from-to-event.is-cancelled")?;
            resolve_block_from_to_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn block_from_to_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "block-from-to-event.block-from-to-event.set-cancelled",
            )?;
            resolve_block_from_to_event_mut(self, self_)
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
