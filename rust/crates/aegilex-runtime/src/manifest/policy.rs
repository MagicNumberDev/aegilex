use super::*;
pub(crate) fn load_plugin_policy(module_path: &Path) -> Result<PluginPolicy, String> {
    let policy_path = module_path
        .parent()
        .ok_or_else(|| "module has no parent directory".to_owned())?
        .join(POLICY_NAME);
    let policy_metadata = match fs::metadata(&policy_path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(PluginPolicy::default());
        }
        Err(error) => {
            return Err(format!(
                "cannot read authorization policy {}: {error}",
                policy_path.display()
            ));
        }
    };
    if !policy_metadata.is_file() {
        return Err(format!(
            "authorization policy is not a regular file: {}",
            policy_path.display()
        ));
    }
    let source = match fs::read_to_string(&policy_path) {
        Ok(source) => source,
        Err(error) => {
            return Err(format!(
                "cannot read authorization policy {}: {error}",
                policy_path.display()
            ));
        }
    };
    let value = source.parse::<toml::Value>().map_err(|error| {
        format!(
            "invalid authorization policy {}: {error}",
            policy_path.display()
        )
    })?;
    let table = value.as_table().ok_or_else(|| {
        format!(
            "authorization policy {} must contain a top-level table",
            policy_path.display()
        )
    })?;
    for key in table.keys() {
        if !matches!(key.as_str(), "capabilities" | "paths" | "network") {
            return Err(format!(
                "unknown authorization policy key in {}: {key}",
                policy_path.display()
            ));
        }
    }

    let capabilities = read_string_array(table, "capabilities", &policy_path)?;
    for capability in &capabilities {
        if !is_known_capability(capability) {
            return Err(format!(
                "unknown capability in {}: {capability}",
                policy_path.display()
            ));
        }
    }
    reject_duplicates(&capabilities, "capability", &policy_path)?;

    let server_root = server_root(module_path)?;
    let configured_paths = read_string_array(table, "paths", &policy_path)?;
    reject_duplicates(&configured_paths, "path", &policy_path)?;
    let paths = configured_paths
        .iter()
        .map(|path| resolve_preopen_path(&server_root, path, &policy_path))
        .collect::<Result<Vec<_>, _>>()?;
    let configured_network = read_string_array(table, "network", &policy_path)?;
    reject_duplicates(&configured_network, "network rule", &policy_path)?;
    let network = configured_network
        .into_iter()
        .map(|rule| parse_network_rule(&rule, &policy_path))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(PluginPolicy {
        capabilities,
        paths,
        network,
    })
}

fn read_string_array(
    table: &toml::map::Map<String, toml::Value>,
    key: &str,
    policy_path: &Path,
) -> Result<Vec<String>, String> {
    let Some(value) = table.get(key) else {
        return Ok(Vec::new());
    };
    let values = value.as_array().ok_or_else(|| {
        format!(
            "authorization policy {} key {key} must be an array of strings",
            policy_path.display()
        )
    })?;
    values
        .iter()
        .map(|value| {
            value.as_str().map(str::to_owned).ok_or_else(|| {
                format!(
                    "authorization policy {} key {key} must be an array of strings",
                    policy_path.display()
                )
            })
        })
        .collect()
}

fn reject_duplicates(values: &[String], kind: &str, policy_path: &Path) -> Result<(), String> {
    for (index, value) in values.iter().enumerate() {
        if values[..index].iter().any(|previous| previous == value) {
            return Err(format!(
                "duplicate {kind} in authorization policy {}: {value}",
                policy_path.display()
            ));
        }
    }
    Ok(())
}

pub(crate) fn is_known_capability(capability: &str) -> bool {
    if capability == "*" {
        return true;
    }
    let mut segments = capability.split('.');
    let Some(interface) = segments.next() else {
        return false;
    };
    let remaining = segments.collect::<Vec<_>>();
    if interface.is_empty()
        || remaining.is_empty()
        || remaining.iter().any(|segment| segment.is_empty())
    {
        return false;
    }
    if remaining.last() == Some(&"*") {
        return remaining.len() <= 2
            && KNOWN_CAPABILITIES.iter().any(|known| {
                known.strip_prefix(interface).is_some_and(|suffix| {
                    suffix.starts_with('.') && suffix.split('.').count() > remaining.len()
                })
            });
    }
    KNOWN_CAPABILITIES.binary_search(&capability).is_ok()
}

fn server_root(module_path: &Path) -> Result<PathBuf, String> {
    let plugin_dir = module_path
        .parent()
        .ok_or_else(|| "module has no parent directory".to_owned())?;
    let plugins_dir = plugin_dir
        .parent()
        .ok_or_else(|| "module directory has no plugins parent".to_owned())?;
    if plugins_dir.file_name().and_then(|name| name.to_str()) != Some("plugins") {
        return Err("module must be directly under the server plugins directory".to_owned());
    }
    let root = plugins_dir
        .parent()
        .ok_or_else(|| "plugins directory has no server root".to_owned())?;
    root.canonicalize()
        .map_err(|error| format!("cannot resolve server root {}: {error}", root.display()))
}

fn resolve_preopen_path(
    server_root: &Path,
    configured_path: &str,
    policy_path: &Path,
) -> Result<PreopenedPath, String> {
    let path = Path::new(configured_path);
    if configured_path.is_empty()
        || path.is_absolute()
        || !path
            .components()
            .all(|component| matches!(component, std::path::Component::Normal(_)))
    {
        return Err(format!(
            "path in authorization policy {} must be a non-empty relative path without . or ..: {configured_path}",
            policy_path.display()
        ));
    }

    let host_path = server_root.join(path).canonicalize().map_err(|error| {
        format!(
            "cannot resolve authorized path {configured_path} in {}: {error}",
            policy_path.display()
        )
    })?;
    if !host_path.is_dir() {
        return Err(format!(
            "authorized path is not a directory in {}: {configured_path}",
            policy_path.display()
        ));
    }
    if !host_path.starts_with(server_root) {
        return Err(format!(
            "authorized path escapes the server root in {}: {configured_path}",
            policy_path.display()
        ));
    }

    let guest_path = path
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/");
    Ok(PreopenedPath {
        host_path,
        guest_path: format!("/{guest_path}"),
    })
}

fn parse_network_rule(rule: &str, policy_path: &Path) -> Result<NetworkRule, String> {
    let Some((protocol, address)) = rule.split_once(':') else {
        return Err(format!(
            "network rule in authorization policy {} must be protocol:ip:port: {rule}",
            policy_path.display()
        ));
    };
    let protocol = match protocol {
        "tcp" => NetworkProtocol::Tcp,
        "udp" => NetworkProtocol::Udp,
        _ => {
            return Err(format!(
                "network rule in authorization policy {} must use tcp or udp: {rule}",
                policy_path.display()
            ));
        }
    };
    let address = address.parse::<SocketAddr>().map_err(|_| {
        format!(
            "network rule in authorization policy {} must contain a literal IP and single port: {rule}",
            policy_path.display()
        )
    })?;
    Ok(NetworkRule { protocol, address })
}
