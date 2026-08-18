//! Core ABI implementation for `native/bindings/endstone/level/level.h`.

use crate::host::endstone::support::*;

impl crate::core_host::imports::HostLevel for PluginStoreState {
    fn get_level(&mut self) -> Result<Result<LevelLevel, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "level.get-level")?;
            let level = resolve_level(self.host.clone()).map_err(map_core_host_error)?;
            Ok(LevelLevel {
                name: level.getName(),
                seed: level.getSeed(),
                time: level.getTime(),
                dimension_count: level.getDimensions().len() as u32,
            })
        })())
    }

    fn get_time(&mut self) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "level.get-time")?;
            resolve_level(self.host.clone())
                .map(|level| level.getTime())
                .map_err(map_core_host_error)
        })())
    }

    fn set_time(&mut self, time: i32) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "level.set-time")?;
            resolve_level(self.host.clone())
                .map(|level| level.setTime(time))
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn list_dimensions(
        &mut self,
    ) -> Result<Result<Vec<DimensionDimension>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "level.list-dimensions")?;
            let level = resolve_level(self.host.clone()).map_err(map_core_host_error)?;
            let mut summaries = Vec::with_capacity(level.getDimensions().len());
            for summary in level.getDimensions() {
                summaries.push(DimensionDimension {
                    name: summary.name,
                    kind: wit_dimension_kind(summary.kind).map_err(map_core_host_error)?,
                    level: summary.level,
                });
            }
            Ok(summaries)
        })())
    }

    fn find_dimension(
        &mut self,
        name: String,
    ) -> Result<Result<Option<DimensionDimension>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "level.find-dimension")?;
            let level = resolve_level(self.host.clone()).map_err(map_core_host_error)?;
            let dimension = level.getDimension(&name);
            if dimension.is_null() {
                return Ok(None);
            }
            wit_dimension_summary_native(&dimension)
                .map(Some)
                .map_err(map_core_host_error)
        })())
    }

    fn list_actors(
        &mut self,
        dimension: Option<String>,
    ) -> Result<Result<Vec<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "level.list-actors")?;
            let level = resolve_level(self.host.clone()).map_err(map_core_host_error)?;
            let actors = level.getActors(dimension.as_deref().unwrap_or(""));
            if actors.is_null() {
                return Err(map_core_host_error(HostError::from_status(
                    AEGILEX_NOT_FOUND,
                )));
            }
            let mut resources = Vec::with_capacity(actors.len());
            for index in 0..actors.len() {
                let actor = actors.get(index);
                if actor.is_null() {
                    return Err(map_core_host_error(HostError::from_status(
                        AEGILEX_NOT_FOUND,
                    )));
                }
                resources.push(
                    self.insert_actor_resource(actor)
                        .map_err(map_core_host_error)?,
                );
            }
            Ok(resources)
        })())
    }
}
