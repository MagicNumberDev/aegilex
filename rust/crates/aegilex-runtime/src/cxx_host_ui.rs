#[allow(clippy::too_many_arguments)]
#[cxx::bridge(namespace = "aegilex::native::ui")]
pub(crate) mod ffi {
    struct ScoreSummary {
        objective_name: String,
        value: i32,
        score_set: bool,
    }

    struct ScoreSummaryList {
        status: u32,
        scores: Vec<ScoreSummary>,
    }

    struct ScoreValue {
        status: u32,
        value: i32,
    }

    unsafe extern "C++" {
        include!("bindings/endstone/boss/boss_bar.h");
        include!("bindings/endstone/scoreboard/scoreboard.h");
        include!("bindings/endstone/scoreboard/objective.h");
        include!("bindings/endstone/scoreboard/score_entry.h");
        include!("bindings/endstone/map/map.h");
        include!("bindings/endstone/server.h");

        type BossBar;
        type Scoreboard;
        type Objective;
        type Map;
        type PlayerCollection;
        type ScoreEntry;
        type ScoreEntryCollection;
        #[namespace = "aegilex::native::server"]
        type Server = crate::cxx_host_server::ffi::Server;

        #[Self = "BossBar"]
        #[rust_name = "create_for_server"]
        fn create(
            server: &Server,
            title: &str,
            color: u32,
            style: u32,
            flags: &[u32],
        ) -> UniquePtr<BossBar>;
        fn getTitle(self: &BossBar) -> String;
        fn setTitle(self: &BossBar, title: &str);
        fn getColor(self: &BossBar) -> u32;
        fn setColor(self: &BossBar, color: u32);
        fn getStyle(self: &BossBar) -> u32;
        fn setStyle(self: &BossBar, style: u32);
        fn hasFlag(self: &BossBar, flag: u32) -> bool;
        fn addFlag(self: &BossBar, flag: u32);
        fn removeFlag(self: &BossBar, flag: u32);
        fn getProgress(self: &BossBar) -> f32;
        fn setProgress(self: &BossBar, progress: f32);
        fn isVisible(self: &BossBar) -> bool;
        fn setVisible(self: &BossBar, visible: bool);
        fn addPlayer(self: &BossBar, player: &Player);
        fn removePlayer(self: &BossBar, player: &Player);
        fn removeAllPlayers(self: &BossBar);
        fn getPlayers(self: &BossBar) -> UniquePtr<PlayerCollection>;
        fn len(self: &PlayerCollection) -> usize;
        fn takePlayer(self: Pin<&mut PlayerCollection>, index: usize) -> UniquePtr<Player>;

        fn getScoreboard(self: &Server) -> UniquePtr<Scoreboard>;
        fn createScoreboard(self: &Server) -> UniquePtr<Scoreboard>;
        fn getObjective(self: &Scoreboard, name: &str) -> UniquePtr<Objective>;
        #[allow(dead_code)]
        fn createObjective(
            self: &Scoreboard,
            name: &str,
            display_name: &str,
            has_render_type: bool,
            render_type: u8,
        ) -> UniquePtr<Objective>;
        fn getObjectiveBySlot(self: &Scoreboard, slot: u32) -> UniquePtr<Objective>;
        fn removeObjective(self: &Scoreboard, name: &str);
        fn listObjectives(self: &Scoreboard) -> Vec<String>;
        fn listObjectivesByCriteria(self: &Scoreboard, criteria: u32) -> Vec<String>;
        fn clearSlot(self: &Scoreboard, slot: u32);
        #[Self = "ScoreEntry"]
        #[rust_name = "from_text"]
        fn fromText(text: &str) -> UniquePtr<ScoreEntry>;
        #[Self = "ScoreEntry"]
        #[rust_name = "from_player"]
        fn fromPlayer(player: &Player) -> UniquePtr<ScoreEntry>;
        #[Self = "ScoreEntry"]
        #[rust_name = "from_actor"]
        fn fromActor(actor: &Actor) -> UniquePtr<ScoreEntry>;
        fn kind(self: &ScoreEntry) -> u8;
        fn asPlayer(self: &ScoreEntry) -> UniquePtr<Player>;
        fn asActor(self: &ScoreEntry) -> UniquePtr<Actor>;
        fn getText(self: &ScoreEntry) -> String;
        fn resetScores(self: &Scoreboard, entry: &ScoreEntry);
        fn getScores(self: &Scoreboard, entry: &ScoreEntry) -> ScoreSummaryList;
        fn listEntries(self: &Scoreboard) -> UniquePtr<ScoreEntryCollection>;
        fn len(self: &ScoreEntryCollection) -> usize;
        fn take(self: Pin<&mut ScoreEntryCollection>, index: usize) -> UniquePtr<ScoreEntry>;

        // Objective: resolved by name on the owning scoreboard.
        fn getName(self: &Objective) -> String;
        fn getDisplayName(self: &Objective) -> String;
        fn setDisplayName(self: &Objective, display_name: &str);
        fn getCriteria(self: &Objective) -> u32;
        fn isModifiable(self: &Objective) -> bool;
        fn isDisplayed(self: &Objective) -> bool;
        fn getDisplaySlot(self: &Objective, out: &mut u32) -> bool;
        fn setDisplaySlot(self: &Objective, has_slot: bool, slot: u32);
        fn getSortOrder(self: &Objective, out: &mut u32) -> bool;
        fn setSortOrder(self: &Objective, order: u32);
        fn setDisplay(self: &Objective, has_slot: bool, slot: u32, order: u32);
        fn getRenderType(self: &Objective) -> u8;
        fn unregister(self: &Objective);
        fn getScoreValue(self: &Objective, entry: &ScoreEntry) -> ScoreValue;
        fn setScoreValue(self: &Objective, entry: &ScoreEntry, value: i32) -> bool;

        // Map: ids are server-allocated i64.
        fn getMap(self: &Server, id: i64) -> UniquePtr<Map>;
        fn createMap(self: &Server, dimension: &str) -> UniquePtr<Map>;
        #[allow(dead_code)]
        fn getId(self: &Map) -> i64;
        fn isVirtual(self: &Map) -> bool;
        fn getScale(self: &Map) -> u8;
        fn setScale(self: &Map, scale: u8);
        fn getCenterX(self: &Map) -> i32;
        fn setCenterX(self: &Map, x: i32);
        fn getCenterZ(self: &Map) -> i32;
        fn setCenterZ(self: &Map, z: i32);
        fn getDimensionName(self: &Map) -> String;
        fn isUnlimitedTracking(self: &Map) -> bool;
        fn setUnlimitedTracking(self: &Map, unlimited: bool);
        fn isLocked(self: &Map) -> bool;
        fn setLocked(self: &Map, locked: bool);
        fn setDimension(self: &Map, dimension: &str) -> bool;
    }

    #[namespace = "aegilex::native::player"]
    unsafe extern "C++" {
        include!("bindings/endstone/actor/player.h");

        type Player = crate::cxx_host_player::ffi::Player;
    }

    #[namespace = "aegilex::native::actor"]
    unsafe extern "C++" {
        include!("bindings/endstone/actor/actor.h");

        type Actor = crate::cxx_host_actor::ffi::Actor;
    }
}
