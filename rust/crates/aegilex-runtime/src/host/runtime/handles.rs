use std::collections::HashMap;

use cxx::UniquePtr;

use crate::cxx_host::ffi::Logger;
use crate::cxx_host_actor::ffi::{Actor, ItemActor, Mob};
use crate::cxx_host_admin::ffi::{BanList, PermissionAttachment, PermissionDefinition};
use crate::cxx_host_common::ffi::{CommandSender, Permissible};
use crate::cxx_host_inventory::ffi::{Inventory, ItemStack, ItemStackRef, PlayerInventory};
use crate::cxx_host_level::ffi::Block;
use crate::cxx_host_player::ffi::Player;
use crate::cxx_host_server::ffi::PluginCommand;
use crate::cxx_host_ui::ffi::{BossBar, ScoreEntry, Scoreboard};
use crate::cxx_runtime::ffi::{
    ActorDamageEventFacade, ActorDeathEventFacade, ActorExplodeEventFacade,
    ActorKnockbackEventFacade, ActorRemoveEventFacade, ActorSpawnEventFacade,
    ActorTeleportEventFacade, BlockBreakEventFacade, BlockCookEventFacade, BlockExplodeEventFacade,
    BlockFromToEventFacade, BlockGrowEventFacade, BlockPistonEventFacade, BlockPlaceEventFacade,
    BroadcastMessageEventFacade, ChunkEventFacade, LeavesDecayEventFacade,
    MapInitializeEventFacade, PacketReceiveEventFacade, PacketSendEventFacade,
    PlayerBedEnterEventFacade, PlayerBedLeaveEventFacade, PlayerChatEventFacade,
    PlayerCommandEventFacade, PlayerDeathEventFacade, PlayerDimensionChangeEventFacade,
    PlayerDropItemEventFacade, PlayerEmoteEventFacade, PlayerGameModeChangeEventFacade,
    PlayerInteractActorEventFacade, PlayerInteractEventFacade, PlayerItemConsumeEventFacade,
    PlayerItemHeldEventFacade, PlayerJoinEventFacade, PlayerKickEventFacade,
    PlayerLoginEventFacade, PlayerMoveEventFacade, PlayerPickupItemEventFacade,
    PlayerQuitEventFacade, PlayerRespawnEventFacade, PlayerSkinChangeEventFacade,
    PluginLifecycleEventFacade, ScriptMessageEventFacade, ServerCommandEventFacade,
    ServerListPingEventFacade, ServerLoadEventFacade, ThunderChangeEventFacade,
    WeatherChangeEventFacade,
};

enum GuestHandle {
    Actor(UniquePtr<Actor>),
    BanList(UniquePtr<BanList>),
    Block(UniquePtr<Block>),
    BossBar(UniquePtr<BossBar>),
    CommandSender(UniquePtr<CommandSender>),
    Inventory(UniquePtr<Inventory>),
    ItemStack(UniquePtr<ItemStack>),
    ItemStackRef(UniquePtr<ItemStackRef>),
    ItemActor(UniquePtr<ItemActor>),
    Logger(UniquePtr<Logger>),
    Mob(UniquePtr<Mob>),
    Player(UniquePtr<Player>),
    ActorDamageEvent(UniquePtr<ActorDamageEventFacade>),
    ActorDeathEvent(UniquePtr<ActorDeathEventFacade>),
    ActorExplodeEvent(UniquePtr<ActorExplodeEventFacade>),
    BlockExplodeEvent(UniquePtr<BlockExplodeEventFacade>),
    ActorKnockbackEvent(UniquePtr<ActorKnockbackEventFacade>),
    ActorRemoveEvent(UniquePtr<ActorRemoveEventFacade>),
    ActorSpawnEvent(UniquePtr<ActorSpawnEventFacade>),
    ActorTeleportEvent(UniquePtr<ActorTeleportEventFacade>),
    BlockBreakEvent(UniquePtr<BlockBreakEventFacade>),
    BlockCookEvent(UniquePtr<BlockCookEventFacade>),
    BlockFromToEvent(UniquePtr<BlockFromToEventFacade>),
    BlockGrowEvent(UniquePtr<BlockGrowEventFacade>),
    BlockPistonEvent(UniquePtr<BlockPistonEventFacade>),
    BlockPlaceEvent(UniquePtr<BlockPlaceEventFacade>),
    LeavesDecayEvent(UniquePtr<LeavesDecayEventFacade>),
    BroadcastMessageEvent(UniquePtr<BroadcastMessageEventFacade>),
    PlayerBedEnterEvent(UniquePtr<PlayerBedEnterEventFacade>),
    PlayerBedLeaveEvent(UniquePtr<PlayerBedLeaveEventFacade>),
    PlayerChatEvent(UniquePtr<PlayerChatEventFacade>),
    PlayerJoinEvent(UniquePtr<PlayerJoinEventFacade>),
    PlayerQuitEvent(UniquePtr<PlayerQuitEventFacade>),
    PlayerDeathEvent(UniquePtr<PlayerDeathEventFacade>),
    PlayerCommandEvent(UniquePtr<PlayerCommandEventFacade>),
    PlayerKickEvent(UniquePtr<PlayerKickEventFacade>),
    PlayerLoginEvent(UniquePtr<PlayerLoginEventFacade>),
    PlayerGameModeChangeEvent(UniquePtr<PlayerGameModeChangeEventFacade>),
    PlayerEmoteEvent(UniquePtr<PlayerEmoteEventFacade>),
    PlayerSkinChangeEvent(UniquePtr<PlayerSkinChangeEventFacade>),
    PlayerDimensionChangeEvent(UniquePtr<PlayerDimensionChangeEventFacade>),
    PlayerRespawnEvent(UniquePtr<PlayerRespawnEventFacade>),
    PlayerItemHeldEvent(UniquePtr<PlayerItemHeldEventFacade>),
    PlayerMoveEvent(UniquePtr<PlayerMoveEventFacade>),
    PlayerDropItemEvent(UniquePtr<PlayerDropItemEventFacade>),
    PlayerInteractEvent(UniquePtr<PlayerInteractEventFacade>),
    PlayerInteractActorEvent(UniquePtr<PlayerInteractActorEventFacade>),
    PlayerItemConsumeEvent(UniquePtr<PlayerItemConsumeEventFacade>),
    PlayerPickupItemEvent(UniquePtr<PlayerPickupItemEventFacade>),
    ServerCommandEvent(UniquePtr<ServerCommandEventFacade>),
    ServerListPingEvent(UniquePtr<ServerListPingEventFacade>),
    PluginLifecycleEvent(UniquePtr<PluginLifecycleEventFacade>),
    ServerLoadEvent(UniquePtr<ServerLoadEventFacade>),
    ChunkEvent(UniquePtr<ChunkEventFacade>),
    WeatherChangeEvent(UniquePtr<WeatherChangeEventFacade>),
    ThunderChangeEvent(UniquePtr<ThunderChangeEventFacade>),
    PacketSendEvent(UniquePtr<PacketSendEventFacade>),
    PacketReceiveEvent(UniquePtr<PacketReceiveEventFacade>),
    MapInitializeEvent(UniquePtr<MapInitializeEventFacade>),
    ScriptMessageEvent(UniquePtr<ScriptMessageEventFacade>),
    PlayerInventory(UniquePtr<PlayerInventory>),
    PluginCommand(UniquePtr<PluginCommand>),
    PermissionAttachment(UniquePtr<PermissionAttachment>),
    PermissionDefinition(UniquePtr<PermissionDefinition>),
    Permissible(UniquePtr<Permissible>),
    Scoreboard(UniquePtr<Scoreboard>),
    ScoreEntry(UniquePtr<ScoreEntry>),
}

struct Entry {
    invocation_id: u64,
    lifetime: ResourceLifetime,
    value: GuestHandle,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum ResourceKind {
    Actor,
    Block,
    CommandSender,
    ItemStack,
    ItemStackRef,
    ItemActor,
    Logger,
    Mob,
    Player,
    PluginCommand,
    Inventory,
    PlayerInventory,
    BossBar,
    Scoreboard,
    ScoreEntry,
    BanList,
    PermissionAttachment,
    PermissionDefinition,
    Permissible,
    ActorDamageEvent,
    ActorDeathEvent,
    ActorExplodeEvent,
    BlockExplodeEvent,
    ActorKnockbackEvent,
    ActorRemoveEvent,
    ActorSpawnEvent,
    ActorTeleportEvent,
    BlockBreakEvent,
    BlockCookEvent,
    BlockFromToEvent,
    BlockGrowEvent,
    BlockPistonEvent,
    BlockPlaceEvent,
    LeavesDecayEvent,
    BroadcastMessageEvent,
    PlayerBedEnterEvent,
    PlayerBedLeaveEvent,
    PlayerChatEvent,
    PlayerJoinEvent,
    PlayerQuitEvent,
    PlayerDeathEvent,
    PlayerCommandEvent,
    PlayerKickEvent,
    PlayerLoginEvent,
    PlayerGameModeChangeEvent,
    PlayerEmoteEvent,
    PlayerSkinChangeEvent,
    PlayerDimensionChangeEvent,
    PlayerRespawnEvent,
    PlayerItemHeldEvent,
    PlayerMoveEvent,
    PlayerDropItemEvent,
    PlayerInteractEvent,
    PlayerInteractActorEvent,
    PlayerItemConsumeEvent,
    PlayerPickupItemEvent,
    ServerCommandEvent,
    ServerListPingEvent,
    PluginLifecycleEvent,
    ServerLoadEvent,
    ChunkEvent,
    WeatherChangeEvent,
    ThunderChangeEvent,
    PacketSendEvent,
    PacketReceiveEvent,
    MapInitializeEvent,
    ScriptMessageEvent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ResourceLifetime {
    GuestOwned,
    HostBorrowed,
    PluginOwned,
}

/// Maps an unforgeable WIT resource representation to the invocation-scoped
/// native facade retained by `GuestHandles`.
pub(crate) struct ResourceSlot {
    pub(crate) invocation_id: u64,
    pub(crate) handle: u64,
    pub(crate) lifetime: ResourceLifetime,
}

/// Rust owns guest-visible resource ids. Native facades are only the values
/// stored behind those ids and never resolve or allocate guest handles.
pub(crate) struct GuestHandles {
    entries: HashMap<u64, Entry>,
    next_id: u64,
}

impl GuestHandles {
    pub(crate) fn new() -> Self {
        Self {
            entries: HashMap::new(),
            next_id: 1,
        }
    }

    pub(crate) fn clear_invocation(&mut self, invocation_id: u64) {
        self.entries.retain(|_, entry| {
            entry.lifetime == ResourceLifetime::PluginOwned || entry.invocation_id != invocation_id
        });
    }

    pub(crate) fn remove_handles(&mut self, handles: &[u64]) {
        for handle in handles {
            self.entries.remove(handle);
        }
    }

    pub(crate) fn insert_actor(&mut self, invocation_id: u64, value: UniquePtr<Actor>) -> u64 {
        self.insert(invocation_id, GuestHandle::Actor(value))
    }

    pub(crate) fn is_invocation_at_capacity(&self, invocation_id: u64, limit: usize) -> bool {
        self.entries
            .values()
            .filter(|entry| entry.invocation_id == invocation_id)
            .count()
            >= limit
    }

    pub(crate) fn insert_block(&mut self, invocation_id: u64, value: UniquePtr<Block>) -> u64 {
        self.insert(invocation_id, GuestHandle::Block(value))
    }

    pub(crate) fn insert_ban_list(&mut self, invocation_id: u64, value: UniquePtr<BanList>) -> u64 {
        self.insert(invocation_id, GuestHandle::BanList(value))
    }

    pub(crate) fn insert_boss_bar(&mut self, invocation_id: u64, value: UniquePtr<BossBar>) -> u64 {
        self.insert_plugin_owned(invocation_id, GuestHandle::BossBar(value))
    }

    pub(crate) fn insert_command_sender(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<CommandSender>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::CommandSender(value))
    }

    pub(crate) fn insert_inventory(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<Inventory>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::Inventory(value))
    }

    pub(crate) fn insert_item_stack(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<ItemStack>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::ItemStack(value))
    }

    pub(crate) fn insert_item_stack_ref(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<ItemStackRef>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::ItemStackRef(value))
    }

    pub(crate) fn insert_item_actor(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<ItemActor>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::ItemActor(value))
    }

    pub(crate) fn insert_logger(&mut self, invocation_id: u64, value: UniquePtr<Logger>) -> u64 {
        self.insert(invocation_id, GuestHandle::Logger(value))
    }

    pub(crate) fn insert_mob(&mut self, invocation_id: u64, value: UniquePtr<Mob>) -> u64 {
        self.insert(invocation_id, GuestHandle::Mob(value))
    }

    pub(crate) fn insert_player(&mut self, invocation_id: u64, value: UniquePtr<Player>) -> u64 {
        self.insert(invocation_id, GuestHandle::Player(value))
    }

    pub(crate) fn insert_actor_damage_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<ActorDamageEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::ActorDamageEvent(value))
    }

    pub(crate) fn insert_plugin_lifecycle_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PluginLifecycleEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::PluginLifecycleEvent(value))
    }

    pub(crate) fn insert_server_load_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<ServerLoadEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::ServerLoadEvent(value))
    }

    pub(crate) fn insert_chunk_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<ChunkEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::ChunkEvent(value))
    }

    pub(crate) fn insert_actor_death_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<ActorDeathEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::ActorDeathEvent(value))
    }

    pub(crate) fn insert_actor_explode_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<ActorExplodeEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::ActorExplodeEvent(value))
    }

    pub(crate) fn insert_block_explode_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<BlockExplodeEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::BlockExplodeEvent(value))
    }

    pub(crate) fn insert_actor_knockback_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<ActorKnockbackEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::ActorKnockbackEvent(value))
    }

    pub(crate) fn insert_actor_remove_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<ActorRemoveEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::ActorRemoveEvent(value))
    }

    pub(crate) fn insert_actor_spawn_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<ActorSpawnEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::ActorSpawnEvent(value))
    }

    pub(crate) fn insert_actor_teleport_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<ActorTeleportEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::ActorTeleportEvent(value))
    }

    pub(crate) fn insert_block_break_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<BlockBreakEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::BlockBreakEvent(value))
    }

    pub(crate) fn insert_block_cook_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<BlockCookEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::BlockCookEvent(value))
    }

    pub(crate) fn insert_block_from_to_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<BlockFromToEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::BlockFromToEvent(value))
    }

    pub(crate) fn insert_block_grow_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<BlockGrowEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::BlockGrowEvent(value))
    }

    pub(crate) fn insert_block_piston_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<BlockPistonEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::BlockPistonEvent(value))
    }

    pub(crate) fn insert_leaves_decay_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<LeavesDecayEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::LeavesDecayEvent(value))
    }

    pub(crate) fn insert_block_place_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<BlockPlaceEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::BlockPlaceEvent(value))
    }

    pub(crate) fn insert_player_chat_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PlayerChatEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::PlayerChatEvent(value))
    }

    pub(crate) fn insert_player_join_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PlayerJoinEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::PlayerJoinEvent(value))
    }

    pub(crate) fn insert_player_quit_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PlayerQuitEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::PlayerQuitEvent(value))
    }

    pub(crate) fn insert_player_bed_enter_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PlayerBedEnterEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::PlayerBedEnterEvent(value))
    }

    pub(crate) fn insert_player_bed_leave_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PlayerBedLeaveEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::PlayerBedLeaveEvent(value))
    }

    pub(crate) fn insert_player_death_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PlayerDeathEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::PlayerDeathEvent(value))
    }

    pub(crate) fn insert_broadcast_message_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<BroadcastMessageEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::BroadcastMessageEvent(value))
    }

    pub(crate) fn insert_packet_send_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PacketSendEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::PacketSendEvent(value))
    }

    pub(crate) fn insert_packet_receive_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PacketReceiveEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::PacketReceiveEvent(value))
    }

    pub(crate) fn insert_map_initialize_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<MapInitializeEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::MapInitializeEvent(value))
    }

    pub(crate) fn insert_script_message_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<ScriptMessageEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::ScriptMessageEvent(value))
    }

    pub(crate) fn insert_player_command_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PlayerCommandEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::PlayerCommandEvent(value))
    }

    pub(crate) fn insert_player_kick_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PlayerKickEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::PlayerKickEvent(value))
    }

    pub(crate) fn insert_player_login_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PlayerLoginEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::PlayerLoginEvent(value))
    }

    pub(crate) fn insert_player_game_mode_change_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PlayerGameModeChangeEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::PlayerGameModeChangeEvent(value))
    }

    pub(crate) fn insert_player_emote_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PlayerEmoteEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::PlayerEmoteEvent(value))
    }

    pub(crate) fn insert_player_skin_change_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PlayerSkinChangeEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::PlayerSkinChangeEvent(value))
    }

    pub(crate) fn insert_player_dimension_change_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PlayerDimensionChangeEventFacade>,
    ) -> u64 {
        self.insert(
            invocation_id,
            GuestHandle::PlayerDimensionChangeEvent(value),
        )
    }

    pub(crate) fn insert_player_respawn_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PlayerRespawnEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::PlayerRespawnEvent(value))
    }

    pub(crate) fn insert_player_item_held_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PlayerItemHeldEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::PlayerItemHeldEvent(value))
    }

    pub(crate) fn insert_player_move_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PlayerMoveEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::PlayerMoveEvent(value))
    }

    pub(crate) fn insert_player_pickup_item_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PlayerPickupItemEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::PlayerPickupItemEvent(value))
    }

    pub(crate) fn insert_player_drop_item_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PlayerDropItemEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::PlayerDropItemEvent(value))
    }

    pub(crate) fn insert_player_interact_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PlayerInteractEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::PlayerInteractEvent(value))
    }

    pub(crate) fn insert_player_interact_actor_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PlayerInteractActorEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::PlayerInteractActorEvent(value))
    }

    pub(crate) fn insert_player_item_consume_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PlayerItemConsumeEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::PlayerItemConsumeEvent(value))
    }

    pub(crate) fn insert_server_command_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<ServerCommandEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::ServerCommandEvent(value))
    }

    pub(crate) fn insert_server_list_ping_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<ServerListPingEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::ServerListPingEvent(value))
    }

    pub(crate) fn insert_weather_change_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<WeatherChangeEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::WeatherChangeEvent(value))
    }

    pub(crate) fn insert_thunder_change_event(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<ThunderChangeEventFacade>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::ThunderChangeEvent(value))
    }

    pub(crate) fn insert_player_inventory(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PlayerInventory>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::PlayerInventory(value))
    }

    pub(crate) fn insert_plugin_command(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PluginCommand>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::PluginCommand(value))
    }

    pub(crate) fn insert_permission_attachment(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PermissionAttachment>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::PermissionAttachment(value))
    }

    pub(crate) fn insert_permission_definition(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<PermissionDefinition>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::PermissionDefinition(value))
    }

    pub(crate) fn insert_permissible(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<Permissible>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::Permissible(value))
    }

    pub(crate) fn insert_scoreboard(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<Scoreboard>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::Scoreboard(value))
    }

    pub(crate) fn insert_score_entry(
        &mut self,
        invocation_id: u64,
        value: UniquePtr<ScoreEntry>,
    ) -> u64 {
        self.insert(invocation_id, GuestHandle::ScoreEntry(value))
    }

    pub(crate) fn actor(&self, invocation_id: u64, handle: u64) -> Option<&Actor> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::Actor(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn command_sender(&self, invocation_id: u64, handle: u64) -> Option<&CommandSender> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::CommandSender(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn block(&self, invocation_id: u64, handle: u64) -> Option<&Block> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::Block(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn ban_list(&self, invocation_id: u64, handle: u64) -> Option<&BanList> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::BanList(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn boss_bar(&self, invocation_id: u64, handle: u64) -> Option<&BossBar> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::BossBar(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn inventory(&self, invocation_id: u64, handle: u64) -> Option<&Inventory> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::Inventory(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn item_stack(&self, invocation_id: u64, handle: u64) -> Option<&ItemStack> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::ItemStack(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn item_stack_ref(&self, invocation_id: u64, handle: u64) -> Option<&ItemStackRef> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::ItemStackRef(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn item_actor(&self, invocation_id: u64, handle: u64) -> Option<&ItemActor> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::ItemActor(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn logger(&self, invocation_id: u64, handle: u64) -> Option<&Logger> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::Logger(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn mob(&self, invocation_id: u64, handle: u64) -> Option<&Mob> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::Mob(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn player(&self, invocation_id: u64, handle: u64) -> Option<&Player> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::Player(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn score_entry(&self, invocation_id: u64, handle: u64) -> Option<&ScoreEntry> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::ScoreEntry(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn player_chat_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&PlayerChatEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PlayerChatEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn player_join_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&PlayerJoinEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PlayerJoinEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn player_join_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut PlayerJoinEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::PlayerJoinEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn player_quit_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&PlayerQuitEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PlayerQuitEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn player_quit_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut PlayerQuitEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::PlayerQuitEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn player_death_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&PlayerDeathEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PlayerDeathEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn player_death_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut PlayerDeathEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::PlayerDeathEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn actor_damage_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&ActorDamageEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::ActorDamageEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn actor_damage_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut ActorDamageEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::ActorDamageEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn actor_death_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&ActorDeathEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::ActorDeathEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn actor_death_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut ActorDeathEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::ActorDeathEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn actor_explode_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&ActorExplodeEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::ActorExplodeEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn actor_explode_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut ActorExplodeEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::ActorExplodeEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn block_explode_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&BlockExplodeEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::BlockExplodeEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn block_explode_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut BlockExplodeEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::BlockExplodeEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn actor_knockback_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&ActorKnockbackEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::ActorKnockbackEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn actor_knockback_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut ActorKnockbackEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::ActorKnockbackEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn actor_remove_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&ActorRemoveEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::ActorRemoveEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn actor_remove_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut ActorRemoveEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::ActorRemoveEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn actor_spawn_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&ActorSpawnEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::ActorSpawnEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn actor_spawn_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut ActorSpawnEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::ActorSpawnEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn actor_teleport_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&ActorTeleportEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::ActorTeleportEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn actor_teleport_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut ActorTeleportEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::ActorTeleportEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn broadcast_message_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&BroadcastMessageEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::BroadcastMessageEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn broadcast_message_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut BroadcastMessageEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::BroadcastMessageEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn packet_send_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&PacketSendEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PacketSendEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn packet_send_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut PacketSendEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::PacketSendEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn packet_receive_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&PacketReceiveEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PacketReceiveEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn packet_receive_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut PacketReceiveEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::PacketReceiveEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn map_initialize_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&MapInitializeEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::MapInitializeEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn map_initialize_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut MapInitializeEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::MapInitializeEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn script_message_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&ScriptMessageEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::ScriptMessageEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn script_message_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut ScriptMessageEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::ScriptMessageEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn weather_change_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&WeatherChangeEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::WeatherChangeEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn weather_change_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut WeatherChangeEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::WeatherChangeEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn thunder_change_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&ThunderChangeEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::ThunderChangeEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn thunder_change_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut ThunderChangeEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::ThunderChangeEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn player_chat_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut PlayerChatEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::PlayerChatEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn player_move_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&PlayerMoveEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PlayerMoveEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn player_game_mode_change_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&PlayerGameModeChangeEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PlayerGameModeChangeEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn player_game_mode_change_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut PlayerGameModeChangeEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::PlayerGameModeChangeEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn player_emote_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&PlayerEmoteEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PlayerEmoteEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn player_emote_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut PlayerEmoteEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::PlayerEmoteEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn player_skin_change_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&PlayerSkinChangeEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PlayerSkinChangeEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn player_skin_change_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut PlayerSkinChangeEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::PlayerSkinChangeEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn player_dimension_change_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&PlayerDimensionChangeEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PlayerDimensionChangeEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn player_bed_enter_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&PlayerBedEnterEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PlayerBedEnterEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn player_bed_enter_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut PlayerBedEnterEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::PlayerBedEnterEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn player_bed_leave_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&PlayerBedLeaveEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PlayerBedLeaveEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn player_bed_leave_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut PlayerBedLeaveEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::PlayerBedLeaveEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn player_dimension_change_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut PlayerDimensionChangeEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::PlayerDimensionChangeEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn player_respawn_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&PlayerRespawnEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PlayerRespawnEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn player_respawn_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut PlayerRespawnEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::PlayerRespawnEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn player_item_held_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&PlayerItemHeldEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PlayerItemHeldEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn player_item_held_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut PlayerItemHeldEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::PlayerItemHeldEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn player_move_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut PlayerMoveEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::PlayerMoveEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn player_pickup_item_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&PlayerPickupItemEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PlayerPickupItemEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn player_drop_item_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&PlayerDropItemEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PlayerDropItemEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn player_drop_item_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut PlayerDropItemEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::PlayerDropItemEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn block_break_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&BlockBreakEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::BlockBreakEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn block_break_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut BlockBreakEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::BlockBreakEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn block_cook_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&BlockCookEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::BlockCookEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn block_cook_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut BlockCookEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::BlockCookEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn block_from_to_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&BlockFromToEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::BlockFromToEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn block_from_to_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut BlockFromToEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::BlockFromToEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn block_grow_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&BlockGrowEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::BlockGrowEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn block_grow_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut BlockGrowEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::BlockGrowEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn block_piston_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&BlockPistonEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::BlockPistonEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn block_piston_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut BlockPistonEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::BlockPistonEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn leaves_decay_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&LeavesDecayEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::LeavesDecayEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn leaves_decay_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut LeavesDecayEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::LeavesDecayEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn block_place_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&BlockPlaceEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::BlockPlaceEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn block_place_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut BlockPlaceEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::BlockPlaceEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn player_interact_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&PlayerInteractEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PlayerInteractEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn player_interact_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut PlayerInteractEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::PlayerInteractEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn player_interact_actor_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&PlayerInteractActorEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PlayerInteractActorEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn player_interact_actor_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut PlayerInteractActorEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::PlayerInteractActorEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn player_item_consume_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&PlayerItemConsumeEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PlayerItemConsumeEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn player_item_consume_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut PlayerItemConsumeEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::PlayerItemConsumeEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn player_pickup_item_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut PlayerPickupItemEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::PlayerPickupItemEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn player_kick_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&PlayerKickEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PlayerKickEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn player_command_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&PlayerCommandEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PlayerCommandEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn player_command_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut PlayerCommandEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::PlayerCommandEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn player_kick_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut PlayerKickEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::PlayerKickEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn player_login_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&PlayerLoginEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PlayerLoginEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn player_login_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut PlayerLoginEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::PlayerLoginEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn server_command_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&ServerCommandEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::ServerCommandEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn server_command_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut ServerCommandEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::ServerCommandEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn server_list_ping_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&ServerListPingEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::ServerListPingEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn server_list_ping_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut ServerListPingEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::ServerListPingEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn plugin_lifecycle_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&PluginLifecycleEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PluginLifecycleEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn plugin_lifecycle_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut PluginLifecycleEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::PluginLifecycleEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn server_load_event(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&ServerLoadEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::ServerLoadEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn server_load_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut ServerLoadEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::ServerLoadEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn chunk_event(&self, invocation_id: u64, handle: u64) -> Option<&ChunkEventFacade> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::ChunkEvent(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn chunk_event_mut(
        &mut self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<std::pin::Pin<&mut ChunkEventFacade>> {
        let entry = self.entries.get_mut(&handle)?;
        if entry.invocation_id != invocation_id {
            return None;
        }
        match entry.value {
            GuestHandle::ChunkEvent(ref mut value) => value.as_mut(),
            _ => None,
        }
    }

    pub(crate) fn player_inventory(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&PlayerInventory> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PlayerInventory(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn plugin_command(&self, invocation_id: u64, handle: u64) -> Option<&PluginCommand> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PluginCommand(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn permission_attachment(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&PermissionAttachment> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PermissionAttachment(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn permission_definition(
        &self,
        invocation_id: u64,
        handle: u64,
    ) -> Option<&PermissionDefinition> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::PermissionDefinition(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn permissible(&self, invocation_id: u64, handle: u64) -> Option<&Permissible> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::Permissible(ref value) => value.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn permission_attachment_aliases(
        &self,
        invocation_id: u64,
        attachment: &PermissionAttachment,
    ) -> Vec<u64> {
        self.entries
            .iter()
            .filter_map(|(&handle, entry)| {
                (entry.invocation_id == invocation_id
                    && matches!(&entry.value, GuestHandle::PermissionAttachment(value) if value.as_ref().is_some_and(|value| value.isSame(attachment))))
                .then_some(handle)
            })
            .collect()
    }

    pub(crate) fn permission_definitions_named(&self, invocation_id: u64, name: &str) -> Vec<u64> {
        self.entries
            .iter()
            .filter_map(|(&handle, entry)| {
                (entry.invocation_id == invocation_id
                    && matches!(&entry.value, GuestHandle::PermissionDefinition(value) if value.as_ref().is_some_and(|value| value.getName() == name)))
                .then_some(handle)
            })
            .collect()
    }

    pub(crate) fn scoreboard(&self, invocation_id: u64, handle: u64) -> Option<&Scoreboard> {
        match self.entry(invocation_id, handle)?.value {
            GuestHandle::Scoreboard(ref value) => value.as_ref(),
            _ => None,
        }
    }

    fn entry(&self, invocation_id: u64, handle: u64) -> Option<&Entry> {
        let entry = self.entries.get(&handle)?;
        (entry.lifetime == ResourceLifetime::PluginOwned || entry.invocation_id == invocation_id)
            .then_some(entry)
    }

    fn insert(&mut self, invocation_id: u64, value: GuestHandle) -> u64 {
        self.insert_with_lifetime(invocation_id, ResourceLifetime::GuestOwned, value)
    }

    fn insert_plugin_owned(&mut self, invocation_id: u64, value: GuestHandle) -> u64 {
        self.insert_with_lifetime(invocation_id, ResourceLifetime::PluginOwned, value)
    }

    fn insert_with_lifetime(
        &mut self,
        invocation_id: u64,
        lifetime: ResourceLifetime,
        value: GuestHandle,
    ) -> u64 {
        let is_null = match &value {
            GuestHandle::Actor(value) => value.is_null(),
            GuestHandle::BanList(value) => value.is_null(),
            GuestHandle::Block(value) => value.is_null(),
            GuestHandle::BossBar(value) => value.is_null(),
            GuestHandle::CommandSender(value) => value.is_null(),
            GuestHandle::Inventory(value) => value.is_null(),
            GuestHandle::ItemStack(value) => value.is_null(),
            GuestHandle::ItemStackRef(value) => value.is_null(),
            GuestHandle::ItemActor(value) => value.is_null(),
            GuestHandle::Logger(value) => value.is_null(),
            GuestHandle::Mob(value) => value.is_null(),
            GuestHandle::Player(value) => value.is_null(),
            GuestHandle::BlockBreakEvent(value) => value.is_null(),
            GuestHandle::BlockCookEvent(value) => value.is_null(),
            GuestHandle::BlockFromToEvent(value) => value.is_null(),
            GuestHandle::BlockGrowEvent(value) => value.is_null(),
            GuestHandle::BlockPistonEvent(value) => value.is_null(),
            GuestHandle::BlockPlaceEvent(value) => value.is_null(),
            GuestHandle::LeavesDecayEvent(value) => value.is_null(),
            GuestHandle::BroadcastMessageEvent(value) => value.is_null(),
            GuestHandle::WeatherChangeEvent(value) => value.is_null(),
            GuestHandle::ThunderChangeEvent(value) => value.is_null(),
            GuestHandle::PacketSendEvent(value) => value.is_null(),
            GuestHandle::PacketReceiveEvent(value) => value.is_null(),
            GuestHandle::MapInitializeEvent(value) => value.is_null(),
            GuestHandle::ScriptMessageEvent(value) => value.is_null(),
            GuestHandle::PlayerChatEvent(value) => value.is_null(),
            GuestHandle::PlayerJoinEvent(value) => value.is_null(),
            GuestHandle::PlayerQuitEvent(value) => value.is_null(),
            GuestHandle::PlayerDeathEvent(value) => value.is_null(),
            GuestHandle::ActorDamageEvent(value) => value.is_null(),
            GuestHandle::ActorExplodeEvent(value) => value.is_null(),
            GuestHandle::BlockExplodeEvent(value) => value.is_null(),
            GuestHandle::ActorDeathEvent(value) => value.is_null(),
            GuestHandle::ActorKnockbackEvent(value) => value.is_null(),
            GuestHandle::ActorRemoveEvent(value) => value.is_null(),
            GuestHandle::ActorSpawnEvent(value) => value.is_null(),
            GuestHandle::ActorTeleportEvent(value) => value.is_null(),
            GuestHandle::PlayerCommandEvent(value) => value.is_null(),
            GuestHandle::PlayerKickEvent(value) => value.is_null(),
            GuestHandle::PlayerLoginEvent(value) => value.is_null(),
            GuestHandle::PlayerGameModeChangeEvent(value) => value.is_null(),
            GuestHandle::PlayerEmoteEvent(value) => value.is_null(),
            GuestHandle::PlayerSkinChangeEvent(value) => value.is_null(),
            GuestHandle::PlayerDimensionChangeEvent(value) => value.is_null(),
            GuestHandle::PlayerBedEnterEvent(value) => value.is_null(),
            GuestHandle::PlayerBedLeaveEvent(value) => value.is_null(),
            GuestHandle::PlayerRespawnEvent(value) => value.is_null(),
            GuestHandle::PlayerItemHeldEvent(value) => value.is_null(),
            GuestHandle::PlayerMoveEvent(value) => value.is_null(),
            GuestHandle::PlayerDropItemEvent(value) => value.is_null(),
            GuestHandle::PlayerInteractEvent(value) => value.is_null(),
            GuestHandle::PlayerInteractActorEvent(value) => value.is_null(),
            GuestHandle::PlayerItemConsumeEvent(value) => value.is_null(),
            GuestHandle::PlayerPickupItemEvent(value) => value.is_null(),
            GuestHandle::ServerCommandEvent(value) => value.is_null(),
            GuestHandle::ServerListPingEvent(value) => value.is_null(),
            GuestHandle::PluginLifecycleEvent(value) => value.is_null(),
            GuestHandle::ServerLoadEvent(value) => value.is_null(),
            GuestHandle::ChunkEvent(value) => value.is_null(),
            GuestHandle::PlayerInventory(value) => value.is_null(),
            GuestHandle::PluginCommand(value) => value.is_null(),
            GuestHandle::PermissionAttachment(value) => value.is_null(),
            GuestHandle::PermissionDefinition(value) => value.is_null(),
            GuestHandle::Permissible(value) => value.is_null(),
            GuestHandle::Scoreboard(value) => value.is_null(),
            GuestHandle::ScoreEntry(value) => value.is_null(),
        };
        if invocation_id == 0 || is_null || self.next_id == u64::MAX {
            return 0;
        }
        let handle = self.next_id;
        self.next_id += 1;
        self.entries.insert(
            handle,
            Entry {
                invocation_id,
                lifetime,
                value,
            },
        );
        handle
    }
}

// Wasmtime requires Send store data, while Runtime pins all facade access and
// destruction to its primary thread. The opaque Endstone pointers never cross it.
unsafe impl Send for GuestHandles {}
