//! Core ABI implementation for `native/bindings/endstone/events/block_explode_event_facade.h`.

use super::support::*;

fn resolve_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::BlockExplodeEventFacade>, HostError> {
    let handle = state
        .resource_slot(event, ResourceKind::BlockExplodeEvent)
        .map_err(|_| HostError::from_status(AEGILEX_NOT_FOUND))?
        .handle;
    let invocation_id = state.invocation_id;
    state
        .handles
        .block_explode_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_block_explode_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::BlockExplodeEventFacade, HostError> {
    let handle = event_handle(state, event, ResourceKind::BlockExplodeEvent)?;
    state
        .handles
        .block_explode_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_block_explode_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::BlockExplodeEventFacade>, HostError> {
    let handle = event_handle(state, event, ResourceKind::BlockExplodeEvent)?;
    let invocation_id = state.invocation_id;
    state
        .handles
        .block_explode_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl crate::core_host::imports::HostBlockExplodeEvent for PluginStoreState {
    fn block_explode_event_get_block(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-explode-event.block-explode-event.get-block")?;
            let block = resolve_block_explode_event(self, self_)
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

    fn block_explode_event_get_block_list(
        &mut self,
        self_: u32,
    ) -> Result<Result<Vec<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "block-explode-event.block-explode-event.get-block-list",
            )?;
            let blocks = {
                let event =
                    resolve_block_explode_event(self, self_).map_err(map_core_host_error)?;
                let mut blocks = Vec::with_capacity(event.getBlockCount() as usize);
                for index in 0..event.getBlockCount() {
                    let block = event.getAffectedBlock(index);
                    if block.is_null() {
                        return Err(map_core_host_error(HostError::from_status(
                            AEGILEX_NOT_FOUND,
                        )));
                    }
                    blocks.push(block);
                }
                blocks
            };
            blocks
                .into_iter()
                .map(|block| {
                    self.insert_block_resource(block)
                        .map_err(map_core_host_error)
                })
                .collect()
        })())
    }

    fn block_explode_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-explode-event.block-explode-event.is-cancelled")?;
            resolve_block_explode_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn block_explode_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "block-explode-event.block-explode-event.set-cancelled",
            )?;
            resolve_block_explode_event_mut(self, self_)
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
