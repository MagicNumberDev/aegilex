//! Core ABI implementation for `native/bindings/endstone/scoreboard/scoreboard.h`.

use crate::host::endstone::support::*;

impl crate::core_host::imports::HostScoreboard for PluginStoreState {
    fn scoreboard_get_objective(
        &mut self,
        self_: u32,
        name: String,
    ) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "scoreboard.scoreboard.get-objective")?;
            let handle = scoreboard_handle(self, self_).map_err(map_core_host_error)?;
            let objective = resolve_scoreboard(self, handle)
                .map_err(map_core_host_error)?
                .getObjective(&name);
            if objective.is_null() {
                return Ok(None);
            }
            insert_objective_value(self, handle, name)
                .map(Some)
                .map_err(map_core_host_error)
        })())
    }

    fn scoreboard_create_objective(
        &mut self,
        self_: u32,
        name: String,
        criteria: CriteriaCriteria,
        display_name: String,
        render_type: Option<RenderTypeRenderType>,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "scoreboard.scoreboard.create-objective")?;
            if criteria.name != "dummy" || criteria.read_only {
                return Err(map_core_host_error(invalid_input()));
            }
            let handle = scoreboard_handle(self, self_).map_err(map_core_host_error)?;
            let render_type_raw = render_type.map(to_render_type);
            let objective = resolve_scoreboard(self, handle)
                .map_err(map_core_host_error)?
                .createObjective(
                    &name,
                    &display_name,
                    render_type_raw.is_some(),
                    render_type_raw.unwrap_or(0),
                );
            if objective.is_null() {
                return Err(map_core_host_error(not_found()));
            }
            insert_objective_value(self, handle, name).map_err(map_core_host_error)
        })())
    }

    fn scoreboard_remove_objective(
        &mut self,
        self_: u32,
        name: String,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "scoreboard.scoreboard.remove-objective")?;
            resolve_scoreboard(
                self,
                scoreboard_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?
            .removeObjective(&name);
            Ok(())
        })())
    }

    fn scoreboard_get_objective_by_slot(
        &mut self,
        self_: u32,
        slot: DisplaySlotDisplaySlot,
    ) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "scoreboard.scoreboard.get-objective-by-slot")?;
            let handle = scoreboard_handle(self, self_).map_err(map_core_host_error)?;
            let objective = resolve_scoreboard(self, handle)
                .map_err(map_core_host_error)?
                .getObjectiveBySlot(slot as u32);
            if objective.is_null() {
                return Ok(None);
            }
            insert_objective_value(self, handle, objective.getName())
                .map(Some)
                .map_err(map_core_host_error)
        })())
    }

    fn scoreboard_list_objectives(
        &mut self,
        self_: u32,
    ) -> Result<Result<Vec<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "scoreboard.scoreboard.list-objectives")?;
            let handle = scoreboard_handle(self, self_).map_err(map_core_host_error)?;
            let names = resolve_scoreboard(self, handle)
                .map_err(map_core_host_error)?
                .listObjectives();
            names
                .into_iter()
                .map(|name| insert_objective_value(self, handle, name).map_err(map_core_host_error))
                .collect()
        })())
    }

    fn scoreboard_list_objectives_by_criteria(
        &mut self,
        self_: u32,
        criteria_name: String,
    ) -> Result<Result<Vec<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "scoreboard.scoreboard.list-objectives-by-criteria")?;
            if criteria_name != "dummy" {
                return Err(map_core_host_error(invalid_input()));
            }
            let handle = scoreboard_handle(self, self_).map_err(map_core_host_error)?;
            let names = resolve_scoreboard(self, handle)
                .map_err(map_core_host_error)?
                .listObjectivesByCriteria(0);
            names
                .into_iter()
                .map(|name| insert_objective_value(self, handle, name).map_err(map_core_host_error))
                .collect()
        })())
    }

    fn scoreboard_clear_slot(
        &mut self,
        self_: u32,
        slot: DisplaySlotDisplaySlot,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "scoreboard.scoreboard.clear-slot")?;
            resolve_scoreboard(
                self,
                scoreboard_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?
            .clearSlot(slot as u32);
            Ok(())
        })())
    }

    fn scoreboard_reset_scores(
        &mut self,
        self_: u32,
        entry: u32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "scoreboard.scoreboard.reset-scores")?;
            let handle = scoreboard_handle(self, self_).map_err(map_core_host_error)?;
            let scoreboard = resolve_scoreboard(self, handle).map_err(map_core_host_error)?;
            let entry = resolve_score_entry(
                self,
                score_entry_handle(self, entry).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?;
            scoreboard.resetScores(entry);
            Ok(())
        })())
    }

    fn scoreboard_get_scores(
        &mut self,
        self_: u32,
        entry: u32,
    ) -> Result<Result<Vec<ScoreScore>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "scoreboard.scoreboard.get-scores")?;
            let handle = scoreboard_handle(self, self_).map_err(map_core_host_error)?;
            let scoreboard = resolve_scoreboard(self, handle).map_err(map_core_host_error)?;
            let entry = resolve_score_entry(
                self,
                score_entry_handle(self, entry).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?;
            get_scores(scoreboard, entry).map_err(map_core_host_error)
        })())
    }

    fn scoreboard_list_entries(
        &mut self,
        self_: u32,
    ) -> Result<Result<Vec<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "scoreboard.scoreboard.list-entries")?;
            let handle = scoreboard_handle(self, self_).map_err(map_core_host_error)?;
            let mut entries = resolve_scoreboard(self, handle)
                .map_err(map_core_host_error)?
                .listEntries();
            if entries.is_null() {
                return Err(map_core_host_error(not_found()));
            }
            let mut result = Vec::with_capacity(entries.len());
            for index in 0..entries.len() {
                let entry = entries.pin_mut().take(index);
                if entry.is_null() {
                    return Err(map_core_host_error(not_found()));
                }
                result.push(
                    self.insert_score_entry_resource(entry)
                        .map_err(map_core_host_error)?,
                );
            }
            Ok(result)
        })())
    }

    fn get_primary(&mut self) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "scoreboard.get-primary")?;
            let scoreboard = self
                .host
                .server()
                .map_err(map_core_host_error)?
                .getScoreboard();
            if scoreboard.is_null() {
                return Ok(None);
            }
            self.insert_scoreboard_resource(scoreboard)
                .map(Some)
                .map_err(map_core_host_error)
        })())
    }

    fn create(&mut self) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "scoreboard.create")?;
            let scoreboard = self
                .host
                .server()
                .map_err(map_core_host_error)?
                .createScoreboard();
            if scoreboard.is_null() {
                return Err(map_core_host_error(not_found()));
            }
            self.insert_scoreboard_resource(scoreboard)
                .map_err(map_core_host_error)
        })())
    }

    fn get_player_scoreboard(
        &mut self,
        player: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "scoreboard.get-player-scoreboard")?;
            let scoreboard = resolve_player(
                self,
                player_handle(self, player).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?
            .getScoreboard();
            if scoreboard.is_null() {
                return Err(map_core_host_error(not_found()));
            }
            self.insert_scoreboard_resource(scoreboard)
                .map_err(map_core_host_error)
        })())
    }

    fn set_player_scoreboard(
        &mut self,
        player: u32,
        scoreboard: u32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "scoreboard.set-player-scoreboard")?;
            let player = resolve_player(
                self,
                player_handle(self, player).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?;
            let scoreboard = resolve_scoreboard(
                self,
                scoreboard_handle(self, scoreboard).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?;
            player.setScoreboard(scoreboard);
            Ok(())
        })())
    }
}
