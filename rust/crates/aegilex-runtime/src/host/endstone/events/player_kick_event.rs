//! Core ABI implementation for `native/bindings/endstone/events/player_kick_event_facade.h`.

use super::support::*;



fn player_kick_event_handle(state: &PluginStoreState, event: u32) -> Result<u64, HostError> {
    state
        .resource_slot(event, ResourceKind::PlayerKickEvent)
        .and_then(|slot| {
            (slot.lifetime == ResourceLifetime::HostBorrowed)
                .then_some(slot.handle)
                .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
        })
}

fn resolve_player_kick_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::PlayerKickEventFacade, HostError> {
    let handle = player_kick_event_handle(state, event)?;
    state
        .handles
        .player_kick_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_player_kick_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::PlayerKickEventFacade>, HostError> {
    let handle = player_kick_event_handle(state, event)?;
    let invocation_id = state.invocation_id;
    state
        .handles
        .player_kick_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl HostPlayerKickEvent for PluginStoreState {
    fn player_kick_event_get_player(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-kick-event.player-kick-event.get-player")?;
            let player = resolve_player_kick_event(self, self_)
                .and_then(|event| {
                    let player = event.getPlayer();
                    (!player.is_null())
                        .then_some(player)
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            let handle = self
                .insert_handle(
                    ResourceKind::Player,
                    player,
                    crate::host::runtime::handles::GuestHandles::insert_player,
                )
                .map_err(map_core_host_error)?;
            self.resource_from_handle(ResourceKind::Player, handle)
                .map_err(map_core_host_error)
        })())
    }

    fn player_kick_event_get_reason(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-kick-event.player-kick-event.get-reason")?;
            resolve_player_kick_event(self, self_)
                .map(|event| event.getReasonForRust())
                .map_err(map_core_host_error)
        })())
    }

    fn player_kick_event_set_reason(
        &mut self,
        self_: u32,
        reason: String,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-kick-event.player-kick-event.set-reason")?;
            resolve_player_kick_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setReasonForRust(&reason)
                        .then_some(())
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_kick_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-kick-event.player-kick-event.is-cancelled")?;
            resolve_player_kick_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn player_kick_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-kick-event.player-kick-event.set-cancelled")?;
            resolve_player_kick_event_mut(self, self_)
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
