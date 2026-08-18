use super::*;

impl Runtime {
    pub(crate) fn should_dispatch_event(&self, plugin_id: &str, subscription: &str) -> bool {
        self.plugins
            .iter()
            .find(|plugin| plugin.id == plugin_id)
            .is_some_and(|plugin| {
                plugin.enabled && plugin.subscriptions.iter().any(|item| item == subscription)
            })
    }

    pub(crate) fn register_command_sender(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        sender: cxx::UniquePtr<cxx_common::CommandSender>,
    ) -> Result<u32, u32> {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return Err(AEGILEX_NOT_FOUND);
        };
        if !plugin.enabled || sender.is_null() {
            return Err(AEGILEX_NOT_FOUND);
        }
        let state = plugin.store.data_mut();
        state.push_invocation(invocation_id);
        let resource = state
            .insert_command_sender_resource(sender)
            .map_err(|error| error.status());
        if resource.is_err() {
            state.clear_invocation_resources(invocation_id);
        }
        state.pop_invocation();
        resource
    }

    pub(crate) fn discard_invocation_handles(&mut self, plugin_id: &str, invocation_id: u64) {
        if let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
        }
    }

    pub(crate) fn dispatch_player_join_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::PlayerJoinEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "player-join")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }
        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_player_join_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::PlayerJoin(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_player_quit_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::PlayerQuitEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "player-quit")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }
        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_player_quit_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::PlayerQuit(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_player_chat_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::PlayerChatEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "player-chat")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_player_chat_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::PlayerChat(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_actor_damage_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::ActorDamageEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "actor-damage")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_actor_damage_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::ActorDamage(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_actor_death_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::ActorDeathEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "actor-death")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_actor_death_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::ActorDeath(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_actor_explode_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::ActorExplodeEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "actor-explode")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_actor_explode_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::ActorExplode(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_block_explode_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::BlockExplodeEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "block-explode")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_block_explode_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::BlockExplode(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_actor_knockback_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::ActorKnockbackEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "actor-knockback")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_actor_knockback_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::ActorKnockback(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_actor_remove_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::ActorRemoveEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "actor-remove")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_actor_remove_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::ActorRemove(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_actor_spawn_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::ActorSpawnEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "actor-spawn")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_actor_spawn_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::ActorSpawn(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_actor_teleport_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::ActorTeleportEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "actor-teleport")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_actor_teleport_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::ActorTeleport(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_player_move_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        subscription: &str,
        facade: cxx::UniquePtr<cxx_event::PlayerMoveEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !matches!(
                subscription,
                "player-move" | "player-teleport" | "player-jump" | "player-portal"
            )
            || !plugin
                .subscriptions
                .iter()
                .any(|value| value == subscription)
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_player_move_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let event_rep = event;
        let event = match subscription {
            "player-move" => crate::core_host::EventsEvent::PlayerMove(event_rep),
            "player-teleport" => crate::core_host::EventsEvent::PlayerTeleport(event_rep),
            "player-jump" => crate::core_host::EventsEvent::PlayerJump(event_rep),
            "player-portal" => crate::core_host::EventsEvent::PlayerPortal(event_rep),
            _ => unreachable!(),
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin
                .exports
                .call_events_on_event(&plugin.instance, store, event)
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_player_game_mode_change_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::PlayerGameModeChangeEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "player-game-mode-change")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_player_game_mode_change_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::PlayerGameModeChange(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_player_emote_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::PlayerEmoteEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "player-emote")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_player_emote_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::PlayerEmote(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_player_skin_change_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::PlayerSkinChangeEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "player-skin-change")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_player_skin_change_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::PlayerSkinChange(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_player_death_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::PlayerDeathEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "player-death")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_player_death_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::PlayerDeath(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_player_dimension_change_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::PlayerDimensionChangeEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "player-dimension-change")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_player_dimension_change_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::PlayerDimensionChange(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_player_bed_enter_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::PlayerBedEnterEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "player-bed-enter")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_player_bed_enter_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::PlayerBedEnter(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_player_bed_leave_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::PlayerBedLeaveEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "player-bed-leave")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_player_bed_leave_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::PlayerBedLeave(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_player_respawn_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::PlayerRespawnEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "player-respawn")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_player_respawn_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::PlayerRespawn(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_player_item_held_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::PlayerItemHeldEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "player-item-held")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_player_item_held_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::PlayerItemHeld(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_player_pickup_item_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::PlayerPickupItemEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "player-pickup-item")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_player_pickup_item_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::PlayerPickupItem(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_player_drop_item_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::PlayerDropItemEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "player-drop-item")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_player_drop_item_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::PlayerDropItem(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_leaves_decay_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::LeavesDecayEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "leaves-decay")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }
        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_leaves_decay_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::LeavesDecay(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_block_from_to_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::BlockFromToEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "block-from-to")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }
        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_block_from_to_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::BlockFromTo(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_block_grow_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        subscription: &str,
        facade: cxx::UniquePtr<cxx_event::BlockGrowEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|value| value == subscription)
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }
        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_block_grow_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let event_variant = if subscription == "block-form" {
            crate::core_host::EventsEvent::BlockForm(event)
        } else {
            crate::core_host::EventsEvent::BlockGrow(event)
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin
                .exports
                .call_events_on_event(&plugin.instance, store, event_variant)
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_block_piston_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        subscription: &str,
        facade: cxx::UniquePtr<cxx_event::BlockPistonEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|value| value == subscription)
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }
        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_block_piston_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let event_variant = if subscription == "block-piston-extend" {
            crate::core_host::EventsEvent::BlockPistonExtend(event)
        } else {
            crate::core_host::EventsEvent::BlockPistonRetract(event)
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin
                .exports
                .call_events_on_event(&plugin.instance, store, event_variant)
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_block_break_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::BlockBreakEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "block-break")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_block_break_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::BlockBreak(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_block_cook_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::BlockCookEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "block-cook")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_block_cook_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::BlockCook(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_block_place_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::BlockPlaceEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "block-place")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_block_place_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::BlockPlace(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_player_interact_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::PlayerInteractEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "player-interact")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_player_interact_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::PlayerInteract(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_player_interact_actor_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::PlayerInteractActorEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "player-interact-actor")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_player_interact_actor_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::PlayerInteractActor(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_player_item_consume_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::PlayerItemConsumeEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "player-item-consume")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_player_item_consume_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::PlayerItemConsume(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_broadcast_message_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::BroadcastMessageEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "broadcast-message")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_broadcast_message_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::BroadcastMessage(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_packet_send_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::PacketSendEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "packet-send")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_packet_send_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::PacketSend(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_packet_receive_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::PacketReceiveEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "packet-receive")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_packet_receive_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::PacketReceive(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_map_initialize_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::MapInitializeEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "map-initialize")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_map_initialize_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::MapInitialize(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_script_message_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::ScriptMessageEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "script-message")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_script_message_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::ScriptMessage(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_weather_change_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::WeatherChangeEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "weather-change")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_weather_change_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::WeatherChange(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_thunder_change_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::ThunderChangeEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "thunder-change")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_thunder_change_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::ThunderChange(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_player_kick_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::PlayerKickEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "player-kick")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_player_kick_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::PlayerKick(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_player_login_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::PlayerLoginEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "player-login")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_player_login_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::PlayerLogin(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_player_command_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::PlayerCommandEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "player-command")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_player_command_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::PlayerCommand(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_server_command_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::ServerCommandEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "server-command")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_server_command_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::ServerCommand(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_server_list_ping_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::ServerListPingEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "server-list-ping")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_server_list_ping_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::ServerListPing(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_plugin_enable_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::PluginLifecycleEventFacade>,
    ) -> bool {
        self.dispatch_plugin_lifecycle_event(plugin_id, invocation_id, facade, true)
    }

    pub(crate) fn dispatch_plugin_disable_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::PluginLifecycleEventFacade>,
    ) -> bool {
        self.dispatch_plugin_lifecycle_event(plugin_id, invocation_id, facade, false)
    }

    fn dispatch_plugin_lifecycle_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::PluginLifecycleEventFacade>,
        enabled: bool,
    ) -> bool {
        let subscription = if enabled {
            "plugin-enable"
        } else {
            "plugin-disable"
        };
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|candidate| candidate == subscription)
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_plugin_lifecycle_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let callback_event = if enabled {
            crate::core_host::EventsEvent::PluginEnable(event)
        } else {
            crate::core_host::EventsEvent::PluginDisable(event)
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin
                .exports
                .call_events_on_event(&plugin.instance, store, callback_event)
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_server_load_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        facade: cxx::UniquePtr<cxx_event::ServerLoadEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|subscription| subscription == "server-load")
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin
            .store
            .data_mut()
            .insert_server_load_event_resource(facade)
        {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin.exports.call_events_on_event(
                &plugin.instance,
                store,
                crate::core_host::EventsEvent::ServerLoad(event),
            )
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_chunk_event(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        subscription: &str,
        facade: cxx::UniquePtr<cxx_event::ChunkEventFacade>,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        if !matches!(subscription, "chunk-load" | "chunk-unload")
            || !plugin.enabled
            || !plugin
                .subscriptions
                .iter()
                .any(|configured| configured == subscription)
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return false;
        }

        plugin.store.data_mut().push_invocation(invocation_id);
        let event = match plugin.store.data_mut().insert_chunk_event_resource(facade) {
            Ok(event) => event,
            Err(_) => {
                let state = plugin.store.data_mut();
                state.clear_invocation_resources(invocation_id);
                state.pop_invocation();
                return false;
            }
        };
        let callback_event = match subscription {
            "chunk-load" => crate::core_host::EventsEvent::ChunkLoad(event),
            "chunk-unload" => crate::core_host::EventsEvent::ChunkUnload(event),
            _ => unreachable!("subscription was checked above"),
        };
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin
                .exports
                .call_events_on_event(&plugin.instance, store, callback_event)
        });
        let state = plugin.store.data_mut();
        let dispatched = match outcome {
            Ok(Ok(_)) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-event trapped: {error}", plugin.id),
                );
                false
            }
        };
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        dispatched
    }

    pub(crate) fn dispatch_wit_command(
        &mut self,
        plugin_id: &str,
        invocation_id: u64,
        invocation: crate::core_host::CommandsInvocation,
    ) -> Result<crate::core_host::CommandsOutcome, u32> {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return Err(AEGILEX_NOT_FOUND);
        };
        if !plugin.enabled {
            return Err(AEGILEX_NOT_FOUND);
        }
        if !plugin
            .commands
            .iter()
            .any(|spec| spec.name == invocation.subcommand)
        {
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return Ok(crate::core_host::CommandsOutcome {
                handled: false,
                reply: None,
                error: None,
            });
        }

        if let Err(error) = plugin.store.set_fuel(ENABLE_FUEL) {
            log_loader_error(
                &self.host,
                &format!("{}: cannot reset fuel: {error}", plugin.id),
            );
            plugin
                .store
                .data_mut()
                .clear_invocation_resources(invocation_id);
            return Err(AEGILEX_INTERNAL_ERROR);
        }
        plugin.store.data_mut().push_invocation(invocation_id);
        let outcome = call_guest(&mut plugin.store, |store| {
            plugin
                .exports
                .call_commands_on_command(&plugin.instance, store, invocation)
        });
        let state = plugin.store.data_mut();
        state.clear_invocation_resources(invocation_id);
        state.pop_invocation();
        match outcome {
            Ok(Ok(outcome)) => Ok(crate::core_host::CommandsOutcome {
                handled: outcome.handled,
                reply: outcome.reply,
                error: outcome.error,
            }),
            Ok(Err(text)) => Ok(crate::core_host::CommandsOutcome {
                handled: true,
                reply: None,
                error: Some(text),
            }),
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-command trapped: {error}", plugin.id),
                );
                Err(AEGILEX_TRAP)
            }
        }
    }
}
