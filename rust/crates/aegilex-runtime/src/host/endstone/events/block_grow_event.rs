//! Core ABI implementation for `native/bindings/endstone/events/block_grow_event_facade.h`.

use super::support::*;

fn resolve_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::BlockGrowEventFacade>, HostError> {
    let handle = state
        .resource_slot(event, ResourceKind::BlockGrowEvent)
        .map_err(|_| HostError::from_status(AEGILEX_NOT_FOUND))?
        .handle;
    let invocation_id = state.invocation_id;
    state
        .handles
        .block_grow_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_block_grow_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::BlockGrowEventFacade, HostError> {
    let handle = event_handle(state, event, ResourceKind::BlockGrowEvent)?;
    state
        .handles
        .block_grow_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_block_grow_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::BlockGrowEventFacade>, HostError> {
    let handle = event_handle(state, event, ResourceKind::BlockGrowEvent)?;
    let invocation_id = state.invocation_id;
    state
        .handles
        .block_grow_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl crate::core_host::imports::HostBlockFormEvent for PluginStoreState {
    fn block_form_event_get_block(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-form-event.block-form-event.get-block")?;
            let block = resolve_block_grow_event(self, self_)
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

    fn block_form_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-form-event.block-form-event.is-cancelled")?;
            resolve_block_grow_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn block_form_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-form-event.block-form-event.set-cancelled")?;
            resolve_block_grow_event_mut(self, self_)
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

impl crate::core_host::imports::HostBlockGrowEvent for PluginStoreState {
    fn block_grow_event_get_block(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-grow-event.block-grow-event.get-block")?;
            let block = resolve_block_grow_event(self, self_)
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

    fn block_grow_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-grow-event.block-grow-event.is-cancelled")?;
            resolve_block_grow_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn block_grow_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-grow-event.block-grow-event.set-cancelled")?;
            resolve_block_grow_event_mut(self, self_)
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
