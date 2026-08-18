//! Core ABI implementation for `native/bindings/endstone/events/player_respawn_event_facade.h`.

use super::support::*;

fn resolve_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::PlayerRespawnEventFacade>, HostError> {
    let handle = state
        .resource_slot(event, ResourceKind::PlayerRespawnEvent)
        .map_err(|_| HostError::from_status(AEGILEX_NOT_FOUND))?
        .handle;
    let invocation_id = state.invocation_id;
    state
        .handles
        .player_respawn_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn player_respawn_event_handle(state: &PluginStoreState, event: u32) -> Result<u64, HostError> {
    state
        .resource_slot(event, ResourceKind::PlayerRespawnEvent)
        .and_then(|slot| {
            (slot.lifetime == ResourceLifetime::HostBorrowed)
                .then_some(slot.handle)
                .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
        })
}

fn resolve_player_respawn_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::PlayerRespawnEventFacade, HostError> {
    let handle = player_respawn_event_handle(state, event)?;
    state
        .handles
        .player_respawn_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl HostPlayerRespawnEvent for PluginStoreState {
    fn player_respawn_event_get_player(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-respawn-event.player-respawn-event.get-player")?;
            let player = resolve_player_respawn_event(self, self_)
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
}
