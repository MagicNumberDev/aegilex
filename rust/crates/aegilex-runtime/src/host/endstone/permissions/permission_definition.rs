//! Core ABI implementation for `native/bindings/endstone/permissions/permission_definition.h`.

use crate::host::endstone::support::*;

impl crate::core_host::imports::HostPermissionDefinition for PluginStoreState {
    fn permission_definition_get_name(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "permission-definition.permission-definition.get-name")?;
            resolve_definition(
                self,
                definition_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|definition| definition.getName())
            .map_err(map_core_host_error)
        })())
    }

    fn permission_definition_get_description(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "permission-definition.permission-definition.get-description",
            )?;
            resolve_definition(
                self,
                definition_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|definition| definition.getDescription())
            .map_err(map_core_host_error)
        })())
    }

    fn permission_definition_set_description(
        &mut self,
        self_: u32,
        description: String,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "permission-definition.permission-definition.set-description",
            )?;
            resolve_definition(
                self,
                definition_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|definition| definition.setDescription(&description))
            .map_err(map_core_host_error)
        })())
    }

    fn permission_definition_get_default(
        &mut self,
        self_: u32,
    ) -> Result<Result<PermissionDefaultPermissionDefault, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "permission-definition.permission-definition.get-default",
            )?;
            resolve_definition(
                self,
                definition_handle(self, self_).map_err(map_core_host_error)?,
            )
            .and_then(|definition| permission_default(definition.getDefault()))
            .map_err(map_core_host_error)
        })())
    }

    fn permission_definition_set_default(
        &mut self,
        self_: u32,
        default_value: PermissionDefaultPermissionDefault,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "permission-definition.permission-definition.set-default",
            )?;
            resolve_definition(
                self,
                definition_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|definition| definition.setDefault(native_permission_default(default_value)))
            .map_err(map_core_host_error)
        })())
    }

    fn permission_definition_list_children(
        &mut self,
        self_: u32,
    ) -> Result<Result<Vec<PermissionAttachmentPermissionChild>, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "permission-definition.permission-definition.list-children",
            )?;
            resolve_definition(
                self,
                definition_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|definition| {
                definition
                    .getChildren()
                    .into_iter()
                    .map(|child| PermissionAttachmentPermissionChild {
                        name: child.name,
                        value: child.value,
                    })
                    .collect()
            })
            .map_err(map_core_host_error)
        })())
    }

    fn permission_definition_add_child(
        &mut self,
        self_: u32,
        child_name: String,
        value: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "permission-definition.permission-definition.add-child",
            )?;
            resolve_definition(
                self,
                definition_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|definition| definition.addChild(&child_name, value))
            .map_err(map_core_host_error)
        })())
    }

    fn permission_definition_remove_child(
        &mut self,
        self_: u32,
        child_name: String,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "permission-definition.permission-definition.remove-child",
            )?;
            resolve_definition(
                self,
                definition_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|definition| definition.removeChild(&child_name))
            .map_err(map_core_host_error)
        })())
    }

    fn permission_definition_recalculate_defaults(
        &mut self,
        self_: u32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "permission-definition.permission-definition.recalculate-defaults",
            )?;
            let definition = resolve_definition(
                self,
                definition_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?;
            resolve_server(self)
                .map(|server| server.recalculatePermissionDefaults(definition))
                .map_err(map_core_host_error)
        })())
    }

    fn permission_definition_recalculate_permissibles(
        &mut self,
        self_: u32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "permission-definition.permission-definition.recalculate-permissibles",
            )?;
            resolve_definition(
                self,
                definition_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|definition| definition.recalculatePermissibles())
            .map_err(map_core_host_error)
        })())
    }

    fn permission_definition_add_parent(
        &mut self,
        self_: u32,
        parent_name: String,
        value: bool,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "permission-definition.permission-definition.add-parent",
            )?;
            let parent = resolve_definition(
                self,
                definition_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|definition| definition.addParentByName(&parent_name, value))
            .map_err(map_core_host_error)?;
            self.insert_permission_definition_resource(parent)
                .map_err(map_core_host_error)
        })())
    }

    fn get(&mut self, name: String) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "permission-definition.get")?;
            let definition = resolve_server(self)
                .map(|server| server.getPermissionDefinition(&name))
                .map_err(map_core_host_error)?;
            (!definition.is_null())
                .then(|| {
                    self.insert_permission_definition_resource(definition)
                        .map_err(map_core_host_error)
                })
                .transpose()
        })())
    }

    fn add(
        &mut self,
        name: String,
        description: Option<String>,
        default_value: Option<PermissionDefaultPermissionDefault>,
        children: Vec<PermissionAttachmentPermissionChild>,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "permission-definition.add")?;
            let (has_description, description) =
                description.map_or((false, String::new()), |value| (true, value));
            let (has_default, default_value) =
                default_value.map_or((false, 0), |value| (true, native_permission_default(value)));
            let children = children
                .into_iter()
                .map(|child| cxx_admin::PermissionChild {
                    name: child.name,
                    value: child.value,
                })
                .collect();
            let definition = resolve_server(self)
                .map(|server| {
                    server.addPermissionDefinition(
                        &name,
                        has_description,
                        &description,
                        has_default,
                        default_value,
                        &children,
                    )
                })
                .map_err(map_core_host_error)?;
            self.insert_permission_definition_resource(definition)
                .map_err(map_core_host_error)
        })())
    }

    fn remove(&mut self, name: String) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "permission-definition.remove")?;
            let removed = resolve_server(self)
                .map(|server| server.removePermissionDefinitionByName(&name))
                .map_err(map_core_host_error)?;
            if !removed {
                return Err(TypesHostError::NotFound);
            }
            let handles = self
                .handles
                .permission_definitions_named(self.invocation_id, &name);
            self.handles.remove_handles(&handles);
            Ok(())
        })())
    }

    fn list_defaults(
        &mut self,
        level: PermissionLevelPermissionLevel,
    ) -> Result<Result<Vec<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "permission-definition.list-defaults")?;
            let level = match level {
                PermissionLevelPermissionLevel::Default => 0,
                PermissionLevelPermissionLevel::Operator => 1,
                PermissionLevelPermissionLevel::Console => 2,
            };
            let mut definitions = resolve_server(self)
                .map(|server| server.listDefaultPermissionDefinitions(level))
                .map_err(map_core_host_error)?;
            let len = definitions.len();
            (0..len)
                .map(|index| {
                    self.insert_permission_definition_resource(
                        definitions.pin_mut().takePermissionDefinition(index),
                    )
                    .map_err(map_core_host_error)
                })
                .collect()
        })())
    }
}
