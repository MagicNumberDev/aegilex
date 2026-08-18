//! Core ABI implementation for `native/bindings/endstone/events/plugin_lifecycle_event_facade.h`.

use super::support::*;

fn resolve_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::PluginLifecycleEventFacade>, HostError> {
    let handle = state
        .resource_slot(event, ResourceKind::PluginLifecycleEvent)
        .map_err(|_| HostError::from_status(AEGILEX_NOT_FOUND))?
        .handle;
    let invocation_id = state.invocation_id;
    state
        .handles
        .plugin_lifecycle_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_plugin_lifecycle_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::PluginLifecycleEventFacade, HostError> {
    let handle = event_handle(state, event, ResourceKind::PluginLifecycleEvent)?;
    state
        .handles
        .plugin_lifecycle_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl crate::core_host::imports::HostPluginEnableEvent for PluginStoreState {
    fn plugin_enable_event_get_plugin_name(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "plugin-enable-event.plugin-enable-event.get-plugin-name",
            )?;
            resolve_plugin_lifecycle_event(self, self_)
                .map(|event| event.getPluginNameForRust())
                .map_err(map_core_host_error)
        })())
    }
}

impl crate::core_host::imports::HostPluginDisableEvent for PluginStoreState {
    fn plugin_disable_event_get_plugin_name(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "plugin-disable-event.plugin-disable-event.get-plugin-name",
            )?;
            resolve_plugin_lifecycle_event(self, self_)
                .map(|event| event.getPluginNameForRust())
                .map_err(map_core_host_error)
        })())
    }
}
