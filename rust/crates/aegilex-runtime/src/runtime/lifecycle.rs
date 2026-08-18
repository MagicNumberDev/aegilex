use super::*;
impl Runtime {
    pub(crate) fn inspect_plugin(&self, module_path: &Path) -> Result<PluginInspection, String> {
        self.load_metadata(module_path)
            .map(|metadata| PluginInspection { metadata })
            .inspect_err(|error| {
                self.log_loader_error(error);
            })
    }

    pub(crate) fn prepare_plugin(&mut self, module_path: &Path) -> u32 {
        match self.prepare(module_path) {
            Ok(()) => AEGILEX_OK,
            Err(error) => {
                self.log_loader_error(&error);
                AEGILEX_TRAP
            }
        }
    }

    pub(crate) fn enable_plugin(&mut self, id: &str) -> u32 {
        let Some(plugin) = self.plugins.iter_mut().find(|plugin| plugin.id == id) else {
            return AEGILEX_NOT_FOUND;
        };
        if plugin.enabled {
            return AEGILEX_OK;
        }
        if let Err(error) = plugin.store.set_fuel(ENABLE_FUEL) {
            log_loader_error(
                &self.host,
                &format!("{}: cannot reset fuel: {error}", plugin.id),
            );
            return AEGILEX_INTERNAL_ERROR;
        }
        // Match Endstone's Plugin::setEnabled: lifecycle callbacks observe the
        // plugin as enabled, with failures rolling the state back below.
        plugin.enabled = true;
        let invocation_id = self.host.next_invocation_id();
        let outcome = call_with_invocation(&mut plugin.store, invocation_id, |store| {
            plugin.exports.call_on_enable(&plugin.instance, store)
        });
        match outcome {
            Ok(Ok(())) => AEGILEX_OK,
            Ok(Err(error)) => {
                plugin.enabled = false;
                plugin.store.data_mut().clear_plugin_resources();
                log_loader_error(
                    &self.host,
                    &format!("{}: on-enable rejected: {error}", plugin.id),
                );
                AEGILEX_TRAP
            }
            Err(error) => {
                plugin.enabled = false;
                plugin.store.data_mut().clear_plugin_resources();
                log_loader_error(
                    &self.host,
                    &format!("{}: on-enable trapped: {error}", plugin.id),
                );
                AEGILEX_TRAP
            }
        }
    }

    pub(crate) fn disable_plugin(&mut self, id: &str) -> u32 {
        let Some(plugin) = self.plugins.iter_mut().find(|plugin| plugin.id == id) else {
            return AEGILEX_NOT_FOUND;
        };
        if !plugin.enabled {
            return AEGILEX_OK;
        }
        plugin.enabled = false;
        let invocation_id = self.host.next_invocation_id();
        let outcome = call_with_invocation(&mut plugin.store, invocation_id, |store| {
            plugin.exports.call_on_disable(store)
        });
        plugin.store.data_mut().clear_plugin_resources();
        match outcome {
            Ok(()) => AEGILEX_OK,
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-disable trapped: {error}", plugin.id),
                );
                AEGILEX_TRAP
            }
        }
    }

    pub(crate) fn plugin_summaries(&self) -> Vec<crate::manifest::PluginMetadata> {
        self.plugins
            .iter()
            .map(|plugin| plugin.metadata.clone())
            .collect()
    }

    pub(crate) fn plugin_summary(&self, id: &str) -> Option<crate::manifest::PluginMetadata> {
        self.plugins
            .iter()
            .find(|plugin| plugin.id == id)
            .map(|plugin| plugin.metadata.clone())
    }

    pub(crate) fn is_plugin_enabled(&self, id: &str) -> bool {
        self.plugins
            .iter()
            .find(|plugin| plugin.id == id)
            .is_some_and(|plugin| plugin.enabled)
    }
}
