use super::*;
pub(crate) fn inspect_module_path(
    module_path: &Path,
    config: RuntimeConfig,
) -> Result<PathBuf, String> {
    if module_path.file_name().and_then(|name| name.to_str()) != Some(MODULE_NAME) {
        return Err("Core module must be named plugin.wasm".to_owned());
    }
    let module_path = module_path
        .canonicalize()
        .map_err(|error| format!("cannot resolve plugin.wasm: {error}"))?;
    let metadata =
        fs::metadata(&module_path).map_err(|error| format!("cannot read Core module: {error}"))?;
    if !metadata.is_file() {
        return Err("plugin.wasm is not a regular file".to_owned());
    }
    if config.max_module_bytes != 0 && metadata.len() > config.max_module_bytes {
        return Err(format!(
            "Core module exceeds configured limit of {} bytes",
            config.max_module_bytes
        ));
    }
    Ok(module_path)
}
