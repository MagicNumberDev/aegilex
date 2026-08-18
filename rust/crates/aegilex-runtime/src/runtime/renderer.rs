use super::*;

impl Runtime {
    pub(crate) fn dispatch_map_render(
        &mut self,
        plugin_id: &str,
        renderer_id: u64,
        map_id: i64,
        has_player: bool,
        player_uuid: &[u8],
    ) -> Vec<crate::core_host::MapCanvasMapDrawCommand> {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return Vec::new();
        };
        let Ok(renderer_rep) = u32::try_from(renderer_id) else {
            return Vec::new();
        };
        if !plugin.enabled
            || !plugin
                .store
                .data()
                .map_renderers
                .get(&renderer_rep)
                .is_some_and(|entry| entry.renderer_id == renderer_id)
        {
            return Vec::new();
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            return Vec::new();
        }
        let invocation_id = self.host.next_invocation_id();
        let outcome = call_with_invocation(&mut plugin.store, invocation_id, |store| {
            let Some(player) = resolve_player_for_dispatch(store, has_player, player_uuid) else {
                return Ok(Ok(Vec::new()));
            };
            plugin.exports.call_maprenderercallbacks_on_map_render(
                &plugin.instance,
                store,
                renderer_rep,
                map_id,
                player,
            )
        });
        match outcome {
            Ok(Ok(commands)) => commands,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-map-render rejected: {text}", plugin.id),
                );
                Vec::new()
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-map-render trapped: {error}", plugin.id),
                );
                Vec::new()
            }
        }
    }

    pub(crate) fn dispatch_map_initialize(
        &mut self,
        plugin_id: &str,
        renderer_id: u64,
        map_id: i64,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        let Ok(renderer_rep) = u32::try_from(renderer_id) else {
            return false;
        };
        if !plugin.enabled
            || !plugin
                .store
                .data()
                .map_renderers
                .get(&renderer_rep)
                .is_some_and(|entry| entry.renderer_id == renderer_id)
        {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            return false;
        }
        let invocation_id = self.host.next_invocation_id();
        let outcome = call_with_invocation(&mut plugin.store, invocation_id, |store| {
            plugin.exports.call_maprenderercallbacks_on_map_initialize(
                &plugin.instance,
                store,
                renderer_rep,
                map_id,
            )
        });
        match outcome {
            Ok(Ok(())) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-map-initialize rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-map-initialize trapped: {error}", plugin.id),
                );
                false
            }
        }
    }
}
