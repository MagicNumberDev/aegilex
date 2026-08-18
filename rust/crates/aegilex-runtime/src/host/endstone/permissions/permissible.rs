//! Core ABI implementation for `native/bindings/endstone/permissions/permissible.h`.

use crate::host::endstone::support::*;

impl crate::core_host::imports::HostPermissible for PluginStoreState {
    fn permissible_get_permission_level(
        &mut self,
        self_: u32,
    ) -> Result<Result<PermissionLevelPermissionLevel, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "permissible.permissible.get-permission-level")?;
            resolve_permissible(
                self,
                permissible_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|permissible| match permissible.getPermissionLevel() {
                cxx_common::PermissionLevel::Default => PermissionLevelPermissionLevel::Default,
                cxx_common::PermissionLevel::Operator => PermissionLevelPermissionLevel::Operator,
                cxx_common::PermissionLevel::Console => PermissionLevelPermissionLevel::Console,
                _ => PermissionLevelPermissionLevel::Default,
            })
            .map_err(map_core_host_error)
        })())
    }

    fn permissible_has_permission(
        &mut self,
        self_: u32,
        permission: String,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "permissible.permissible.has-permission")?;
            resolve_permissible(
                self,
                permissible_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|permissible| permissible.hasPermission(&permission))
            .map_err(map_core_host_error)
        })())
    }

    fn permissible_is_permission_set(
        &mut self,
        self_: u32,
        permission: String,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "permissible.permissible.is-permission-set")?;
            resolve_permissible(
                self,
                permissible_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|permissible| permissible.isPermissionSet(&permission))
            .map_err(map_core_host_error)
        })())
    }

    fn permissible_recalculate_permissions(
        &mut self,
        self_: u32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "permissible.permissible.recalculate-permissions")?;
            resolve_permissible(
                self,
                permissible_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|permissible| permissible.recalculatePermissions())
            .map_err(map_core_host_error)
        })())
    }

    fn permissible_get_command_sender(
        &mut self,
        self_: u32,
    ) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "permissible.permissible.get-command-sender")?;
            let sender = resolve_permissible(
                self,
                permissible_handle(self, self_).map_err(map_core_host_error)?,
            )
            .and_then(|permissible| {
                resolve_server(self).map(|server| cxx_common::asCommandSender(permissible, server))
            })
            .map_err(map_core_host_error)?;
            (!sender.is_null())
                .then(|| {
                    self.insert_command_sender_resource(sender)
                        .map_err(map_core_host_error)
                })
                .transpose()
        })())
    }

    fn permissible_attach(
        &mut self,
        self_: u32,
        name: String,
        value: bool,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "permissible.permissible.attach")?;
            let attachment = resolve_permissible(
                self,
                permissible_handle(self, self_).map_err(map_core_host_error)?,
            )
            .and_then(|permissible| {
                resolve_server(self).map(|server| permissible.attach(server, &name, value))
            })
            .map_err(map_core_host_error)?;
            self.insert_permission_attachment_resource(attachment)
                .map_err(map_core_host_error)
        })())
    }

    fn permissible_attach_empty(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "permissible.permissible.attach-empty")?;
            let attachment = resolve_permissible(
                self,
                permissible_handle(self, self_).map_err(map_core_host_error)?,
            )
            .and_then(|permissible| {
                resolve_server(self).map(|server| permissible.attachEmpty(server))
            })
            .map_err(map_core_host_error)?;
            self.insert_permission_attachment_resource(attachment)
                .map_err(map_core_host_error)
        })())
    }

    fn permissible_list_effective(
        &mut self,
        self_: u32,
    ) -> Result<Result<Vec<PermissibleEffectivePermission>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "permissible.permissible.list-effective")?;
            let handle = permissible_handle(self, self_).map_err(map_core_host_error)?;
            let permissions = resolve_permissible(self, handle)
                .map(|permissible| permissible.listEffectivePermissionNames())
                .map_err(map_core_host_error)?;
            permissions
                .into_iter()
                .map(|name| {
                    let name = name.to_string();
                    let result = resolve_permissible(self, handle)
                        .map(|permissible| permissible.getEffectivePermission(&name))
                        .map_err(map_core_host_error)?;
                    let attachment = resolve_permissible(self, handle)
                        .map(|permissible| permissible.getEffectiveAttachment(&name))
                        .map_err(map_core_host_error)?;
                    let attachment = (!attachment.is_null())
                        .then(|| {
                            self.insert_permission_attachment_resource(attachment)
                                .map_err(map_core_host_error)
                        })
                        .transpose()?;
                    Ok(PermissibleEffectivePermission {
                        name,
                        value: result.value,
                        attachment,
                    })
                })
                .collect()
        })())
    }

    fn get(&mut self, sender: u32) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "permissible.get")?;
            let permissible = resolve_sender(
                self,
                sender_handle(self, sender).map_err(map_core_host_error)?,
            )
            .map(cxx_common::permissible_from_command_sender)
            .map_err(map_core_host_error)?;
            self.insert_permissible_resource(permissible)
                .map_err(map_core_host_error)
        })())
    }
}
