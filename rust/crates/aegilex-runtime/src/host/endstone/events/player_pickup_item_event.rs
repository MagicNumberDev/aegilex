//! Core ABI implementation for `native/bindings/endstone/events/player_pickup_item_event_facade.h`.

use super::support::*;



fn player_pickup_item_event_handle(state: &PluginStoreState, event: u32) -> Result<u64, HostError> {
    state
        .resource_slot(event, ResourceKind::PlayerPickupItemEvent)
        .and_then(|slot| {
            (slot.lifetime == ResourceLifetime::HostBorrowed)
                .then_some(slot.handle)
                .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
        })
}

fn resolve_player_pickup_item_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::PlayerPickupItemEventFacade, HostError> {
    let handle = player_pickup_item_event_handle(state, event)?;
    state
        .handles
        .player_pickup_item_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_player_pickup_item_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::PlayerPickupItemEventFacade>, HostError> {
    let handle = player_pickup_item_event_handle(state, event)?;
    let invocation_id = state.invocation_id;
    state
        .handles
        .player_pickup_item_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl HostPlayerPickupItemEvent for PluginStoreState {
    fn player_pickup_item_event_get_player(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "player-pickup-item-event.player-pickup-item-event.get-player",
            )?;
            let player = resolve_player_pickup_item_event(self, self_)
                .and_then(|event| {
                    let player = event.getPlayer();
                    (!player.is_null())
                        .then_some(player)
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            self.insert_player_child_resource(player, self_)
                .map_err(map_core_host_error)
        })())
    }

    fn player_pickup_item_event_get_item_actor(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "player-pickup-item-event.player-pickup-item-event.get-item-actor",
            )?;
            let actor = resolve_player_pickup_item_event(self, self_)
                .and_then(|event| {
                    let actor = event.getItemActor();
                    (!actor.is_null())
                        .then_some(actor)
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            self.insert_actor_child_resource(actor, self_)
                .map_err(map_core_host_error)
        })())
    }

    fn player_pickup_item_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "player-pickup-item-event.player-pickup-item-event.is-cancelled",
            )?;
            resolve_player_pickup_item_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn player_pickup_item_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "player-pickup-item-event.player-pickup-item-event.set-cancelled",
            )?;
            resolve_player_pickup_item_event_mut(self, self_)
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
