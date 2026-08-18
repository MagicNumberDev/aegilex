//! Core ABI implementation for `native/bindings/endstone/events/player_join_event_facade.h`.

use super::support::*;

fn resolve_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::PlayerJoinEventFacade>, HostError> {
    let handle = state
        .resource_slot(event, ResourceKind::PlayerJoinEvent)
        .map_err(|_| HostError::from_status(AEGILEX_NOT_FOUND))?
        .handle;
    let invocation_id = state.invocation_id;
    state
        .handles
        .player_join_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

// ===== player-join-event =====

fn player_join_event_handle(state: &PluginStoreState, event: u32) -> Result<u64, HostError> {
    state
        .resource_slot(event, ResourceKind::PlayerJoinEvent)
        .and_then(|slot| {
            (slot.lifetime == ResourceLifetime::HostBorrowed)
                .then_some(slot.handle)
                .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
        })
}

fn resolve_player_join_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::PlayerJoinEventFacade, HostError> {
    let handle = player_join_event_handle(state, event)?;
    state
        .handles
        .player_join_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_player_join_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::PlayerJoinEventFacade>, HostError> {
    let handle = player_join_event_handle(state, event)?;
    let invocation_id = state.invocation_id;
    state
        .handles
        .player_join_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl HostPlayerJoinEvent for PluginStoreState {
    fn player_join_event_get_player(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-join-event.player-join-event.get-player")?;
            let player = resolve_player_join_event(self, self_)
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

    fn player_join_event_get_join_message(
        &mut self,
        self_: u32,
    ) -> Result<Result<Option<MessageMessage>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-join-event.player-join-event.get-join-message")?;
            resolve_player_join_event(self, self_)
                .map(|event| {
                    event
                        .hasJoinMessage()
                        .then(|| MessageMessage::PlainText(event.getJoinMessageForRust()))
                })
                .map_err(map_core_host_error)
        })())
    }

    fn player_join_event_set_join_message(
        &mut self,
        self_: u32,
        message: Option<MessageMessage>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-join-event.player-join-event.set-join-message")?;
            let (has_message, message) = match message {
                Some(MessageMessage::PlainText(message)) => (true, message),
                Some(MessageMessage::Translatable(_)) => {
                    return Err(map_core_host_error(HostError::from_status(
                        AEGILEX_NOT_FOUND,
                    )));
                }
                None => (false, String::new()),
            };
            resolve_player_join_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setJoinMessageForRust(has_message, &message)
                        .then_some(())
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }
}
