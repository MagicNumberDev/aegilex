//! Core ABI implementation for `native/bindings/endstone/scoreboard/score_entry.h`.

use crate::host::endstone::support::*;

impl crate::core_host::imports::HostScoreEntry for PluginStoreState {
    fn score_entry_as_player(
        &mut self,
        self_: u32,
    ) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "score-entry.score-entry.as-player")?;
            let entry = resolve_score_entry(
                self,
                score_entry_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?;
            let player = entry.asPlayer();
            (!player.is_null())
                .then(|| {
                    self.insert_player_resource(player)
                        .map_err(map_core_host_error)
                })
                .transpose()
        })())
    }

    fn score_entry_as_actor(
        &mut self,
        self_: u32,
    ) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "score-entry.score-entry.as-actor")?;
            let entry = resolve_score_entry(
                self,
                score_entry_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?;
            let actor = entry.asActor();
            (!actor.is_null())
                .then(|| {
                    self.insert_actor_resource(actor)
                        .map_err(map_core_host_error)
                })
                .transpose()
        })())
    }

    fn score_entry_as_text(
        &mut self,
        self_: u32,
    ) -> Result<Result<Option<String>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "score-entry.score-entry.as-text")?;
            let entry = resolve_score_entry(
                self,
                score_entry_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?;
            Ok((entry.kind() == 2).then(|| entry.getText()))
        })())
    }

    fn from_text(&mut self, text: String) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "score-entry.from-text")?;
            let entry = cxx_ui::ScoreEntry::from_text(&text);
            if entry.is_null() {
                return Err(map_core_host_error(invalid_input()));
            }
            self.insert_score_entry_resource(entry)
                .map_err(map_core_host_error)
        })())
    }

    fn from_player(&mut self, player: u32) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "score-entry.from-player")?;
            let player = resolve_player(
                self,
                player_handle(self, player).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?;
            let entry = cxx_ui::ScoreEntry::from_player(player);
            if entry.is_null() {
                return Err(map_core_host_error(not_found()));
            }
            self.insert_score_entry_resource(entry)
                .map_err(map_core_host_error)
        })())
    }

    fn from_actor(&mut self, actor: u32) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "score-entry.from-actor")?;
            let actor = resolve_actor(
                self,
                actor_handle(self, actor).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?;
            let entry = cxx_ui::ScoreEntry::from_actor(actor);
            if entry.is_null() {
                return Err(map_core_host_error(not_found()));
            }
            self.insert_score_entry_resource(entry)
                .map_err(map_core_host_error)
        })())
    }
}
