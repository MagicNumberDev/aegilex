//! Core ABI implementation for `native/bindings/endstone/events/player_game_mode_change_event_facade.h`.

use super::support::*;



fn player_game_mode_change_event_handle(
    state: &PluginStoreState,
    event: u32,
) -> Result<u64, HostError> {
    state
        .resource_slot(event, ResourceKind::PlayerGameModeChangeEvent)
        .and_then(|slot| {
            (slot.lifetime == ResourceLifetime::HostBorrowed)
                .then_some(slot.handle)
                .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
        })
}

fn resolve_player_game_mode_change_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::PlayerGameModeChangeEventFacade, HostError> {
    let handle = player_game_mode_change_event_handle(state, event)?;
    state
        .handles
        .player_game_mode_change_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_player_game_mode_change_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::PlayerGameModeChangeEventFacade>, HostError> {
    let handle = player_game_mode_change_event_handle(state, event)?;
    let invocation_id = state.invocation_id;
    state
        .handles
        .player_game_mode_change_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl HostPlayerGameModeChangeEvent for PluginStoreState {
    fn player_game_mode_change_event_get_player(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "player-game-mode-change-event.player-game-mode-change-event.get-player",
            )?;
            let player = resolve_player_game_mode_change_event(self, self_)
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

    fn player_game_mode_change_event_get_new_game_mode(
        &mut self,
        self_: u32,
    ) -> Result<Result<GameModeGameMode, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "player-game-mode-change-event.player-game-mode-change-event.get-new-game-mode",
            )?;
            resolve_player_game_mode_change_event(self, self_)
                .map(|event| match event.getNewGameMode() {
                    0 => GameModeGameMode::Survival,
                    1 => GameModeGameMode::Creative,
                    2 => GameModeGameMode::Adventure,
                    _ => GameModeGameMode::Spectator,
                })
                .map_err(map_core_host_error)
        })())
    }

    fn player_game_mode_change_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "player-game-mode-change-event.player-game-mode-change-event.is-cancelled",
            )?;
            resolve_player_game_mode_change_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn player_game_mode_change_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "player-game-mode-change-event.player-game-mode-change-event.set-cancelled",
            )?;
            resolve_player_game_mode_change_event_mut(self, self_)
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
