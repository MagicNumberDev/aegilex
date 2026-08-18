//! Core ABI implementation for `native/bindings/endstone/events/player_chat_event_facade.h`.

use super::support::*;



fn player_chat_event_handle(state: &PluginStoreState, event: u32) -> Result<u64, HostError> {
    state
        .resource_slot(event, ResourceKind::PlayerChatEvent)
        .and_then(|slot| {
            (slot.lifetime == ResourceLifetime::HostBorrowed)
                .then_some(slot.handle)
                .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
        })
}

fn resolve_player_chat_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::PlayerChatEventFacade, HostError> {
    let handle = player_chat_event_handle(state, event)?;
    state
        .handles
        .player_chat_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_player_chat_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::PlayerChatEventFacade>, HostError> {
    let handle = player_chat_event_handle(state, event)?;
    let invocation_id = state.invocation_id;
    state
        .handles
        .player_chat_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl HostPlayerChatEvent for PluginStoreState {
    fn player_chat_event_get_player(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-chat-event.player-chat-event.get-player")?;
            let player = resolve_player_chat_event(self, self_)
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

    fn player_chat_event_set_player(
        &mut self,
        self_: u32,
        player: u32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-chat-event.player-chat-event.set-player")?;
            let player_handle = self
                .resource_slot(player, ResourceKind::Player)
                .map(|slot| slot.handle)
                .map_err(map_core_host_error)?;
            let event_handle =
                player_chat_event_handle(self, self_).map_err(map_core_host_error)?;
            let invocation_id = self.invocation_id;
            let player = self
                .handles
                .player(invocation_id, player_handle)
                .ok_or_else(|| map_core_host_error(HostError::from_status(AEGILEX_NOT_FOUND)))?
                as *const _;
            // SAFETY: player and event facades are both owned by self invocation's
            // handle table and remain live for the duration of the host call.
            let player = unsafe { &*player };
            self.handles
                .player_chat_event_mut(invocation_id, event_handle)
                .and_then(|event| event.setPlayer(player).then_some(()))
                .ok_or_else(|| map_core_host_error(HostError::from_status(AEGILEX_NOT_FOUND)))?;
            Ok(())
        })())
    }

    fn player_chat_event_get_message(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-chat-event.player-chat-event.get-message")?;
            resolve_player_chat_event(self, self_)
                .map(|event| event.getMessageForRust())
                .map_err(map_core_host_error)
        })())
    }

    fn player_chat_event_set_message(
        &mut self,
        self_: u32,
        message: String,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-chat-event.player-chat-event.set-message")?;
            resolve_player_chat_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setMessageForRust(&message)
                        .then_some(())
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_chat_event_get_format(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-chat-event.player-chat-event.get-format")?;
            resolve_player_chat_event(self, self_)
                .map(|event| event.getFormatForRust())
                .map_err(map_core_host_error)
        })())
    }

    fn player_chat_event_set_format(
        &mut self,
        self_: u32,
        format: String,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-chat-event.player-chat-event.set-format")?;
            resolve_player_chat_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setFormatForRust(&format)
                        .then_some(())
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_chat_event_get_recipients(
        &mut self,
        self_: u32,
    ) -> Result<Result<Vec<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-chat-event.player-chat-event.get-recipients")?;
            let mut recipients = resolve_player_chat_event(self, self_)
                .map(|event| event.getRecipients())
                .map_err(map_core_host_error)?;
            if recipients.is_null() {
                return Err(map_core_host_error(HostError::from_status(
                    AEGILEX_NOT_FOUND,
                )));
            }
            let len = recipients.len();
            let mut out = Vec::with_capacity(len);
            for index in 0..len {
                let player = recipients.pin_mut().takePlayer(index);
                if player.is_null() {
                    continue;
                }
                let handle = self
                    .insert_handle(
                        ResourceKind::Player,
                        player,
                        crate::host::runtime::handles::GuestHandles::insert_player,
                    )
                    .map_err(map_core_host_error)?;
                out.push(
                    self.resource_from_handle(ResourceKind::Player, handle)
                        .map_err(map_core_host_error)?,
                );
            }
            Ok(out)
        })())
    }

    fn player_chat_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-chat-event.player-chat-event.is-cancelled")?;
            resolve_player_chat_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn player_chat_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-chat-event.player-chat-event.set-cancelled")?;
            resolve_player_chat_event_mut(self, self_)
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
