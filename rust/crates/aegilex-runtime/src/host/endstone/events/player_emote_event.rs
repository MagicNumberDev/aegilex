//! Core ABI implementation for `native/bindings/endstone/events/player_emote_event_facade.h`.

use super::support::*;

fn resolve_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::PlayerEmoteEventFacade>, HostError> {
    let handle = state
        .resource_slot(event, ResourceKind::PlayerEmoteEvent)
        .map_err(|_| HostError::from_status(AEGILEX_NOT_FOUND))?
        .handle;
    let invocation_id = state.invocation_id;
    state
        .handles
        .player_emote_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn player_emote_event_handle(state: &PluginStoreState, event: u32) -> Result<u64, HostError> {
    state
        .resource_slot(event, ResourceKind::PlayerEmoteEvent)
        .and_then(|slot| {
            (slot.lifetime == ResourceLifetime::HostBorrowed)
                .then_some(slot.handle)
                .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
        })
}

fn resolve_player_emote_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::PlayerEmoteEventFacade, HostError> {
    let handle = player_emote_event_handle(state, event)?;
    state
        .handles
        .player_emote_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_player_emote_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::PlayerEmoteEventFacade>, HostError> {
    let handle = player_emote_event_handle(state, event)?;
    let invocation_id = state.invocation_id;
    state
        .handles
        .player_emote_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl HostPlayerEmoteEvent for PluginStoreState {
    fn player_emote_event_get_player(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-emote-event.player-emote-event.get-player")?;
            let player = resolve_player_emote_event(self, self_)
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

    fn player_emote_event_get_emote_id(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-emote-event.player-emote-event.get-emote-id")?;
            resolve_player_emote_event(self, self_)
                .map(|event| event.getEmoteIdForRust())
                .map_err(map_core_host_error)
        })())
    }

    fn player_emote_event_is_muted(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-emote-event.player-emote-event.is-muted")?;
            resolve_player_emote_event(self, self_)
                .map(|event| event.isMuted())
                .map_err(map_core_host_error)
        })())
    }

    fn player_emote_event_set_muted(
        &mut self,
        self_: u32,
        muted: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-emote-event.player-emote-event.set-muted")?;
            resolve_player_emote_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setMuted(muted)
                        .then_some(())
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_emote_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-emote-event.player-emote-event.is-cancelled")?;
            resolve_player_emote_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn player_emote_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-emote-event.player-emote-event.set-cancelled")?;
            resolve_player_emote_event_mut(self, self_)
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
