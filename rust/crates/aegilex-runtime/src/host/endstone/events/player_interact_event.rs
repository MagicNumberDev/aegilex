//! Core ABI implementation for `native/bindings/endstone/events/player_interact_event_facade.h`.

use super::support::*;

fn resolve_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::PlayerInteractEventFacade>, HostError> {
    let handle = state
        .resource_slot(event, ResourceKind::PlayerInteractEvent)
        .map_err(|_| HostError::from_status(AEGILEX_NOT_FOUND))?
        .handle;
    let invocation_id = state.invocation_id;
    state
        .handles
        .player_interact_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn player_interact_event_handle(state: &PluginStoreState, event: u32) -> Result<u64, HostError> {
    state
        .resource_slot(event, ResourceKind::PlayerInteractEvent)
        .and_then(|slot| {
            (slot.lifetime == ResourceLifetime::HostBorrowed)
                .then_some(slot.handle)
                .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
        })
}

fn resolve_player_interact_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::PlayerInteractEventFacade, HostError> {
    let handle = player_interact_event_handle(state, event)?;
    state
        .handles
        .player_interact_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_player_interact_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::PlayerInteractEventFacade>, HostError> {
    let handle = player_interact_event_handle(state, event)?;
    let invocation_id = state.invocation_id;
    state
        .handles
        .player_interact_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn player_interact_block_face(value: u8) -> BlockFaceBlockFace {
    match value {
        0 => BlockFaceBlockFace::Down,
        2 => BlockFaceBlockFace::North,
        3 => BlockFaceBlockFace::South,
        4 => BlockFaceBlockFace::West,
        5 => BlockFaceBlockFace::East,
        _ => BlockFaceBlockFace::Up,
    }
}

impl HostPlayerInteractEvent for PluginStoreState {
    fn player_interact_event_get_player(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "player-interact-event.player-interact-event.get-player",
            )?;
            let player = resolve_player_interact_event(self, self_)
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

    fn player_interact_event_get_action(
        &mut self,
        self_: u32,
    ) -> Result<Result<PlayerInteractEventInteractAction, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "player-interact-event.player-interact-event.get-action",
            )?;
            resolve_player_interact_event(self, self_)
                .map(|event| match event.getAction() {
                    1 => PlayerInteractEventInteractAction::RightClickBlock,
                    2 => PlayerInteractEventInteractAction::LeftClickAir,
                    3 => PlayerInteractEventInteractAction::RightClickAir,
                    _ => PlayerInteractEventInteractAction::LeftClickBlock,
                })
                .map_err(map_core_host_error)
        })())
    }

    fn player_interact_event_get_item(
        &mut self,
        self_: u32,
    ) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-interact-event.player-interact-event.get-item")?;
            let item = resolve_player_interact_event(self, self_)
                .map(|event| event.getItem())
                .map_err(map_core_host_error)?;
            (!item.is_null())
                .then(|| {
                    self.insert_item_stack_ref_child_resource(item, self_)
                        .map_err(map_core_host_error)
                })
                .transpose()
        })())
    }

    fn player_interact_event_get_block(
        &mut self,
        self_: u32,
    ) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "player-interact-event.player-interact-event.get-block",
            )?;
            let block = resolve_player_interact_event(self, self_)
                .map(|event| event.getBlock())
                .map_err(map_core_host_error)?;
            (!block.is_null())
                .then(|| {
                    self.insert_block_resource(block)
                        .map_err(map_core_host_error)
                })
                .transpose()
        })())
    }

    fn player_interact_event_get_block_face(
        &mut self,
        self_: u32,
    ) -> Result<Result<BlockFaceBlockFace, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "player-interact-event.player-interact-event.get-block-face",
            )?;
            resolve_player_interact_event(self, self_)
                .map(|event| player_interact_block_face(event.getBlockFace()))
                .map_err(map_core_host_error)
        })())
    }

    fn player_interact_event_get_clicked_position(
        &mut self,
        self_: u32,
    ) -> Result<Result<Option<VectorVector>, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "player-interact-event.player-interact-event.get-clicked-position",
            )?;
            resolve_player_interact_event(self, self_)
                .map(|event| {
                    event.hasClickedPosition().then(|| {
                        let position = event.getClickedPosition();
                        VectorVector {
                            x: position.x,
                            y: position.y,
                            z: position.z,
                        }
                    })
                })
                .map_err(map_core_host_error)
        })())
    }

    fn player_interact_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "player-interact-event.player-interact-event.is-cancelled",
            )?;
            resolve_player_interact_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn player_interact_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "player-interact-event.player-interact-event.set-cancelled",
            )?;
            resolve_player_interact_event_mut(self, self_)
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
