//! Core ABI implementation for `native/bindings/endstone/events/block_place_event_facade.h`.

use super::support::*;



fn resolve_block_place_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::BlockPlaceEventFacade, HostError> {
    let handle = event_handle(state, event, ResourceKind::BlockPlaceEvent)?;
    state
        .handles
        .block_place_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_block_place_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::BlockPlaceEventFacade>, HostError> {
    let handle = event_handle(state, event, ResourceKind::BlockPlaceEvent)?;
    let invocation_id = state.invocation_id;
    state
        .handles
        .block_place_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl crate::core_host::imports::HostBlockPlaceEvent for PluginStoreState {
    fn block_place_event_get_player(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-place-event.block-place-event.get-player")?;
            let player = resolve_block_place_event(self, self_)
                .and_then(|event| {
                    let player = event.getPlayer();
                    (!player.is_null())
                        .then_some(player)
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            self.insert_player_child_resource(player, self_)
                .map_err(map_core_host_error)
        })())
    }

    fn block_place_event_get_block_replaced(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "block-place-event.block-place-event.get-block-replaced",
            )?;
            let block = resolve_block_place_event(self, self_)
                .and_then(|event| {
                    let block = event.getBlockReplaced();
                    (!block.is_null())
                        .then_some(block)
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            self.insert_block_resource(block)
                .map_err(map_core_host_error)
        })())
    }

    fn block_place_event_get_block_against(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "block-place-event.block-place-event.get-block-against",
            )?;
            let block = resolve_block_place_event(self, self_)
                .and_then(|event| {
                    let block = event.getBlockAgainst();
                    (!block.is_null())
                        .then_some(block)
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            self.insert_block_resource(block)
                .map_err(map_core_host_error)
        })())
    }

    fn block_place_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-place-event.block-place-event.is-cancelled")?;
            resolve_block_place_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn block_place_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-place-event.block-place-event.set-cancelled")?;
            resolve_block_place_event_mut(self, self_)
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
