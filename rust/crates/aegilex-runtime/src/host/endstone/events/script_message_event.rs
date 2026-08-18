//! Core ABI implementation for `native/bindings/endstone/events/script_message_event_facade.h`.

use super::support::*;



fn resolve_script_message_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::ScriptMessageEventFacade, HostError> {
    let handle = event_handle(state, event, ResourceKind::ScriptMessageEvent)?;
    state
        .handles
        .script_message_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_script_message_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::ScriptMessageEventFacade>, HostError> {
    let handle = event_handle(state, event, ResourceKind::ScriptMessageEvent)?;
    let invocation_id = state.invocation_id;
    state
        .handles
        .script_message_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl crate::core_host::imports::HostScriptMessageEvent for PluginStoreState {
    fn script_message_event_get_message_id(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "script-message-event.script-message-event.get-message-id",
            )?;
            resolve_script_message_event(self, self_)
                .map(|event| event.getMessageIdForRust())
                .map_err(map_core_host_error)
        })())
    }

    fn script_message_event_get_message(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "script-message-event.script-message-event.get-message",
            )?;
            resolve_script_message_event(self, self_)
                .map(|event| event.getMessageForRust())
                .map_err(map_core_host_error)
        })())
    }

    fn script_message_event_get_sender(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "script-message-event.script-message-event.get-sender")?;
            let sender = resolve_script_message_event(self, self_)
                .and_then(|event| {
                    let sender = event.getSender();
                    (!sender.is_null())
                        .then_some(sender)
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            self.insert_command_sender_resource(sender)
                .map_err(map_core_host_error)
        })())
    }

    fn script_message_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "script-message-event.script-message-event.is-cancelled",
            )?;
            resolve_script_message_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn script_message_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "script-message-event.script-message-event.set-cancelled",
            )?;
            resolve_script_message_event_mut(self, self_)
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
