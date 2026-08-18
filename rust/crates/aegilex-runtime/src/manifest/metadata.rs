use super::*;
pub(crate) fn validate_metadata(
    module_path: &Path,
    metadata: crate::core_host::PluginMetadataMetadata,
) -> Result<PluginMetadata, String> {
    if metadata.name.is_empty() {
        return Err("metadata name must be non-empty".to_owned());
    }
    if module_path
        .parent()
        .and_then(|path| path.file_name())
        .and_then(|name| name.to_str())
        != Some(metadata.name.as_str())
    {
        return Err("metadata name must match the module directory name".to_owned());
    }
    if let Some(subscription) = metadata
        .subscriptions
        .iter()
        .find(|subscription| !KNOWN_SUBSCRIPTIONS.contains(&subscription.as_str()))
    {
        return Err(format!("unknown subscription: {subscription}"));
    }
    if metadata
        .commands
        .iter()
        .any(|command| command.name.eq_ignore_ascii_case("aegilex"))
    {
        return Err("command name is reserved: aegilex".to_owned());
    }
    if let Some(duplicate) = first_duplicate(
        metadata
            .commands
            .iter()
            .map(|command| command.name.as_str()),
    ) {
        return Err(format!("duplicate command name: {duplicate}"));
    }
    let mut claimed = Vec::new();
    for command in &metadata.commands {
        if command
            .aliases
            .iter()
            .any(|alias| alias.eq_ignore_ascii_case("aegilex"))
        {
            return Err("command alias is reserved: aegilex".to_owned());
        }
        if let Some(duplicate) = first_duplicate(command.aliases.iter().map(String::as_str)) {
            return Err(format!(
                "duplicate alias {duplicate} for command {}",
                command.name
            ));
        }
        claimed.push(command.name.as_str());
        claimed.extend(command.aliases.iter().map(String::as_str));
    }
    if let Some(duplicate) = first_duplicate(claimed.into_iter()) {
        return Err(format!(
            "command name or alias is claimed more than once: {duplicate}"
        ));
    }
    if let Some(duplicate) = first_duplicate(
        metadata
            .permissions
            .iter()
            .map(|permission| permission.name.as_str()),
    ) {
        return Err(format!("duplicate permission name: {duplicate}"));
    }
    for permission in &metadata.permissions {
        if let Some(duplicate) =
            first_duplicate(permission.children.iter().map(|child| child.name.as_str()))
        {
            return Err(format!(
                "duplicate permission child {duplicate} for permission {}",
                permission.name
            ));
        }
    }

    let mut commands: Vec<CommandSpec> = metadata
        .commands
        .into_iter()
        .map(|command| CommandSpec {
            name: command.name,
            description: command.description,
            aliases: command.aliases,
            usages: command.usages,
            permissions: command.permissions,
        })
        .collect();
    commands.sort_by(|left, right| left.name.cmp(&right.name));
    let mut permissions: Vec<PermissionSpec> = metadata
        .permissions
        .into_iter()
        .map(|permission| PermissionSpec {
            name: permission.name,
            description: permission.description,
            default_value: permission.default_value.map(permission_default_value),
            children: permission
                .children
                .into_iter()
                .map(|child| PermissionChild {
                    name: child.name,
                    value: child.value,
                })
                .collect(),
        })
        .collect();
    permissions.sort_by(|left, right| left.name.cmp(&right.name));
    let mut subscriptions = metadata.subscriptions;
    subscriptions.sort();
    subscriptions.dedup();

    Ok(PluginMetadata {
        name: metadata.name,
        version: metadata.version,
        description: metadata.description,
        load_order: match metadata.load_order {
            crate::core_host::PluginTypesLoadOrder::Startup => PluginLoadOrder::Startup,
            crate::core_host::PluginTypesLoadOrder::PostWorld => PluginLoadOrder::PostWorld,
        },
        authors: metadata.authors,
        contributors: metadata.contributors,
        website: metadata.website,
        prefix: metadata.prefix,
        provides: metadata.provides,
        depend: metadata.depend,
        soft_depend: metadata.soft_depend,
        load_before: metadata.load_before,
        default_permission: permission_default_value(metadata.default_permission),
        commands,
        permissions,
        subscriptions,
    })
}

fn permission_default_value(value: crate::core_host::PermissionDefaultPermissionDefault) -> u32 {
    match value {
        crate::core_host::PermissionDefaultPermissionDefault::True => 0,
        crate::core_host::PermissionDefaultPermissionDefault::False => 1,
        crate::core_host::PermissionDefaultPermissionDefault::Operator => 2,
        crate::core_host::PermissionDefaultPermissionDefault::NotOperator => 3,
        crate::core_host::PermissionDefaultPermissionDefault::Console => 4,
    }
}

fn first_duplicate<'a>(mut values: impl Iterator<Item = &'a str>) -> Option<&'a str> {
    let mut seen = std::collections::HashSet::new();
    values.find(|value| !seen.insert(*value))
}
