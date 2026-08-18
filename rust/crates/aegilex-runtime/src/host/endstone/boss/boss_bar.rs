//! Core ABI implementation for `native/bindings/endstone/boss/boss_bar.h`.

use crate::host::endstone::support::*;

impl crate::core_host::imports::HostBossBar for PluginStoreState {
    fn boss_bar_get_title(&mut self, self_: u32) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            resolve_boss_bar(
                self,
                boss_bar_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|bar| bar.getTitle())
            .map_err(map_core_host_error)
        })())
    }

    fn boss_bar_set_title(
        &mut self,
        self_: u32,
        title: String,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            resolve_boss_bar(
                self,
                boss_bar_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|bar| bar.setTitle(&title))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn boss_bar_get_color(
        &mut self,
        self_: u32,
    ) -> Result<Result<BarColorBossBarColor, TypesHostError>, String> {
        Ok((|| {
            resolve_boss_bar(
                self,
                boss_bar_handle(self, self_).map_err(map_core_host_error)?,
            )
            .and_then(|bar| from_color(bar.getColor()))
            .map_err(map_core_host_error)
        })())
    }

    fn boss_bar_set_color(
        &mut self,
        self_: u32,
        color: BarColorBossBarColor,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            resolve_boss_bar(
                self,
                boss_bar_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|bar| bar.setColor(color as u32))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn boss_bar_get_style(
        &mut self,
        self_: u32,
    ) -> Result<Result<BarStyleBossBarStyle, TypesHostError>, String> {
        Ok((|| {
            resolve_boss_bar(
                self,
                boss_bar_handle(self, self_).map_err(map_core_host_error)?,
            )
            .and_then(|bar| from_style(bar.getStyle()))
            .map_err(map_core_host_error)
        })())
    }

    fn boss_bar_set_style(
        &mut self,
        self_: u32,
        style: BarStyleBossBarStyle,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            resolve_boss_bar(
                self,
                boss_bar_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|bar| bar.setStyle(style as u32))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn boss_bar_has_flag(
        &mut self,
        self_: u32,
        flag: BarFlagBossBarFlag,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            resolve_boss_bar(
                self,
                boss_bar_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|bar| bar.hasFlag(flag as u32))
            .map_err(map_core_host_error)
        })())
    }

    fn boss_bar_add_flag(
        &mut self,
        self_: u32,
        flag: BarFlagBossBarFlag,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            resolve_boss_bar(
                self,
                boss_bar_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|bar| bar.addFlag(flag as u32))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn boss_bar_remove_flag(
        &mut self,
        self_: u32,
        flag: BarFlagBossBarFlag,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            resolve_boss_bar(
                self,
                boss_bar_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|bar| bar.removeFlag(flag as u32))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn boss_bar_get_progress(&mut self, self_: u32) -> Result<Result<f32, TypesHostError>, String> {
        Ok((|| {
            resolve_boss_bar(
                self,
                boss_bar_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|bar| bar.getProgress())
            .map_err(map_core_host_error)
        })())
    }

    fn boss_bar_set_progress(
        &mut self,
        self_: u32,
        progress: f32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            resolve_boss_bar(
                self,
                boss_bar_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|bar| bar.setProgress(progress))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn boss_bar_is_visible(&mut self, self_: u32) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            resolve_boss_bar(
                self,
                boss_bar_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|bar| bar.isVisible())
            .map_err(map_core_host_error)
        })())
    }

    fn boss_bar_set_visible(
        &mut self,
        self_: u32,
        visible: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            resolve_boss_bar(
                self,
                boss_bar_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|bar| bar.setVisible(visible))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn boss_bar_add_player(
        &mut self,
        self_: u32,
        player: u32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            let player = resolve_player(
                self,
                player_handle(self, player).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?;
            resolve_boss_bar(
                self,
                boss_bar_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|bar| bar.addPlayer(player))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn boss_bar_remove_player(
        &mut self,
        self_: u32,
        player: u32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            let player = resolve_player(
                self,
                player_handle(self, player).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?;
            resolve_boss_bar(
                self,
                boss_bar_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|bar| bar.removePlayer(player))
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn boss_bar_remove_all_players(
        &mut self,
        self_: u32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            resolve_boss_bar(
                self,
                boss_bar_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|bar| bar.removeAllPlayers())
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn boss_bar_get_players(
        &mut self,
        self_: u32,
    ) -> Result<Result<Vec<u32>, TypesHostError>, String> {
        Ok((|| {
            let mut players = resolve_boss_bar(
                self,
                boss_bar_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?
            .getPlayers();
            if players.is_null() {
                return Err(map_core_host_error(not_found()));
            }
            let mut result = Vec::with_capacity(players.len());
            for index in 0..players.len() {
                result.push(
                    self.insert_player_resource(players.pin_mut().takePlayer(index))
                        .map_err(map_core_host_error)?,
                );
            }
            Ok(result)
        })())
    }

    fn create(
        &mut self,
        title: String,
        color: BarColorBossBarColor,
        style: BarStyleBossBarStyle,
        flag_list: Vec<BarFlagBossBarFlag>,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "boss-bar.create")?;
            let flags = flag_list
                .iter()
                .map(|flag| *flag as u32)
                .collect::<Vec<_>>();
            let bar = cxx_ui::BossBar::create_for_server(
                self.host.server().map_err(map_core_host_error)?,
                &title,
                color as u32,
                style as u32,
                &flags,
            );
            self.insert_boss_bar_resource(bar)
                .map_err(map_core_host_error)
        })())
    }
}
