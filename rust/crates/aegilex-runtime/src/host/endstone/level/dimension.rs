//! Core ABI implementation for `native/bindings/endstone/level/dimension.h`.

use crate::host::endstone::support::*;

impl crate::core_host::imports::HostDimension for PluginStoreState {
    fn list_loaded_chunks(
        &mut self,
        dimension: String,
    ) -> Result<Result<Vec<ChunkChunk>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "dimension.list-loaded-chunks")?;
            let mut chunks = Vec::new();
            native::status_result(
                resolve_level(self.host.clone())
                    .map_err(map_core_host_error)?
                    .listLoadedChunks(&dimension, &mut chunks),
            )
            .map_err(map_core_host_error)?;
            Ok(chunks
                .into_iter()
                .map(|chunk| ChunkChunk {
                    dimension: chunk.dimension,
                    x: chunk.x,
                    z: chunk.z,
                    level_name: chunk.level_name,
                })
                .collect())
        })())
    }

    fn get_block(
        &mut self,
        dimension: String,
        x: i32,
        y: i32,
        z: i32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "dimension.get-block")?;
            let block = resolve_level(self.host.clone())
                .map_err(map_core_host_error)?
                .getBlock(&dimension, x, y, z);
            if block.is_null() {
                return Err(map_core_host_error(HostError::from_status(
                    AEGILEX_NOT_FOUND,
                )));
            }
            self.insert_block_resource(block)
                .map_err(map_core_host_error)
        })())
    }

    fn set_block(
        &mut self,
        dimension: String,
        x: i32,
        y: i32,
        z: i32,
        type_id: String,
        states: Vec<BlockDataBlockStatePair>,
        apply_physics: Option<bool>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "dimension.set-block")?;
            let states = native_block_state_pairs(&states).map_err(map_core_host_error)?;
            native::status_result(
                resolve_level(self.host.clone())
                    .map_err(map_core_host_error)?
                    .setBlock(
                        &dimension,
                        x,
                        y,
                        z,
                        &type_id,
                        &states,
                        apply_physics.is_some(),
                        apply_physics.unwrap_or(false),
                    ),
            )
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn get_highest_block_y(
        &mut self,
        dimension: String,
        x: i32,
        z: i32,
    ) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "dimension.get-highest-block-y")?;
            let mut y = 0;
            native::status_result(
                resolve_level(self.host.clone())
                    .map_err(map_core_host_error)?
                    .getHighestBlockY(&dimension, x, z, &mut y),
            )
            .map_err(map_core_host_error)?;
            Ok(y)
        })())
    }

    fn get_highest_block(
        &mut self,
        dimension: String,
        x: i32,
        z: i32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "dimension.get-highest-block")?;
            let block = resolve_level(self.host.clone())
                .map_err(map_core_host_error)?
                .getHighestBlock(&dimension, x, z);
            if block.is_null() {
                return Err(map_core_host_error(HostError::from_status(
                    AEGILEX_NOT_FOUND,
                )));
            }
            self.insert_block_resource(block)
                .map_err(map_core_host_error)
        })())
    }

    fn get_relative(
        &mut self,
        dimension: String,
        x: i32,
        y: i32,
        z: i32,
        dx: i32,
        dy: i32,
        dz: i32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "dimension.get-relative")?;
            let block = resolve_level(self.host.clone())
                .map_err(map_core_host_error)?
                .getRelativeBlock(&dimension, x, y, z, dx, dy, dz);
            if block.is_null() {
                return Err(map_core_host_error(HostError::from_status(
                    AEGILEX_NOT_FOUND,
                )));
            }
            self.insert_block_resource(block)
                .map_err(map_core_host_error)
        })())
    }

    fn create_block_data(
        &mut self,
        type_id: String,
        states: Vec<BlockDataBlockStatePair>,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "dimension.create-block-data")?;
            let states = native_block_state_pairs(&states).map_err(map_core_host_error)?;
            let mut data = cxx_level::BlockData {
                type_id: String::new(),
                states: Vec::new(),
                runtime_id: 0,
            };
            native::status_result(
                resolve_level(self.host.clone())
                    .map_err(map_core_host_error)?
                    .createBlockData(&type_id, &states, &mut data),
            )
            .map_err(map_core_host_error)?;
            insert_block_data_resource(self, data).map_err(map_core_host_error)
        })())
    }

    fn spawn_actor(
        &mut self,
        dimension: String,
        location: LocationLocation,
        actor_type: String,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "dimension.spawn-actor")?;
            let location = cxx_location_native(&location);
            let actor = resolve_level(self.host.clone())
                .map_err(map_core_host_error)?
                .spawnActor(&dimension, &location, &actor_type);
            if actor.is_null() {
                return Err(map_core_host_error(HostError::from_status(
                    AEGILEX_NOT_FOUND,
                )));
            }
            self.insert_actor_resource(actor)
                .map_err(map_core_host_error)
        })())
    }

    fn drop_item(
        &mut self,
        dimension: String,
        location: LocationLocation,
        item: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "dimension.drop-item")?;
            let location = cxx_location_native(&location);
            let handle = self
                .resource_slot(item, ResourceKind::ItemStack)
                .map_err(map_core_host_error)?
                .handle;
            let item = self
                .handles
                .item_stack(self.invocation_id, handle)
                .ok_or_else(|| map_core_host_error(HostError::from_status(AEGILEX_NOT_FOUND)))?;
            let actor = resolve_level(self.host.clone())
                .map_err(map_core_host_error)?
                .dropItem(&dimension, &location, item);
            if actor.is_null() {
                return Err(map_core_host_error(HostError::from_status(
                    AEGILEX_NOT_FOUND,
                )));
            }
            self.insert_actor_resource(actor)
                .map_err(map_core_host_error)
        })())
    }
}
