//! Core ABI implementation for `native/bindings/endstone/map/map.h`.

use crate::host::endstone::support::*;

impl crate::core_host::imports::HostMapView for PluginStoreState {
    fn exists(&mut self, id: i64) -> Result<Result<bool, TypesHostError>, String> {
        Ok(Ok(resolve_map_native(self, id).is_ok()))
    }

    fn create(&mut self, dimension: String) -> Result<Result<i64, TypesHostError>, String> {
        Ok((|| {
            let map = self
                .host
                .server()
                .map_err(map_core_host_error)?
                .createMap(&dimension);
            (!map.is_null())
                .then(|| map.getId())
                .ok_or_else(|| map_core_host_error(not_found()))
        })())
    }

    fn get_scale(&mut self, id: i64) -> Result<Result<MapViewMapScale, TypesHostError>, String> {
        Ok(resolve_map_native(self, id)
            .and_then(|map| match map.getScale() {
                0 => Ok(MapViewMapScale::Closest),
                1 => Ok(MapViewMapScale::Close),
                2 => Ok(MapViewMapScale::Normal),
                3 => Ok(MapViewMapScale::Far),
                4 => Ok(MapViewMapScale::Farthest),
                _ => Err(invalid_input()),
            })
            .map_err(map_core_host_error))
    }

    fn set_scale(
        &mut self,
        id: i64,
        scale: MapViewMapScale,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            let scale = match scale {
                MapViewMapScale::Closest => 0u8,
                MapViewMapScale::Close => 1u8,
                MapViewMapScale::Normal => 2u8,
                MapViewMapScale::Far => 3u8,
                MapViewMapScale::Farthest => 4u8,
            };
            resolve_map_native(self, id)
                .map(|map| map.setScale(scale))
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn get_center_x(&mut self, id: i64) -> Result<Result<i32, TypesHostError>, String> {
        Ok(resolve_map_native(self, id)
            .map(|map| map.getCenterX())
            .map_err(map_core_host_error))
    }

    fn set_center_x(&mut self, id: i64, x: i32) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            resolve_map_native(self, id)
                .map(|map| map.setCenterX(x))
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn get_center_z(&mut self, id: i64) -> Result<Result<i32, TypesHostError>, String> {
        Ok(resolve_map_native(self, id)
            .map(|map| map.getCenterZ())
            .map_err(map_core_host_error))
    }

    fn set_center_z(&mut self, id: i64, z: i32) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            resolve_map_native(self, id)
                .map(|map| map.setCenterZ(z))
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn get_dimension(&mut self, id: i64) -> Result<Result<String, TypesHostError>, String> {
        Ok(resolve_map_native(self, id)
            .map(|map| map.getDimensionName())
            .map_err(map_core_host_error))
    }

    fn set_dimension(
        &mut self,
        id: i64,
        dimension: String,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok(resolve_map_native(self, id)
            .and_then(|map| {
                map.setDimension(&dimension)
                    .then_some(())
                    .ok_or_else(not_found)
            })
            .map_err(map_core_host_error))
    }

    fn is_virtual(&mut self, id: i64) -> Result<Result<bool, TypesHostError>, String> {
        Ok(resolve_map_native(self, id)
            .map(|map| map.isVirtual())
            .map_err(map_core_host_error))
    }

    fn is_unlimited_tracking(&mut self, id: i64) -> Result<Result<bool, TypesHostError>, String> {
        Ok(resolve_map_native(self, id)
            .map(|map| map.isUnlimitedTracking())
            .map_err(map_core_host_error))
    }

    fn set_unlimited_tracking(
        &mut self,
        id: i64,
        unlimited: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            resolve_map_native(self, id)
                .map(|map| map.setUnlimitedTracking(unlimited))
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn is_locked(&mut self, id: i64) -> Result<Result<bool, TypesHostError>, String> {
        Ok(resolve_map_native(self, id)
            .map(|map| map.isLocked())
            .map_err(map_core_host_error))
    }

    fn set_locked(&mut self, id: i64, locked: bool) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            resolve_map_native(self, id)
                .map(|map| map.setLocked(locked))
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }
}
