//! Core ABI implementation for `native/bindings/endstone/actor/actor.h`.

use crate::host::endstone::support::*;

impl crate::core_host::imports::HostActor for PluginStoreState {
    fn actor_as_mob(&mut self, self_: u32) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.as-mob")?;
            let mob = resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|actor| actor.asMob())
            .map_err(map_core_host_error)?;
            (!mob.is_null())
                .then(|| self.insert_mob_resource(mob).map_err(map_core_host_error))
                .transpose()
        })())
    }

    fn actor_as_item_actor(
        &mut self,
        self_: u32,
    ) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.as-item-actor")?;
            let item_actor = resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|actor| actor.asItemActor())
            .map_err(map_core_host_error)?;
            (!item_actor.is_null())
                .then(|| {
                    self.insert_item_actor_resource(item_actor, self_)
                        .map_err(map_core_host_error)
                })
                .transpose()
        })())
    }

    fn actor_as_player(
        &mut self,
        self_: u32,
    ) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.as-player")?;
            let player = resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|actor| actor.asPlayer())
            .map_err(map_core_host_error)?;
            (!player.is_null())
                .then(|| {
                    self.insert_player_resource(player)
                        .map_err(map_core_host_error)
                })
                .transpose()
        })())
    }

    fn actor_get_runtime_id(&mut self, self_: u32) -> Result<Result<u64, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.get-runtime-id")?;
            resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|actor| actor.getRuntimeId())
            .map_err(map_core_host_error)
        })())
    }

    fn actor_get_id(&mut self, self_: u32) -> Result<Result<i64, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.get-id")?;
            resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|actor| actor.getId())
            .map_err(map_core_host_error)
        })())
    }

    fn actor_is_valid(&mut self, self_: u32) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.is-valid")?;
            resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|actor| actor.isValid())
            .map_err(map_core_host_error)
        })())
    }

    fn actor_is_dead(&mut self, self_: u32) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.is-dead")?;
            resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|actor| actor.isDead())
            .map_err(map_core_host_error)
        })())
    }

    fn actor_is_on_ground(&mut self, self_: u32) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.is-on-ground")?;
            resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|actor| actor.isOnGround())
            .map_err(map_core_host_error)
        })())
    }

    fn actor_is_in_water(&mut self, self_: u32) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.is-in-water")?;
            resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|actor| actor.isInWater())
            .map_err(map_core_host_error)
        })())
    }

    fn actor_is_in_lava(&mut self, self_: u32) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.is-in-lava")?;
            resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|actor| actor.isInLava())
            .map_err(map_core_host_error)
        })())
    }

    fn actor_get_actor_type(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.get-actor-type")?;
            resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|actor| actor.getType())
            .map_err(map_core_host_error)
        })())
    }

    fn actor_get_actor_name(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.get-actor-name")?;
            resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|actor| actor.getName())
            .map_err(map_core_host_error)
        })())
    }

    fn actor_get_actor_location(
        &mut self,
        self_: u32,
    ) -> Result<Result<LocationLocation, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.get-actor-location")?;
            let actor = resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?;
            let location = actor.getLocation();
            Ok(LocationLocation {
                dimension: location.dimension,
                x: location.x,
                y: location.y,
                z: location.z,
                pitch: location.pitch,
                yaw: location.yaw,
            })
        })())
    }

    fn actor_get_actor_dimension_location(
        &mut self,
        self_: u32,
    ) -> Result<Result<LocationLocation, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.get-actor-dimension-location")?;
            let actor = resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?;
            let location = actor.getDimensionLocation();
            Ok(LocationLocation {
                dimension: location.dimension,
                x: location.x,
                y: location.y,
                z: location.z,
                pitch: location.pitch,
                yaw: location.yaw,
            })
        })())
    }

    fn actor_get_actor_velocity(
        &mut self,
        self_: u32,
    ) -> Result<Result<VectorVector, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.get-actor-velocity")?;
            let actor = resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?;
            let velocity = actor.getVelocity();
            Ok(VectorVector {
                x: velocity.x,
                y: velocity.y,
                z: velocity.z,
            })
        })())
    }

    fn actor_get_actor_level_name(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.get-actor-level-name")?;
            resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|actor| actor.getLevelName())
            .map_err(map_core_host_error)
        })())
    }

    fn actor_is_name_tag_visible(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.is-name-tag-visible")?;
            resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|actor| actor.isNameTagVisible())
            .map_err(map_core_host_error)
        })())
    }

    fn actor_is_name_tag_always_visible(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.is-name-tag-always-visible")?;
            resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|actor| actor.isNameTagAlwaysVisible())
            .map_err(map_core_host_error)
        })())
    }

    fn actor_get_name_tag(&mut self, self_: u32) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.get-name-tag")?;
            resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|actor| actor.getNameTag())
            .map_err(map_core_host_error)
        })())
    }

    fn actor_get_score_tag(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.get-score-tag")?;
            resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|actor| actor.getScoreTag())
            .map_err(map_core_host_error)
        })())
    }

    fn actor_set_name_tag_visible(
        &mut self,
        self_: u32,
        visible: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.set-name-tag-visible")?;
            resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|actor| actor.setNameTagVisible(visible))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn actor_set_name_tag_always_visible(
        &mut self,
        self_: u32,
        always_visible: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.set-name-tag-always-visible")?;
            resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|actor| actor.setNameTagAlwaysVisible(always_visible))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn actor_set_name_tag(
        &mut self,
        self_: u32,
        name_tag: String,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.set-name-tag")?;
            resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|actor| actor.setNameTag(&name_tag))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn actor_set_score_tag(
        &mut self,
        self_: u32,
        score_tag: String,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.set-score-tag")?;
            resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|actor| actor.setScoreTag(&score_tag))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn actor_list_scoreboard_tags(
        &mut self,
        self_: u32,
    ) -> Result<Result<Vec<String>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.list-scoreboard-tags")?;
            resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|actor| actor.getScoreboardTags())
            .map_err(map_core_host_error)
        })())
    }

    fn actor_add_scoreboard_tag(
        &mut self,
        self_: u32,
        tag: String,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.add-scoreboard-tag")?;
            resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|actor| actor.addScoreboardTag(&tag))
            .map_err(map_core_host_error)
        })())
    }

    fn actor_remove_scoreboard_tag(
        &mut self,
        self_: u32,
        tag: String,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.remove-scoreboard-tag")?;
            resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|actor| actor.removeScoreboardTag(&tag))
            .map_err(map_core_host_error)
        })())
    }

    fn actor_set_rotation(
        &mut self,
        self_: u32,
        yaw: f32,
        pitch: f32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.set-rotation")?;
            resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|actor| actor.setRotation(yaw, pitch))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn actor_teleport(
        &mut self,
        self_: u32,
        location: LocationLocation,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.teleport")?;
            let actor = resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?;
            let location = cxx_actor::Location {
                x: location.x,
                y: location.y,
                z: location.z,
                pitch: location.pitch,
                yaw: location.yaw,
                dimension: location.dimension,
            };
            Ok(actor.teleport(&location))
        })())
    }

    fn actor_teleport_to_actor(
        &mut self,
        self_: u32,
        target: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.teleport-to-actor")?;
            let actor = resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?;
            let target = resolve_actor(
                self,
                actor_handle(self, target).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?;
            Ok(actor.teleportToActor(target))
        })())
    }

    fn actor_remove(&mut self, self_: u32) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.actor.remove")?;
            resolve_actor(
                self,
                actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|actor| actor.remove())
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn mob_as_actor(&mut self, self_: u32) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.mob.as-actor")?;
            let actor = resolve_mob(self, mob_handle(self, self_).map_err(map_core_host_error)?)
                .map(|mob| mob.asActor())
                .map_err(map_core_host_error)?;
            self.insert_actor_resource(actor)
                .map_err(map_core_host_error)
        })())
    }

    fn mob_is_gliding(&mut self, self_: u32) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.mob.is-gliding")?;
            resolve_mob(self, mob_handle(self, self_).map_err(map_core_host_error)?)
                .map(|mob| mob.isGliding())
                .map_err(map_core_host_error)
        })())
    }

    fn mob_get_health(&mut self, self_: u32) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.mob.get-health")?;
            resolve_mob(self, mob_handle(self, self_).map_err(map_core_host_error)?)
                .map(|mob| mob.getHealth())
                .map_err(map_core_host_error)
        })())
    }

    fn mob_get_max_health(&mut self, self_: u32) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.mob.get-max-health")?;
            resolve_mob(self, mob_handle(self, self_).map_err(map_core_host_error)?)
                .map(|mob| mob.getMaxHealth())
                .map_err(map_core_host_error)
        })())
    }

    fn mob_set_health(
        &mut self,
        self_: u32,
        health: i32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.mob.set-health")?;
            resolve_mob(self, mob_handle(self, self_).map_err(map_core_host_error)?)
                .map(|mob| mob.setHealth(health))
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn mob_set_max_health(
        &mut self,
        self_: u32,
        health: i32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.mob.set-max-health")?;
            resolve_mob(self, mob_handle(self, self_).map_err(map_core_host_error)?)
                .map(|mob| mob.setMaxHealth(health))
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn item_actor_as_actor(&mut self, self_: u32) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.item-actor.as-actor")?;
            let actor = resolve_item_actor(
                self,
                item_actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|item_actor| item_actor.asActor())
            .map_err(map_core_host_error)?;
            self.insert_actor_resource(actor)
                .map_err(map_core_host_error)
        })())
    }

    fn item_actor_get_item_stack(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.item-actor.get-item-stack")?;
            let item = resolve_item_actor(
                self,
                item_actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?
            .getItemStack();
            if item.is_null() {
                return Err(map_core_host_error(HostError::from_status(
                    AEGILEX_NOT_FOUND,
                )));
            }
            self.insert_item_stack_resource(item)
                .map_err(map_core_host_error)
        })())
    }

    fn item_actor_set_item_stack(
        &mut self,
        self_: u32,
        item: u32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.item-actor.set-item-stack")?;
            let item_handle = self
                .resource_slot(item, ResourceKind::ItemStack)
                .map(|slot| slot.handle)
                .map_err(map_core_host_error)?;
            let item = self
                .handles
                .item_stack(self.invocation_id, item_handle)
                .ok_or_else(|| map_core_host_error(HostError::from_status(AEGILEX_NOT_FOUND)))?;
            let updated = resolve_item_actor(
                self,
                item_actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?
            .setItemStack(item);
            updated.then_some(()).ok_or(TypesHostError::Unavailable)
        })())
    }

    fn item_actor_get_pickup_delay(
        &mut self,
        self_: u32,
    ) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.item-actor.get-pickup-delay")?;
            resolve_item_actor(
                self,
                item_actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|item| item.getPickupDelay())
            .map_err(map_core_host_error)
        })())
    }

    fn item_actor_set_pickup_delay(
        &mut self,
        self_: u32,
        delay: i32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.item-actor.set-pickup-delay")?;
            if delay < 0 {
                return Err(TypesHostError::InvalidInput);
            }
            let updated = resolve_item_actor(
                self,
                item_actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?
            .setPickupDelay(delay);
            updated.then_some(()).ok_or(TypesHostError::Unavailable)
        })())
    }

    fn item_actor_is_unlimited_lifetime(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.item-actor.is-unlimited-lifetime")?;
            resolve_item_actor(
                self,
                item_actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|item| item.isUnlimitedLifetime())
            .map_err(map_core_host_error)
        })())
    }

    fn item_actor_set_unlimited_lifetime(
        &mut self,
        self_: u32,
        unlimited: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.item-actor.set-unlimited-lifetime")?;
            let updated = resolve_item_actor(
                self,
                item_actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?
            .setUnlimitedLifetime(unlimited);
            updated.then_some(()).ok_or(TypesHostError::Unavailable)
        })())
    }

    fn item_actor_get_thrower(
        &mut self,
        self_: u32,
    ) -> Result<Result<Option<i64>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.item-actor.get-thrower")?;
            let mut has_thrower = false;
            let mut thrower = 0;
            let read = resolve_item_actor(
                self,
                item_actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?
            .getThrower(&mut has_thrower, &mut thrower);
            read.then_some(has_thrower.then_some(thrower))
                .ok_or(TypesHostError::Unavailable)
        })())
    }

    fn item_actor_set_thrower(
        &mut self,
        self_: u32,
        thrower: Option<i64>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.item-actor.set-thrower")?;
            let updated = resolve_item_actor(
                self,
                item_actor_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?
            .setThrower(thrower.is_some(), thrower.unwrap_or(0));
            updated.then_some(()).ok_or(TypesHostError::Unavailable)
        })())
    }

    fn player_as_actor(&mut self, self_: u32) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.as-actor")?;
            let actor = resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.asActor())
            .map_err(map_core_host_error)?;
            self.insert_actor_resource(actor)
                .map_err(map_core_host_error)
        })())
    }

    fn player_get_name(&mut self, self_: u32) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.get-name")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.getName())
            .map_err(map_core_host_error)
        })())
    }

    fn player_get_unique_id(
        &mut self,
        self_: u32,
    ) -> Result<Result<Vec<u8>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.get-unique-id")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.getUniqueId())
            .map_err(map_core_host_error)
        })())
    }

    fn player_get_xuid(&mut self, self_: u32) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.get-xuid")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.getXuid())
            .map_err(map_core_host_error)
        })())
    }

    fn player_is_operator(&mut self, self_: u32) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.is-operator")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.isOp())
            .map_err(map_core_host_error)
        })())
    }

    fn player_set_operator(
        &mut self,
        self_: u32,
        value: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.set-operator")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.setOp(value))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_get_ping(&mut self, self_: u32) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.get-ping")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.getPing())
            .map_err(map_core_host_error)
        })())
    }

    fn player_get_locale(&mut self, self_: u32) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.get-locale")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.getLocale())
            .map_err(map_core_host_error)
        })())
    }

    fn player_get_game_version(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.get-game-version")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.getGameVersion())
            .map_err(map_core_host_error)
        })())
    }

    fn player_get_device_os(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.get-device-os")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.getDeviceOS())
            .map_err(map_core_host_error)
        })())
    }

    fn player_get_device_id(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.get-device-id")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.getDeviceId())
            .map_err(map_core_host_error)
        })())
    }

    fn player_get_address(
        &mut self,
        self_: u32,
    ) -> Result<Result<crate::core_host::SocketAddressSocketAddress, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.get-address")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| {
                let address = player.getAddress();
                crate::core_host::SocketAddressSocketAddress {
                    hostname: address.hostname,
                    port: address.port,
                }
            })
            .map_err(map_core_host_error)
        })())
    }

    fn player_send_packet(
        &mut self,
        self_: u32,
        packet_id: i32,
        payload: Vec<u8>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.send-packet")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.sendPacket(packet_id, &payload))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_transfer(
        &mut self,
        self_: u32,
        host: String,
        port: u16,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.transfer")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.transfer(&host, port))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_kick(
        &mut self,
        self_: u32,
        message: String,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.kick")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.kick(&message))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_perform_command(
        &mut self,
        self_: u32,
        command: String,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.perform-command")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.performCommand(&command))
            .map_err(map_core_host_error)
        })())
    }

    fn player_update_commands(&mut self, self_: u32) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.update-commands")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.updateCommands())
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_is_sneaking(&mut self, self_: u32) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.is-sneaking")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.isSneaking())
            .map_err(map_core_host_error)
        })())
    }

    fn player_set_sneaking(
        &mut self,
        self_: u32,
        sneaking: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.set-sneaking")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.setSneaking(sneaking))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_is_sprinting(&mut self, self_: u32) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.is-sprinting")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.isSprinting())
            .map_err(map_core_host_error)
        })())
    }

    fn player_set_sprinting(
        &mut self,
        self_: u32,
        sprinting: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.set-sprinting")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.setSprinting(sprinting))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_get_exp_progress(
        &mut self,
        self_: u32,
    ) -> Result<Result<f32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.get-exp-progress")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.getExpProgress())
            .map_err(map_core_host_error)
        })())
    }

    fn player_set_exp_progress(
        &mut self,
        self_: u32,
        progress: f32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.set-exp-progress")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.setExpProgress(progress))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_get_exp_level(&mut self, self_: u32) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.get-exp-level")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.getExpLevel())
            .map_err(map_core_host_error)
        })())
    }

    fn player_set_exp_level(
        &mut self,
        self_: u32,
        level: i32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.set-exp-level")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.setExpLevel(level))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_get_total_exp(&mut self, self_: u32) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.get-total-exp")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.getTotalExp())
            .map_err(map_core_host_error)
        })())
    }

    fn player_give_exp(
        &mut self,
        self_: u32,
        amount: i32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.give-exp")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.giveExp(amount))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_give_exp_levels(
        &mut self,
        self_: u32,
        amount: i32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.give-exp-levels")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.giveExpLevels(amount))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_get_allow_flight(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.get-allow-flight")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.getAllowFlight())
            .map_err(map_core_host_error)
        })())
    }

    fn player_set_allow_flight(
        &mut self,
        self_: u32,
        allow: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.set-allow-flight")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.setAllowFlight(allow))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_is_flying(&mut self, self_: u32) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.is-flying")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.isFlying())
            .map_err(map_core_host_error)
        })())
    }

    fn player_set_flying(
        &mut self,
        self_: u32,
        flying: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.set-flying")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.setFlying(flying))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_get_fly_speed(&mut self, self_: u32) -> Result<Result<f32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.get-fly-speed")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.getFlySpeed())
            .map_err(map_core_host_error)
        })())
    }

    fn player_set_fly_speed(
        &mut self,
        self_: u32,
        speed: f32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.set-fly-speed")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.setFlySpeed(speed))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_get_walk_speed(&mut self, self_: u32) -> Result<Result<f32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.get-walk-speed")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.getWalkSpeed())
            .map_err(map_core_host_error)
        })())
    }

    fn player_set_walk_speed(
        &mut self,
        self_: u32,
        speed: f32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.set-walk-speed")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.setWalkSpeed(speed))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_send_message(
        &mut self,
        self_: u32,
        message: MessageMessage,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.send-message")?;
            let MessageMessage::PlainText(message) = message else {
                return Err(map_core_host_error(HostError::from_status(
                    AEGILEX_NOT_FOUND,
                )));
            };
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.sendMessage(&message))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_send_popup(
        &mut self,
        self_: u32,
        text: String,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.send-popup")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.sendPopup(&text))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_send_tip(
        &mut self,
        self_: u32,
        text: String,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.send-tip")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.sendTip(&text))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_send_toast(
        &mut self,
        self_: u32,
        title: String,
        content: String,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.send-toast")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.sendToast(&title, &content))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_send_title(
        &mut self,
        self_: u32,
        title: String,
        subtitle: String,
        fade_in: Option<i32>,
        stay: Option<i32>,
        fade_out: Option<i32>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.send-title")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| {
                player.sendTitle(
                    &title,
                    &subtitle,
                    fade_in.unwrap_or(-1),
                    stay.unwrap_or(-1),
                    fade_out.unwrap_or(-1),
                )
            })
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_reset_title(&mut self, self_: u32) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.reset-title")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.resetTitle())
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_play_sound(
        &mut self,
        self_: u32,
        location: LocationLocation,
        sound: String,
        volume: f32,
        pitch: f32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.play-sound")?;
            let location = cxx_player::Location {
                x: location.x,
                y: location.y,
                z: location.z,
                pitch: location.pitch,
                yaw: location.yaw,
                dimension: location.dimension,
            };
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.playSound(&location, &sound, volume, pitch))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_stop_sound(
        &mut self,
        self_: u32,
        sound: String,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.stop-sound")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.stopSound(&sound))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_stop_all_sounds(&mut self, self_: u32) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.stop-all-sounds")?;
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.stopAllSounds())
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_spawn_particle(
        &mut self,
        self_: u32,
        name: String,
        x: f32,
        y: f32,
        z: f32,
        molang_json: Option<String>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.spawn-particle")?;
            {
                let empty = String::new();
                let molang = molang_json.as_ref().unwrap_or(&empty);
                resolve_player(
                    self,
                    player_handle(self, self_).map_err(map_core_host_error)?,
                )
                .map(|player| player.spawnParticle(&name, x, y, z, molang))
                .map_err(map_core_host_error)?;
            }
            Ok(())
        })())
    }

    fn player_get_game_mode(
        &mut self,
        self_: u32,
    ) -> Result<Result<GameModeGameMode, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.get-game-mode")?;
            let mode = resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.getGameMode())
            .map_err(map_core_host_error)?;
            Ok(from_game_mode(mode))
        })())
    }

    fn player_set_game_mode(
        &mut self,
        self_: u32,
        mode: GameModeGameMode,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.set-game-mode")?;
            let mode = to_game_mode(mode);
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.setGameMode(mode))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_get_skin(
        &mut self,
        self_: u32,
    ) -> Result<Result<crate::core_host::SkinSkin, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.get-skin")?;
            let skin = resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.getSkin())
            .map_err(map_core_host_error)?;
            Ok(crate::core_host::SkinSkin {
                id: skin.id,
                image: crate::core_host::ImageImage {
                    width: skin.width,
                    height: skin.height,
                    depth: 4,
                    data: skin.pixels,
                },
                cape_id: None,
                cape_image: None,
            })
        })())
    }

    fn player_send_map(
        &mut self,
        self_: u32,
        map_id: i64,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.send-map")?;
            let map = resolve_server(self)
                .map_err(map_core_host_error)?
                .getMap(map_id);
            resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|player| player.sendMap(&map))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_get_inventory(&mut self, self_: u32) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.get-inventory")?;
            let player = resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?;
            let inventory = player.getInventory();
            if inventory.is_null() {
                return Err(map_core_host_error(HostError::from_status(
                    AEGILEX_NOT_FOUND,
                )));
            }
            self.insert_player_inventory_resource(inventory)
                .map_err(map_core_host_error)
        })())
    }

    fn player_get_ender_chest(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "actor.player.get-ender-chest")?;
            let player = resolve_player(
                self,
                player_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?;
            let inventory = player.getEnderChest();
            if inventory.is_null() {
                return Err(map_core_host_error(HostError::from_status(
                    AEGILEX_NOT_FOUND,
                )));
            }
            self.insert_inventory_resource(inventory)
                .map_err(map_core_host_error)
        })())
    }
    fn drop_actor(&mut self, handle: u32) -> Result<(), String> {
        self.drop_resource(handle, ResourceKind::Actor)
            .map_err(|error| format!("{error:?}"))
    }

    fn drop_mob(&mut self, handle: u32) -> Result<(), String> {
        self.drop_resource(handle, ResourceKind::Mob)
            .map_err(|error| format!("{error:?}"))
    }

    fn drop_item_actor(&mut self, handle: u32) -> Result<(), String> {
        self.drop_resource(handle, ResourceKind::ItemActor)
            .map_err(|error| format!("{error:?}"))
    }

    fn drop_player(&mut self, handle: u32) -> Result<(), String> {
        self.drop_resource(handle, ResourceKind::Player)
            .map_err(|error| format!("{error:?}"))
    }
}
