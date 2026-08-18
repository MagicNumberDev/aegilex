#[allow(clippy::too_many_arguments)]
#[cxx::bridge(namespace = "aegilex::native::actor")]
pub(crate) mod ffi {
    struct Location {
        x: f32,
        y: f32,
        z: f32,
        pitch: f32,
        yaw: f32,
        dimension: String,
    }

    struct Vector {
        x: f32,
        y: f32,
        z: f32,
    }

    unsafe extern "C++" {
        include!("bindings/endstone/actor/actor.h");
        include!("bindings/endstone/actor/item_actor.h");
        include!("bindings/endstone/actor/player.h");
        include!("bindings/endstone/inventory/item_stack.h");
        include!("bindings/endstone/actor/mob.h");

        type Actor;
        type Mob;
        type ItemActor;
        #[namespace = "aegilex::native::player"]
        type Player = crate::cxx_host_player::ffi::Player;
        #[namespace = "aegilex::native::inventory"]
        type ItemStack = crate::cxx_host_inventory::ffi::ItemStack;

        fn getName(self: &Actor) -> String;
        fn getType(self: &Actor) -> String;
        fn getRuntimeId(self: &Actor) -> u64;
        fn getId(self: &Actor) -> i64;
        fn isValid(self: &Actor) -> bool;
        fn isDead(self: &Actor) -> bool;
        fn isOnGround(self: &Actor) -> bool;
        fn isInWater(self: &Actor) -> bool;
        fn isInLava(self: &Actor) -> bool;
        fn getLocation(self: &Actor) -> Location;
        fn getDimensionLocation(self: &Actor) -> Location;
        fn getVelocity(self: &Actor) -> Vector;
        fn getLevelName(self: &Actor) -> String;
        fn isNameTagVisible(self: &Actor) -> bool;
        fn isNameTagAlwaysVisible(self: &Actor) -> bool;
        fn getNameTag(self: &Actor) -> String;
        fn getScoreTag(self: &Actor) -> String;
        fn getScoreboardTags(self: &Actor) -> Vec<String>;
        fn addScoreboardTag(self: &Actor, tag: &str) -> bool;
        fn removeScoreboardTag(self: &Actor, tag: &str) -> bool;
        fn setRotation(self: &Actor, yaw: f32, pitch: f32);
        fn setNameTagVisible(self: &Actor, visible: bool);
        fn setNameTagAlwaysVisible(self: &Actor, always_visible: bool);
        fn setNameTag(self: &Actor, name_tag: &str);
        fn setScoreTag(self: &Actor, score_tag: &str);
        fn teleport(self: &Actor, location: &Location) -> bool;
        fn teleportToActor(self: &Actor, target: &Actor) -> bool;
        fn remove(self: &Actor);
        fn asMob(self: &Actor) -> UniquePtr<Mob>;
        fn asItemActor(self: &Actor) -> UniquePtr<ItemActor>;
        fn asPlayer(self: &Actor) -> UniquePtr<Player>;

        fn getItemStack(self: &ItemActor) -> UniquePtr<ItemStack>;
        fn setItemStack(self: &ItemActor, item: &ItemStack) -> bool;
        fn getPickupDelay(self: &ItemActor) -> i32;
        fn setPickupDelay(self: &ItemActor, delay: i32) -> bool;
        fn isUnlimitedLifetime(self: &ItemActor) -> bool;
        fn setUnlimitedLifetime(self: &ItemActor, unlimited: bool) -> bool;
        fn getThrower(self: &ItemActor, has_thrower: &mut bool, thrower: &mut i64) -> bool;
        fn setThrower(self: &ItemActor, has_thrower: bool, thrower: i64) -> bool;
        fn asActor(self: &ItemActor) -> UniquePtr<Actor>;

        fn isGliding(self: &Mob) -> bool;
        fn getHealth(self: &Mob) -> i32;
        fn getMaxHealth(self: &Mob) -> i32;
        fn setHealth(self: &Mob, health: i32);
        fn setMaxHealth(self: &Mob, health: i32);
        // Facade symmetry: Mob is not an Actor subclass on the Rust side.
        #[allow(dead_code)]
        fn asActor(self: &Mob) -> UniquePtr<Actor>;

    }
}
