//! Core ABI implementation for `native/bindings/endstone/events/actor_spawn_event_facade.h`.

use super::support::*;



fn resolve_actor_spawn_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::ActorSpawnEventFacade, HostError> {
    let handle = event_handle(state, event, ResourceKind::ActorSpawnEvent)?;
    state
        .handles
        .actor_spawn_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_actor_spawn_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::ActorSpawnEventFacade>, HostError> {
    let handle = event_handle(state, event, ResourceKind::ActorSpawnEvent)?;
    let invocation_id = state.invocation_id;
    state
        .handles
        .actor_spawn_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl crate::core_host::imports::HostActorSpawnEvent for PluginStoreState {
    fn actor_spawn_event_get_actor(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor-spawn-event.actor-spawn-event.get-actor")?;
            let actor = resolve_actor_spawn_event(self, self_)
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

    fn actor_spawn_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor-spawn-event.actor-spawn-event.is-cancelled")?;
            resolve_actor_spawn_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn actor_spawn_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor-spawn-event.actor-spawn-event.set-cancelled")?;
            resolve_actor_spawn_event_mut(self, self_)
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
