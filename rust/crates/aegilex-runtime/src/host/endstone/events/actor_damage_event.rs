//! Core ABI implementation for `native/bindings/endstone/events/actor_damage_event_facade.h`.

use super::support::*;



// --- actor-damage-event ---

fn resolve_actor_damage_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::ActorDamageEventFacade, HostError> {
    let handle = event_handle(state, event, ResourceKind::ActorDamageEvent)?;
    state
        .handles
        .actor_damage_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_actor_damage_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::ActorDamageEventFacade>, HostError> {
    let handle = event_handle(state, event, ResourceKind::ActorDamageEvent)?;
    let invocation_id = state.invocation_id;
    state
        .handles
        .actor_damage_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl crate::core_host::imports::HostActorDamageEvent for PluginStoreState {
    fn actor_damage_event_get_actor(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor-damage-event.actor-damage-event.get-actor")?;
            let actor = resolve_actor_damage_event(self, self_)
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

    fn actor_damage_event_get_damage(
        &mut self,
        self_: u32,
    ) -> Result<Result<f32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor-damage-event.actor-damage-event.get-damage")?;
            resolve_actor_damage_event(self, self_)
                .map(|event| event.getDamage())
                .map_err(map_core_host_error)
        })())
    }

    fn actor_damage_event_set_damage(
        &mut self,
        self_: u32,
        damage: f32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor-damage-event.actor-damage-event.set-damage")?;
            resolve_actor_damage_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setDamage(damage)
                        .then_some(())
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn actor_damage_event_get_damage_source(
        &mut self,
        self_: u32,
    ) -> Result<Result<DamageSourceDamageSource, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "actor-damage-event.actor-damage-event.get-damage-source",
            )?;
            resolve_actor_damage_event(self, self_)
                .map(|event| {
                    let source = event.getDamageSource();
                    DamageSourceDamageSource {
                        type_id: source.type_id,
                        actor_id: source.has_actor_id.then_some(source.actor_id),
                        damaging_actor_id: source
                            .has_damaging_actor_id
                            .then_some(source.damaging_actor_id),
                        indirect: source.indirect,
                    }
                })
                .map_err(map_core_host_error)
        })())
    }

    fn actor_damage_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor-damage-event.actor-damage-event.is-cancelled")?;
            resolve_actor_damage_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn actor_damage_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor-damage-event.actor-damage-event.set-cancelled")?;
            resolve_actor_damage_event_mut(self, self_)
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
