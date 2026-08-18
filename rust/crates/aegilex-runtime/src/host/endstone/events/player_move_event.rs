//! Core ABI implementation for `native/bindings/endstone/events/player_move_event_facade.h`.

use super::support::*;



fn player_move_event_handle(state: &PluginStoreState, event: u32) -> Result<u64, HostError> {
    state
        .resource_slot(event, ResourceKind::PlayerMoveEvent)
        .and_then(|slot| {
            (slot.lifetime == ResourceLifetime::HostBorrowed)
                .then_some(slot.handle)
                .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
        })
}

fn resolve_player_move_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::PlayerMoveEventFacade, HostError> {
    let handle = player_move_event_handle(state, event)?;
    state
        .handles
        .player_move_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_player_move_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::PlayerMoveEventFacade>, HostError> {
    let handle = player_move_event_handle(state, event)?;
    let invocation_id = state.invocation_id;
    state
        .handles
        .player_move_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn player_move_location(location: cxx_event::LocationData) -> LocationLocation {
    LocationLocation {
        dimension: location.dimension,
        x: location.x,
        y: location.y,
        z: location.z,
        pitch: location.pitch,
        yaw: location.yaw,
    }
}

fn player_move_location_data(location: LocationLocation) -> cxx_event::LocationData {
    cxx_event::LocationData {
        dimension: location.dimension,
        x: location.x,
        y: location.y,
        z: location.z,
        pitch: location.pitch,
        yaw: location.yaw,
    }
}

impl HostPlayerMoveEvent for PluginStoreState {
    fn player_move_event_get_player(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-move-event.player-move-event.get-player")?;
            let player = resolve_player_move_event(self, self_)
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

    fn player_move_event_get_from(
        &mut self,
        self_: u32,
    ) -> Result<Result<LocationLocation, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-move-event.player-move-event.get-from")?;
            resolve_player_move_event(self, self_)
                .map(|event| player_move_location(event.getFrom()))
                .map_err(map_core_host_error)
        })())
    }

    fn player_move_event_set_from(
        &mut self,
        self_: u32,
        from: LocationLocation,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-move-event.player-move-event.set-from")?;
            resolve_player_move_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setFrom(&player_move_location_data(from))
                        .then_some(())
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_move_event_get_to(
        &mut self,
        self_: u32,
    ) -> Result<Result<LocationLocation, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-move-event.player-move-event.get-to")?;
            resolve_player_move_event(self, self_)
                .map(|event| player_move_location(event.getTo()))
                .map_err(map_core_host_error)
        })())
    }

    fn player_move_event_set_to(
        &mut self,
        self_: u32,
        to: LocationLocation,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-move-event.player-move-event.set-to")?;
            resolve_player_move_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setTo(&player_move_location_data(to))
                        .then_some(())
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_move_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-move-event.player-move-event.is-cancelled")?;
            resolve_player_move_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn player_move_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-move-event.player-move-event.set-cancelled")?;
            resolve_player_move_event_mut(self, self_)
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

fn move_event_handle(state: &PluginStoreState, event: u32) -> Result<u64, HostError> {
    state
        .resource_slot(event, ResourceKind::PlayerMoveEvent)
        .and_then(|slot| {
            (slot.lifetime == ResourceLifetime::HostBorrowed)
                .then_some(slot.handle)
                .ok_or_else(not_found)
        })
}

fn resolve_move_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::PlayerMoveEventFacade, HostError> {
    let handle = move_event_handle(state, event)?;
    state
        .handles
        .player_move_event(state.invocation_id, handle)
        .ok_or_else(not_found)
}

fn resolve_move_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::PlayerMoveEventFacade>, HostError> {
    let handle = move_event_handle(state, event)?;
    let invocation_id = state.invocation_id;
    state
        .handles
        .player_move_event_mut(invocation_id, handle)
        .ok_or_else(not_found)
}

fn move_event_location(location: cxx_event::LocationData) -> LocationLocation {
    LocationLocation {
        dimension: location.dimension,
        x: location.x,
        y: location.y,
        z: location.z,
        pitch: location.pitch,
        yaw: location.yaw,
    }
}

fn move_event_location_data(location: LocationLocation) -> cxx_event::LocationData {
    cxx_event::LocationData {
        dimension: location.dimension,
        x: location.x,
        y: location.y,
        z: location.z,
        pitch: location.pitch,
        yaw: location.yaw,
    }
}

impl HostPlayerTeleportEvent for PluginStoreState {
    fn player_teleport_event_get_player(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "player-teleport-event.player-teleport-event.get-player",
            )?;
            let player = resolve_move_event(self, self_)
                .and_then(|event| {
                    let player = event.getPlayer();
                    (!player.is_null()).then_some(player).ok_or_else(not_found)
                })
                .map_err(map_core_host_error)?;
            let parent = self_;
            self.insert_player_child_resource(player, parent)
                .map_err(map_core_host_error)
        })())
    }

    fn player_teleport_event_get_from(
        &mut self,
        self_: u32,
    ) -> Result<Result<LocationLocation, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-teleport-event.player-teleport-event.get-from")?;
            resolve_move_event(self, self_)
                .map(|event| move_event_location(event.getFrom()))
                .map_err(map_core_host_error)
        })())
    }

    fn player_teleport_event_set_from(
        &mut self,
        self_: u32,
        location: LocationLocation,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-teleport-event.player-teleport-event.set-from")?;
            resolve_move_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setFrom(&move_event_location_data(location))
                        .then_some(())
                        .ok_or_else(not_found)
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_teleport_event_get_to(
        &mut self,
        self_: u32,
    ) -> Result<Result<LocationLocation, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-teleport-event.player-teleport-event.get-to")?;
            resolve_move_event(self, self_)
                .map(|event| move_event_location(event.getTo()))
                .map_err(map_core_host_error)
        })())
    }

    fn player_teleport_event_set_to(
        &mut self,
        self_: u32,
        location: LocationLocation,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-teleport-event.player-teleport-event.set-to")?;
            resolve_move_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setTo(&move_event_location_data(location))
                        .then_some(())
                        .ok_or_else(not_found)
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_teleport_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "player-teleport-event.player-teleport-event.is-cancelled",
            )?;
            resolve_move_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn player_teleport_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "player-teleport-event.player-teleport-event.set-cancelled",
            )?;
            resolve_move_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setCancelled(cancelled)
                        .then_some(())
                        .ok_or_else(not_found)
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }
}

impl HostPlayerJumpEvent for PluginStoreState {
    fn player_jump_event_get_player(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-jump-event.player-jump-event.get-player")?;
            let player = resolve_move_event(self, self_)
                .and_then(|event| {
                    let player = event.getPlayer();
                    (!player.is_null()).then_some(player).ok_or_else(not_found)
                })
                .map_err(map_core_host_error)?;
            let parent = self_;
            self.insert_player_child_resource(player, parent)
                .map_err(map_core_host_error)
        })())
    }

    fn player_jump_event_get_from(
        &mut self,
        self_: u32,
    ) -> Result<Result<LocationLocation, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-jump-event.player-jump-event.get-from")?;
            resolve_move_event(self, self_)
                .map(|event| move_event_location(event.getFrom()))
                .map_err(map_core_host_error)
        })())
    }

    fn player_jump_event_set_from(
        &mut self,
        self_: u32,
        location: LocationLocation,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-jump-event.player-jump-event.set-from")?;
            resolve_move_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setFrom(&move_event_location_data(location))
                        .then_some(())
                        .ok_or_else(not_found)
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_jump_event_get_to(
        &mut self,
        self_: u32,
    ) -> Result<Result<LocationLocation, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-jump-event.player-jump-event.get-to")?;
            resolve_move_event(self, self_)
                .map(|event| move_event_location(event.getTo()))
                .map_err(map_core_host_error)
        })())
    }

    fn player_jump_event_set_to(
        &mut self,
        self_: u32,
        location: LocationLocation,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-jump-event.player-jump-event.set-to")?;
            resolve_move_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setTo(&move_event_location_data(location))
                        .then_some(())
                        .ok_or_else(not_found)
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_jump_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-jump-event.player-jump-event.is-cancelled")?;
            resolve_move_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn player_jump_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-jump-event.player-jump-event.set-cancelled")?;
            resolve_move_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setCancelled(cancelled)
                        .then_some(())
                        .ok_or_else(not_found)
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }
}

impl HostPlayerPortalEvent for PluginStoreState {
    fn player_portal_event_get_player(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-portal-event.player-portal-event.get-player")?;
            let player = resolve_move_event(self, self_)
                .and_then(|event| {
                    let player = event.getPlayer();
                    (!player.is_null()).then_some(player).ok_or_else(not_found)
                })
                .map_err(map_core_host_error)?;
            let parent = self_;
            self.insert_player_child_resource(player, parent)
                .map_err(map_core_host_error)
        })())
    }

    fn player_portal_event_get_from(
        &mut self,
        self_: u32,
    ) -> Result<Result<LocationLocation, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-portal-event.player-portal-event.get-from")?;
            resolve_move_event(self, self_)
                .map(|event| move_event_location(event.getFrom()))
                .map_err(map_core_host_error)
        })())
    }

    fn player_portal_event_set_from(
        &mut self,
        self_: u32,
        location: LocationLocation,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-portal-event.player-portal-event.set-from")?;
            resolve_move_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setFrom(&move_event_location_data(location))
                        .then_some(())
                        .ok_or_else(not_found)
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_portal_event_get_to(
        &mut self,
        self_: u32,
    ) -> Result<Result<LocationLocation, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-portal-event.player-portal-event.get-to")?;
            resolve_move_event(self, self_)
                .map(|event| move_event_location(event.getTo()))
                .map_err(map_core_host_error)
        })())
    }

    fn player_portal_event_set_to(
        &mut self,
        self_: u32,
        location: LocationLocation,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-portal-event.player-portal-event.set-to")?;
            resolve_move_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setTo(&move_event_location_data(location))
                        .then_some(())
                        .ok_or_else(not_found)
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_portal_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-portal-event.player-portal-event.is-cancelled")?;
            resolve_move_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn player_portal_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "player-portal-event.player-portal-event.set-cancelled",
            )?;
            resolve_move_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setCancelled(cancelled)
                        .then_some(())
                        .ok_or_else(not_found)
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }
}
