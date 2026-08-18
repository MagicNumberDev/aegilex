//! Core ABI implementation for `native/bindings/endstone/events/server_list_ping_event_facade.h`.

use super::support::*;

fn resolve_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::ServerListPingEventFacade>, HostError> {
    let handle = state
        .resource_slot(event, ResourceKind::ServerListPingEvent)
        .map_err(|_| HostError::from_status(AEGILEX_NOT_FOUND))?
        .handle;
    let invocation_id = state.invocation_id;
    state
        .handles
        .server_list_ping_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_server_list_ping_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::ServerListPingEventFacade, HostError> {
    let handle = event_handle(state, event, ResourceKind::ServerListPingEvent)?;
    state
        .handles
        .server_list_ping_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_server_list_ping_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::ServerListPingEventFacade>, HostError> {
    let handle = event_handle(state, event, ResourceKind::ServerListPingEvent)?;
    let invocation_id = state.invocation_id;
    state
        .handles
        .server_list_ping_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl crate::core_host::imports::HostServerListPingEvent for PluginStoreState {
    fn server_list_ping_event_get_motd(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "server-list-ping-event.server-list-ping-event.get-motd",
            )?;
            resolve_server_list_ping_event(self, self_)
                .map(|event| event.getMotdForRust())
                .map_err(map_core_host_error)
        })())
    }

    fn server_list_ping_event_set_motd(
        &mut self,
        self_: u32,
        motd: String,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "server-list-ping-event.server-list-ping-event.set-motd",
            )?;
            resolve_server_list_ping_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setMotdForRust(&motd)
                        .then_some(())
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn server_list_ping_event_get_server_guid(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "server-list-ping-event.server-list-ping-event.get-server-guid",
            )?;
            resolve_server_list_ping_event(self, self_)
                .map(|event| event.getServerGuidForRust())
                .map_err(map_core_host_error)
        })())
    }

    fn server_list_ping_event_set_server_guid(
        &mut self,
        self_: u32,
        server_guid: String,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "server-list-ping-event.server-list-ping-event.set-server-guid",
            )?;
            resolve_server_list_ping_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setServerGuidForRust(&server_guid)
                        .then_some(())
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn server_list_ping_event_get_local_port(
        &mut self,
        self_: u32,
    ) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "server-list-ping-event.server-list-ping-event.get-local-port",
            )?;
            resolve_server_list_ping_event(self, self_)
                .map(|event| event.getLocalPort())
                .map_err(map_core_host_error)
        })())
    }

    fn server_list_ping_event_set_local_port(
        &mut self,
        self_: u32,
        local_port: i32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "server-list-ping-event.server-list-ping-event.set-local-port",
            )?;
            resolve_server_list_ping_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setLocalPort(local_port)
                        .then_some(())
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn server_list_ping_event_get_local_port_v6(
        &mut self,
        self_: u32,
    ) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "server-list-ping-event.server-list-ping-event.get-local-port-v6",
            )?;
            resolve_server_list_ping_event(self, self_)
                .map(|event| event.getLocalPortV6())
                .map_err(map_core_host_error)
        })())
    }

    fn server_list_ping_event_set_local_port_v6(
        &mut self,
        self_: u32,
        local_port_v6: i32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "server-list-ping-event.server-list-ping-event.set-local-port-v6",
            )?;
            resolve_server_list_ping_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setLocalPortV6(local_port_v6)
                        .then_some(())
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn server_list_ping_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "server-list-ping-event.server-list-ping-event.is-cancelled",
            )?;
            resolve_server_list_ping_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn server_list_ping_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "server-list-ping-event.server-list-ping-event.set-cancelled",
            )?;
            resolve_server_list_ping_event_mut(self, self_)
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
