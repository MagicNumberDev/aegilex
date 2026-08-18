//! Core ABI implementation for `native/bindings/endstone/events/broadcast_message_event_facade.h`.

use super::support::*;



fn resolve_broadcast_message_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::BroadcastMessageEventFacade, HostError> {
    let handle = event_handle(state, event, ResourceKind::BroadcastMessageEvent)?;
    state
        .handles
        .broadcast_message_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_broadcast_message_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::BroadcastMessageEventFacade>, HostError> {
    let handle = event_handle(state, event, ResourceKind::BroadcastMessageEvent)?;
    let invocation_id = state.invocation_id;
    state
        .handles
        .broadcast_message_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl crate::core_host::imports::HostBroadcastMessageEvent for PluginStoreState {
    fn broadcast_message_event_get_message(
        &mut self,
        self_: u32,
    ) -> Result<Result<MessageMessage, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "broadcast-message-event.broadcast-message-event.get-message",
            )?;
            resolve_broadcast_message_event(self, self_)
                .map(|event| MessageMessage::PlainText(event.getMessageForRust()))
                .map_err(map_core_host_error)
        })())
    }

    fn broadcast_message_event_set_message(
        &mut self,
        self_: u32,
        message: MessageMessage,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "broadcast-message-event.broadcast-message-event.set-message",
            )?;
            let MessageMessage::PlainText(message) = message else {
                return Err(map_core_host_error(HostError::from_status(
                    AEGILEX_NOT_FOUND,
                )));
            };
            resolve_broadcast_message_event_mut(self, self_)
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

    fn broadcast_message_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "broadcast-message-event.broadcast-message-event.is-cancelled",
            )?;
            resolve_broadcast_message_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn broadcast_message_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "broadcast-message-event.broadcast-message-event.set-cancelled",
            )?;
            resolve_broadcast_message_event_mut(self, self_)
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
