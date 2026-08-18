//! Core ABI implementation for `native/bindings/endstone/events/actor_teleport_event_facade.h`.

use super::support::*;

fn resolve_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::ActorTeleportEventFacade>, HostError> {
    let handle = state
        .resource_slot(event, ResourceKind::ActorTeleportEvent)
        .map_err(|_| HostError::from_status(AEGILEX_NOT_FOUND))?
        .handle;
    let invocation_id = state.invocation_id;
    state
        .handles
        .actor_teleport_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_actor_teleport_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::ActorTeleportEventFacade, HostError> {
    let handle = event_handle(state, event, ResourceKind::ActorTeleportEvent)?;
    state
        .handles
        .actor_teleport_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_actor_teleport_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::ActorTeleportEventFacade>, HostError> {
    let handle = event_handle(state, event, ResourceKind::ActorTeleportEvent)?;
    let invocation_id = state.invocation_id;
    state
        .handles
        .actor_teleport_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl crate::core_host::imports::HostActorTeleportEvent for PluginStoreState {
    fn actor_teleport_event_get_actor(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor-teleport-event.actor-teleport-event.get-actor")?;
            let actor = resolve_actor_teleport_event(self, self_)
                .and_then(|event| {
                    let actor = event.getActor();
                    (!actor.is_null())
                        .then_some(actor)
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            self.insert_actor_child_resource(actor, self_)
                .map_err(map_core_host_error)
        })())
    }

    fn actor_teleport_event_get_from(
        &mut self,
        self_: u32,
    ) -> Result<Result<LocationLocation, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor-teleport-event.actor-teleport-event.get-from")?;
            resolve_actor_teleport_event(self, self_)
                .map(|event| location_from_cxx(event.getFrom()))
                .map_err(map_core_host_error)
        })())
    }

    fn actor_teleport_event_set_from(
        &mut self,
        self_: u32,
        location: LocationLocation,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor-teleport-event.actor-teleport-event.set-from")?;
            resolve_actor_teleport_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setFrom(&location_to_cxx(location))
                        .then_some(())
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn actor_teleport_event_get_to(
        &mut self,
        self_: u32,
    ) -> Result<Result<LocationLocation, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor-teleport-event.actor-teleport-event.get-to")?;
            resolve_actor_teleport_event(self, self_)
                .map(|event| location_from_cxx(event.getTo()))
                .map_err(map_core_host_error)
        })())
    }

    fn actor_teleport_event_set_to(
        &mut self,
        self_: u32,
        location: LocationLocation,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor-teleport-event.actor-teleport-event.set-to")?;
            resolve_actor_teleport_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setTo(&location_to_cxx(location))
                        .then_some(())
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn actor_teleport_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "actor-teleport-event.actor-teleport-event.is-cancelled",
            )?;
            resolve_actor_teleport_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn actor_teleport_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "actor-teleport-event.actor-teleport-event.set-cancelled",
            )?;
            resolve_actor_teleport_event_mut(self, self_)
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
