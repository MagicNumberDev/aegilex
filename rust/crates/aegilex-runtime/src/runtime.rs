use std::marker::PhantomData;
use std::net::SocketAddr;
use std::path::Path;
use std::rc::Rc;

use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview1::WasiP1Ctx;
use wasmtime_wasi::{DirPerms, FilePerms, SocketAddrUse, WasiCtxBuilder};

use crate::abi::{
    AEGILEX_DENIED, AEGILEX_INTERNAL_ERROR, AEGILEX_INVALID_ARGUMENT, AEGILEX_LIMIT_EXCEEDED,
    AEGILEX_NOT_FOUND, AEGILEX_OK, AEGILEX_TRAP,
};
use crate::bindings::endstone::logger::LogLevel;
use crate::config::RuntimeConfig;
use crate::core_resources::{CoreResourceTable, ResourceTableError, ResourceToken};
use crate::cxx_host_actor::ffi as cxx_actor;
use crate::cxx_host_admin::ffi as cxx_admin;
use crate::cxx_host_common::ffi as cxx_common;
use crate::cxx_host_inventory::ffi as cxx_inventory;
use crate::cxx_host_player::ffi as cxx_player;
use crate::cxx_runtime::ffi as cxx_event;
use crate::host::runtime::handles::{GuestHandles, ResourceKind, ResourceLifetime, ResourceSlot};
use crate::host::runtime::native::{self, HostContext};
use crate::manifest::{self, NetworkProtocol, PluginMetadata, PluginPolicy};

const ENABLE_FUEL: u64 = 10_000_000;
const RESOURCE_OWNER: u64 = 1;
const RESOURCE_KIND_OWNED: u32 = 0;
mod events;
mod lifecycle;
mod loader;
mod renderer;
mod service;

pub(crate) struct PluginStoreState {
    pub(crate) host: HostContext,
    pub(crate) handles: GuestHandles,
    pub(crate) plugin_id: String,
    pub(crate) invocation_id: u64,
    pub(crate) invocation_frames: Vec<u64>,
    pub(crate) subscriptions: Vec<String>,
    pub(crate) commands: Vec<crate::manifest::CommandSpec>,
    pub(crate) instance: Option<wasmtime::Instance>,
    pub(crate) policy: PluginPolicy,
    pub(crate) config: RuntimeConfig,
    pub(crate) wasi: WasiP1Ctx,
    pub(crate) resources: CoreResourceTable,
    pub(crate) forms: std::collections::HashMap<u32, crate::host::runtime::forms::GuestForm>,
    pub(crate) service_providers: std::collections::HashMap<u32, u64>,
    pub(crate) service_calls: std::collections::HashMap<u32, u64>,
    pub(crate) map_renderers:
        std::collections::HashMap<u32, crate::host::endstone::map::renderer::GuestMapRenderer>,
    pub(crate) resource_slot_count: usize,
    pub(crate) host_borrowed_slots: Vec<(u64, u32)>,
    pub(crate) plugin_owned_slots: Vec<u32>,
}

impl PluginStoreState {
    pub(crate) fn push_invocation(&mut self, invocation_id: u64) {
        self.invocation_frames.push(self.invocation_id);
        self.invocation_id = invocation_id;
    }

    pub(crate) fn pop_invocation(&mut self) {
        if let Some(previous) = self.invocation_frames.pop() {
            self.invocation_id = previous;
        }
    }

    pub(crate) fn require_capability(
        &self,
        capability: &str,
    ) -> Result<(), crate::core_host::TypesHostError> {
        if self.policy.capabilities.iter().any(|allowed| {
            allowed == "*"
                || allowed == capability
                || allowed.strip_suffix(".*").is_some_and(|prefix| {
                    capability
                        .strip_prefix(prefix)
                        .is_some_and(|suffix| suffix.starts_with('.'))
                })
        }) {
            Ok(())
        } else {
            Err(crate::core_host::TypesHostError::Denied)
        }
    }

    pub(crate) fn insert_actor_resource(
        &mut self,
        actor: cxx::UniquePtr<cxx_actor::Actor>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(ResourceKind::Actor, actor, GuestHandles::insert_actor)?;
        self.resource_from_handle(ResourceKind::Actor, handle)
    }

    pub(crate) fn insert_mob_resource(
        &mut self,
        mob: cxx::UniquePtr<cxx_actor::Mob>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(ResourceKind::Mob, mob, GuestHandles::insert_mob)?;
        self.resource_from_handle(ResourceKind::Mob, handle)
    }

    pub(crate) fn insert_item_stack_resource(
        &mut self,
        item_stack: cxx::UniquePtr<cxx_inventory::ItemStack>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::ItemStack,
            item_stack,
            GuestHandles::insert_item_stack,
        )?;
        self.resource_from_handle(ResourceKind::ItemStack, handle)
    }

    pub(crate) fn insert_inventory_resource(
        &mut self,
        inventory: cxx::UniquePtr<cxx_inventory::Inventory>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::Inventory,
            inventory,
            GuestHandles::insert_inventory,
        )?;
        self.resource_from_handle(ResourceKind::Inventory, handle)
    }

    pub(crate) fn insert_player_inventory_resource(
        &mut self,
        inventory: cxx::UniquePtr<cxx_inventory::PlayerInventory>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PlayerInventory,
            inventory,
            GuestHandles::insert_player_inventory,
        )?;
        self.resource_from_handle(ResourceKind::PlayerInventory, handle)
    }

    #[cfg(test)]
    pub(crate) fn insert_item_stack_ref_resource(
        &mut self,
        item_stack: cxx::UniquePtr<cxx_inventory::ItemStackRef>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::ItemStackRef,
            item_stack,
            GuestHandles::insert_item_stack_ref,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::ItemStackRef, handle)
    }

    pub(crate) fn insert_item_stack_ref_child_resource(
        &mut self,
        item_stack: cxx::UniquePtr<cxx_inventory::ItemStackRef>,
        parent: u32,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::ItemStackRef,
            item_stack,
            GuestHandles::insert_item_stack_ref,
        )?;
        self.host_borrowed_child_resource_from_handle(ResourceKind::ItemStackRef, handle, parent)
    }

    pub(crate) fn insert_item_actor_resource(
        &mut self,
        item_actor: cxx::UniquePtr<cxx_actor::ItemActor>,
        parent: u32,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::ItemActor,
            item_actor,
            GuestHandles::insert_item_actor,
        )?;
        self.host_borrowed_child_resource_from_handle(ResourceKind::ItemActor, handle, parent)
    }

    pub(crate) fn insert_player_resource(
        &mut self,
        player: cxx::UniquePtr<cxx_player::Player>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle =
            self.insert_handle(ResourceKind::Player, player, GuestHandles::insert_player)?;
        self.resource_from_handle(ResourceKind::Player, handle)
    }

    pub(crate) fn insert_logger_resource(
        &mut self,
        logger: cxx::UniquePtr<crate::cxx_host::ffi::Logger>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle =
            self.insert_handle(ResourceKind::Logger, logger, GuestHandles::insert_logger)?;
        self.resource_from_handle(ResourceKind::Logger, handle)
    }

    pub(crate) fn insert_plugin_command_resource(
        &mut self,
        command: cxx::UniquePtr<crate::cxx_host_server::ffi::PluginCommand>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PluginCommand,
            command,
            GuestHandles::insert_plugin_command,
        )?;
        self.resource_from_handle(ResourceKind::PluginCommand, handle)
    }

    pub(crate) fn insert_boss_bar_resource(
        &mut self,
        bar: cxx::UniquePtr<crate::cxx_host_ui::ffi::BossBar>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle =
            self.insert_handle(ResourceKind::BossBar, bar, GuestHandles::insert_boss_bar)?;
        self.resource_from_handle_with_lifetime(
            ResourceKind::BossBar,
            handle,
            ResourceLifetime::PluginOwned,
        )
    }

    pub(crate) fn insert_scoreboard_resource(
        &mut self,
        scoreboard: cxx::UniquePtr<crate::cxx_host_ui::ffi::Scoreboard>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::Scoreboard,
            scoreboard,
            GuestHandles::insert_scoreboard,
        )?;
        self.resource_from_handle(ResourceKind::Scoreboard, handle)
    }

    pub(crate) fn insert_score_entry_resource(
        &mut self,
        entry: cxx::UniquePtr<crate::cxx_host_ui::ffi::ScoreEntry>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::ScoreEntry,
            entry,
            GuestHandles::insert_score_entry,
        )?;
        self.resource_from_handle(ResourceKind::ScoreEntry, handle)
    }

    pub(crate) fn insert_ban_list_resource(
        &mut self,
        ban_list: cxx::UniquePtr<cxx_admin::BanList>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::BanList,
            ban_list,
            GuestHandles::insert_ban_list,
        )?;
        self.resource_from_handle(ResourceKind::BanList, handle)
    }

    pub(crate) fn insert_permission_attachment_resource(
        &mut self,
        attachment: cxx::UniquePtr<cxx_admin::PermissionAttachment>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PermissionAttachment,
            attachment,
            GuestHandles::insert_permission_attachment,
        )?;
        self.resource_from_handle(ResourceKind::PermissionAttachment, handle)
    }

    pub(crate) fn insert_permission_definition_resource(
        &mut self,
        definition: cxx::UniquePtr<cxx_admin::PermissionDefinition>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PermissionDefinition,
            definition,
            GuestHandles::insert_permission_definition,
        )?;
        self.resource_from_handle(ResourceKind::PermissionDefinition, handle)
    }

    pub(crate) fn insert_permissible_resource(
        &mut self,
        permissible: cxx::UniquePtr<cxx_common::Permissible>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::Permissible,
            permissible,
            GuestHandles::insert_permissible,
        )?;
        self.resource_from_handle(ResourceKind::Permissible, handle)
    }

    pub(crate) fn insert_player_child_resource(
        &mut self,
        player: cxx::UniquePtr<cxx_player::Player>,
        parent: u32,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle =
            self.insert_handle(ResourceKind::Player, player, GuestHandles::insert_player)?;
        self.host_borrowed_child_resource_from_handle(ResourceKind::Player, handle, parent)
    }

    pub(crate) fn insert_block_break_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::BlockBreakEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::BlockBreakEvent,
            event,
            GuestHandles::insert_block_break_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::BlockBreakEvent, handle)
    }

    pub(crate) fn insert_block_cook_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::BlockCookEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::BlockCookEvent,
            event,
            GuestHandles::insert_block_cook_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::BlockCookEvent, handle)
    }

    pub(crate) fn insert_leaves_decay_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::LeavesDecayEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::LeavesDecayEvent,
            event,
            GuestHandles::insert_leaves_decay_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::LeavesDecayEvent, handle)
    }

    pub(crate) fn insert_block_from_to_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::BlockFromToEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::BlockFromToEvent,
            event,
            GuestHandles::insert_block_from_to_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::BlockFromToEvent, handle)
    }

    pub(crate) fn insert_block_grow_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::BlockGrowEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::BlockGrowEvent,
            event,
            GuestHandles::insert_block_grow_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::BlockGrowEvent, handle)
    }

    pub(crate) fn insert_block_piston_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::BlockPistonEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::BlockPistonEvent,
            event,
            GuestHandles::insert_block_piston_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::BlockPistonEvent, handle)
    }

    pub(crate) fn insert_block_place_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::BlockPlaceEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::BlockPlaceEvent,
            event,
            GuestHandles::insert_block_place_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::BlockPlaceEvent, handle)
    }

    pub(crate) fn insert_actor_child_resource(
        &mut self,
        actor: cxx::UniquePtr<cxx_actor::Actor>,
        parent: u32,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(ResourceKind::Actor, actor, GuestHandles::insert_actor)?;
        self.host_borrowed_child_resource_from_handle(ResourceKind::Actor, handle, parent)
    }

    pub(crate) fn insert_actor_damage_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::ActorDamageEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::ActorDamageEvent,
            event,
            GuestHandles::insert_actor_damage_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::ActorDamageEvent, handle)
    }

    pub(crate) fn insert_actor_death_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::ActorDeathEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::ActorDeathEvent,
            event,
            GuestHandles::insert_actor_death_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::ActorDeathEvent, handle)
    }

    pub(crate) fn insert_actor_explode_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::ActorExplodeEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::ActorExplodeEvent,
            event,
            GuestHandles::insert_actor_explode_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::ActorExplodeEvent, handle)
    }

    pub(crate) fn insert_block_explode_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::BlockExplodeEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::BlockExplodeEvent,
            event,
            GuestHandles::insert_block_explode_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::BlockExplodeEvent, handle)
    }

    pub(crate) fn insert_actor_knockback_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::ActorKnockbackEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::ActorKnockbackEvent,
            event,
            GuestHandles::insert_actor_knockback_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::ActorKnockbackEvent, handle)
    }

    pub(crate) fn insert_actor_remove_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::ActorRemoveEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::ActorRemoveEvent,
            event,
            GuestHandles::insert_actor_remove_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::ActorRemoveEvent, handle)
    }

    pub(crate) fn insert_actor_spawn_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::ActorSpawnEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::ActorSpawnEvent,
            event,
            GuestHandles::insert_actor_spawn_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::ActorSpawnEvent, handle)
    }

    pub(crate) fn insert_actor_teleport_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::ActorTeleportEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::ActorTeleportEvent,
            event,
            GuestHandles::insert_actor_teleport_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::ActorTeleportEvent, handle)
    }

    pub(crate) fn insert_player_chat_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::PlayerChatEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PlayerChatEvent,
            event,
            GuestHandles::insert_player_chat_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::PlayerChatEvent, handle)
    }

    pub(crate) fn insert_player_join_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::PlayerJoinEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PlayerJoinEvent,
            event,
            GuestHandles::insert_player_join_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::PlayerJoinEvent, handle)
    }

    pub(crate) fn insert_player_quit_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::PlayerQuitEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PlayerQuitEvent,
            event,
            GuestHandles::insert_player_quit_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::PlayerQuitEvent, handle)
    }

    pub(crate) fn insert_player_bed_enter_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::PlayerBedEnterEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PlayerBedEnterEvent,
            event,
            GuestHandles::insert_player_bed_enter_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::PlayerBedEnterEvent, handle)
    }

    pub(crate) fn insert_player_bed_leave_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::PlayerBedLeaveEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PlayerBedLeaveEvent,
            event,
            GuestHandles::insert_player_bed_leave_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::PlayerBedLeaveEvent, handle)
    }

    pub(crate) fn insert_broadcast_message_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::BroadcastMessageEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::BroadcastMessageEvent,
            event,
            GuestHandles::insert_broadcast_message_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::BroadcastMessageEvent, handle)
    }

    pub(crate) fn insert_packet_send_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::PacketSendEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PacketSendEvent,
            event,
            GuestHandles::insert_packet_send_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::PacketSendEvent, handle)
    }

    pub(crate) fn insert_packet_receive_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::PacketReceiveEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PacketReceiveEvent,
            event,
            GuestHandles::insert_packet_receive_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::PacketReceiveEvent, handle)
    }

    pub(crate) fn insert_map_initialize_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::MapInitializeEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::MapInitializeEvent,
            event,
            GuestHandles::insert_map_initialize_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::MapInitializeEvent, handle)
    }

    pub(crate) fn insert_script_message_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::ScriptMessageEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::ScriptMessageEvent,
            event,
            GuestHandles::insert_script_message_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::ScriptMessageEvent, handle)
    }

    pub(crate) fn insert_weather_change_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::WeatherChangeEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::WeatherChangeEvent,
            event,
            GuestHandles::insert_weather_change_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::WeatherChangeEvent, handle)
    }

    pub(crate) fn insert_thunder_change_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::ThunderChangeEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::ThunderChangeEvent,
            event,
            GuestHandles::insert_thunder_change_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::ThunderChangeEvent, handle)
    }

    pub(crate) fn insert_player_command_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::PlayerCommandEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PlayerCommandEvent,
            event,
            GuestHandles::insert_player_command_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::PlayerCommandEvent, handle)
    }

    pub(crate) fn insert_player_kick_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::PlayerKickEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PlayerKickEvent,
            event,
            GuestHandles::insert_player_kick_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::PlayerKickEvent, handle)
    }

    pub(crate) fn insert_player_login_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::PlayerLoginEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PlayerLoginEvent,
            event,
            GuestHandles::insert_player_login_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::PlayerLoginEvent, handle)
    }

    pub(crate) fn insert_player_game_mode_change_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::PlayerGameModeChangeEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PlayerGameModeChangeEvent,
            event,
            GuestHandles::insert_player_game_mode_change_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::PlayerGameModeChangeEvent, handle)
    }

    pub(crate) fn insert_player_emote_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::PlayerEmoteEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PlayerEmoteEvent,
            event,
            GuestHandles::insert_player_emote_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::PlayerEmoteEvent, handle)
    }

    pub(crate) fn insert_player_skin_change_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::PlayerSkinChangeEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PlayerSkinChangeEvent,
            event,
            GuestHandles::insert_player_skin_change_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::PlayerSkinChangeEvent, handle)
    }

    pub(crate) fn insert_player_death_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::PlayerDeathEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PlayerDeathEvent,
            event,
            GuestHandles::insert_player_death_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::PlayerDeathEvent, handle)
    }

    pub(crate) fn insert_player_dimension_change_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::PlayerDimensionChangeEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PlayerDimensionChangeEvent,
            event,
            GuestHandles::insert_player_dimension_change_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::PlayerDimensionChangeEvent, handle)
    }

    pub(crate) fn insert_player_respawn_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::PlayerRespawnEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PlayerRespawnEvent,
            event,
            GuestHandles::insert_player_respawn_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::PlayerRespawnEvent, handle)
    }

    pub(crate) fn insert_player_item_held_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::PlayerItemHeldEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PlayerItemHeldEvent,
            event,
            GuestHandles::insert_player_item_held_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::PlayerItemHeldEvent, handle)
    }

    pub(crate) fn insert_player_move_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::PlayerMoveEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PlayerMoveEvent,
            event,
            GuestHandles::insert_player_move_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::PlayerMoveEvent, handle)
    }

    pub(crate) fn insert_player_pickup_item_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::PlayerPickupItemEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PlayerPickupItemEvent,
            event,
            GuestHandles::insert_player_pickup_item_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::PlayerPickupItemEvent, handle)
    }

    pub(crate) fn insert_player_drop_item_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::PlayerDropItemEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PlayerDropItemEvent,
            event,
            GuestHandles::insert_player_drop_item_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::PlayerDropItemEvent, handle)
    }

    pub(crate) fn insert_player_interact_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::PlayerInteractEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PlayerInteractEvent,
            event,
            GuestHandles::insert_player_interact_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::PlayerInteractEvent, handle)
    }

    pub(crate) fn insert_player_interact_actor_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::PlayerInteractActorEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PlayerInteractActorEvent,
            event,
            GuestHandles::insert_player_interact_actor_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::PlayerInteractActorEvent, handle)
    }

    pub(crate) fn insert_player_item_consume_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::PlayerItemConsumeEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PlayerItemConsumeEvent,
            event,
            GuestHandles::insert_player_item_consume_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::PlayerItemConsumeEvent, handle)
    }

    pub(crate) fn insert_server_command_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::ServerCommandEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::ServerCommandEvent,
            event,
            GuestHandles::insert_server_command_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::ServerCommandEvent, handle)
    }

    pub(crate) fn insert_server_list_ping_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::ServerListPingEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::ServerListPingEvent,
            event,
            GuestHandles::insert_server_list_ping_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::ServerListPingEvent, handle)
    }

    pub(crate) fn insert_plugin_lifecycle_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::PluginLifecycleEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::PluginLifecycleEvent,
            event,
            GuestHandles::insert_plugin_lifecycle_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::PluginLifecycleEvent, handle)
    }

    pub(crate) fn insert_server_load_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::ServerLoadEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::ServerLoadEvent,
            event,
            GuestHandles::insert_server_load_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::ServerLoadEvent, handle)
    }

    pub(crate) fn insert_chunk_event_resource(
        &mut self,
        event: cxx::UniquePtr<cxx_event::ChunkEventFacade>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::ChunkEvent,
            event,
            GuestHandles::insert_chunk_event,
        )?;
        self.host_borrowed_resource_from_handle(ResourceKind::ChunkEvent, handle)
    }

    pub(crate) fn insert_command_sender_resource(
        &mut self,
        sender: cxx::UniquePtr<cxx_common::CommandSender>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(
            ResourceKind::CommandSender,
            sender,
            GuestHandles::insert_command_sender,
        )?;
        self.resource_from_handle(ResourceKind::CommandSender, handle)
    }

    pub(crate) fn insert_block_resource(
        &mut self,
        block: cxx::UniquePtr<crate::cxx_host_level::ffi::Block>,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let handle = self.insert_handle(ResourceKind::Block, block, GuestHandles::insert_block)?;
        self.resource_from_handle(ResourceKind::Block, handle)
    }

    pub(crate) fn insert_handle<T: cxx::memory::UniquePtrTarget>(
        &mut self,
        _kind: ResourceKind,
        value: cxx::UniquePtr<T>,
        insert: fn(&mut GuestHandles, u64, cxx::UniquePtr<T>) -> u64,
    ) -> Result<u64, crate::host::runtime::native::HostError> {
        if self.config.max_invocation_native_resources != 0
            && self.handles.is_invocation_at_capacity(
                self.invocation_id,
                self.config.max_invocation_native_resources as usize,
            )
        {
            return Err(crate::host::runtime::native::HostError::from_status(
                crate::abi::AEGILEX_LIMIT_EXCEEDED,
            ));
        }
        let handle = insert(&mut self.handles, self.invocation_id, value);
        (handle != 0)
            .then_some(handle)
            .ok_or_else(|| crate::host::runtime::native::HostError::from_status(AEGILEX_NOT_FOUND))
    }

    pub(crate) fn resource_slot(
        &self,
        rep: u32,
        kind: ResourceKind,
    ) -> Result<&ResourceSlot, crate::host::runtime::native::HostError> {
        self.resources
            .get::<ResourceSlot>(
                ResourceToken::from_rep(rep),
                kind as u32,
                RESOURCE_OWNER,
                self.invocation_id,
            )
            .map_err(|_| crate::host::runtime::native::HostError::from_status(AEGILEX_NOT_FOUND))
    }

    pub(crate) fn insert_owned_resource<V: Send + 'static>(
        &mut self,
        value: V,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        if self.config.max_plugin_resource_slots != 0
            && self.resource_slot_count >= self.config.max_plugin_resource_slots as usize
        {
            return Err(crate::host::runtime::native::HostError::from_status(
                crate::abi::AEGILEX_LIMIT_EXCEEDED,
            ));
        }
        let token = self
            .resources
            .insert_value(
                value,
                RESOURCE_KIND_OWNED,
                RESOURCE_OWNER,
                ResourceLifetime::HostBorrowed,
                self.invocation_id,
            )
            .map_err(|_| crate::host::runtime::native::HostError::from_status(AEGILEX_NOT_FOUND))?;
        self.resource_slot_count += 1;
        Ok(token.rep())
    }

    pub(crate) fn owned_resource<V: Send + 'static>(
        &self,
        rep: u32,
    ) -> Result<&V, crate::host::runtime::native::HostError> {
        self.resources
            .get::<V>(
                ResourceToken::from_rep(rep),
                RESOURCE_KIND_OWNED,
                RESOURCE_OWNER,
                self.invocation_id,
            )
            .map_err(|_| crate::host::runtime::native::HostError::from_status(AEGILEX_NOT_FOUND))
    }

    pub(crate) fn owned_resource_mut<V: Send + 'static>(
        &mut self,
        rep: u32,
    ) -> Result<&mut V, crate::host::runtime::native::HostError> {
        self.resources
            .get_mut::<V>(
                ResourceToken::from_rep(rep),
                RESOURCE_KIND_OWNED,
                RESOURCE_OWNER,
                self.invocation_id,
            )
            .map_err(|_| crate::host::runtime::native::HostError::from_status(AEGILEX_NOT_FOUND))
    }

    pub(crate) fn drop_resource(
        &mut self,
        rep: u32,
        kind: ResourceKind,
    ) -> Result<(), crate::host::runtime::native::HostError> {
        let token = ResourceToken::from_rep(rep);
        let handle = self
            .resources
            .get_raw::<ResourceSlot>(token)
            .ok()
            .map(|slot| slot.handle);
        if !self
            .resources
            .remove_checked(token, kind as u32, RESOURCE_OWNER, self.invocation_id)
        {
            return Ok(());
        }
        if let Some(handle) = handle {
            self.handles.remove_handles(&[handle]);
        }
        self.host_borrowed_slots
            .retain(|(_, slot_rep)| *slot_rep != rep);
        self.plugin_owned_slots.retain(|slot_rep| *slot_rep != rep);
        self.resource_slot_count = self.resource_slot_count.saturating_sub(1);
        Ok(())
    }

    pub(crate) fn clear_invocation_resources(&mut self, invocation_id: u64) {
        let mut pending = self
            .host_borrowed_slots
            .iter()
            .filter(|(slot_invocation_id, _)| *slot_invocation_id == invocation_id)
            .map(|(_, rep)| *rep)
            .collect::<Vec<_>>();

        // Delete children before their parents, retrying entries blocked by a
        // CoreResourceTable parent-child relationship.
        while !pending.is_empty() {
            let mut remaining = Vec::new();
            let mut progress = false;
            for rep in pending.into_iter().rev() {
                let token = ResourceToken::from_rep(rep);
                let is_matching_slot =
                    self.resources
                        .get_raw::<ResourceSlot>(token)
                        .is_ok_and(|slot| {
                            slot.invocation_id == invocation_id
                                && slot.lifetime == ResourceLifetime::HostBorrowed
                        });
                if !is_matching_slot {
                    self.untrack_host_borrowed_slot(invocation_id, rep);
                    progress = true;
                    continue;
                }

                if self.resources.remove_raw(token) {
                    self.untrack_host_borrowed_slot(invocation_id, rep);
                    self.resource_slot_count = self.resource_slot_count.saturating_sub(1);
                    progress = true;
                } else {
                    remaining.push(rep);
                }
            }
            if !progress {
                break;
            }
            pending = remaining;
        }

        // Guest-owned resource slots remain so their representations cannot be
        // reused while held, but their native facades expire with the invocation.
        self.handles.clear_invocation(invocation_id);
    }

    pub(crate) fn clear_plugin_resources(&mut self) {
        for rep in std::mem::take(&mut self.plugin_owned_slots)
            .into_iter()
            .rev()
        {
            let token = ResourceToken::from_rep(rep);
            let handle = self
                .resources
                .get_raw::<ResourceSlot>(token)
                .ok()
                .map(|s| s.handle);
            if self.resources.remove_raw(token) {
                if let Some(handle) = handle {
                    self.handles.remove_handles(&[handle]);
                }
                self.resource_slot_count = self.resource_slot_count.saturating_sub(1);
            }
        }
    }

    pub(crate) fn resource_from_handle(
        &mut self,
        kind: ResourceKind,
        handle: u64,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        self.resource_from_handle_with_lifetime(kind, handle, ResourceLifetime::GuestOwned)
    }

    #[allow(dead_code)]
    pub(crate) fn host_borrowed_resource_from_handle(
        &mut self,
        kind: ResourceKind,
        handle: u64,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        self.resource_from_handle_with_lifetime(kind, handle, ResourceLifetime::HostBorrowed)
    }

    #[allow(dead_code)]
    pub(crate) fn host_borrowed_child_resource_from_handle(
        &mut self,
        kind: ResourceKind,
        handle: u64,
        parent: u32,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let parent_invocation = self
            .resources
            .get_raw::<ResourceSlot>(ResourceToken::from_rep(parent))
            .ok()
            .map(|slot| slot.invocation_id);
        let Some(parent_invocation) = parent_invocation else {
            self.handles.remove_handles(&[handle]);
            return Err(crate::host::runtime::native::HostError::from_status(
                AEGILEX_NOT_FOUND,
            ));
        };
        if parent_invocation != self.invocation_id {
            self.handles.remove_handles(&[handle]);
            return Err(crate::host::runtime::native::HostError::from_status(
                AEGILEX_NOT_FOUND,
            ));
        }
        let parent_token = ResourceToken::from_rep(parent);
        let invocation_id = self.invocation_id;
        self.resource_from_handle_with_lifetime_and_insert(
            handle,
            ResourceLifetime::HostBorrowed,
            |resources, slot| {
                resources.insert_value_child(
                    slot,
                    kind as u32,
                    RESOURCE_OWNER,
                    ResourceLifetime::HostBorrowed,
                    invocation_id,
                    parent_token,
                )
            },
        )
    }

    fn resource_from_handle_with_lifetime(
        &mut self,
        kind: ResourceKind,
        handle: u64,
        lifetime: ResourceLifetime,
    ) -> Result<u32, crate::host::runtime::native::HostError> {
        let invocation_id = self.invocation_id;
        self.resource_from_handle_with_lifetime_and_insert(handle, lifetime, |resources, slot| {
            resources.insert_value(slot, kind as u32, RESOURCE_OWNER, lifetime, invocation_id)
        })
    }

    fn resource_from_handle_with_lifetime_and_insert<F>(
        &mut self,
        handle: u64,
        lifetime: ResourceLifetime,
        insert: F,
    ) -> Result<u32, crate::host::runtime::native::HostError>
    where
        F: FnOnce(
            &mut CoreResourceTable,
            ResourceSlot,
        ) -> Result<ResourceToken, ResourceTableError>,
    {
        if handle == 0 {
            return Err(crate::host::runtime::native::HostError::from_status(
                AEGILEX_NOT_FOUND,
            ));
        }
        if self.config.max_plugin_resource_slots != 0
            && self.resource_slot_count >= self.config.max_plugin_resource_slots as usize
        {
            self.handles.remove_handles(&[handle]);
            return Err(crate::host::runtime::native::HostError::from_status(
                crate::abi::AEGILEX_LIMIT_EXCEEDED,
            ));
        }
        let token = insert(
            &mut self.resources,
            ResourceSlot {
                invocation_id: self.invocation_id,
                handle,
                lifetime,
            },
        )
        .map_err(|_| {
            self.handles.remove_handles(&[handle]);
            crate::host::runtime::native::HostError::from_status(AEGILEX_NOT_FOUND)
        })?;
        self.resource_slot_count += 1;
        if lifetime == ResourceLifetime::HostBorrowed {
            self.host_borrowed_slots
                .push((self.invocation_id, token.rep()));
        }
        if lifetime == ResourceLifetime::PluginOwned {
            self.plugin_owned_slots.push(token.rep());
        }
        Ok(token.rep())
    }

    fn untrack_host_borrowed_slot(&mut self, invocation_id: u64, rep: u32) {
        self.host_borrowed_slots
            .retain(|(slot_invocation_id, slot_rep)| {
                *slot_invocation_id != invocation_id || *slot_rep != rep
            });
    }
}

struct LoadedPlugin {
    id: String,
    metadata: PluginMetadata,
    store: Store<PluginStoreState>,
    instance: wasmtime::Instance,
    exports: crate::core_host::CoreExports,
    enabled: bool,
    subscriptions: Vec<String>,
    commands: Vec<crate::manifest::CommandSpec>,
}

pub(crate) struct Runtime {
    engine: Engine,
    host: HostContext,
    config: RuntimeConfig,
    plugins: Vec<LoadedPlugin>,
    services: std::collections::HashMap<u64, ServiceEntry>,
    service_calls: std::collections::HashMap<u64, ServiceCallState>,
    next_service_id: u64,
    next_service_call_id: u64,
    _primary_thread: PhantomData<Rc<()>>,
}

pub(crate) const SERVICE_STATUS_PENDING: u32 = 0;
pub(crate) const SERVICE_STATUS_COMPLETED: u32 = 1;
pub(crate) const SERVICE_STATUS_REJECTED: u32 = 2;
pub(crate) const SERVICE_STATUS_FAILED: u32 = 3;
pub(crate) const SERVICE_STATUS_CANCELLED: u32 = 4;
pub(crate) const SERVICE_STATUS_EXPIRED: u32 = 5;

pub(crate) struct ServiceEntry {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) methods: Vec<String>,
    pub(crate) priority: u32,
    pub(crate) owner: String,
}

pub(crate) struct ServiceCallState {
    pub(crate) status: u32,
    pub(crate) payload: Vec<u8>,
    pub(crate) error: String,
    pub(crate) taken: bool,
}

pub(crate) struct PluginInspection {
    pub(crate) metadata: PluginMetadata,
}

impl Runtime {
    pub(crate) fn new(host: HostContext, config: RuntimeConfig) -> Result<Self, u32> {
        let mut wasmtime_config = Config::new();
        wasmtime_config.consume_fuel(true);
        let engine = Engine::new(&wasmtime_config).map_err(|_| AEGILEX_INTERNAL_ERROR)?;

        Ok(Self {
            engine,
            host,
            config,
            plugins: Vec::new(),
            services: std::collections::HashMap::new(),
            service_calls: std::collections::HashMap::new(),
            next_service_id: 1,
            next_service_call_id: 1,
            _primary_thread: PhantomData,
        })
    }

    pub(crate) fn next_invocation_id(&self) -> u64 {
        self.host.next_invocation_id()
    }

    pub(crate) fn dispatch_task(&mut self, plugin_id: &str, task_id: u64) -> Result<(), u32> {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return Err(AEGILEX_NOT_FOUND);
        };
        if !plugin.enabled {
            return Err(AEGILEX_NOT_FOUND);
        }
        if let Err(error) = plugin.store.set_fuel(ENABLE_FUEL) {
            log_loader_error(
                &self.host,
                &format!("{}: cannot reset fuel: {error}", plugin.id),
            );
            return Err(AEGILEX_INTERNAL_ERROR);
        }
        let invocation_id = self.host.next_invocation_id();
        let outcome = call_with_invocation(&mut plugin.store, invocation_id, |store| {
            plugin
                .exports
                .call_tasks_on_task(&plugin.instance, store, task_id)
        });
        match outcome {
            Ok(Ok(())) => Ok(()),
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-task rejected: {text}", plugin.id),
                );
                Ok(())
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-task trapped: {error}", plugin.id),
                );
                Err(AEGILEX_TRAP)
            }
        }
    }

    pub(crate) fn dispatch_form_submit(
        &mut self,
        plugin_id: &str,
        form_id: u64,
        has_player: bool,
        player_uuid: &[u8],
        response: cxx_event::FormResponseData,
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        let Ok(form_id) = u32::try_from(form_id) else {
            return false;
        };
        if !plugin.enabled || !plugin.store.data().forms.contains_key(&form_id) {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            return false;
        }
        let invocation_id = self.host.next_invocation_id();
        let outcome = call_with_invocation(&mut plugin.store, invocation_id, |store| {
            let player = match resolve_player_for_dispatch(store, has_player, player_uuid) {
                Some(player) => player,
                None => return Ok(Err("form player is not online".to_owned())),
            };
            let form = form_id;
            let wit_response = match response.kind {
                crate::host::runtime::forms::FORM_ACTION => {
                    crate::core_host::FormCallbacksFormResponse::Action(
                        crate::core_host::ActionFormActionResponse {
                            selected_index: response.selected_index,
                        },
                    )
                }
                crate::host::runtime::forms::FORM_MESSAGE => {
                    crate::core_host::FormCallbacksFormResponse::Message(
                        match response.message_button {
                            1 => crate::core_host::MessageFormMessageResponse::Button2,
                            _ => crate::core_host::MessageFormMessageResponse::Button1,
                        },
                    )
                }
                crate::host::runtime::forms::FORM_MODAL => {
                    crate::core_host::FormCallbacksFormResponse::Modal(
                        crate::core_host::ModalFormModalResponse {
                            json: response.modal_json,
                        },
                    )
                }
                _ => return Ok(Err("unknown form kind".to_owned())),
            };
            plugin.exports.call_formcallbacks_on_form_submit(
                &plugin.instance,
                store,
                form,
                player,
                wit_response,
            )
        });
        match outcome {
            Ok(Ok(())) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-form-submit rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-form-submit trapped: {error}", plugin.id),
                );
                false
            }
        }
    }

    pub(crate) fn dispatch_form_close(
        &mut self,
        plugin_id: &str,
        form_id: u64,
        has_player: bool,
        player_uuid: &[u8],
    ) -> bool {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id)
        else {
            return false;
        };
        let Ok(form_id) = u32::try_from(form_id) else {
            return false;
        };
        if !plugin.enabled || !plugin.store.data().forms.contains_key(&form_id) {
            return false;
        }
        if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
            return false;
        }
        let invocation_id = self.host.next_invocation_id();
        let outcome = call_with_invocation(&mut plugin.store, invocation_id, |store| {
            let player = match resolve_player_for_dispatch(store, has_player, player_uuid) {
                Some(player) => player,
                None => return Ok(Err("form player is not online".to_owned())),
            };
            plugin.exports.call_formcallbacks_on_form_close(
                &plugin.instance,
                store,
                form_id,
                player,
            )
        });
        match outcome {
            Ok(Ok(())) => true,
            Ok(Err(text)) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-form-close rejected: {text}", plugin.id),
                );
                false
            }
            Err(error) => {
                log_loader_error(
                    &self.host,
                    &format!("{}: on-form-close trapped: {error}", plugin.id),
                );
                false
            }
        }
    }

    fn log_loader_error(&self, message: &str) {
        log_loader_error(&self.host, message);
    }
}

fn build_wasi(policy: &PluginPolicy) -> Result<WasiP1Ctx, String> {
    let mut builder = WasiCtxBuilder::new();
    for path in &policy.paths {
        builder
            .preopened_dir(
                &path.host_path,
                &path.guest_path,
                DirPerms::all(),
                FilePerms::all(),
            )
            .map_err(|error| {
                format!(
                    "cannot preopen authorized directory {}: {error}",
                    path.host_path.display()
                )
            })?;
    }

    let allow_tcp = policy
        .network
        .iter()
        .any(|rule| rule.protocol == NetworkProtocol::Tcp);
    let allow_udp = policy
        .network
        .iter()
        .any(|rule| rule.protocol == NetworkProtocol::Udp);
    let network = policy.network.clone();
    builder
        .allow_tcp(allow_tcp)
        .allow_udp(allow_udp)
        .allow_ip_name_lookup(false)
        .socket_addr_check(move |address, usage| {
            let allowed = network_allows(&network, address, usage);
            Box::pin(async move { allowed })
        });
    Ok(builder.build_p1())
}

fn network_allows(
    rules: &[manifest::NetworkRule],
    address: SocketAddr,
    usage: SocketAddrUse,
) -> bool {
    match usage {
        SocketAddrUse::TcpConnect => rules
            .iter()
            .any(|rule| rule.protocol == NetworkProtocol::Tcp && rule.address == address),
        SocketAddrUse::UdpConnect | SocketAddrUse::UdpOutgoingDatagram => rules
            .iter()
            .any(|rule| rule.protocol == NetworkProtocol::Udp && rule.address == address),
        // WASI creates servers by binding first, so rejecting all bind operations
        // also rejects TCP listens. It makes UDP rules intentionally unusable until
        // a future policy format explicitly grants a local client bind.
        SocketAddrUse::TcpBind | SocketAddrUse::UdpBind => false,
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        let host = &self.host;
        for plugin in self.plugins.iter_mut().rev() {
            if plugin.enabled
                && let Err(error) =
                    call_with_invocation(&mut plugin.store, host.next_invocation_id(), |store| {
                        plugin.exports.call_on_disable(store)
                    })
            {
                log_loader_error(host, &format!("{}: on-disable trapped: {error}", plugin.id));
            }
        }
    }
}

fn call_with_invocation<T>(
    store: &mut Store<PluginStoreState>,
    invocation_id: u64,
    call: impl FnOnce(&mut Store<PluginStoreState>) -> T,
) -> T {
    store.data_mut().push_invocation(invocation_id);
    let result = call_guest(store, call);
    let state = store.data_mut();
    state.clear_invocation_resources(invocation_id);
    state.pop_invocation();
    result
}
fn call_guest<T>(
    store: &mut Store<PluginStoreState>,
    call: impl FnOnce(&mut Store<PluginStoreState>) -> T,
) -> T {
    call(store)
}

fn resolve_player_for_dispatch(
    store: &mut Store<PluginStoreState>,
    has_player: bool,
    player_uuid: &[u8],
) -> Option<u32> {
    if !has_player || player_uuid.len() != 16 {
        return None;
    }
    let uuid: [u8; 16] = player_uuid.try_into().ok()?;
    let state = store.data();
    let server = state.host.server().ok()?;
    let player = server.findPlayerByUuid(&uuid);
    if player.is_null() {
        return None;
    }
    store.data_mut().insert_player_resource(player).ok()
}

pub(crate) fn resolve_player_for_caller_dispatch(
    caller: &mut wasmtime::Caller<'_, PluginStoreState>,
    has_player: bool,
    player_uuid: &[u8],
) -> Option<u32> {
    if !has_player || player_uuid.len() != 16 {
        return None;
    }
    let uuid: [u8; 16] = player_uuid.try_into().ok()?;
    let state = caller.data();
    let server = state.host.server().ok()?;
    let player = server.findPlayerByUuid(&uuid);
    if player.is_null() {
        return None;
    }
    caller.data_mut().insert_player_resource(player).ok()
}

fn log_loader_error(host: &HostContext, message: &str) {
    if let Ok(logger) = native::get_logger(host, "aegilex", 0) {
        let _ = native::logger_log(&logger, LogLevel::Warning, message);
    }
}

#[cfg(test)]
#[path = "runtime_tests.rs"]
mod tests;
