//! Core ABI implementation for `native/bindings/endstone/events/thunder_change_event_facade.h`.

use super::support::*;



fn resolve_thunder_change_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::ThunderChangeEventFacade, HostError> {
    let handle = event_handle(state, event, ResourceKind::ThunderChangeEvent)?;
    state
        .handles
        .thunder_change_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_thunder_change_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::ThunderChangeEventFacade>, HostError> {
    let handle = event_handle(state, event, ResourceKind::ThunderChangeEvent)?;
    let invocation_id = state.invocation_id;
    state
        .handles
        .thunder_change_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl crate::core_host::imports::HostThunderChangeEvent for PluginStoreState {
    fn thunder_change_event_get_to_thunder(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "thunder-change-event.thunder-change-event.get-to-thunder",
            )?;
            resolve_thunder_change_event(self, self_)
                .map(|event| event.getToThunder())
                .map_err(map_core_host_error)
        })())
    }

    fn thunder_change_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "thunder-change-event.thunder-change-event.is-cancelled",
            )?;
            resolve_thunder_change_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn thunder_change_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "thunder-change-event.thunder-change-event.set-cancelled",
            )?;
            resolve_thunder_change_event_mut(self, self_)
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
