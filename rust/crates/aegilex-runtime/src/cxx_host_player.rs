#[allow(clippy::too_many_arguments)]
#[cxx::bridge(namespace = "aegilex::native::player")]
pub(crate) mod ffi {
    struct Location {
        x: f32,
        y: f32,
        z: f32,
        pitch: f32,
        yaw: f32,
        dimension: String,
    }

    struct SkinData {
        id: String,
        width: u32,
        height: u32,
        pixels: Vec<u8>,
    }

    struct SocketAddress {
        hostname: String,
        port: u32,
    }

    enum GameMode {
        Survival = 0,
        Creative = 1,
        Adventure = 2,
        Spectator = 3,
    }

    unsafe extern "C++" {
        include!("bindings/endstone/actor/player.h");
        include!("bindings/endstone/actor/actor.h");
        include!("bindings/endstone/map/map.h");
        include!("bindings/endstone/scoreboard/scoreboard.h");

        type Player;
        #[namespace = "aegilex::native::actor"]
        type Actor = crate::cxx_host_actor::ffi::Actor;
        #[namespace = "aegilex::native::ui"]
        type Map = crate::cxx_host_ui::ffi::Map;
        #[namespace = "aegilex::native::ui"]
        type Scoreboard = crate::cxx_host_ui::ffi::Scoreboard;

        #[allow(dead_code)]
        fn clone(self: &Player) -> UniquePtr<Player>;
        fn asActor(self: &Player) -> UniquePtr<Actor>;

        // Identity.
        fn getName(self: &Player) -> String;
        fn getUniqueId(self: &Player) -> Vec<u8>;
        fn getXuid(self: &Player) -> String;
        fn isOp(self: &Player) -> bool;
        fn setOp(self: &Player, value: bool);
        fn getPing(self: &Player) -> u32;
        fn getLocale(self: &Player) -> String;
        fn getGameVersion(self: &Player) -> String;
        fn getDeviceOS(self: &Player) -> String;
        fn getDeviceId(self: &Player) -> String;
        fn getAddress(self: &Player) -> SocketAddress;
        fn sendPacket(self: &Player, packet_id: i32, payload: &[u8]);

        // Lifecycle.
        fn transfer(self: &Player, host: &str, port: u16);
        fn kick(self: &Player, message: &str);
        fn performCommand(self: &Player, command: &str) -> bool;
        fn updateCommands(self: &Player);

        // Movement.
        fn isSneaking(self: &Player) -> bool;
        fn setSneaking(self: &Player, sneaking: bool);
        fn isSprinting(self: &Player) -> bool;
        fn setSprinting(self: &Player, sprinting: bool);

        // Experience.
        fn getExpProgress(self: &Player) -> f32;
        fn setExpProgress(self: &Player, progress: f32);
        fn getExpLevel(self: &Player) -> i32;
        fn setExpLevel(self: &Player, level: i32);
        fn getTotalExp(self: &Player) -> i32;
        fn giveExp(self: &Player, amount: i32);
        fn giveExpLevels(self: &Player, amount: i32);

        // Flight and movement speed.
        fn getAllowFlight(self: &Player) -> bool;
        fn setAllowFlight(self: &Player, allow: bool);
        fn isFlying(self: &Player) -> bool;
        fn setFlying(self: &Player, flying: bool);
        fn getFlySpeed(self: &Player) -> f32;
        fn setFlySpeed(self: &Player, speed: f32);
        fn getWalkSpeed(self: &Player) -> f32;
        fn setWalkSpeed(self: &Player, speed: f32);

        // Messaging.
        fn sendMessage(self: &Player, message: &str);
        fn sendPopup(self: &Player, text: &str);
        fn sendTip(self: &Player, text: &str);
        fn sendToast(self: &Player, title: &str, content: &str);
        // fade_in < 0 means "default timings" (Endstone's two-argument form).
        fn sendTitle(
            self: &Player,
            title: &str,
            subtitle: &str,
            fade_in: i32,
            stay: i32,
            fade_out: i32,
        );
        fn resetTitle(self: &Player);

        // Sound and particles. An empty molang_json means no molang variables.
        fn playSound(self: &Player, location: &Location, sound: &str, volume: f32, pitch: f32);
        fn stopSound(self: &Player, sound: &str);
        fn stopAllSounds(self: &Player);
        fn spawnParticle(self: &Player, name: &str, x: f32, y: f32, z: f32, molang_json: &str);

        // Game mode and skin.
        fn getGameMode(self: &Player) -> GameMode;
        fn setGameMode(self: &Player, mode: GameMode);
        fn getSkin(self: &Player) -> SkinData;

        // Rust resolves guest-visible IDs before combining typed facades.
        fn getScoreboard(self: &Player) -> UniquePtr<Scoreboard>;
        fn setScoreboard(self: &Player, scoreboard: &Scoreboard);
        fn sendMap(self: &Player, map: &Map);
    }
}
