//! Core ABI implementation for `native/bindings/endstone/permissions/permission_attachment.h`.

use crate::host::endstone::support::*;

impl crate::core_host::imports::HostPermissionAttachment for PluginStoreState {
    fn permission_attachment_detach(
        &mut self,
        self_: u32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "permission-attachment.permission-attachment.detach")?;
            let handle = attachment_handle(self, self_).map_err(map_core_host_error)?;
            let aliases = resolve_attachment(self, handle)
                .map(|attachment| {
                    self.handles
                        .permission_attachment_aliases(self.invocation_id, attachment)
                })
                .map_err(map_core_host_error)?;
            let removed = resolve_attachment(self, handle)
                .map(|attachment| attachment.remove())
                .map_err(map_core_host_error)?;
            if !removed {
                return Err(TypesHostError::NotFound);
            }
            self.handles.remove_handles(&aliases);
            Ok(())
        })())
    }

    fn permission_attachment_set(
        &mut self,
        self_: u32,
        permission: String,
        value: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "permission-attachment.permission-attachment.set")?;
            resolve_attachment(
                self,
                attachment_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|attachment| attachment.setPermission(&permission, value))
            .map_err(map_core_host_error)
        })())
    }

    fn permission_attachment_unset(
        &mut self,
        self_: u32,
        permission: String,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "permission-attachment.permission-attachment.unset")?;
            resolve_attachment(
                self,
                attachment_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|attachment| attachment.unsetPermission(&permission))
            .map_err(map_core_host_error)
        })())
    }

    fn permission_attachment_list_permissions(
        &mut self,
        self_: u32,
    ) -> Result<Result<Vec<PermissionAttachmentPermissionChild>, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "permission-attachment.permission-attachment.list-permissions",
            )?;
            resolve_attachment(
                self,
                attachment_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|attachment| {
                attachment
                    .getPermissions()
                    .into_iter()
                    .filter_map(|name| {
                        let value = attachment.getPermissionValue(&name);
                        value.has.then_some(PermissionAttachmentPermissionChild {
                            name,
                            value: value.value,
                        })
                    })
                    .collect()
            })
            .map_err(map_core_host_error)
        })())
    }

    fn permission_attachment_get_value(
        &mut self,
        self_: u32,
        permission: String,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "permission-attachment.permission-attachment.get-value",
            )?;
            let value = resolve_attachment(
                self,
                attachment_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|attachment| attachment.getPermissionValue(&permission))
            .map_err(map_core_host_error)?;
            value
                .has
                .then_some(value.value)
                .ok_or(TypesHostError::NotFound)
        })())
    }
}
