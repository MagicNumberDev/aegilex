//! Core ABI implementation for `native/bindings/endstone/events/actor_explode_event_facade.h`.

use super::support::*;



fn resolve_actor_explode_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::ActorExplodeEventFacade, HostError> {
    let handle = event_handle(state, event, ResourceKind::ActorExplodeEvent)?;
    state
        .handles
        .actor_explode_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_actor_explode_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::ActorExplodeEventFacade>, HostError> {
    let handle = event_handle(state, event, ResourceKind::ActorExplodeEvent)?;
    let invocation_id = state.invocation_id;
    state
        .handles
        .actor_explode_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl crate::core_host::imports::HostActorExplodeEvent for PluginStoreState {
    fn actor_explode_event_get_actor(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor-explode-event.actor-explode-event.get-actor")?;
            let actor = resolve_actor_explode_event(self, self_)
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

    fn actor_explode_event_get_location(
        &mut self,
        self_: u32,
    ) -> Result<Result<LocationLocation, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor-explode-event.actor-explode-event.get-location")?;
            resolve_actor_explode_event(self, self_)
                .map(|event| location_from_cxx(event.getLocation()))
                .map_err(map_core_host_error)
        })())
    }

    fn actor_explode_event_get_block_list(
        &mut self,
        self_: u32,
    ) -> Result<Result<Vec<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "actor-explode-event.actor-explode-event.get-block-list",
            )?;
            let blocks = {
                let event =
                    resolve_actor_explode_event(self, self_).map_err(map_core_host_error)?;
                let mut blocks = Vec::with_capacity(event.getBlockCount() as usize);
                for index in 0..event.getBlockCount() {
                    let block = event.getBlock(index);
                    if block.is_null() {
                        return Err(map_core_host_error(HostError::from_status(
                            AEGILEX_NOT_FOUND,
                        )));
                    }
                    blocks.push(block);
                }
                blocks
            };
            blocks
                .into_iter()
                .map(|block| {
                    self.insert_block_resource(block)
                        .map_err(map_core_host_error)
                })
                .collect()
        })())
    }

    fn actor_explode_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor-explode-event.actor-explode-event.is-cancelled")?;
            resolve_actor_explode_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn actor_explode_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "actor-explode-event.actor-explode-event.set-cancelled",
            )?;
            resolve_actor_explode_event_mut(self, self_)
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
