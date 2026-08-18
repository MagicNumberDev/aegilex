//! Core ABI implementation for `native/bindings/endstone/events/server_load_event_facade.h`.

use super::support::*;



fn resolve_server_load_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::ServerLoadEventFacade, HostError> {
    let handle = event_handle(state, event, ResourceKind::ServerLoadEvent)?;
    state
        .handles
        .server_load_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl crate::core_host::imports::HostServerLoadEvent for PluginStoreState {
    fn server_load_event_get_load_type(
        &mut self,
        self_: u32,
    ) -> Result<Result<ServerLoadEventServerLoadType, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "server-load-event.server-load-event.get-load-type")?;
            resolve_server_load_event(self, self_)
                .map(|event| match event.getLoadType() {
                    0 => ServerLoadEventServerLoadType::Startup,
                    _ => ServerLoadEventServerLoadType::Reload,
                })
                .map_err(map_core_host_error)
        })())
    }
}
