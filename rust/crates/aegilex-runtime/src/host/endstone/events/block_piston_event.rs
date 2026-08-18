//! Core ABI implementation for `native/bindings/endstone/events/block_piston_event_facade.h`.

use super::support::*;

fn resolve_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::BlockPistonEventFacade>, HostError> {
    let handle = state
        .resource_slot(event, ResourceKind::BlockPistonEvent)
        .map_err(|_| HostError::from_status(AEGILEX_NOT_FOUND))?
        .handle;
    let invocation_id = state.invocation_id;
    state
        .handles
        .block_piston_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_block_piston_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::BlockPistonEventFacade, HostError> {
    let handle = event_handle(state, event, ResourceKind::BlockPistonEvent)?;
    state
        .handles
        .block_piston_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_block_piston_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::BlockPistonEventFacade>, HostError> {
    let handle = event_handle(state, event, ResourceKind::BlockPistonEvent)?;
    let invocation_id = state.invocation_id;
    state
        .handles
        .block_piston_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl crate::core_host::imports::HostBlockPistonExtendEvent for PluginStoreState {
    fn block_piston_extend_event_get_block(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "block-piston-extend-event.block-piston-extend-event.get-block",
            )?;
            let block = resolve_block_piston_event(self, self_)
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

    fn block_piston_extend_event_get_direction(
        &mut self,
        self_: u32,
    ) -> Result<Result<BlockFaceBlockFace, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "block-piston-extend-event.block-piston-extend-event.get-direction",
            )?;
            resolve_block_piston_event(self, self_)
                .map(|event| block_face(event.getDirection()))
                .map_err(map_core_host_error)
        })())
    }

    fn block_piston_extend_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "block-piston-extend-event.block-piston-extend-event.is-cancelled",
            )?;
            resolve_block_piston_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn block_piston_extend_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "block-piston-extend-event.block-piston-extend-event.set-cancelled",
            )?;
            resolve_block_piston_event_mut(self, self_)
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

impl crate::core_host::imports::HostBlockPistonRetractEvent for PluginStoreState {
    fn block_piston_retract_event_get_block(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "block-piston-retract-event.block-piston-retract-event.get-block",
            )?;
            let block = resolve_block_piston_event(self, self_)
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

    fn block_piston_retract_event_get_direction(
        &mut self,
        self_: u32,
    ) -> Result<Result<BlockFaceBlockFace, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "block-piston-retract-event.block-piston-retract-event.get-direction",
            )?;
            resolve_block_piston_event(self, self_)
                .map(|event| block_face(event.getDirection()))
                .map_err(map_core_host_error)
        })())
    }

    fn block_piston_retract_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "block-piston-retract-event.block-piston-retract-event.is-cancelled",
            )?;
            resolve_block_piston_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn block_piston_retract_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "block-piston-retract-event.block-piston-retract-event.set-cancelled",
            )?;
            resolve_block_piston_event_mut(self, self_)
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
