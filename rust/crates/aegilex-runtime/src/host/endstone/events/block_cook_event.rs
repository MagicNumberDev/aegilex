//! Core ABI implementation for `native/bindings/endstone/events/block_cook_event_facade.h`.

use super::support::*;



fn resolve_block_cook_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::BlockCookEventFacade, HostError> {
    let handle = event_handle(state, event, ResourceKind::BlockCookEvent)?;
    state
        .handles
        .block_cook_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_block_cook_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::BlockCookEventFacade>, HostError> {
    let handle = event_handle(state, event, ResourceKind::BlockCookEvent)?;
    let invocation_id = state.invocation_id;
    state
        .handles
        .block_cook_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl crate::core_host::imports::HostBlockCookEvent for PluginStoreState {
    fn block_cook_event_get_block(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-cook-event.block-cook-event.get-block")?;
            let block = resolve_block_cook_event(self, self_)
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

    fn block_cook_event_get_source(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-cook-event.block-cook-event.get-source")?;
            let source = resolve_block_cook_event(self, self_)
                .and_then(|event| {
                    let source = event.getSource();
                    (!source.is_null())
                        .then_some(source)
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            self.insert_item_stack_ref_child_resource(source, self_)
                .map_err(map_core_host_error)
        })())
    }

    fn block_cook_event_get_result(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-cook-event.block-cook-event.get-result")?;
            let result = resolve_block_cook_event(self, self_)
                .and_then(|event| {
                    let result = event.getResult();
                    (!result.is_null())
                        .then_some(result)
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            self.insert_item_stack_ref_child_resource(result, self_)
                .map_err(map_core_host_error)
        })())
    }

    fn block_cook_event_set_result(
        &mut self,
        self_: u32,
        item: u32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-cook-event.block-cook-event.set-result")?;
            let result_handle =
                item_stack_resource_handle(self, item).map_err(map_core_host_error)?;
            let invocation_id = self.invocation_id;
            let result = self
                .handles
                .item_stack(invocation_id, result_handle)
                .ok_or_else(|| map_core_host_error(HostError::from_status(AEGILEX_NOT_FOUND)))?
                .cloneItemStack();
            if result.is_null() {
                return Err(map_core_host_error(HostError::from_status(
                    AEGILEX_NOT_FOUND,
                )));
            }
            resolve_block_cook_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setResult(&result)
                        .then_some(())
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn block_cook_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-cook-event.block-cook-event.is-cancelled")?;
            resolve_block_cook_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn block_cook_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-cook-event.block-cook-event.set-cancelled")?;
            resolve_block_cook_event_mut(self, self_)
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
