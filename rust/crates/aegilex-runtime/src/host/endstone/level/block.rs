//! Core ABI implementation for `native/bindings/endstone/level/block.h`.

use crate::host::endstone::support::*;

impl crate::core_host::imports::HostBlock for PluginStoreState {
    fn block_get_type(&mut self, self_: u32) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block.block.get-type")?;
            resolve_block(
                self,
                block_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|block| block.getType())
            .map_err(map_core_host_error)
        })())
    }

    fn block_set_type(
        &mut self,
        self_: u32,
        type_id: String,
        apply_physics: Option<bool>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block.block.set-type")?;
            resolve_block(
                self,
                block_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|block| block.setType(&type_id, apply_physics.unwrap_or(true)))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn block_get_x(&mut self, self_: u32) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block.block.get-x")?;
            resolve_block(
                self,
                block_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|block| block.getX())
            .map_err(map_core_host_error)
        })())
    }

    fn block_get_y(&mut self, self_: u32) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block.block.get-y")?;
            resolve_block(
                self,
                block_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|block| block.getY())
            .map_err(map_core_host_error)
        })())
    }

    fn block_get_z(&mut self, self_: u32) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block.block.get-z")?;
            resolve_block(
                self,
                block_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|block| block.getZ())
            .map_err(map_core_host_error)
        })())
    }

    fn block_get_location(
        &mut self,
        self_: u32,
    ) -> Result<Result<LocationLocation, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block.block.get-location")?;
            resolve_block(
                self,
                block_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|block| wit_location_native(block.getLocation()))
            .map_err(map_core_host_error)
        })())
    }

    fn block_get_data(&mut self, self_: u32) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block.block.get-data")?;
            let data = resolve_block(
                self,
                block_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|block| block.getData())
            .map_err(map_core_host_error)?;
            insert_block_data_resource(self, data).map_err(map_core_host_error)
        })())
    }

    fn block_set_data(
        &mut self,
        self_: u32,
        data: u32,
        apply_physics: Option<bool>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block.block.set-data")?;
            let data =
                copy_native_block_data(block_data_value(self, data).map_err(map_core_host_error)?);
            let block = resolve_block(
                self,
                block_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?;
            native::status_result(block.setData(&data, apply_physics.unwrap_or(true)))
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn block_get_relative(
        &mut self,
        self_: u32,
        dx: i32,
        dy: i32,
        dz: i32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block.block.get-relative")?;
            let block = resolve_block(
                self,
                block_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|block| block.getRelative(dx, dy, dz))
            .map_err(map_core_host_error)?;
            self.insert_block_resource(block)
                .map_err(map_core_host_error)
        })())
    }

    fn block_clone(&mut self, self_: u32) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block.block.clone")?;
            let block = resolve_block(
                self,
                block_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|block| block.clone())
            .map_err(map_core_host_error)?;
            self.insert_block_resource(block)
                .map_err(map_core_host_error)
        })())
    }

    fn drop_block(&mut self, handle: u32) -> Result<(), String> {
        self.drop_resource(handle, ResourceKind::Block)
            .map_err(|error| format!("{error:?}"))
    }
}

// ---------------------------------------------------------------------------
// block-data
// ---------------------------------------------------------------------------

impl crate::core_host::imports::HostBlockData for PluginStoreState {
    fn block_data_get_type(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-data.block-data.get-type")?;
            block_data_value(self, self_)
                .map(|data| data.type_id.clone())
                .map_err(map_core_host_error)
        })())
    }

    fn block_data_get_block_states(
        &mut self,
        self_: u32,
    ) -> Result<Result<Vec<BlockDataBlockStatePair>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-data.block-data.get-block-states")?;
            block_data_value(self, self_)
                .map(|data| {
                    data.states
                        .iter()
                        .map(wit_block_state_pair_native)
                        .collect()
                })
                .map_err(map_core_host_error)
        })())
    }

    fn block_data_get_runtime_id(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-data.block-data.get-runtime-id")?;
            block_data_value(self, self_)
                .map(|data| data.runtime_id)
                .map_err(map_core_host_error)
        })())
    }
}

// ---------------------------------------------------------------------------
// block-type
// ---------------------------------------------------------------------------

impl crate::core_host::imports::HostBlockType for PluginStoreState {
    fn has_item_type(&mut self, type_id: String) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-type.has-item-type")?;
            let mut has_item_type = false;
            let level = resolve_level(self.host.clone()).map_err(map_core_host_error)?;
            native::status_result(level.blockTypeHasItem(&type_id, &mut has_item_type))
                .map_err(map_core_host_error)?;
            Ok(has_item_type)
        })())
    }

    fn create_block_data(
        &mut self,
        type_id: String,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-type.create-block-data")?;
            let mut data = cxx_level::BlockData {
                type_id: String::new(),
                states: Vec::new(),
                runtime_id: 0,
            };
            let level = resolve_level(self.host.clone()).map_err(map_core_host_error)?;
            native::status_result(level.createBlockData(&type_id, &Vec::new(), &mut data))
                .map_err(map_core_host_error)?;
            insert_block_data_resource(self, data).map_err(map_core_host_error)
        })())
    }
}

// ---------------------------------------------------------------------------
// block-state
// ---------------------------------------------------------------------------

impl crate::core_host::imports::HostBlockState for PluginStoreState {
    fn block_state_get_block(&mut self, self_: u32) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-state.block-state.get-block")?;
            let state = block_state_value(self, self_).map_err(map_core_host_error)?;
            let block = resolve_level(self.host.clone())
                .map_err(map_core_host_error)?
                .getBlock(&state.dimension, state.x, state.y, state.z);
            if block.is_null() {
                return Err(map_core_host_error(HostError::from_status(
                    AEGILEX_NOT_FOUND,
                )));
            }
            self.insert_block_resource(block)
                .map_err(map_core_host_error)
        })())
    }

    fn block_state_get_type(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-state.block-state.get-type")?;
            block_state_value(self, self_)
                .map(|state| state.type_id.clone())
                .map_err(map_core_host_error)
        })())
    }

    fn block_state_set_type(
        &mut self,
        self_: u32,
        type_id: String,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-state.block-state.set-type")?;
            block_state_value_mut(self, self_)
                .map(|state| state.type_id = type_id)
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn block_state_get_data(&mut self, self_: u32) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-state.block-state.get-data")?;
            let data = block_state_value(self, self_)
                .map(|state| cxx_level::BlockData {
                    type_id: state.type_id.clone(),
                    states: state
                        .states
                        .iter()
                        .map(|pair| cxx_level::BlockStatePair {
                            key: pair.key.clone(),
                            value_kind: pair.value_kind,
                            boolean: pair.boolean,
                            text: pair.text.clone(),
                            integer: pair.integer,
                        })
                        .collect(),
                    runtime_id: state.runtime_id,
                })
                .map_err(map_core_host_error)?;
            insert_block_data_resource(self, data).map_err(map_core_host_error)
        })())
    }

    fn block_state_set_data(
        &mut self,
        self_: u32,
        data: u32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-state.block-state.set-data")?;
            let data =
                copy_native_block_data(block_data_value(self, data).map_err(map_core_host_error)?);
            let state = block_state_value_mut(self, self_).map_err(map_core_host_error)?;
            state.type_id = data.type_id;
            state.states = data.states;
            state.runtime_id = data.runtime_id;
            Ok(())
        })())
    }

    fn block_state_get_x(&mut self, self_: u32) -> Result<Result<i32, TypesHostError>, String> {
        Ok(block_state_value(self, self_)
            .map(|state| state.x)
            .map_err(map_core_host_error))
    }

    fn block_state_get_y(&mut self, self_: u32) -> Result<Result<i32, TypesHostError>, String> {
        Ok(block_state_value(self, self_)
            .map(|state| state.y)
            .map_err(map_core_host_error))
    }

    fn block_state_get_z(&mut self, self_: u32) -> Result<Result<i32, TypesHostError>, String> {
        Ok(block_state_value(self, self_)
            .map(|state| state.z)
            .map_err(map_core_host_error))
    }

    fn block_state_get_location(
        &mut self,
        self_: u32,
    ) -> Result<Result<LocationLocation, TypesHostError>, String> {
        Ok(block_state_value(self, self_)
            .map_err(map_core_host_error)
            .map(|state| LocationLocation {
                dimension: state.dimension.clone(),
                x: state.x as f32,
                y: state.y as f32,
                z: state.z as f32,
                pitch: 0.0,
                yaw: 0.0,
            }))
    }

    fn block_state_update(
        &mut self,
        self_: u32,
        force: Option<bool>,
        apply_physics: Option<bool>,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-state.block-state.update")?;
            let state = block_state_value(self, self_).map_err(map_core_host_error)?;
            let mut applied = false;
            native::status_result(
                resolve_level(self.host.clone())
                    .map_err(map_core_host_error)?
                    .updateBlockState(
                        state,
                        force.is_some(),
                        force.unwrap_or(false),
                        apply_physics.is_some(),
                        apply_physics.unwrap_or(false),
                        &mut applied,
                    ),
            )
            .map_err(map_core_host_error)?;
            Ok(applied)
        })())
    }

    fn capture(&mut self, block: u32) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-state.capture")?;
            let snapshot = resolve_block(
                self,
                block_handle(self, block).map_err(map_core_host_error)?,
            )
            .map(|block| block.captureState())
            .map_err(map_core_host_error)?;
            insert_block_state_resource(self, snapshot).map_err(map_core_host_error)
        })())
    }
}
