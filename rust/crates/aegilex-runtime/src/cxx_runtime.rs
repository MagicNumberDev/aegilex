use std::path::Path;

use cxx::CxxString;

use crate::abi::{AEGILEX_INVALID_ARGUMENT, AEGILEX_OK};
use crate::config::RuntimeConfig as CoreRuntimeConfig;
use crate::cxx_runtime::ffi::{
    ActorDamageEventFacade, ActorDeathEventFacade, ActorExplodeEventFacade,
    ActorKnockbackEventFacade, ActorRemoveEventFacade, ActorSpawnEventFacade,
    ActorTeleportEventFacade, BlockBreakEventFacade, BlockCookEventFacade, BlockFromToEventFacade,
    BlockGrowEventFacade, BlockPistonEventFacade, BlockPlaceEventFacade,
    BroadcastMessageEventFacade, ChunkEventFacade, LeavesDecayEventFacade,
    MapInitializeEventFacade, PacketReceiveEventFacade, PacketSendEventFacade,
    PlayerBedEnterEventFacade, PlayerBedLeaveEventFacade, PlayerChatEventFacade,
    PlayerCommandEventFacade, PlayerDeathEventFacade, PlayerDimensionChangeEventFacade,
    PlayerDropItemEventFacade, PlayerEmoteEventFacade, PlayerGameModeChangeEventFacade,
    PlayerItemConsumeEventFacade, PlayerItemHeldEventFacade, PlayerJoinEventFacade,
    PlayerKickEventFacade, PlayerLoginEventFacade, PlayerMoveEventFacade,
    PlayerPickupItemEventFacade, PlayerQuitEventFacade, PlayerRespawnEventFacade,
    PlayerSkinChangeEventFacade, PluginLifecycleEventFacade, ScriptMessageEventFacade,
    ServerCommandEventFacade, ServerListPingEventFacade, ServerLoadEventFacade,
    ThunderChangeEventFacade, WeatherChangeEventFacade,
};
use crate::host::runtime::native::HostContext as RuntimeHostContext;
use crate::manifest::{PluginLoadOrder, PluginMetadata};
use crate::runtime::Runtime;

pub struct RuntimeHandle {
    runtime: Runtime,
}

#[allow(clippy::ptr_arg)]
#[cxx::bridge(namespace = "aegilex::runtime")]
pub(crate) mod ffi {
    struct RuntimeConfig {
        max_module_bytes: u64,
        max_nested_dispatch_depth: u64,
        max_nbt_depth: u64,
        max_nbt_nodes: u64,
        max_nbt_compound_entries: u64,
        max_nbt_string_bytes: u64,
        max_nbt_array_bytes: u64,
        max_invocation_native_resources: u64,
        max_plugin_resource_slots: u64,
    }

    struct RuntimeCommandSpec {
        name: String,
        description: String,
        aliases: Vec<String>,
        usages: Vec<String>,
        permissions: Vec<String>,
    }

    struct RuntimePermissionChild {
        name: String,
        value: bool,
    }

    struct RuntimePermissionSpec {
        name: String,
        description: String,
        has_default_value: bool,
        default_value: u32,
        children: Vec<RuntimePermissionChild>,
    }

    struct RuntimePluginMetadata {
        name: String,
        version: String,
        description: String,
        load_order: u32,
        authors: Vec<String>,
        contributors: Vec<String>,
        website: String,
        prefix: String,
        provides: Vec<String>,
        depend: Vec<String>,
        soft_depend: Vec<String>,
        load_before: Vec<String>,
        default_permission: u32,
        commands: Vec<RuntimeCommandSpec>,
        permissions: Vec<RuntimePermissionSpec>,
        subscriptions: Vec<String>,
    }

    struct RuntimeInspectResult {
        status: u32,
        error: String,
        metadata: RuntimePluginMetadata,
    }

    struct LocationData {
        dimension: String,
        x: f32,
        y: f32,
        z: f32,
        pitch: f32,
        yaw: f32,
    }

    struct VectorData {
        x: f32,
        y: f32,
        z: f32,
    }

    struct SocketAddress {
        hostname: String,
        port: u32,
    }

    // Form control kinds: 0=button 1=label 2=header 3=divider 4=dropdown
    // 5=slider 6=step-slider 7=text-input 8=toggle. The text member carries
    // the control's title; optional members are present-flagged.
    struct FormControlData {
        text: String,
        icon: String,
        options: Vec<String>,
        placeholder: String,
        default_text: String,
        kind: u32,
        min: f32,
        max: f32,
        step: f32,
        default_float: f32,
        default_index: u32,
        has_icon: bool,
        has_min: bool,
        has_max: bool,
        has_step: bool,
        has_default_float: bool,
        has_default_index: bool,
        has_default_text: bool,
        has_default_bool: bool,
        default_bool: bool,
    }

    // Form kinds: 0=action 1=message 2=modal.
    struct FormSpecData {
        title: String,
        content: String,
        button1: String,
        button2: String,
        controls: Vec<FormControlData>,
        submit_button: String,
        icon: String,
        kind: u32,
        has_content: bool,
        has_button1: bool,
        has_button2: bool,
        has_submit_button: bool,
        has_icon: bool,
    }

    // Submit responses: action -> selected_index, message -> message_button
    // (1 or 2), modal -> raw JSON payload string.
    struct FormResponseData {
        kind: u32,
        has_selected_index: bool,
        selected_index: u32,
        message_button: u8,
        modal_json: String,
    }

    // A complete immutable plugin-manager snapshot; empty metadata name marks
    // not found. Reuse the validated metadata DTO rather than duplicating it.
    struct PluginInfoData {
        metadata: RuntimePluginMetadata,
        enabled: bool,
    }

    struct PluginList {
        status: u32,
        plugins: Vec<PluginInfoData>,
    }

    struct ServiceProviderData {
        id: u64,
        name: String,
        version: String,
        methods: Vec<String>,
        priority: u32,
    }

    struct ServiceListData {
        status: u32,
        providers: Vec<ServiceProviderData>,
    }

    // kind is the service call status (0=pending 1=completed 2=rejected
    // 3=failed 4=cancelled 5=expired); payload holds success bytes, error the
    // rejection text.
    struct ServiceResponseData {
        status: u32,
        kind: u32,
        payload: Vec<u8>,
        error: String,
    }

    struct ServiceCallStatusData {
        status: u32,
        call_status: u32,
    }

    struct MapCursorData {
        x: i8,
        y: i8,
        direction: u8,
        cursor_type: u8,
        visible: bool,
        caption: String,
    }

    // Draw command kinds: 0=set-pixel 1=fill-rect 2=draw-image 3=set-cursors.
    struct MapDrawCommandData {
        kind: u32,
        x: u8,
        y: u8,
        width: u8,
        height: u8,
        argb: u32,
        pixels: Vec<u32>,
        cursors: Vec<MapCursorData>,
    }

    struct DamageSourceData {
        type_id: String,
        has_actor_id: bool,
        actor_id: i64,
        has_damaging_actor_id: bool,
        damaging_actor_id: i64,
        indirect: bool,
    }

    #[allow(dead_code)]
    struct ServerListPingEventData {
        motd: String,
        server_guid: String,
        local_port: i32,
        local_port_v6: i32,
        is_cancelled: bool,
    }

    struct CommandData {
        sender_kind: u8,
        has_player_id: bool,
        player_id: Vec<u8>,
        sender_name: String,
        subcommand: String,
        args: Vec<String>,
    }

    struct CommandOutcome {
        handled: bool,
        reply: String,
        error: String,
    }

    unsafe extern "C++" {
        include!("bindings/endstone/command_sender.h");

        #[namespace = "aegilex::native::host"]
        type CommandSender = crate::cxx_host_common::ffi::CommandSender;
    }

    #[namespace = "aegilex::native::player"]
    unsafe extern "C++" {
        include!("bindings/endstone/actor/player.h");

        type Player = crate::cxx_host_player::ffi::Player;
    }

    #[namespace = "aegilex::native::server"]
    unsafe extern "C++" {
        include!("bindings/endstone/server.h");

        type PlayerCollection = crate::cxx_host_server::ffi::PlayerCollection;
    }

    #[namespace = "aegilex::native::actor"]
    unsafe extern "C++" {
        include!("bindings/endstone/actor/actor.h");

        type Actor = crate::cxx_host_actor::ffi::Actor;
    }

    #[namespace = "aegilex::native::inventory"]
    unsafe extern "C++" {
        include!("bindings/endstone/inventory/item_stack.h");

        type ItemStack = crate::cxx_host_inventory::ffi::ItemStack;
        type ItemStackRef = crate::cxx_host_inventory::ffi::ItemStackRef;
    }

    #[namespace = "aegilex::native::level"]
    unsafe extern "C++" {
        include!("bindings/endstone/level/block.h");

        type Block = crate::cxx_host_level::ffi::Block;
    }

    #[namespace = "aegilex::native::endstone_binding::events"]
    unsafe extern "C++" {
        include!("bindings/endstone/events/player_chat_event_facade.h");
        include!("bindings/endstone/events/player_join_event_facade.h");
        include!("bindings/endstone/events/player_quit_event_facade.h");
        include!("bindings/endstone/events/actor_damage_event_facade.h");
        include!("bindings/endstone/events/actor_death_event_facade.h");
        include!("bindings/endstone/events/block_explode_event_facade.h");
        include!("bindings/endstone/events/actor_explode_event_facade.h");
        include!("bindings/endstone/events/actor_knockback_event_facade.h");
        include!("bindings/endstone/events/actor_remove_event_facade.h");
        include!("bindings/endstone/events/plugin_lifecycle_event_facade.h");
        include!("bindings/endstone/events/server_load_event_facade.h");
        include!("bindings/endstone/events/chunk_event_facade.h");
        include!("bindings/endstone/events/actor_spawn_event_facade.h");
        include!("bindings/endstone/events/actor_teleport_event_facade.h");
        include!("bindings/endstone/events/broadcast_message_event_facade.h");
        include!("bindings/endstone/events/player_command_event_facade.h");
        include!("bindings/endstone/events/player_kick_event_facade.h");
        include!("bindings/endstone/events/player_login_event_facade.h");
        include!("bindings/endstone/events/player_game_mode_change_event_facade.h");
        include!("bindings/endstone/events/player_emote_event_facade.h");
        include!("bindings/endstone/events/player_skin_change_event_facade.h");
        include!("bindings/endstone/events/player_death_event_facade.h");
        include!("bindings/endstone/events/player_dimension_change_event_facade.h");
        include!("bindings/endstone/events/player_bed_enter_event_facade.h");
        include!("bindings/endstone/events/player_bed_leave_event_facade.h");
        include!("bindings/endstone/events/player_respawn_event_facade.h");
        include!("bindings/endstone/events/player_item_held_event_facade.h");
        include!("bindings/endstone/events/player_move_event_facade.h");
        include!("bindings/endstone/events/player_drop_item_event_facade.h");
        include!("bindings/endstone/events/block_break_event_facade.h");
        include!("bindings/endstone/events/block_cook_event_facade.h");
        include!("bindings/endstone/events/block_from_to_event_facade.h");
        include!("bindings/endstone/events/block_grow_event_facade.h");
        include!("bindings/endstone/events/block_piston_event_facade.h");
        include!("bindings/endstone/events/block_place_event_facade.h");
        include!("bindings/endstone/events/leaves_decay_event_facade.h");
        include!("bindings/endstone/events/player_interact_event_facade.h");
        include!("bindings/endstone/events/player_interact_actor_event_facade.h");
        include!("bindings/endstone/events/player_item_consume_event_facade.h");
        include!("bindings/endstone/events/player_pickup_item_event_facade.h");
        include!("bindings/endstone/events/server_command_event_facade.h");
        include!("bindings/endstone/events/server_list_ping_event_facade.h");
        include!("bindings/endstone/events/weather_change_event_facade.h");
        include!("bindings/endstone/events/thunder_change_event_facade.h");
        include!("bindings/endstone/events/packet_send_event_facade.h");
        include!("bindings/endstone/events/packet_receive_event_facade.h");
        include!("bindings/endstone/events/map_initialize_event_facade.h");
        include!("bindings/endstone/events/script_message_event_facade.h");

        type PlayerChatEventFacade;
        type PlayerJoinEventFacade;
        type PlayerQuitEventFacade;
        type ActorDamageEventFacade;
        type ActorDeathEventFacade;
        type BlockExplodeEventFacade;
        type ActorExplodeEventFacade;
        type ActorKnockbackEventFacade;
        type ActorRemoveEventFacade;
        type PluginLifecycleEventFacade;
        type ServerLoadEventFacade;
        type ChunkEventFacade;
        type ActorSpawnEventFacade;
        type ActorTeleportEventFacade;
        type BroadcastMessageEventFacade;
        type PacketSendEventFacade;
        type PacketReceiveEventFacade;
        type MapInitializeEventFacade;
        type PlayerCommandEventFacade;
        type PlayerKickEventFacade;
        type PlayerLoginEventFacade;
        type PlayerGameModeChangeEventFacade;
        type PlayerEmoteEventFacade;
        type PlayerSkinChangeEventFacade;
        type PlayerDeathEventFacade;
        type PlayerDimensionChangeEventFacade;
        type PlayerBedEnterEventFacade;
        type PlayerBedLeaveEventFacade;
        type PlayerRespawnEventFacade;
        type PlayerItemHeldEventFacade;
        type PlayerMoveEventFacade;
        type PlayerDropItemEventFacade;
        type BlockBreakEventFacade;
        type BlockCookEventFacade;
        type BlockFromToEventFacade;
        type BlockGrowEventFacade;
        type BlockPistonEventFacade;
        type BlockPlaceEventFacade;
        type LeavesDecayEventFacade;
        type PlayerInteractEventFacade;
        type PlayerInteractActorEventFacade;
        type PlayerItemConsumeEventFacade;
        type PlayerPickupItemEventFacade;
        type ServerCommandEventFacade;
        type ServerListPingEventFacade;
        type WeatherChangeEventFacade;
        type ThunderChangeEventFacade;
        type ScriptMessageEventFacade;

        fn getPlayer(self: &PlayerChatEventFacade) -> UniquePtr<Player>;
        fn setPlayer(self: Pin<&mut PlayerChatEventFacade>, player: &Player) -> bool;
        fn getMessageForRust(self: &PlayerChatEventFacade) -> String;
        fn getFormatForRust(self: &PlayerChatEventFacade) -> String;
        fn getRecipients(self: &PlayerChatEventFacade) -> UniquePtr<PlayerCollection>;
        fn isCancelled(self: &PlayerChatEventFacade) -> bool;
        fn setMessageForRust(self: Pin<&mut PlayerChatEventFacade>, message: &str) -> bool;
        fn setFormatForRust(self: Pin<&mut PlayerChatEventFacade>, format: &str) -> bool;
        fn setCancelled(self: Pin<&mut PlayerChatEventFacade>, cancelled: bool) -> bool;

        fn getPlayer(self: &PlayerJoinEventFacade) -> UniquePtr<Player>;
        fn hasJoinMessage(self: &PlayerJoinEventFacade) -> bool;
        fn getJoinMessageForRust(self: &PlayerJoinEventFacade) -> String;
        fn setJoinMessageForRust(
            self: Pin<&mut PlayerJoinEventFacade>,
            has_message: bool,
            message: &str,
        ) -> bool;

        fn getPlayer(self: &PlayerQuitEventFacade) -> UniquePtr<Player>;
        fn hasQuitMessage(self: &PlayerQuitEventFacade) -> bool;
        fn getQuitMessageForRust(self: &PlayerQuitEventFacade) -> String;
        fn setQuitMessageForRust(
            self: Pin<&mut PlayerQuitEventFacade>,
            has_message: bool,
            message: &str,
        ) -> bool;

        fn getActor(self: &ActorDamageEventFacade) -> UniquePtr<Actor>;
        fn getDamage(self: &ActorDamageEventFacade) -> f32;
        fn setDamage(self: Pin<&mut ActorDamageEventFacade>, damage: f32) -> bool;
        fn getDamageSource(self: &ActorDamageEventFacade) -> DamageSourceData;
        fn isCancelled(self: &ActorDamageEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut ActorDamageEventFacade>, cancelled: bool) -> bool;

        fn getMessageIdForRust(self: &ScriptMessageEventFacade) -> String;
        fn getMessageForRust(self: &ScriptMessageEventFacade) -> String;
        fn getSender(self: &ScriptMessageEventFacade) -> UniquePtr<CommandSender>;
        fn isCancelled(self: &ScriptMessageEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut ScriptMessageEventFacade>, cancelled: bool) -> bool;

        fn getActor(self: &ActorDeathEventFacade) -> UniquePtr<Actor>;

        fn getBlock(self: &BlockExplodeEventFacade) -> UniquePtr<Block>;
        fn getBlockCount(self: &BlockExplodeEventFacade) -> u64;
        fn getAffectedBlock(self: &BlockExplodeEventFacade, index: u64) -> UniquePtr<Block>;
        fn isCancelled(self: &BlockExplodeEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut BlockExplodeEventFacade>, cancelled: bool) -> bool;

        fn getActor(self: &ActorExplodeEventFacade) -> UniquePtr<Actor>;
        fn getLocation(self: &ActorExplodeEventFacade) -> LocationData;
        fn getBlockCount(self: &ActorExplodeEventFacade) -> u64;
        fn getBlock(self: &ActorExplodeEventFacade, index: u64) -> UniquePtr<Block>;
        fn isCancelled(self: &ActorExplodeEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut ActorExplodeEventFacade>, cancelled: bool) -> bool;

        fn getActor(self: &ActorKnockbackEventFacade) -> UniquePtr<Actor>;
        fn getSource(self: &ActorKnockbackEventFacade) -> UniquePtr<Actor>;
        fn getKnockback(self: &ActorKnockbackEventFacade) -> VectorData;
        fn setKnockback(self: Pin<&mut ActorKnockbackEventFacade>, knockback: &VectorData) -> bool;
        fn isCancelled(self: &ActorKnockbackEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut ActorKnockbackEventFacade>, cancelled: bool) -> bool;

        fn getActor(self: &ActorRemoveEventFacade) -> UniquePtr<Actor>;

        fn getPluginNameForRust(self: &PluginLifecycleEventFacade) -> String;

        fn getLoadType(self: &ServerLoadEventFacade) -> u8;

        fn getChunkX(self: &ChunkEventFacade) -> i32;
        fn getChunkZ(self: &ChunkEventFacade) -> i32;
        fn getDimensionForRust(self: &ChunkEventFacade) -> String;

        fn getActor(self: &ActorSpawnEventFacade) -> UniquePtr<Actor>;
        fn isCancelled(self: &ActorSpawnEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut ActorSpawnEventFacade>, cancelled: bool) -> bool;

        fn getActor(self: &ActorTeleportEventFacade) -> UniquePtr<Actor>;
        fn getFrom(self: &ActorTeleportEventFacade) -> LocationData;
        fn setFrom(self: Pin<&mut ActorTeleportEventFacade>, location: &LocationData) -> bool;
        fn getTo(self: &ActorTeleportEventFacade) -> LocationData;
        fn setTo(self: Pin<&mut ActorTeleportEventFacade>, location: &LocationData) -> bool;
        fn isCancelled(self: &ActorTeleportEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut ActorTeleportEventFacade>, cancelled: bool) -> bool;

        fn getMessageForRust(self: &BroadcastMessageEventFacade) -> String;
        fn isCancelled(self: &BroadcastMessageEventFacade) -> bool;
        fn setMessageForRust(self: Pin<&mut BroadcastMessageEventFacade>, message: &str) -> bool;
        fn setCancelled(self: Pin<&mut BroadcastMessageEventFacade>, cancelled: bool) -> bool;

        fn getPacketId(self: &PacketSendEventFacade) -> i32;
        fn getPayloadForRust(self: &PacketSendEventFacade) -> Vec<u8>;
        fn setPayloadForRust(self: Pin<&mut PacketSendEventFacade>, payload: &[u8]) -> bool;
        fn getPlayer(self: &PacketSendEventFacade) -> UniquePtr<Player>;
        fn getAddress(self: &PacketSendEventFacade) -> SocketAddress;
        fn getSubClientId(self: &PacketSendEventFacade) -> u8;
        fn isCancelled(self: &PacketSendEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut PacketSendEventFacade>, cancelled: bool) -> bool;

        fn getPacketId(self: &PacketReceiveEventFacade) -> i32;
        fn getPayloadForRust(self: &PacketReceiveEventFacade) -> Vec<u8>;
        fn setPayloadForRust(self: Pin<&mut PacketReceiveEventFacade>, payload: &[u8]) -> bool;
        fn getPlayer(self: &PacketReceiveEventFacade) -> UniquePtr<Player>;
        fn getAddress(self: &PacketReceiveEventFacade) -> SocketAddress;
        fn getSubClientId(self: &PacketReceiveEventFacade) -> u8;
        fn isCancelled(self: &PacketReceiveEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut PacketReceiveEventFacade>, cancelled: bool) -> bool;

        fn getMapIdForRust(self: &MapInitializeEventFacade) -> i64;

        fn getPlayer(self: &PlayerCommandEventFacade) -> UniquePtr<Player>;
        fn getCommandForRust(self: &PlayerCommandEventFacade) -> String;
        fn isCancelled(self: &PlayerCommandEventFacade) -> bool;
        fn setCommandForRust(self: Pin<&mut PlayerCommandEventFacade>, command: &str) -> bool;
        fn setCancelled(self: Pin<&mut PlayerCommandEventFacade>, cancelled: bool) -> bool;

        fn getPlayer(self: &PlayerKickEventFacade) -> UniquePtr<Player>;
        fn getReasonForRust(self: &PlayerKickEventFacade) -> String;
        fn isCancelled(self: &PlayerKickEventFacade) -> bool;
        fn setReasonForRust(self: Pin<&mut PlayerKickEventFacade>, reason: &str) -> bool;
        fn setCancelled(self: Pin<&mut PlayerKickEventFacade>, cancelled: bool) -> bool;

        fn getPlayer(self: &PlayerLoginEventFacade) -> UniquePtr<Player>;
        fn getKickMessageForRust(self: &PlayerLoginEventFacade) -> String;
        fn isCancelled(self: &PlayerLoginEventFacade) -> bool;
        fn setKickMessageForRust(self: Pin<&mut PlayerLoginEventFacade>, message: &str) -> bool;
        fn setCancelled(self: Pin<&mut PlayerLoginEventFacade>, cancelled: bool) -> bool;

        fn getPlayer(self: &PlayerGameModeChangeEventFacade) -> UniquePtr<Player>;
        fn getNewGameMode(self: &PlayerGameModeChangeEventFacade) -> u8;
        fn isCancelled(self: &PlayerGameModeChangeEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut PlayerGameModeChangeEventFacade>, cancelled: bool) -> bool;

        fn getPlayer(self: &PlayerEmoteEventFacade) -> UniquePtr<Player>;
        fn getEmoteIdForRust(self: &PlayerEmoteEventFacade) -> String;
        fn isMuted(self: &PlayerEmoteEventFacade) -> bool;
        fn setMuted(self: Pin<&mut PlayerEmoteEventFacade>, muted: bool) -> bool;
        fn isCancelled(self: &PlayerEmoteEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut PlayerEmoteEventFacade>, cancelled: bool) -> bool;

        fn getPlayer(self: &PlayerSkinChangeEventFacade) -> UniquePtr<Player>;
        fn hasSkinChangeMessage(self: &PlayerSkinChangeEventFacade) -> bool;
        fn getSkinChangeMessageForRust(self: &PlayerSkinChangeEventFacade) -> String;
        fn setSkinChangeMessageForRust(
            self: Pin<&mut PlayerSkinChangeEventFacade>,
            has_message: bool,
            message: &str,
        ) -> bool;
        fn isCancelled(self: &PlayerSkinChangeEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut PlayerSkinChangeEventFacade>, cancelled: bool) -> bool;

        fn getPlayer(self: &PlayerDeathEventFacade) -> UniquePtr<Player>;
        fn hasDeathMessage(self: &PlayerDeathEventFacade) -> bool;
        fn getDeathMessageForRust(self: &PlayerDeathEventFacade) -> String;
        fn setDeathMessageForRust(
            self: Pin<&mut PlayerDeathEventFacade>,
            has_message: bool,
            message: &str,
        ) -> bool;

        fn getPlayer(self: &PlayerDimensionChangeEventFacade) -> UniquePtr<Player>;
        fn getFromForRust(self: &PlayerDimensionChangeEventFacade) -> String;
        fn getToForRust(self: &PlayerDimensionChangeEventFacade) -> String;

        fn getPlayer(self: &PlayerBedEnterEventFacade) -> UniquePtr<Player>;
        fn getBed(self: &PlayerBedEnterEventFacade) -> UniquePtr<Block>;
        fn isCancelled(self: &PlayerBedEnterEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut PlayerBedEnterEventFacade>, cancelled: bool) -> bool;

        fn getPlayer(self: &PlayerBedLeaveEventFacade) -> UniquePtr<Player>;
        fn getBed(self: &PlayerBedLeaveEventFacade) -> UniquePtr<Block>;

        fn getPlayer(self: &PlayerRespawnEventFacade) -> UniquePtr<Player>;

        fn getPlayer(self: &PlayerItemHeldEventFacade) -> UniquePtr<Player>;
        fn getPreviousSlot(self: &PlayerItemHeldEventFacade) -> i32;
        fn getNewSlot(self: &PlayerItemHeldEventFacade) -> i32;
        fn isCancelled(self: &PlayerItemHeldEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut PlayerItemHeldEventFacade>, cancelled: bool) -> bool;

        fn getPlayer(self: &PlayerMoveEventFacade) -> UniquePtr<Player>;
        fn getFrom(self: &PlayerMoveEventFacade) -> LocationData;
        fn setFrom(self: Pin<&mut PlayerMoveEventFacade>, location: &LocationData) -> bool;
        fn getTo(self: &PlayerMoveEventFacade) -> LocationData;
        fn setTo(self: Pin<&mut PlayerMoveEventFacade>, location: &LocationData) -> bool;
        fn isCancelled(self: &PlayerMoveEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut PlayerMoveEventFacade>, cancelled: bool) -> bool;

        fn getPlayer(self: &PlayerDropItemEventFacade) -> UniquePtr<Player>;
        fn getItem(self: &PlayerDropItemEventFacade) -> UniquePtr<ItemStackRef>;
        fn isCancelled(self: &PlayerDropItemEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut PlayerDropItemEventFacade>, cancelled: bool) -> bool;

        fn getPlayer(self: &BlockBreakEventFacade) -> UniquePtr<Player>;
        fn getBlock(self: &BlockBreakEventFacade) -> UniquePtr<Block>;
        fn isCancelled(self: &BlockBreakEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut BlockBreakEventFacade>, cancelled: bool) -> bool;

        fn getBlock(self: &BlockCookEventFacade) -> UniquePtr<Block>;
        fn getSource(self: &BlockCookEventFacade) -> UniquePtr<ItemStackRef>;
        fn getResult(self: &BlockCookEventFacade) -> UniquePtr<ItemStackRef>;
        fn setResult(self: Pin<&mut BlockCookEventFacade>, result: &ItemStack) -> bool;
        fn isCancelled(self: &BlockCookEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut BlockCookEventFacade>, cancelled: bool) -> bool;

        fn getBlock(self: &LeavesDecayEventFacade) -> UniquePtr<Block>;
        fn isCancelled(self: &LeavesDecayEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut LeavesDecayEventFacade>, cancelled: bool) -> bool;

        fn getBlock(self: &BlockFromToEventFacade) -> UniquePtr<Block>;
        fn getToBlock(self: &BlockFromToEventFacade) -> UniquePtr<Block>;
        fn isCancelled(self: &BlockFromToEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut BlockFromToEventFacade>, cancelled: bool) -> bool;

        fn getBlock(self: &BlockGrowEventFacade) -> UniquePtr<Block>;
        fn isCancelled(self: &BlockGrowEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut BlockGrowEventFacade>, cancelled: bool) -> bool;

        fn getBlock(self: &BlockPistonEventFacade) -> UniquePtr<Block>;
        fn getDirection(self: &BlockPistonEventFacade) -> u8;
        fn isCancelled(self: &BlockPistonEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut BlockPistonEventFacade>, cancelled: bool) -> bool;

        fn getPlayer(self: &BlockPlaceEventFacade) -> UniquePtr<Player>;
        fn getBlockReplaced(self: &BlockPlaceEventFacade) -> UniquePtr<Block>;
        fn getBlockAgainst(self: &BlockPlaceEventFacade) -> UniquePtr<Block>;
        fn isCancelled(self: &BlockPlaceEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut BlockPlaceEventFacade>, cancelled: bool) -> bool;

        fn getPlayer(self: &PlayerInteractEventFacade) -> UniquePtr<Player>;
        fn getAction(self: &PlayerInteractEventFacade) -> u8;
        fn getItem(self: &PlayerInteractEventFacade) -> UniquePtr<ItemStackRef>;
        fn getBlock(self: &PlayerInteractEventFacade) -> UniquePtr<Block>;
        fn getBlockFace(self: &PlayerInteractEventFacade) -> u8;
        fn hasClickedPosition(self: &PlayerInteractEventFacade) -> bool;
        fn getClickedPosition(self: &PlayerInteractEventFacade) -> VectorData;
        fn isCancelled(self: &PlayerInteractEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut PlayerInteractEventFacade>, cancelled: bool) -> bool;

        fn getPlayer(self: &PlayerInteractActorEventFacade) -> UniquePtr<Player>;
        fn getActor(self: &PlayerInteractActorEventFacade) -> UniquePtr<Actor>;
        fn isCancelled(self: &PlayerInteractActorEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut PlayerInteractActorEventFacade>, cancelled: bool) -> bool;

        fn getPlayer(self: &PlayerItemConsumeEventFacade) -> UniquePtr<Player>;
        fn getItem(self: &PlayerItemConsumeEventFacade) -> UniquePtr<ItemStackRef>;
        fn getHand(self: &PlayerItemConsumeEventFacade) -> u8;
        fn isCancelled(self: &PlayerItemConsumeEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut PlayerItemConsumeEventFacade>, cancelled: bool) -> bool;

        fn getPlayer(self: &PlayerPickupItemEventFacade) -> UniquePtr<Player>;
        fn getItemActor(self: &PlayerPickupItemEventFacade) -> UniquePtr<Actor>;
        fn isCancelled(self: &PlayerPickupItemEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut PlayerPickupItemEventFacade>, cancelled: bool) -> bool;

        fn getSenderNameForRust(self: &ServerCommandEventFacade) -> String;
        fn getCommandForRust(self: &ServerCommandEventFacade) -> String;
        fn isCancelled(self: &ServerCommandEventFacade) -> bool;
        fn setCommandForRust(self: Pin<&mut ServerCommandEventFacade>, command: &str) -> bool;
        fn setCancelled(self: Pin<&mut ServerCommandEventFacade>, cancelled: bool) -> bool;

        fn getMotdForRust(self: &ServerListPingEventFacade) -> String;
        fn setMotdForRust(self: Pin<&mut ServerListPingEventFacade>, motd: &str) -> bool;
        fn getServerGuidForRust(self: &ServerListPingEventFacade) -> String;
        fn setServerGuidForRust(self: Pin<&mut ServerListPingEventFacade>, guid: &str) -> bool;
        fn getLocalPort(self: &ServerListPingEventFacade) -> i32;
        fn setLocalPort(self: Pin<&mut ServerListPingEventFacade>, port: i32) -> bool;
        fn getLocalPortV6(self: &ServerListPingEventFacade) -> i32;
        fn setLocalPortV6(self: Pin<&mut ServerListPingEventFacade>, port: i32) -> bool;
        fn isCancelled(self: &ServerListPingEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut ServerListPingEventFacade>, cancelled: bool) -> bool;

        fn getToWeather(self: &WeatherChangeEventFacade) -> bool;
        fn isCancelled(self: &WeatherChangeEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut WeatherChangeEventFacade>, cancelled: bool) -> bool;

        fn getToThunder(self: &ThunderChangeEventFacade) -> bool;
        fn isCancelled(self: &ThunderChangeEventFacade) -> bool;
        fn setCancelled(self: Pin<&mut ThunderChangeEventFacade>, cancelled: bool) -> bool;

    }

    #[namespace = "aegilex::native"]
    unsafe extern "C++" {
        include!("host_context.h");

        type HostContext = crate::cxx_host::ffi::HostContext;

        fn form_show(
            self: &HostContext,
            plugin_id: &CxxString,
            uuid: &[u8],
            spec: &FormSpecData,
            out_form_id: Pin<&mut u64>,
        ) -> u32;
        fn form_close(self: &HostContext, uuid: &[u8]) -> u32;
        fn list_plugins(self: &HostContext) -> PluginList;
        fn get_plugin(
            self: &HostContext,
            plugin_id: &CxxString,
            out: &mut PluginInfoData,
        ) -> u32;
        fn enable_plugin(self: &HostContext, plugin_id: &CxxString) -> u32;
        fn disable_plugin(self: &HostContext, plugin_id: &CxxString) -> u32;
        fn service_publish(
            self: &HostContext,
            plugin_id: &CxxString,
            name: &str,
            version: &str,
            methods: &Vec<String>,
            priority: u32,
        ) -> u64;
        fn service_unpublish(self: &HostContext, plugin_id: &CxxString, provider_id: u64) -> u32;
        fn service_list(self: &HostContext, name: &str) -> ServiceListData;
        fn service_call(
            self: &HostContext,
            plugin_id: &CxxString,
            provider_id: u64,
            method: &str,
            payload: &[u8],
            deadline: u64,
        ) -> u64;
        fn service_call_status(self: &HostContext, call_id: u64) -> ServiceCallStatusData;
        fn service_take_response(self: &HostContext, call_id: u64) -> ServiceResponseData;
        fn service_cancel(self: &HostContext, call_id: u64) -> u32;
        fn map_renderer_register(
            self: &HostContext,
            plugin_id: &CxxString,
            map_id: i64,
            contextual: bool,
            out_renderer_id: Pin<&mut u64>,
        ) -> u32;
        fn map_renderer_unregister(
            self: &HostContext,
            plugin_id: &CxxString,
            renderer_id: u64,
        ) -> u32;
    }

    extern "Rust" {
        type RuntimeHandle;

        fn default_runtime_config() -> RuntimeConfig;
        fn create_runtime(
            host_context: SharedPtr<HostContext>,
            config: RuntimeConfig,
        ) -> Result<Box<RuntimeHandle>>;
        fn inspect_plugin(
            runtime: &mut RuntimeHandle,
            component_path: &CxxString,
        ) -> RuntimeInspectResult;
        fn prepare_plugin(runtime: &mut RuntimeHandle, component_path: &CxxString) -> u32;
        fn enable_plugin(runtime: &mut RuntimeHandle, plugin_id: &CxxString) -> u32;
        fn disable_plugin(runtime: &mut RuntimeHandle, plugin_id: &CxxString) -> u32;
        fn should_dispatch_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            subscription: &CxxString,
        ) -> bool;
        fn discard_invocation_handles(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
        );
        fn dispatch_command(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            sender: UniquePtr<CommandSender>,
            command: &CommandData,
        ) -> CommandOutcome;
        fn dispatch_task(runtime: &mut RuntimeHandle, plugin_id: &CxxString, task_id: u64) -> u32;
        fn dispatch_player_join_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<PlayerJoinEventFacade>,
        ) -> bool;
        fn dispatch_player_quit_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<PlayerQuitEventFacade>,
        ) -> bool;
        fn dispatch_player_chat_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<PlayerChatEventFacade>,
        ) -> bool;
        fn dispatch_actor_damage_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<ActorDamageEventFacade>,
        ) -> bool;
        fn dispatch_actor_death_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<ActorDeathEventFacade>,
        ) -> bool;
        fn dispatch_actor_remove_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<ActorRemoveEventFacade>,
        ) -> bool;
        fn dispatch_actor_spawn_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<ActorSpawnEventFacade>,
        ) -> bool;
        fn dispatch_actor_teleport_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<ActorTeleportEventFacade>,
        ) -> bool;
        fn dispatch_actor_knockback_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<ActorKnockbackEventFacade>,
        ) -> bool;
        fn dispatch_broadcast_message_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<BroadcastMessageEventFacade>,
        ) -> bool;
        fn dispatch_packet_send_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<PacketSendEventFacade>,
        ) -> bool;
        fn dispatch_packet_receive_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<PacketReceiveEventFacade>,
        ) -> bool;
        fn dispatch_map_initialize_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<MapInitializeEventFacade>,
        ) -> bool;
        fn dispatch_script_message_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<ScriptMessageEventFacade>,
        ) -> bool;
        fn dispatch_player_kick_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<PlayerKickEventFacade>,
        ) -> bool;
        fn dispatch_player_login_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<PlayerLoginEventFacade>,
        ) -> bool;
        fn dispatch_player_drop_item_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<PlayerDropItemEventFacade>,
        ) -> bool;
        fn dispatch_block_break_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<BlockBreakEventFacade>,
        ) -> bool;
        fn dispatch_block_cook_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<BlockCookEventFacade>,
        ) -> bool;
        fn dispatch_block_place_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<BlockPlaceEventFacade>,
        ) -> bool;
        fn dispatch_player_item_consume_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<PlayerItemConsumeEventFacade>,
        ) -> bool;
        fn dispatch_player_pickup_item_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<PlayerPickupItemEventFacade>,
        ) -> bool;
        fn dispatch_player_command_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<PlayerCommandEventFacade>,
        ) -> bool;
        fn dispatch_server_command_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<ServerCommandEventFacade>,
        ) -> bool;
        fn dispatch_server_list_ping_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<ServerListPingEventFacade>,
        ) -> bool;
        fn dispatch_weather_change_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<WeatherChangeEventFacade>,
        ) -> bool;
        fn dispatch_thunder_change_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<ThunderChangeEventFacade>,
        ) -> bool;
        fn dispatch_player_game_mode_change_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<PlayerGameModeChangeEventFacade>,
        ) -> bool;
        fn dispatch_player_dimension_change_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<PlayerDimensionChangeEventFacade>,
        ) -> bool;
        fn dispatch_player_respawn_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<PlayerRespawnEventFacade>,
        ) -> bool;
        fn dispatch_player_item_held_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<PlayerItemHeldEventFacade>,
        ) -> bool;
        fn dispatch_player_move_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            subscription: &CxxString,
            event: UniquePtr<PlayerMoveEventFacade>,
        ) -> bool;
        fn dispatch_player_bed_enter_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<PlayerBedEnterEventFacade>,
        ) -> bool;
        fn dispatch_player_bed_leave_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<PlayerBedLeaveEventFacade>,
        ) -> bool;
        fn dispatch_player_emote_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<PlayerEmoteEventFacade>,
        ) -> bool;
        fn dispatch_player_skin_change_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<PlayerSkinChangeEventFacade>,
        ) -> bool;
        fn dispatch_player_interact_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<PlayerInteractEventFacade>,
        ) -> bool;
        fn dispatch_player_interact_actor_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<PlayerInteractActorEventFacade>,
        ) -> bool;
        fn dispatch_player_death_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<PlayerDeathEventFacade>,
        ) -> bool;
        fn dispatch_leaves_decay_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<LeavesDecayEventFacade>,
        ) -> bool;
        fn dispatch_block_explode_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<BlockExplodeEventFacade>,
        ) -> bool;
        fn dispatch_block_from_to_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<BlockFromToEventFacade>,
        ) -> bool;
        fn dispatch_block_grow_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            subscription: &CxxString,
            event: UniquePtr<BlockGrowEventFacade>,
        ) -> bool;
        fn dispatch_block_piston_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            subscription: &CxxString,
            event: UniquePtr<BlockPistonEventFacade>,
        ) -> bool;
        fn dispatch_chunk_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            subscription: &CxxString,
            event: UniquePtr<ChunkEventFacade>,
        ) -> bool;
        fn dispatch_actor_explode_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<ActorExplodeEventFacade>,
        ) -> bool;
        fn dispatch_plugin_enable_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<PluginLifecycleEventFacade>,
        ) -> bool;
        fn dispatch_plugin_disable_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<PluginLifecycleEventFacade>,
        ) -> bool;
        fn dispatch_server_load_event(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            invocation_id: u64,
            event: UniquePtr<ServerLoadEventFacade>,
        ) -> bool;
        fn dispatch_form_submit(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            form_id: u64,
            has_player: bool,
            player_uuid: &[u8],
            response: &FormResponseData,
        ) -> bool;

        fn dispatch_form_close(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            form_id: u64,
            has_player: bool,
            player_uuid: &[u8],
        ) -> bool;
        fn plugin_manager_list(runtime: &mut RuntimeHandle) -> PluginList;
        fn plugin_manager_get(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
        ) -> PluginInfoData;
        fn plugin_manager_enable(runtime: &mut RuntimeHandle, plugin_id: &CxxString) -> u32;
        fn plugin_manager_disable(runtime: &mut RuntimeHandle, plugin_id: &CxxString) -> u32;
        fn service_publish(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            name: &str,
            version: &str,
            methods: &Vec<String>,
            priority: u32,
        ) -> u64;
        fn service_unpublish(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            provider_id: u64,
        ) -> u32;
        fn service_list(runtime: &mut RuntimeHandle, name: &str) -> ServiceListData;
        fn service_call(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            provider_id: u64,
            method: &str,
            payload: &[u8],
            deadline: u64,
        ) -> u64;
        fn service_call_status(runtime: &mut RuntimeHandle, call_id: u64) -> ServiceCallStatusData;
        fn service_take_response(runtime: &mut RuntimeHandle, call_id: u64) -> ServiceResponseData;
        fn service_cancel(runtime: &mut RuntimeHandle, call_id: u64) -> u32;
        fn dispatch_map_render(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            renderer_id: u64,
            map_id: i64,
            has_player: bool,
            player_uuid: &[u8],
        ) -> Vec<MapDrawCommandData>;
        fn dispatch_map_initialize(
            runtime: &mut RuntimeHandle,
            plugin_id: &CxxString,
            renderer_id: u64,
            map_id: i64,
        ) -> bool;
    }
}

pub fn default_runtime_config() -> ffi::RuntimeConfig {
    ffi::RuntimeConfig {
        max_module_bytes: 8 * 1024 * 1024,
        max_nested_dispatch_depth: crate::config::DEFAULT_MAX_NESTED_DISPATCH_DEPTH,
        max_nbt_depth: 0,
        max_nbt_nodes: 0,
        max_nbt_compound_entries: 0,
        max_nbt_string_bytes: 0,
        max_nbt_array_bytes: 0,
        max_invocation_native_resources: 0,
        max_plugin_resource_slots: 0,
    }
}

pub fn create_runtime(
    host_context: cxx::SharedPtr<crate::cxx_host::ffi::HostContext>,
    config: ffi::RuntimeConfig,
) -> Result<Box<RuntimeHandle>, Box<dyn std::error::Error>> {
    let host_context = RuntimeHostContext::new(host_context)
        .map_err(|error| format!("invalid host context: {}", error.status()))?;
    let runtime = Runtime::new(
        host_context,
        CoreRuntimeConfig {
            max_module_bytes: config.max_module_bytes,
            max_nested_dispatch_depth: config.max_nested_dispatch_depth,
            max_nbt_depth: config.max_nbt_depth,
            max_nbt_nodes: config.max_nbt_nodes,
            max_nbt_compound_entries: config.max_nbt_compound_entries,
            max_nbt_string_bytes: config.max_nbt_string_bytes,
            max_nbt_array_bytes: config.max_nbt_array_bytes,
            max_invocation_native_resources: config.max_invocation_native_resources,
            max_plugin_resource_slots: config.max_plugin_resource_slots,
        },
    )
    .map_err(|status| format!("runtime initialization failed with status {status}"))?;
    Ok(Box::new(RuntimeHandle { runtime }))
}

pub fn inspect_plugin(
    runtime: &mut RuntimeHandle,
    module_path: &CxxString,
) -> ffi::RuntimeInspectResult {
    let Some(module_path) = cxx_string(module_path) else {
        return inspect_result(
            AEGILEX_INVALID_ARGUMENT,
            "module path must be non-empty UTF-8".to_owned(),
            empty_metadata(),
        );
    };
    match runtime.runtime.inspect_plugin(Path::new(module_path)) {
        Ok(inspection) => inspect_result(AEGILEX_OK, String::new(), inspection.metadata),
        Err(error) => inspect_result(AEGILEX_INVALID_ARGUMENT, error, empty_metadata()),
    }
}

pub fn prepare_plugin(runtime: &mut RuntimeHandle, module_path: &CxxString) -> u32 {
    let Some(module_path) = cxx_string(module_path) else {
        return AEGILEX_INVALID_ARGUMENT;
    };
    runtime.runtime.prepare_plugin(Path::new(module_path))
}

pub fn enable_plugin(runtime: &mut RuntimeHandle, plugin_id: &CxxString) -> u32 {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return AEGILEX_INVALID_ARGUMENT;
    };
    runtime.runtime.enable_plugin(plugin_id)
}

pub fn disable_plugin(runtime: &mut RuntimeHandle, plugin_id: &CxxString) -> u32 {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return AEGILEX_INVALID_ARGUMENT;
    };
    runtime.runtime.disable_plugin(plugin_id)
}

pub fn should_dispatch_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    subscription: &CxxString,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let Some(subscription) = cxx_string(subscription) else {
        return false;
    };
    runtime
        .runtime
        .should_dispatch_event(plugin_id, subscription)
}

pub fn discard_invocation_handles(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
) {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return;
    };
    runtime
        .runtime
        .discard_invocation_handles(plugin_id, invocation_id);
}
pub fn dispatch_command(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    sender: cxx::UniquePtr<crate::cxx_host_common::ffi::CommandSender>,
    command: &ffi::CommandData,
) -> ffi::CommandOutcome {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return ffi::CommandOutcome {
            handled: false,
            reply: String::new(),
            error: String::new(),
        };
    };
    let Ok(sender) = runtime
        .runtime
        .register_command_sender(plugin_id, invocation_id, sender)
    else {
        return ffi::CommandOutcome {
            handled: false,
            reply: String::new(),
            error: String::new(),
        };
    };
    let invocation = crate::core_host::CommandsInvocation {
        subcommand: command.subcommand.clone(),
        args: command.args.clone(),
        sender,
    };
    match runtime
        .runtime
        .dispatch_wit_command(plugin_id, invocation_id, invocation)
    {
        Ok(outcome) => ffi::CommandOutcome {
            handled: outcome.handled,
            reply: outcome.reply.unwrap_or_default(),
            error: outcome.error.unwrap_or_default(),
        },
        Err(_) => ffi::CommandOutcome {
            handled: false,
            reply: String::new(),
            error: String::new(),
        },
    }
}

pub fn dispatch_task(runtime: &mut RuntimeHandle, plugin_id: &CxxString, task_id: u64) -> u32 {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return AEGILEX_INVALID_ARGUMENT;
    };
    let invocation_id = runtime.runtime.next_invocation_id();
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        crate::reentry::nested_dispatch_task(caller, invocation_id, task_id)
    }) {
        return match result {
            Ok(()) => AEGILEX_OK,
            Err(status) => status,
        };
    }
    match runtime.runtime.dispatch_task(plugin_id, task_id) {
        Ok(()) => AEGILEX_OK,
        Err(status) => status,
    }
}

pub fn dispatch_form_submit(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    form_id: u64,
    has_player: bool,
    player_uuid: &[u8],
    response: &ffi::FormResponseData,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let Ok(form_id) = u32::try_from(form_id) else {
        return false;
    };
    let response = ffi::FormResponseData {
        kind: response.kind,
        has_selected_index: response.has_selected_index,
        selected_index: response.selected_index,
        message_button: response.message_button,
        modal_json: response.modal_json.clone(),
    };
    let mut response = Some(response);
    let invocation_id = runtime.runtime.next_invocation_id();
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        crate::reentry::nested_dispatch_form_submit(
            caller,
            invocation_id,
            form_id,
            has_player,
            player_uuid,
            response.take().expect("nested form route runs once"),
        )
    }) {
        return result;
    }
    runtime.runtime.dispatch_form_submit(
        plugin_id,
        u64::from(form_id),
        has_player,
        player_uuid,
        response.expect("form response not consumed by nested route"),
    )
}

pub fn dispatch_form_close(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    form_id: u64,
    has_player: bool,
    player_uuid: &[u8],
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let Ok(form_id) = u32::try_from(form_id) else {
        return false;
    };
    let invocation_id = runtime.runtime.next_invocation_id();
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        crate::reentry::nested_dispatch_form_close(
            caller,
            invocation_id,
            form_id,
            has_player,
            player_uuid,
        )
    }) {
        return result;
    }
    runtime.runtime.dispatch_form_close(
        plugin_id,
        u64::from(form_id),
        has_player,
        player_uuid,
    )
}

fn plugin_info_to_cxx(metadata: &PluginMetadata, enabled: bool) -> ffi::PluginInfoData {
    ffi::PluginInfoData {
        metadata: metadata_to_cxx(metadata.clone()),
        enabled,
    }
}

pub fn plugin_manager_list(runtime: &mut RuntimeHandle) -> ffi::PluginList {
    let metadata = runtime.runtime.plugin_summaries();
    let mut plugins = Vec::with_capacity(metadata.len());
    for plugin in &metadata {
        let enabled = runtime.runtime.is_plugin_enabled(&plugin.name);
        plugins.push(plugin_info_to_cxx(plugin, enabled));
    }
    ffi::PluginList {
        status: AEGILEX_OK,
        plugins,
    }
}

pub fn plugin_manager_get(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
) -> ffi::PluginInfoData {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return empty_plugin_info();
    };
    let enabled = runtime.runtime.is_plugin_enabled(plugin_id);
    runtime
        .runtime
        .plugin_summary(plugin_id)
        .map(|metadata| plugin_info_to_cxx(&metadata, enabled))
        .unwrap_or_else(empty_plugin_info)
}

fn empty_plugin_info() -> ffi::PluginInfoData {
    ffi::PluginInfoData {
        metadata: metadata_to_cxx(empty_metadata()),
        enabled: false,
    }
}

pub fn plugin_manager_enable(runtime: &mut RuntimeHandle, plugin_id: &CxxString) -> u32 {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return AEGILEX_INVALID_ARGUMENT;
    };
    runtime.runtime.enable_plugin(plugin_id)
}

pub fn plugin_manager_disable(runtime: &mut RuntimeHandle, plugin_id: &CxxString) -> u32 {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return AEGILEX_INVALID_ARGUMENT;
    };
    runtime.runtime.disable_plugin(plugin_id)
}

pub fn service_publish(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    name: &str,
    version: &str,
    methods: &[String],
    priority: u32,
) -> u64 {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return 0;
    };
    runtime
        .runtime
        .service_publish(plugin_id, name, version, methods.to_vec(), priority)
        .ok()
        .unwrap_or(0)
}

pub fn service_unpublish(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    provider_id: u64,
) -> u32 {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return AEGILEX_INVALID_ARGUMENT;
    };
    runtime.runtime.service_unpublish(provider_id, plugin_id)
}

pub fn service_list(runtime: &mut RuntimeHandle, name: &str) -> ffi::ServiceListData {
    let providers = runtime.runtime.service_list(name);
    let mut rows = Vec::with_capacity(providers.len());
    for (id, name, version, methods, priority) in providers {
        rows.push(ffi::ServiceProviderData {
            id,
            name,
            version,
            methods,
            priority,
        });
    }
    ffi::ServiceListData {
        status: AEGILEX_OK,
        providers: rows,
    }
}

pub fn service_call(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    provider_id: u64,
    method: &str,
    payload: &[u8],
    deadline: u64,
) -> u64 {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return 0;
    };
    runtime
        .runtime
        .service_call(plugin_id, provider_id, method, payload.to_vec(), deadline)
        .ok()
        .unwrap_or(0)
}

pub fn service_call_status(
    runtime: &mut RuntimeHandle,
    call_id: u64,
) -> ffi::ServiceCallStatusData {
    match runtime.runtime.service_call_status(call_id) {
        Ok(call_status) => ffi::ServiceCallStatusData {
            status: AEGILEX_OK,
            call_status,
        },
        Err(status) => ffi::ServiceCallStatusData {
            status,
            call_status: crate::runtime::SERVICE_STATUS_PENDING,
        },
    }
}

pub fn service_take_response(
    runtime: &mut RuntimeHandle,
    call_id: u64,
) -> ffi::ServiceResponseData {
    match runtime.runtime.service_take_response(call_id) {
        Ok((kind, payload, error)) => ffi::ServiceResponseData {
            status: AEGILEX_OK,
            kind,
            payload,
            error,
        },
        Err(status) => ffi::ServiceResponseData {
            status,
            kind: crate::runtime::SERVICE_STATUS_PENDING,
            payload: Vec::new(),
            error: String::new(),
        },
    }
}

pub fn service_cancel(runtime: &mut RuntimeHandle, call_id: u64) -> u32 {
    runtime.runtime.service_cancel(call_id)
}

fn cursor_to_cxx(cursor: &crate::core_host::MapCursorMapCursor) -> ffi::MapCursorData {
    ffi::MapCursorData {
        x: cursor.x,
        y: cursor.y,
        direction: cursor.direction,
        cursor_type: match cursor.cursor_type {
            crate::core_host::MapCursorMapCursorType::Player => 0,
            crate::core_host::MapCursorMapCursorType::Frame => 1,
            crate::core_host::MapCursorMapCursorType::RedMarker => 2,
            crate::core_host::MapCursorMapCursorType::BlueMarker => 3,
            crate::core_host::MapCursorMapCursorType::TargetX => 4,
            crate::core_host::MapCursorMapCursorType::TargetPoint => 5,
            crate::core_host::MapCursorMapCursorType::PlayerOffMap => 6,
            crate::core_host::MapCursorMapCursorType::SignMarker => 7,
            crate::core_host::MapCursorMapCursorType::PinkMarker => 8,
            crate::core_host::MapCursorMapCursorType::OrangeMarker => 9,
            crate::core_host::MapCursorMapCursorType::YellowMarker => 10,
            crate::core_host::MapCursorMapCursorType::CyanMarker => 11,
            crate::core_host::MapCursorMapCursorType::GreenPoint => 12,
            crate::core_host::MapCursorMapCursorType::PlayerOffLimits => 13,
            crate::core_host::MapCursorMapCursorType::Mansion => 14,
            crate::core_host::MapCursorMapCursorType::Monument => 15,
            crate::core_host::MapCursorMapCursorType::VillageDesert => 16,
            crate::core_host::MapCursorMapCursorType::VillagePlains => 17,
            crate::core_host::MapCursorMapCursorType::VillageSavanna => 18,
            crate::core_host::MapCursorMapCursorType::VillageSnowy => 19,
            crate::core_host::MapCursorMapCursorType::VillageTaiga => 20,
            crate::core_host::MapCursorMapCursorType::JungleTemple => 21,
            crate::core_host::MapCursorMapCursorType::SwampHut => 22,
            crate::core_host::MapCursorMapCursorType::TrialChambers => 23,
        },
        visible: cursor.visible,
        caption: cursor.caption.clone(),
    }
}

fn map_draw_command_to_cxx(
    command: crate::core_host::MapCanvasMapDrawCommand,
) -> ffi::MapDrawCommandData {
    match command {
        crate::core_host::MapCanvasMapDrawCommand::SetPixel(pixel) => ffi::MapDrawCommandData {
            kind: 0,
            x: pixel.x,
            y: pixel.y,
            width: 0,
            height: 0,
            argb: pixel.argb,
            pixels: Vec::new(),
            cursors: Vec::new(),
        },
        crate::core_host::MapCanvasMapDrawCommand::FillRect(rect) => ffi::MapDrawCommandData {
            kind: 1,
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
            argb: rect.argb,
            pixels: Vec::new(),
            cursors: Vec::new(),
        },
        crate::core_host::MapCanvasMapDrawCommand::DrawImage(image) => ffi::MapDrawCommandData {
            kind: 2,
            x: image.x,
            y: image.y,
            width: image.width,
            height: image.height,
            argb: 0,
            pixels: image.pixels,
            cursors: Vec::new(),
        },
        crate::core_host::MapCanvasMapDrawCommand::SetCursors(cursors) => ffi::MapDrawCommandData {
            kind: 3,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            argb: 0,
            pixels: Vec::new(),
            cursors: cursors.iter().map(cursor_to_cxx).collect(),
        },
    }
}

pub fn dispatch_map_render(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    renderer_id: u64,
    map_id: i64,
    has_player: bool,
    player_uuid: &[u8],
) -> Vec<ffi::MapDrawCommandData> {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return Vec::new();
    };
    let Ok(renderer) = u32::try_from(renderer_id) else {
        return Vec::new();
    };
    let invocation_id = runtime.runtime.next_invocation_id();
    let commands = crate::reentry::route_nested(plugin_id, |caller| {
        crate::reentry::nested_dispatch_map_render(
            caller,
            invocation_id,
            renderer,
            map_id,
            has_player,
            player_uuid,
        )
    })
    .unwrap_or_else(|| {
        runtime
            .runtime
            .dispatch_map_render(plugin_id, renderer_id, map_id, has_player, player_uuid)
    });
    commands.into_iter().map(map_draw_command_to_cxx).collect()
}

pub fn dispatch_map_initialize(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    renderer_id: u64,
    map_id: i64,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let Ok(renderer) = u32::try_from(renderer_id) else {
        return false;
    };
    let invocation_id = runtime.runtime.next_invocation_id();
    crate::reentry::route_nested(plugin_id, |caller| {
        crate::reentry::nested_dispatch_map_initialize(caller, invocation_id, renderer, map_id)
    })
    .unwrap_or_else(|| {
        runtime
            .runtime
            .dispatch_map_initialize(plugin_id, renderer_id, map_id)
    })
}

fn cxx_string(value: &CxxString) -> Option<&str> {
    match value.to_str() {
        Ok(value) if !value.is_empty() => Some(value),
        _ => None,
    }
}

fn inspect_result(
    status: u32,
    error: String,
    metadata: PluginMetadata,
) -> ffi::RuntimeInspectResult {
    ffi::RuntimeInspectResult {
        status,
        error,
        metadata: metadata_to_cxx(metadata),
    }
}

fn empty_metadata() -> PluginMetadata {
    PluginMetadata {
        name: String::new(),
        version: String::new(),
        description: String::new(),
        load_order: PluginLoadOrder::PostWorld,
        authors: Vec::new(),
        contributors: Vec::new(),
        website: String::new(),
        prefix: String::new(),
        provides: Vec::new(),
        depend: Vec::new(),
        soft_depend: Vec::new(),
        load_before: Vec::new(),
        default_permission: 2,
        commands: Vec::new(),
        permissions: Vec::new(),
        subscriptions: Vec::new(),
    }
}

fn metadata_to_cxx(metadata: PluginMetadata) -> ffi::RuntimePluginMetadata {
    ffi::RuntimePluginMetadata {
        name: metadata.name,
        version: metadata.version,
        description: metadata.description,
        load_order: match metadata.load_order {
            PluginLoadOrder::Startup => 0,
            PluginLoadOrder::PostWorld => 1,
        },
        authors: metadata.authors,
        contributors: metadata.contributors,
        website: metadata.website,
        prefix: metadata.prefix,
        provides: metadata.provides,
        depend: metadata.depend,
        soft_depend: metadata.soft_depend,
        load_before: metadata.load_before,
        default_permission: metadata.default_permission,
        commands: metadata
            .commands
            .into_iter()
            .map(|command| ffi::RuntimeCommandSpec {
                name: command.name,
                description: command.description.unwrap_or_default(),
                aliases: command.aliases,
                usages: command.usages,
                permissions: command.permissions,
            })
            .collect(),
        permissions: metadata
            .permissions
            .into_iter()
            .map(|permission| ffi::RuntimePermissionSpec {
                name: permission.name,
                description: permission.description.unwrap_or_default(),
                has_default_value: permission.default_value.is_some(),
                default_value: permission.default_value.unwrap_or(0),
                children: permission
                    .children
                    .into_iter()
                    .map(|child| ffi::RuntimePermissionChild {
                        name: child.name,
                        value: child.value,
                    })
                    .collect(),
            })
            .collect(),
        subscriptions: metadata.subscriptions,
    }
}

pub fn dispatch_player_join_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<PlayerJoinEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "player-join",
            |state| state.insert_player_join_event_resource(event),
            crate::core_host::EventsEvent::PlayerJoin,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_player_join_event(plugin_id, invocation_id, event)
}

pub fn dispatch_player_quit_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<PlayerQuitEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "player-quit",
            |state| state.insert_player_quit_event_resource(event),
            crate::core_host::EventsEvent::PlayerQuit,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_player_quit_event(plugin_id, invocation_id, event)
}

pub fn dispatch_player_chat_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<PlayerChatEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "player-chat",
            |state| state.insert_player_chat_event_resource(event),
            crate::core_host::EventsEvent::PlayerChat,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_player_chat_event(plugin_id, invocation_id, event)
}

pub fn dispatch_script_message_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<ScriptMessageEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "script-message",
            |state| state.insert_script_message_event_resource(event),
            crate::core_host::EventsEvent::ScriptMessage,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_script_message_event(plugin_id, invocation_id, event)
}

pub fn dispatch_player_kick_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<PlayerKickEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "player-kick",
            |state| state.insert_player_kick_event_resource(event),
            crate::core_host::EventsEvent::PlayerKick,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_player_kick_event(plugin_id, invocation_id, event)
}

pub fn dispatch_player_login_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<PlayerLoginEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "player-login",
            |state| state.insert_player_login_event_resource(event),
            crate::core_host::EventsEvent::PlayerLogin,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_player_login_event(plugin_id, invocation_id, event)
}

pub fn dispatch_player_drop_item_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<PlayerDropItemEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "player-drop-item",
            |state| state.insert_player_drop_item_event_resource(event),
            crate::core_host::EventsEvent::PlayerDropItem,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_player_drop_item_event(plugin_id, invocation_id, event)
}

pub fn dispatch_block_break_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<BlockBreakEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "block-break",
            |state| state.insert_block_break_event_resource(event),
            crate::core_host::EventsEvent::BlockBreak,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_block_break_event(plugin_id, invocation_id, event)
}

pub fn dispatch_block_cook_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<BlockCookEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "block-cook",
            |state| state.insert_block_cook_event_resource(event),
            crate::core_host::EventsEvent::BlockCook,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_block_cook_event(plugin_id, invocation_id, event)
}

pub fn dispatch_block_place_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<BlockPlaceEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "block-place",
            |state| state.insert_block_place_event_resource(event),
            crate::core_host::EventsEvent::BlockPlace,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_block_place_event(plugin_id, invocation_id, event)
}

pub fn dispatch_player_item_consume_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<PlayerItemConsumeEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "player-item-consume",
            |state| state.insert_player_item_consume_event_resource(event),
            crate::core_host::EventsEvent::PlayerItemConsume,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_player_item_consume_event(plugin_id, invocation_id, event)
}

pub fn dispatch_player_pickup_item_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<PlayerPickupItemEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "player-pickup-item",
            |state| state.insert_player_pickup_item_event_resource(event),
            crate::core_host::EventsEvent::PlayerPickupItem,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_player_pickup_item_event(plugin_id, invocation_id, event)
}

pub fn dispatch_player_command_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<PlayerCommandEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "player-command",
            |state| state.insert_player_command_event_resource(event),
            crate::core_host::EventsEvent::PlayerCommand,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_player_command_event(plugin_id, invocation_id, event)
}

pub fn dispatch_player_game_mode_change_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<PlayerGameModeChangeEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "player-game-mode-change",
            |state| state.insert_player_game_mode_change_event_resource(event),
            crate::core_host::EventsEvent::PlayerGameModeChange,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_player_game_mode_change_event(plugin_id, invocation_id, event)
}

pub fn dispatch_player_dimension_change_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<PlayerDimensionChangeEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "player-dimension-change",
            |state| state.insert_player_dimension_change_event_resource(event),
            crate::core_host::EventsEvent::PlayerDimensionChange,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_player_dimension_change_event(plugin_id, invocation_id, event)
}

pub fn dispatch_player_respawn_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<PlayerRespawnEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "player-respawn",
            |state| state.insert_player_respawn_event_resource(event),
            crate::core_host::EventsEvent::PlayerRespawn,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_player_respawn_event(plugin_id, invocation_id, event)
}

pub fn dispatch_player_item_held_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<PlayerItemHeldEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "player-item-held",
            |state| state.insert_player_item_held_event_resource(event),
            crate::core_host::EventsEvent::PlayerItemHeld,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_player_item_held_event(plugin_id, invocation_id, event)
}

pub fn dispatch_player_move_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    subscription: &CxxString,
    event: cxx::UniquePtr<PlayerMoveEventFacade>,
) -> bool {
    let (Some(plugin_id), Some(subscription)) = (cxx_string(plugin_id), cxx_string(subscription))
    else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            subscription,
            |state| state.insert_player_move_event_resource(event),
            |event| match subscription {
                "player-move" => crate::core_host::EventsEvent::PlayerMove(event),
                "player-teleport" => crate::core_host::EventsEvent::PlayerTeleport(event),
                "player-jump" => crate::core_host::EventsEvent::PlayerJump(event),
                "player-portal" => crate::core_host::EventsEvent::PlayerPortal(event),
                _ => crate::core_host::EventsEvent::PlayerMove(event),
            },
        )
    }) {
        return result;
    }
    runtime.runtime.dispatch_player_move_event(
        plugin_id,
        invocation_id,
        subscription,
        event.expect("event not consumed by nested route"),
    )
}

pub fn dispatch_player_bed_enter_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<PlayerBedEnterEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "player-bed-enter",
            |state| state.insert_player_bed_enter_event_resource(event),
            crate::core_host::EventsEvent::PlayerBedEnter,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_player_bed_enter_event(plugin_id, invocation_id, event)
}

pub fn dispatch_player_bed_leave_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<PlayerBedLeaveEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "player-bed-leave",
            |state| state.insert_player_bed_leave_event_resource(event),
            crate::core_host::EventsEvent::PlayerBedLeave,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_player_bed_leave_event(plugin_id, invocation_id, event)
}

pub fn dispatch_player_emote_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<PlayerEmoteEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "player-emote",
            |state| state.insert_player_emote_event_resource(event),
            crate::core_host::EventsEvent::PlayerEmote,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_player_emote_event(plugin_id, invocation_id, event)
}

pub fn dispatch_player_interact_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<ffi::PlayerInteractEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "player-interact",
            |state| state.insert_player_interact_event_resource(event),
            crate::core_host::EventsEvent::PlayerInteract,
        )
    }) {
        return result;
    }
    runtime.runtime.dispatch_player_interact_event(
        plugin_id,
        invocation_id,
        event.expect("event not consumed by nested route"),
    )
}

pub fn dispatch_player_interact_actor_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<ffi::PlayerInteractActorEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "player-interact-actor",
            |state| state.insert_player_interact_actor_event_resource(event),
            crate::core_host::EventsEvent::PlayerInteractActor,
        )
    }) {
        return result;
    }
    runtime.runtime.dispatch_player_interact_actor_event(
        plugin_id,
        invocation_id,
        event.expect("event not consumed by nested route"),
    )
}

pub fn dispatch_player_skin_change_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<PlayerSkinChangeEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "player-skin-change",
            |state| state.insert_player_skin_change_event_resource(event),
            crate::core_host::EventsEvent::PlayerSkinChange,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_player_skin_change_event(plugin_id, invocation_id, event)
}

pub fn dispatch_player_death_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<PlayerDeathEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "player-death",
            |state| state.insert_player_death_event_resource(event),
            crate::core_host::EventsEvent::PlayerDeath,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_player_death_event(plugin_id, invocation_id, event)
}

pub fn dispatch_leaves_decay_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<LeavesDecayEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "leaves-decay",
            |state| state.insert_leaves_decay_event_resource(event),
            crate::core_host::EventsEvent::LeavesDecay,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_leaves_decay_event(plugin_id, invocation_id, event)
}

pub fn dispatch_block_explode_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<ffi::BlockExplodeEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "block-explode",
            |state| state.insert_block_explode_event_resource(event),
            crate::core_host::EventsEvent::BlockExplode,
        )
    }) {
        return result;
    }
    runtime.runtime.dispatch_block_explode_event(
        plugin_id,
        invocation_id,
        event.expect("event not consumed by nested route"),
    )
}

pub fn dispatch_block_from_to_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<BlockFromToEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "block-from-to",
            |state| state.insert_block_from_to_event_resource(event),
            crate::core_host::EventsEvent::BlockFromTo,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_block_from_to_event(plugin_id, invocation_id, event)
}

pub fn dispatch_block_grow_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    subscription: &CxxString,
    event: cxx::UniquePtr<BlockGrowEventFacade>,
) -> bool {
    let (Some(plugin_id), Some(subscription)) = (cxx_string(plugin_id), cxx_string(subscription))
    else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            subscription,
            |state| state.insert_block_grow_event_resource(event),
            |event| match subscription {
                "block-form" => crate::core_host::EventsEvent::BlockForm(event),
                _ => crate::core_host::EventsEvent::BlockGrow(event),
            },
        )
    }) {
        return result;
    }
    runtime.runtime.dispatch_block_grow_event(
        plugin_id,
        invocation_id,
        subscription,
        event.expect("event not consumed by nested route"),
    )
}

pub fn dispatch_block_piston_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    subscription: &CxxString,
    event: cxx::UniquePtr<BlockPistonEventFacade>,
) -> bool {
    let (Some(plugin_id), Some(subscription)) = (cxx_string(plugin_id), cxx_string(subscription))
    else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            subscription,
            |state| state.insert_block_piston_event_resource(event),
            |event| match subscription {
                "block-piston-extend" => crate::core_host::EventsEvent::BlockPistonExtend(event),
                _ => crate::core_host::EventsEvent::BlockPistonRetract(event),
            },
        )
    }) {
        return result;
    }
    runtime.runtime.dispatch_block_piston_event(
        plugin_id,
        invocation_id,
        subscription,
        event.expect("event not consumed by nested route"),
    )
}

pub fn dispatch_chunk_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    subscription: &CxxString,
    event: cxx::UniquePtr<ChunkEventFacade>,
) -> bool {
    let (Some(plugin_id), Some(subscription)) = (cxx_string(plugin_id), cxx_string(subscription))
    else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            subscription,
            |state| state.insert_chunk_event_resource(event),
            |event| match subscription {
                "chunk-load" => crate::core_host::EventsEvent::ChunkLoad(event),
                _ => crate::core_host::EventsEvent::ChunkUnload(event),
            },
        )
    }) {
        return result;
    }
    runtime.runtime.dispatch_chunk_event(
        plugin_id,
        invocation_id,
        subscription,
        event.expect("event not consumed by nested route"),
    )
}

pub fn dispatch_weather_change_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<WeatherChangeEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "weather-change",
            |state| state.insert_weather_change_event_resource(event),
            crate::core_host::EventsEvent::WeatherChange,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_weather_change_event(plugin_id, invocation_id, event)
}

pub fn dispatch_thunder_change_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<ThunderChangeEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "thunder-change",
            |state| state.insert_thunder_change_event_resource(event),
            crate::core_host::EventsEvent::ThunderChange,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_thunder_change_event(plugin_id, invocation_id, event)
}

pub fn dispatch_actor_damage_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<ActorDamageEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "actor-damage",
            |state| state.insert_actor_damage_event_resource(event),
            crate::core_host::EventsEvent::ActorDamage,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_actor_damage_event(plugin_id, invocation_id, event)
}

pub fn dispatch_actor_death_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<ActorDeathEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "actor-death",
            |state| state.insert_actor_death_event_resource(event),
            crate::core_host::EventsEvent::ActorDeath,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_actor_death_event(plugin_id, invocation_id, event)
}

pub fn dispatch_actor_remove_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<ActorRemoveEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "actor-remove",
            |state| state.insert_actor_remove_event_resource(event),
            crate::core_host::EventsEvent::ActorRemove,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_actor_remove_event(plugin_id, invocation_id, event)
}

pub fn dispatch_actor_spawn_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<ActorSpawnEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "actor-spawn",
            |state| state.insert_actor_spawn_event_resource(event),
            crate::core_host::EventsEvent::ActorSpawn,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_actor_spawn_event(plugin_id, invocation_id, event)
}

pub fn dispatch_actor_teleport_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<ActorTeleportEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "actor-teleport",
            |state| state.insert_actor_teleport_event_resource(event),
            crate::core_host::EventsEvent::ActorTeleport,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_actor_teleport_event(plugin_id, invocation_id, event)
}

pub fn dispatch_actor_explode_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<ActorExplodeEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "actor-explode",
            |state| state.insert_actor_explode_event_resource(event),
            crate::core_host::EventsEvent::ActorExplode,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_actor_explode_event(plugin_id, invocation_id, event)
}

pub fn dispatch_actor_knockback_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<ActorKnockbackEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "actor-knockback",
            |state| state.insert_actor_knockback_event_resource(event),
            crate::core_host::EventsEvent::ActorKnockback,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_actor_knockback_event(plugin_id, invocation_id, event)
}

pub fn dispatch_server_command_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<ServerCommandEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "server-command",
            |state| state.insert_server_command_event_resource(event),
            crate::core_host::EventsEvent::ServerCommand,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_server_command_event(plugin_id, invocation_id, event)
}

pub fn dispatch_broadcast_message_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<BroadcastMessageEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "broadcast-message",
            |state| state.insert_broadcast_message_event_resource(event),
            crate::core_host::EventsEvent::BroadcastMessage,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_broadcast_message_event(plugin_id, invocation_id, event)
}

pub fn dispatch_packet_send_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<PacketSendEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "packet-send",
            |state| state.insert_packet_send_event_resource(event),
            crate::core_host::EventsEvent::PacketSend,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_packet_send_event(plugin_id, invocation_id, event)
}

pub fn dispatch_packet_receive_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<PacketReceiveEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "packet-receive",
            |state| state.insert_packet_receive_event_resource(event),
            crate::core_host::EventsEvent::PacketReceive,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_packet_receive_event(plugin_id, invocation_id, event)
}

pub fn dispatch_map_initialize_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<MapInitializeEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "map-initialize",
            |state| state.insert_map_initialize_event_resource(event),
            crate::core_host::EventsEvent::MapInitialize,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_map_initialize_event(plugin_id, invocation_id, event)
}

pub fn dispatch_server_list_ping_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<ServerListPingEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "server-list-ping",
            |state| state.insert_server_list_ping_event_resource(event),
            crate::core_host::EventsEvent::ServerListPing,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_server_list_ping_event(plugin_id, invocation_id, event)
}

pub fn dispatch_plugin_enable_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<PluginLifecycleEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "plugin-enable",
            |state| state.insert_plugin_lifecycle_event_resource(event),
            crate::core_host::EventsEvent::PluginEnable,
        )
    }) {
        return result;
    }
    runtime.runtime.dispatch_plugin_enable_event(
        plugin_id,
        invocation_id,
        event.expect("event not consumed by nested route"),
    )
}

pub fn dispatch_plugin_disable_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<PluginLifecycleEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "plugin-disable",
            |state| state.insert_plugin_lifecycle_event_resource(event),
            crate::core_host::EventsEvent::PluginDisable,
        )
    }) {
        return result;
    }
    runtime.runtime.dispatch_plugin_disable_event(
        plugin_id,
        invocation_id,
        event.expect("event not consumed by nested route"),
    )
}

pub fn dispatch_server_load_event(
    runtime: &mut RuntimeHandle,
    plugin_id: &CxxString,
    invocation_id: u64,
    event: cxx::UniquePtr<ServerLoadEventFacade>,
) -> bool {
    let Some(plugin_id) = cxx_string(plugin_id) else {
        return false;
    };
    let mut event = Some(event);
    if let Some(result) = crate::reentry::route_nested(plugin_id, |caller| {
        let event = event.take().expect("nested route runs once");
        crate::reentry::nested_dispatch_event(
            caller,
            invocation_id,
            "server-load",
            |state| state.insert_server_load_event_resource(event),
            crate::core_host::EventsEvent::ServerLoad,
        )
    }) {
        return result;
    }
    let event = event.expect("event not consumed by nested route");
    runtime
        .runtime
        .dispatch_server_load_event(plugin_id, invocation_id, event)
}
