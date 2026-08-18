//! Core ABI implementation for `native/bindings/endstone/server.h`.

use crate::host::endstone::support::*;

fn resolve_server(
    state: &PluginStoreState,
) -> Result<&crate::cxx_host_server::ffi::Server, HostError> {
    state.host.server()
}

impl crate::core_host::imports::HostServer for PluginStoreState {
    fn get_name(&mut self) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "server.get-name")?;
            resolve_server(self)
                .map(|server| server.getName())
                .map_err(map_core_host_error)
        })())
    }

    fn get_version(&mut self) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "server.get-version")?;
            resolve_server(self)
                .map(|server| server.getVersion())
                .map_err(map_core_host_error)
        })())
    }

    fn get_minecraft_version(&mut self) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "server.get-minecraft-version")?;
            resolve_server(self)
                .map(|server| server.getMinecraftVersion())
                .map_err(map_core_host_error)
        })())
    }

    fn get_protocol_version(&mut self) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "server.get-protocol-version")?;
            resolve_server(self)
                .map(|server| server.getProtocolVersion())
                .map_err(map_core_host_error)
        })())
    }

    fn get_max_players(&mut self) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "server.get-max-players")?;
            resolve_server(self)
                .map(|server| server.getMaxPlayers())
                .map_err(map_core_host_error)
        })())
    }

    fn set_max_players(&mut self, max: i32) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "server.set-max-players")?;
            let server = resolve_server(self).map_err(map_core_host_error)?;
            crate::host::runtime::native::status_result(server.setMaxPlayers(max))
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn get_port(&mut self) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "server.get-port")?;
            resolve_server(self)
                .map(|server| server.getPort())
                .map_err(map_core_host_error)
        })())
    }

    fn get_port_v6(&mut self) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "server.get-port-v6")?;
            resolve_server(self)
                .map(|server| server.getPortV6())
                .map_err(map_core_host_error)
        })())
    }

    fn get_online_mode(&mut self) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "server.get-online-mode")?;
            resolve_server(self)
                .map(|server| server.getOnlineMode())
                .map_err(map_core_host_error)
        })())
    }

    fn is_primary_thread(&mut self) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "server.is-primary-thread")?;
            resolve_server(self)
                .map(|server| server.isPrimaryThread())
                .map_err(map_core_host_error)
        })())
    }

    fn get_current_ms_per_tick(&mut self) -> Result<Result<f32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "server.get-current-ms-per-tick")?;
            resolve_server(self)
                .map(|server| server.getCurrentMillisecondsPerTick())
                .map_err(map_core_host_error)
        })())
    }

    fn get_average_ms_per_tick(&mut self) -> Result<Result<f32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "server.get-average-ms-per-tick")?;
            resolve_server(self)
                .map(|server| server.getAverageMillisecondsPerTick())
                .map_err(map_core_host_error)
        })())
    }

    fn get_current_tps(&mut self) -> Result<Result<f32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "server.get-current-tps")?;
            resolve_server(self)
                .map(|server| server.getCurrentTicksPerSecond())
                .map_err(map_core_host_error)
        })())
    }

    fn get_average_tps(&mut self) -> Result<Result<f32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "server.get-average-tps")?;
            resolve_server(self)
                .map(|server| server.getAverageTicksPerSecond())
                .map_err(map_core_host_error)
        })())
    }

    fn get_current_tick_usage(&mut self) -> Result<Result<f32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "server.get-current-tick-usage")?;
            resolve_server(self)
                .map(|server| server.getCurrentTickUsage())
                .map_err(map_core_host_error)
        })())
    }

    fn get_average_tick_usage(&mut self) -> Result<Result<f32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "server.get-average-tick-usage")?;
            resolve_server(self)
                .map(|server| server.getAverageTickUsage())
                .map_err(map_core_host_error)
        })())
    }

    fn get_start_time_ms(&mut self) -> Result<Result<i64, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "server.get-start-time-ms")?;
            resolve_server(self)
                .map(|server| server.getStartTimeMilliseconds())
                .map_err(map_core_host_error)
        })())
    }

    fn list_online_players(&mut self) -> Result<Result<Vec<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "server.list-online-players")?;
            let server = resolve_server(self).map_err(map_core_host_error)?;
            let mut players = server.listOnlinePlayers();
            if players.is_null() {
                return Err(map_core_host_error(HostError::from_status(
                    AEGILEX_NOT_FOUND,
                )));
            }
            let mut resources = Vec::with_capacity(players.len());
            for index in 0..players.len() {
                resources.push(
                    self.insert_player_resource(players.pin_mut().takePlayer(index))
                        .map_err(map_core_host_error)?,
                );
            }
            Ok(resources)
        })())
    }

    fn find_player_by_name(
        &mut self,
        name: String,
    ) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "server.find-player-by-name")?;
            let server = resolve_server(self).map_err(map_core_host_error)?;
            let player = server.findPlayerByName(&name);
            if player.is_null() {
                return Ok(None);
            }
            Ok(Some(
                self.insert_player_resource(player)
                    .map_err(map_core_host_error)?,
            ))
        })())
    }

    fn find_player_by_uuid(
        &mut self,
        id: Vec<u8>,
    ) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "server.find-player-by-uuid")?;
            let id: [u8; 16] = id.try_into().map_err(|_| TypesHostError::InvalidInput)?;
            let server = resolve_server(self).map_err(map_core_host_error)?;
            let player = server.findPlayerByUuid(&id);
            if player.is_null() {
                return Ok(None);
            }
            Ok(Some(
                self.insert_player_resource(player)
                    .map_err(map_core_host_error)?,
            ))
        })())
    }

    fn broadcast(
        &mut self,
        message: crate::core_host::MessageMessage,
        permission: Option<String>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "server.broadcast")?;
            let crate::core_host::MessageMessage::PlainText(message) = message else {
                return Err(TypesHostError::NotFound);
            };
            let server = resolve_server(self).map_err(map_core_host_error)?;
            let (has_permission, permission) =
                permission.map_or((false, String::new()), |value| (true, value));
            crate::host::runtime::native::status_result(server.broadcast(
                &message,
                has_permission,
                &permission,
            ))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn dispatch_console_command(
        &mut self,
        command_line: String,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "server.dispatch-console-command")?;
            resolve_server(self)
                .map(|server| server.dispatchConsoleCommand(&command_line))
                .map_err(map_core_host_error)
        })())
    }

    fn get_command_sender(&mut self) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "server.get-command-sender")?;
            let server = resolve_server(self).map_err(map_core_host_error)?;
            let sender = server.getCommandSender();
            self.insert_command_sender_resource(sender)
                .map_err(map_core_host_error)
        })())
    }

    fn get_plugin_command(
        &mut self,
        name: String,
    ) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "server.get-plugin-command")?;
            let server = resolve_server(self).map_err(map_core_host_error)?;
            let command = server.getPluginCommand(&name);
            if command.is_null() {
                return Ok(None);
            }
            Ok(Some(
                self.insert_plugin_command_resource(command)
                    .map_err(map_core_host_error)?,
            ))
        })())
    }

    fn registry_item_type_get(
        &mut self,
        type_id: String,
    ) -> Result<Result<Option<crate::core_host::ItemTypeItemTypeData>, TypesHostError>, String>
    {
        Ok((|| {
            check_capability(self, "server.registry-item-type-get")?;
            let server = resolve_server(self).map_err(map_core_host_error)?;
            let item_type = server.getRegistryItemType(&type_id);
            if item_type.is_null() {
                return Ok(None);
            }
            Ok(Some(crate::core_host::ItemTypeItemTypeData {
                type_id: item_type.getId(),
                translation_key: item_type.getTranslationKey(),
                max_stack_size: item_type.getMaxStackSize(),
                max_durability: item_type.getMaxDurability(),
            }))
        })())
    }

    fn registry_item_type_list(
        &mut self,
    ) -> Result<Result<Vec<crate::core_host::ItemTypeItemTypeData>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "server.registry-item-type-list")?;
            let server = resolve_server(self).map_err(map_core_host_error)?;
            let item_types = server.listRegistryItemTypes();
            if item_types.is_null() {
                return Err(map_core_host_error(HostError::from_status(
                    AEGILEX_NOT_FOUND,
                )));
            }
            let mut items = Vec::with_capacity(item_types.len());
            for index in 0..item_types.len() {
                let item_type = item_types.takeItemType(index);
                if item_type.is_null() {
                    return Err(map_core_host_error(HostError::from_status(
                        AEGILEX_NOT_FOUND,
                    )));
                }
                items.push(crate::core_host::ItemTypeItemTypeData {
                    type_id: item_type.getId(),
                    translation_key: item_type.getTranslationKey(),
                    max_stack_size: item_type.getMaxStackSize(),
                    max_durability: item_type.getMaxDurability(),
                });
            }
            Ok(items)
        })())
    }

    fn registry_enchantment_get(
        &mut self,
        id: String,
    ) -> Result<Result<Option<crate::core_host::ServerEnchantmentData>, TypesHostError>, String>
    {
        Ok((|| {
            check_capability(self, "server.registry-enchantment-get")?;
            let server = resolve_server(self).map_err(map_core_host_error)?;
            let enchantment = server.getRegistryEnchantment(&id);
            if enchantment.is_null() {
                return Ok(None);
            }
            Ok(Some(crate::core_host::ServerEnchantmentData {
                id: enchantment.getId(),
                translation_key: enchantment.getTranslationKey(),
                max_level: enchantment.getMaxLevel(),
                start_level: enchantment.getStartLevel(),
            }))
        })())
    }

    fn registry_enchantment_list(
        &mut self,
    ) -> Result<Result<Vec<crate::core_host::ServerEnchantmentData>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "server.registry-enchantment-list")?;
            let server = resolve_server(self).map_err(map_core_host_error)?;
            let enchantments = server.listRegistryEnchantments();
            if enchantments.is_null() {
                return Err(map_core_host_error(HostError::from_status(
                    AEGILEX_NOT_FOUND,
                )));
            }
            let mut items = Vec::with_capacity(enchantments.len());
            for index in 0..enchantments.len() {
                let enchantment = enchantments.takeEnchantment(index);
                if enchantment.is_null() {
                    return Err(map_core_host_error(HostError::from_status(
                        AEGILEX_NOT_FOUND,
                    )));
                }
                items.push(crate::core_host::ServerEnchantmentData {
                    id: enchantment.getId(),
                    translation_key: enchantment.getTranslationKey(),
                    max_level: enchantment.getMaxLevel(),
                    start_level: enchantment.getStartLevel(),
                });
            }
            Ok(items)
        })())
    }
}
