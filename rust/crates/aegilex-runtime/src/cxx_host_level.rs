#[allow(clippy::too_many_arguments)]
#[cxx::bridge(namespace = "aegilex::native::level")]
pub(crate) mod ffi {
    struct Location {
        x: f32,
        y: f32,
        z: f32,
        pitch: f32,
        yaw: f32,
        dimension: String,
    }

    struct DimensionSummary {
        name: String,
        kind: u32,
        level: String,
    }

    struct ChunkSummary {
        dimension: String,
        x: i32,
        z: i32,
        level_name: String,
    }

    // Block states are flat pairs: value_kind selects the active member
    // (0 = boolean, 1 = text, 2 = integer). cxx cannot express a Rust
    // enum with payload, so the conversion lives in host/level.rs.
    struct BlockStatePair {
        key: String,
        value_kind: u32,
        boolean: bool,
        text: String,
        integer: i32,
    }

    struct BlockData {
        type_id: String,
        states: Vec<BlockStatePair>,
        runtime_id: u32,
    }

    struct BlockSnapshot {
        dimension: String,
        x: i32,
        y: i32,
        z: i32,
        type_id: String,
        states: Vec<BlockStatePair>,
        runtime_id: u32,
    }

    unsafe extern "C++" {
        include!("bindings/endstone/level/level.h");
        include!("bindings/endstone/level/dimension.h");
        include!("bindings/endstone/level/chunk.h");
        include!("bindings/endstone/level/block.h");
        include!("bindings/endstone/actor/actor.h");
        include!("bindings/endstone/inventory/item_stack.h");
        include!("bindings/endstone/server.h");

        type Level;
        type Dimension;
        type Chunk;
        type Block;
        type ActorCollection;
        #[namespace = "aegilex::native::server"]
        type Server = crate::cxx_host_server::ffi::Server;
        #[namespace = "aegilex::native::actor"]
        type Actor = crate::cxx_host_actor::ffi::Actor;
        #[namespace = "aegilex::native::inventory"]
        type ItemStack = crate::cxx_host_inventory::ffi::ItemStack;

        // Level facade — mirrors endstone/level/level.h.
        fn getLevel(self: &Server) -> UniquePtr<Level>;
        fn getName(self: &Level) -> String;
        fn getTime(self: &Level) -> i32;
        fn setTime(self: &Level, time: i32);
        fn getSeed(self: &Level) -> i64;
        fn getDimensions(self: &Level) -> Vec<DimensionSummary>;
        fn getDimension(self: &Level, name: &str) -> UniquePtr<Dimension>;

        // Dimension facade — mirrors endstone/level/dimension.h. The type is
        // the raw endstone enum value (0 = Overworld, 1 = Nether, 2 = TheEnd,
        // 999 = Custom); conversion to the WIT enum happens in host/level.rs.
        fn getName(self: &Dimension) -> String;
        fn getType(self: &Dimension) -> u32;
        fn getLevelName(self: &Dimension) -> String;

        // Chunk facade — mirrors endstone/level/chunk.h. The WIT surface only
        // consumes chunk summaries, so the facade methods are reserved for
        // future chunk handles.
        #[allow(dead_code)]
        fn getX(self: &Chunk) -> i32;
        #[allow(dead_code)]
        fn getZ(self: &Chunk) -> i32;
        #[allow(dead_code)]
        fn getLevelName(self: &Chunk) -> String;
        #[allow(dead_code)]
        fn getDimensionName(self: &Chunk) -> String;

        // Block facade — mirrors endstone/block/block.h. Every facade owns
        // an invocation-scoped Endstone block view.
        fn getType(self: &Block) -> String;
        fn setType(self: &Block, type_id: &str, apply_physics: bool);
        fn getX(self: &Block) -> i32;
        fn getY(self: &Block) -> i32;
        fn getZ(self: &Block) -> i32;
        fn getLocation(self: &Block) -> Location;
        fn getData(self: &Block) -> BlockData;
        fn captureState(self: &Block) -> BlockSnapshot;
        fn getRelative(self: &Block, dx: i32, dy: i32, dz: i32) -> UniquePtr<Block>;
        fn clone(self: &Block) -> UniquePtr<Block>;

        fn setBlock(
            self: &Level,
            dimension: &str,
            x: i32,
            y: i32,
            z: i32,
            type_id: &str,
            states: &Vec<BlockStatePair>,
            has_apply_physics: bool,
            apply_physics: bool,
        ) -> u32;
        fn getHighestBlockY(self: &Level, dimension: &str, x: i32, z: i32, out_y: &mut i32) -> u32;
        fn listLoadedChunks(self: &Level, dimension: &str, out: &mut Vec<ChunkSummary>) -> u32;
        fn updateBlockState(
            self: &Level,
            state: &BlockSnapshot,
            has_force: bool,
            force: bool,
            has_apply_physics: bool,
            apply_physics: bool,
            out_applied: &mut bool,
        ) -> u32;
        fn createBlockData(
            self: &Level,
            type_id: &str,
            states: &Vec<BlockStatePair>,
            out: &mut BlockData,
        ) -> u32;
        fn blockTypeHasItem(self: &Level, type_id: &str, out_has_item: &mut bool) -> u32;
        fn setData(self: &Block, data: &BlockData, apply_physics: bool) -> u32;

        fn getBlock(self: &Level, dimension: &str, x: i32, y: i32, z: i32) -> UniquePtr<Block>;
        fn getHighestBlock(self: &Level, dimension: &str, x: i32, z: i32) -> UniquePtr<Block>;
        fn getRelativeBlock(
            self: &Level,
            dimension: &str,
            x: i32,
            y: i32,
            z: i32,
            dx: i32,
            dy: i32,
            dz: i32,
        ) -> UniquePtr<Block>;
        fn getActors(self: &Level, dimension: &str) -> UniquePtr<ActorCollection>;
        fn spawnActor(
            self: &Level,
            dimension: &str,
            location: &Location,
            actor_type: &str,
        ) -> UniquePtr<Actor>;
        fn dropItem(
            self: &Level,
            dimension: &str,
            location: &Location,
            item: &ItemStack,
        ) -> UniquePtr<Actor>;
        fn len(self: &ActorCollection) -> usize;
        fn get(self: &ActorCollection, index: usize) -> UniquePtr<Actor>;
    }
}
