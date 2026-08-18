//! Core ABI implementation for `native/bindings/endstone/events/map_initialize_event_facade.h`.

use super::support::*;

fn resolve_map_initialize_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::MapInitializeEventFacade, HostError> {
    let handle = event_handle(state, event, ResourceKind::MapInitializeEvent)?;
    state
        .handles
        .map_initialize_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl crate::core_host::imports::HostMapInitializeEvent for PluginStoreState {
    fn map_initialize_event_get_map_id(
        &mut self,
        self_: u32,
    ) -> Result<Result<i64, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "map-initialize-event.map-initialize-event.get-map-id")?;
            resolve_map_initialize_event(self, self_)
                .map(|event| event.getMapIdForRust())
                .map_err(map_core_host_error)
        })())
    }
}
