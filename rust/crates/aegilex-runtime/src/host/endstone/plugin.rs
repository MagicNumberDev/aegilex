//! Core ABI implementation for `native/bindings/endstone/plugin.h`.

use crate::host::endstone::support::*;

impl crate::core_host::imports::HostPluginContext for PluginStoreState {
    fn get_name(&mut self) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "plugin-context.get-name")?;
            plugin_context_plugin(self)
                .map(|plugin| plugin.getName())
                .map_err(map_core_host_error)
        })())
    }

    fn get_version(&mut self) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "plugin-context.get-version")?;
            plugin_context_plugin(self)
                .map(|plugin| plugin.getVersion())
                .map_err(map_core_host_error)
        })())
    }

    fn get_full_name(&mut self) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "plugin-context.get-full-name")?;
            plugin_context_plugin(self)
                .map(|plugin| plugin.getFullName())
                .map_err(map_core_host_error)
        })())
    }

    fn get_api_version(&mut self) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "plugin-context.get-api-version")?;
            plugin_context_plugin(self)
                .map(|plugin| plugin.getApiVersion())
                .map_err(map_core_host_error)
        })())
    }

    fn get_description(&mut self) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "plugin-context.get-description")?;
            plugin_context_plugin(self)
                .map(|plugin| plugin.getDescription())
                .map_err(map_core_host_error)
        })())
    }

    fn get_load_order(&mut self) -> Result<Result<PluginTypesLoadOrder, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "plugin-context.get-load-order")?;
            plugin_context_plugin(self)
                .and_then(|plugin| plugin_load_order(plugin.getLoadOrder()))
                .map_err(map_core_host_error)
        })())
    }

    fn list_authors(&mut self) -> Result<Result<Vec<String>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "plugin-context.list-authors")?;
            plugin_context_plugin(self)
                .map(|plugin| plugin.listAuthors())
                .map_err(map_core_host_error)
        })())
    }

    fn list_contributors(&mut self) -> Result<Result<Vec<String>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "plugin-context.list-contributors")?;
            plugin_context_plugin(self)
                .map(|plugin| plugin.listContributors())
                .map_err(map_core_host_error)
        })())
    }

    fn get_website(&mut self) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "plugin-context.get-website")?;
            plugin_context_plugin(self)
                .map(|plugin| plugin.getWebsite())
                .map_err(map_core_host_error)
        })())
    }

    fn get_prefix(&mut self) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "plugin-context.get-prefix")?;
            plugin_context_plugin(self)
                .map(|plugin| plugin.getPrefix())
                .map_err(map_core_host_error)
        })())
    }

    fn list_provides(&mut self) -> Result<Result<Vec<String>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "plugin-context.list-provides")?;
            plugin_context_plugin(self)
                .map(|plugin| plugin.listProvides())
                .map_err(map_core_host_error)
        })())
    }

    fn list_depend(&mut self) -> Result<Result<Vec<String>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "plugin-context.list-depend")?;
            plugin_context_plugin(self)
                .map(|plugin| plugin.listDepend())
                .map_err(map_core_host_error)
        })())
    }

    fn list_soft_depend(&mut self) -> Result<Result<Vec<String>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "plugin-context.list-soft-depend")?;
            plugin_context_plugin(self)
                .map(|plugin| plugin.listSoftDepend())
                .map_err(map_core_host_error)
        })())
    }

    fn list_load_before(&mut self) -> Result<Result<Vec<String>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "plugin-context.list-load-before")?;
            plugin_context_plugin(self)
                .map(|plugin| plugin.listLoadBefore())
                .map_err(map_core_host_error)
        })())
    }

    fn get_default_permission(
        &mut self,
    ) -> Result<Result<PermissionDefaultPermissionDefault, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "plugin-context.get-default-permission")?;
            plugin_context_plugin(self)
                .and_then(|plugin| permission_default(plugin.getDefaultPermission()))
                .map_err(map_core_host_error)
        })())
    }

    fn list_commands(&mut self) -> Result<Result<Vec<String>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "plugin-context.list-commands")?;
            plugin_context_plugin(self)
                .map(|plugin| plugin.listCommands())
                .map_err(map_core_host_error)
        })())
    }

    fn is_enabled(&mut self) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "plugin-context.is-enabled")?;
            plugin_context_plugin(self)
                .map(|plugin| plugin.isEnabled())
                .map_err(map_core_host_error)
        })())
    }

    fn get_data_folder(&mut self) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "plugin-context.get-data-folder")?;
            plugin_context_plugin(self)
                .map(|plugin| plugin.getDataFolder())
                .map_err(map_core_host_error)
        })())
    }

    fn list_loader_file_filters(&mut self) -> Result<Result<Vec<String>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "plugin-context.list-loader-file-filters")?;
            plugin_context_plugin(self)
                .map(|plugin| plugin.listLoaderFileFilters())
                .map_err(map_core_host_error)
        })())
    }

    fn get_command(&mut self, name: String) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "plugin-context.get-command")?;
            let command = plugin_context_plugin(self)
                .map_err(map_core_host_error)?
                .getCommand(&name);
            if command.is_null() {
                return Ok(None);
            }
            Ok(Some(
                self.insert_plugin_command_resource(command)
                    .map_err(map_core_host_error)?,
            ))
        })())
    }
}


fn plugin_info(info: cxx_event::PluginInfoData) -> PluginManagerPluginInfo {
    let metadata = info.metadata;
    PluginManagerPluginInfo {
        metadata: crate::core_host::PluginMetadataMetadata {
            name: metadata.name,
            version: metadata.version,
            description: metadata.description,
            load_order: match metadata.load_order {
                0 => PluginTypesLoadOrder::Startup,
                _ => PluginTypesLoadOrder::PostWorld,
            },
            authors: metadata.authors,
            contributors: metadata.contributors,
            website: metadata.website,
            prefix: metadata.prefix,
            provides: metadata.provides,
            depend: metadata.depend,
            soft_depend: metadata.soft_depend,
            load_before: metadata.load_before,
            default_permission: match metadata.default_permission {
                0 => PermissionDefaultPermissionDefault::True,
                1 => PermissionDefaultPermissionDefault::False,
                2 => PermissionDefaultPermissionDefault::Operator,
                3 => PermissionDefaultPermissionDefault::NotOperator,
                _ => PermissionDefaultPermissionDefault::Console,
            },
            commands: metadata
                .commands
                .into_iter()
                .map(|command| crate::core_host::PluginTypesCommand {
                    name: command.name,
                    description: (!command.description.is_empty()).then_some(command.description),
                    aliases: command.aliases,
                    usages: command.usages,
                    permissions: command.permissions,
                })
                .collect(),
            permissions: metadata
                .permissions
                .into_iter()
                .map(|permission| crate::core_host::PluginTypesPluginPermission {
                    name: permission.name,
                    description: (!permission.description.is_empty()).then_some(permission.description),
                    default_value: permission.has_default_value.then_some(match permission.default_value {
                        0 => PermissionDefaultPermissionDefault::True,
                        1 => PermissionDefaultPermissionDefault::False,
                        2 => PermissionDefaultPermissionDefault::Operator,
                        3 => PermissionDefaultPermissionDefault::NotOperator,
                        _ => PermissionDefaultPermissionDefault::Console,
                    }),
                    children: permission
                        .children
                        .into_iter()
                        .map(|child| PermissionAttachmentPermissionChild {
                            name: child.name,
                            value: child.value,
                        })
                        .collect(),
                })
                .collect(),
            subscriptions: metadata.subscriptions,
        },
        enabled: info.enabled,
    }
}

impl crate::core_host::imports::HostPluginManager for PluginStoreState {
    fn list_plugins(
        &mut self,
    ) -> Result<Result<Vec<PluginManagerPluginInfo>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "plugin-manager.list-plugins")?;
            let result = native::plugin_manager_list(&self.host).map_err(map_core_host_error)?;
            Ok(result.plugins.into_iter().map(plugin_info).collect())
        })())
    }

    fn get(
        &mut self,
        name: String,
    ) -> Result<Result<Option<PluginManagerPluginInfo>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "plugin-manager.get")?;
            let info = native::plugin_manager_get(&self.host, &name).map_err(map_core_host_error)?;
            if info.metadata.name.is_empty() {
                return Ok(None);
            }
            Ok(Some(plugin_info(info)))
        })())
    }

    fn enable(
        &mut self,
        name: String,
    ) -> Result<Result<PluginManagerPluginInfo, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "plugin-manager.enable")?;
            native::plugin_manager_enable(&self.host, &name).map_err(map_core_host_error)?;
            let info = native::plugin_manager_get(&self.host, &name).map_err(map_core_host_error)?;
            if info.metadata.name.is_empty() {
                return Err(map_core_host_error(not_found()));
            }
            Ok(plugin_info(info))
        })())
    }

    fn disable(
        &mut self,
        name: String,
    ) -> Result<Result<PluginManagerPluginInfo, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "plugin-manager.disable")?;
            native::plugin_manager_disable(&self.host, &name).map_err(map_core_host_error)?;
            let info = native::plugin_manager_get(&self.host, &name).map_err(map_core_host_error)?;
            if info.metadata.name.is_empty() {
                return Err(map_core_host_error(not_found()));
            }
            Ok(plugin_info(info))
        })())
    }
}
