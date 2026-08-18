//! Map-renderer resource support and Core ABI implementation.

use crate::runtime::PluginStoreState;

pub(crate) struct GuestMapRenderer {
    pub(crate) renderer_id: u64,
    pub(crate) map_id: i64,
    pub(crate) contextual: bool,
}

impl PluginStoreState {
    pub(crate) fn insert_map_renderer(&mut self, rep: u32, entry: GuestMapRenderer) {
        self.map_renderers.insert(rep, entry);
    }

    pub(crate) fn remove_map_renderer(&mut self, rep: u32) -> Option<GuestMapRenderer> {
        self.map_renderers.remove(&rep)
    }
}

use crate::host::endstone::support::*;

impl crate::core_host::imports::HostMapRenderer for PluginStoreState {
    fn map_renderer_get_map_id(
        &mut self,
        self_: u32,
    ) -> Result<Result<i64, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "map-renderer.map-renderer.get-map-id")?;
            self.map_renderers
                .get(&self_)
                .map(|entry| entry.map_id)
                .ok_or_else(not_found)
                .map_err(map_core_host_error)
        })())
    }

    fn map_renderer_is_contextual(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "map-renderer.map-renderer.is-contextual")?;
            self.map_renderers
                .get(&self_)
                .map(|entry| entry.contextual)
                .ok_or_else(not_found)
                .map_err(map_core_host_error)
        })())
    }

    fn map_renderer_unregister(
        &mut self,
        self_: u32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "map-renderer.map-renderer.unregister")?;
            let entry = self
                .remove_map_renderer(self_)
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?;
            native::map_renderer_unregister(&self.host, &self.plugin_id, entry.renderer_id)
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn register(
        &mut self,
        map_id: i64,
        contextual: bool,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "map-renderer.register")?;
            let renderer_id =
                native::map_renderer_register(&self.host, &self.plugin_id, map_id, contextual)
                    .map_err(map_core_host_error)?;
            let rep =
                u32::try_from(renderer_id).map_err(|_| map_core_host_error(limit_exceeded()))?;
            self.insert_map_renderer(
                rep,
                GuestMapRenderer {
                    renderer_id,
                    map_id,
                    contextual,
                },
            );
            Ok(rep)
        })())
    }

    fn drop_map_renderer(&mut self, self_: u32) -> Result<(), String> {
        if let Some(entry) = self.remove_map_renderer(self_) {
            let _ = native::map_renderer_unregister(&self.host, &self.plugin_id, entry.renderer_id);
        }
        Ok(())
    }
}
