//! Core ABI implementation for `native/bindings/endstone/events/chunk_event_facade.h`.

use super::support::*;



fn resolve_chunk_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::ChunkEventFacade, HostError> {
    let handle = event_handle(state, event, ResourceKind::ChunkEvent)?;
    state
        .handles
        .chunk_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl crate::core_host::imports::HostChunkLoadEvent for PluginStoreState {
    fn chunk_load_event_get_chunk_x(
        &mut self,
        self_: u32,
    ) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "chunk-load-event.chunk-load-event.get-chunk-x")?;
            resolve_chunk_event(self, self_)
                .map(|event| event.getChunkX())
                .map_err(map_core_host_error)
        })())
    }

    fn chunk_load_event_get_chunk_z(
        &mut self,
        self_: u32,
    ) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "chunk-load-event.chunk-load-event.get-chunk-z")?;
            resolve_chunk_event(self, self_)
                .map(|event| event.getChunkZ())
                .map_err(map_core_host_error)
        })())
    }

    fn chunk_load_event_get_dimension(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "chunk-load-event.chunk-load-event.get-dimension")?;
            resolve_chunk_event(self, self_)
                .map(|event| event.getDimensionForRust())
                .map_err(map_core_host_error)
        })())
    }
}

impl crate::core_host::imports::HostChunkUnloadEvent for PluginStoreState {
    fn chunk_unload_event_get_chunk_x(
        &mut self,
        self_: u32,
    ) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "chunk-unload-event.chunk-unload-event.get-chunk-x")?;
            resolve_chunk_event(self, self_)
                .map(|event| event.getChunkX())
                .map_err(map_core_host_error)
        })())
    }

    fn chunk_unload_event_get_chunk_z(
        &mut self,
        self_: u32,
    ) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "chunk-unload-event.chunk-unload-event.get-chunk-z")?;
            resolve_chunk_event(self, self_)
                .map(|event| event.getChunkZ())
                .map_err(map_core_host_error)
        })())
    }

    fn chunk_unload_event_get_dimension(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "chunk-unload-event.chunk-unload-event.get-dimension")?;
            resolve_chunk_event(self, self_)
                .map(|event| event.getDimensionForRust())
                .map_err(map_core_host_error)
        })())
    }
}
