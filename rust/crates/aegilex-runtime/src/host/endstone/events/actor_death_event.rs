//! Core ABI implementation for `native/bindings/endstone/events/actor_death_event_facade.h`.

use super::support::*;

fn resolve_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::ActorDeathEventFacade>, HostError> {
    let handle = state
        .resource_slot(event, ResourceKind::ActorDeathEvent)
        .map_err(|_| HostError::from_status(AEGILEX_NOT_FOUND))?
        .handle;
    let invocation_id = state.invocation_id;
    state
        .handles
        .actor_death_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_actor_death_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::ActorDeathEventFacade, HostError> {
    let handle = event_handle(state, event, ResourceKind::ActorDeathEvent)?;
    state
        .handles
        .actor_death_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl crate::core_host::imports::HostActorDeathEvent for PluginStoreState {
    fn actor_death_event_get_actor(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor-death-event.actor-death-event.get-actor")?;
            let actor = resolve_actor_death_event(self, self_)
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
}
