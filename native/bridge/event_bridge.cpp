#include "event_bridge.h"

#include "../bindings/endstone/validation.h"
#include "../bindings/endstone/actor/actor.h"
#include "../bindings/endstone/actor/player.h"
#include "../bindings/endstone/events/player_chat_event_facade.h"
#include "../bindings/endstone/events/player_join_event_facade.h"
#include "../bindings/endstone/events/player_quit_event_facade.h"
#include "../bindings/endstone/events/actor_damage_event_facade.h"
#include "../bindings/endstone/events/actor_death_event_facade.h"
#include "../bindings/endstone/events/block_explode_event_facade.h"
#include "../bindings/endstone/events/actor_explode_event_facade.h"
#include "../bindings/endstone/events/actor_knockback_event_facade.h"
#include "../bindings/endstone/events/actor_remove_event_facade.h"
#include "../bindings/endstone/events/plugin_lifecycle_event_facade.h"
#include "../bindings/endstone/events/server_load_event_facade.h"
#include "../bindings/endstone/events/chunk_event_facade.h"
#include "../bindings/endstone/events/actor_spawn_event_facade.h"
#include "../bindings/endstone/events/actor_teleport_event_facade.h"
#include "../bindings/endstone/events/broadcast_message_event_facade.h"
#include "../bindings/endstone/events/packet_send_event_facade.h"
#include "../bindings/endstone/events/packet_receive_event_facade.h"
#include "../bindings/endstone/events/map_initialize_event_facade.h"
#include "../bindings/endstone/events/script_message_event_facade.h"
#include "../bindings/endstone/events/player_kick_event_facade.h"
#include "../bindings/endstone/events/player_game_mode_change_event_facade.h"
#include "../bindings/endstone/events/player_emote_event_facade.h"
#include "../bindings/endstone/events/player_skin_change_event_facade.h"
#include "../bindings/endstone/events/player_death_event_facade.h"
#include "../bindings/endstone/events/player_dimension_change_event_facade.h"
#include "../bindings/endstone/events/player_bed_enter_event_facade.h"
#include "../bindings/endstone/events/player_bed_leave_event_facade.h"
#include "../bindings/endstone/events/player_respawn_event_facade.h"
#include "../bindings/endstone/events/player_item_held_event_facade.h"
#include "../bindings/endstone/events/player_drop_item_event_facade.h"
#include "../bindings/endstone/events/block_break_event_facade.h"
#include "../bindings/endstone/events/block_cook_event_facade.h"
#include "../bindings/endstone/events/block_place_event_facade.h"
#include "../bindings/endstone/events/block_from_to_event_facade.h"
#include "../bindings/endstone/events/block_grow_event_facade.h"
#include "../bindings/endstone/events/block_piston_event_facade.h"
#include "../bindings/endstone/events/leaves_decay_event_facade.h"
#include "../bindings/endstone/events/player_interact_event_facade.h"
#include "../bindings/endstone/events/player_interact_actor_event_facade.h"
#include "../bindings/endstone/events/player_item_consume_event_facade.h"
#include "../bindings/endstone/events/player_pickup_item_event_facade.h"
#include "../bindings/endstone/events/player_move_event_facade.h"
#include "../bindings/endstone/events/server_command_event_facade.h"
#include "../bindings/endstone/events/server_list_ping_event_facade.h"
#include "../bindings/endstone/events/weather_change_event_facade.h"
#include "../bindings/endstone/events/thunder_change_event_facade.h"
#include "../bindings/endstone/inventory/item_stack.h"
#include "../host_context.h"

#include <endstone/actor/mob.h>
#include <endstone/block/block.h>
#include <endstone/block/block_face.h>
#include <endstone/event/actor/actor_damage_event.h>
#include <endstone/event/actor/actor_death_event.h>
#include <endstone/event/actor/actor_explode_event.h>
#include <endstone/event/actor/actor_knockback_event.h>
#include <endstone/event/actor/actor_remove_event.h>
#include <endstone/event/actor/actor_spawn_event.h>
#include <endstone/event/actor/actor_teleport_event.h>
#include <endstone/event/actor/player_death_event.h>
#include <endstone/event/block/block_break_event.h>
#include <endstone/event/block/block_cook_event.h>
#include <endstone/event/block/block_explode_event.h>
#include <endstone/event/block/block_form_event.h>
#include <endstone/event/block/block_from_to_event.h>
#include <endstone/event/block/block_grow_event.h>
#include <endstone/event/block/block_piston_event.h>
#include <endstone/event/block/block_piston_extend_event.h>
#include <endstone/event/block/block_piston_retract_event.h>
#include <endstone/event/block/block_place_event.h>
#include <endstone/event/block/leaves_decay_event.h>
#include <endstone/event/chunk/chunk_event.h>
#include <endstone/event/chunk/chunk_load_event.h>
#include <endstone/event/chunk/chunk_unload_event.h>
#include <endstone/event/event_priority.h>
#include <endstone/event/player/player_bed_enter_event.h>
#include <endstone/event/player/player_bed_leave_event.h>
#include <endstone/event/player/player_chat_event.h>
#include <endstone/event/player/player_command_event.h>
#include <endstone/event/player/player_dimension_change_event.h>
#include <endstone/event/player/player_drop_item_event.h>
#include <endstone/event/player/player_emote_event.h>
#include <endstone/event/player/player_game_mode_change_event.h>
#include <endstone/event/player/player_interact_actor_event.h>
#include <endstone/event/player/player_interact_event.h>
#include <endstone/event/player/player_item_consume_event.h>
#include <endstone/event/player/player_item_held_event.h>
#include <endstone/event/player/player_join_event.h>
#include <endstone/event/player/player_jump_event.h>
#include <endstone/event/player/player_kick_event.h>
#include <endstone/event/player/player_login_event.h>
#include <endstone/event/player/player_move_event.h>
#include <endstone/event/player/player_pickup_item_event.h>
#include <endstone/event/player/player_portal_event.h>
#include <endstone/event/player/player_quit_event.h>
#include <endstone/event/player/player_respawn_event.h>
#include <endstone/event/player/player_skin_change_event.h>
#include <endstone/event/player/player_teleport_event.h>
#include <endstone/event/server/broadcast_message_event.h>
#include <endstone/event/server/map_initialize_event.h>
#include <endstone/event/server/packet_receive_event.h>
#include <endstone/event/server/packet_send_event.h>
#include <endstone/event/server/plugin_disable_event.h>
#include <endstone/event/server/plugin_enable_event.h>
#include <endstone/event/server/script_message_event.h>
#include <endstone/event/server/server_command_event.h>
#include <endstone/event/server/server_list_ping_event.h>
#include <endstone/event/server/server_load_event.h>
#include <endstone/event/weather/thunder_change_event.h>
#include <endstone/event/weather/weather_change_event.h>
#include <endstone/game_mode.h>
#include <endstone/level/chunk.h>
#include <endstone/level/dimension.h>
#include <endstone/level/level.h>
#include <endstone/message.h>
#include <endstone/plugin/plugin.h>
#include <endstone/server.h>
#include <endstone/skin.h>
#include <endstone/util/image.h>
#include <endstone/lang/translatable.h>

#include <cstdint>
#include <functional>
#include <memory>
#include <string>
#include <type_traits>
#include <variant>
#include <vector>

namespace {

[[nodiscard]] std::string message_text(const endstone::Message &message) noexcept
{
    if (const auto *text = std::get_if<std::string>(&message)) {
        return *text;
    }
    if (const auto *translatable = std::get_if<endstone::Translatable>(&message)) {
        return translatable->getText();
    }
    return {};
}

[[nodiscard]] std::unique_ptr<aegilex::native::inventory::ItemStack> item_stack_facade(endstone::ItemStack stack)
{
    return std::make_unique<aegilex::native::inventory::ItemStack>(std::move(stack));
}

template <typename PlayerEvent>
[[nodiscard]] std::unique_ptr<aegilex::native::player::Player> event_player(PlayerEvent &event)
{
    return std::make_unique<aegilex::native::player::Player>(&event.getPlayer());
}

template <typename ActorEvent>
[[nodiscard]] std::unique_ptr<aegilex::native::actor::Actor> event_actor(ActorEvent &event)
{
    return std::make_unique<aegilex::native::actor::Actor>(&event.getActor());
}

} // namespace

namespace aegilex::native {

EventBridge::EventBridge(HostContext &context, endstone::Plugin &plugin, Runtime *runtime)
    : context_(context), plugin_(plugin), runtime_(runtime)
{
}

EventBridge::~EventBridge()
{
    unregister_all();
}

void EventBridge::register_listeners()
{
    if (registered_) {
        return;
    }
    plugin_.registerEvent<endstone::PlayerJoinEvent>([this](endstone::PlayerJoinEvent &event) { handle_join(event); },
                                                     endstone::EventPriority::Normal, false);
    plugin_.registerEvent<endstone::PlayerQuitEvent>([this](endstone::PlayerQuitEvent &event) { handle_quit(event); },
                                                     endstone::EventPriority::Normal, false);
    plugin_.registerEvent<endstone::PlayerChatEvent>([this](endstone::PlayerChatEvent &event) { handle_chat(event); },
                                                     endstone::EventPriority::Normal, false);
    plugin_.registerEvent<endstone::PlayerKickEvent>([this](endstone::PlayerKickEvent &event) { handle_kick(event); },
                                                     endstone::EventPriority::Normal, false);
    plugin_.registerEvent<endstone::PlayerCommandEvent>(
        [this](endstone::PlayerCommandEvent &event) { handle_command(event); }, endstone::EventPriority::Normal, false);
    plugin_.registerEvent<endstone::PlayerGameModeChangeEvent>(
        [this](endstone::PlayerGameModeChangeEvent &event) { handle_game_mode_change(event); },
        endstone::EventPriority::Normal, false);
    plugin_.registerEvent<endstone::PlayerTeleportEvent>(
        [this](endstone::PlayerTeleportEvent &event) { handle_teleport(event); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::PlayerDropItemEvent>(
        [this](endstone::PlayerDropItemEvent &event) { handle_drop_item(event); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::BlockBreakEvent>(
        [this](endstone::BlockBreakEvent &event) { handle_block_break(event); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::BlockPlaceEvent>(
        [this](endstone::BlockPlaceEvent &event) { handle_block_place(event); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::ActorDamageEvent>(
        [this](endstone::ActorDamageEvent &event) { handle_actor_damage(event); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::ServerCommandEvent>(
        [this](endstone::ServerCommandEvent &event) { handle_server_command(event); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::PlayerBedEnterEvent>(
        [this](endstone::PlayerBedEnterEvent &event) { handle_bed_enter(event); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::PlayerBedLeaveEvent>(
        [this](endstone::PlayerBedLeaveEvent &event) { handle_bed_leave(event); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::PlayerDimensionChangeEvent>(
        [this](endstone::PlayerDimensionChangeEvent &event) { handle_dimension_change(event); },
        endstone::EventPriority::Normal, false);
    plugin_.registerEvent<endstone::PlayerEmoteEvent>(
        [this](endstone::PlayerEmoteEvent &event) { handle_emote(event); }, endstone::EventPriority::Normal, false);
    plugin_.registerEvent<endstone::PlayerInteractEvent>(
        [this](endstone::PlayerInteractEvent &event) { handle_interact(event); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::PlayerInteractActorEvent>(
        [this](endstone::PlayerInteractActorEvent &event) { handle_interact_actor(event); },
        endstone::EventPriority::Normal, false);
    plugin_.registerEvent<endstone::PlayerItemConsumeEvent>(
        [this](endstone::PlayerItemConsumeEvent &event) { handle_item_consume(event); },
        endstone::EventPriority::Normal, false);
    plugin_.registerEvent<endstone::PlayerItemHeldEvent>(
        [this](endstone::PlayerItemHeldEvent &event) { handle_item_held(event); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::PlayerJumpEvent>(
        [this](endstone::PlayerJumpEvent &event) { handle_move_like(event, "player-jump"); },
        endstone::EventPriority::Normal, false);
    plugin_.registerEvent<endstone::PlayerLoginEvent>(
        [this](endstone::PlayerLoginEvent &event) { handle_login(event); }, endstone::EventPriority::Normal, false);
    plugin_.registerEvent<endstone::PlayerMoveEvent>(
        [this](endstone::PlayerMoveEvent &event) { handle_move_like(event, "player-move"); },
        endstone::EventPriority::Normal, false);
    plugin_.registerEvent<endstone::PlayerPickupItemEvent>(
        [this](endstone::PlayerPickupItemEvent &event) { handle_pickup_item(event); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::PlayerPortalEvent>(
        [this](endstone::PlayerPortalEvent &event) { handle_move_like(event, "player-portal"); },
        endstone::EventPriority::Normal, false);
    plugin_.registerEvent<endstone::PlayerRespawnEvent>(
        [this](endstone::PlayerRespawnEvent &event) { handle_respawn(event); }, endstone::EventPriority::Normal, false);
    plugin_.registerEvent<endstone::PlayerSkinChangeEvent>(
        [this](endstone::PlayerSkinChangeEvent &event) { handle_skin_change(event); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::BlockCookEvent>(
        [this](endstone::BlockCookEvent &event) { handle_block_cook(event); }, endstone::EventPriority::Normal, false);
    plugin_.registerEvent<endstone::BlockExplodeEvent>(
        [this](endstone::BlockExplodeEvent &event) { handle_block_explode(event); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::BlockFormEvent>(
        [this](endstone::BlockFormEvent &event) { handle_grow_like(event, "block-form"); },
        endstone::EventPriority::Normal, false);
    plugin_.registerEvent<endstone::BlockFromToEvent>(
        [this](endstone::BlockFromToEvent &event) { handle_block_from_to(event); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::BlockGrowEvent>(
        [this](endstone::BlockGrowEvent &event) { handle_grow_like(event, "block-grow"); },
        endstone::EventPriority::Normal, false);
    plugin_.registerEvent<endstone::BlockPistonExtendEvent>(
        [this](endstone::BlockPistonExtendEvent &event) { handle_piston_like(event, "block-piston-extend"); },
        endstone::EventPriority::Normal, false);
    plugin_.registerEvent<endstone::BlockPistonRetractEvent>(
        [this](endstone::BlockPistonRetractEvent &event) { handle_piston_like(event, "block-piston-retract"); },
        endstone::EventPriority::Normal, false);
    plugin_.registerEvent<endstone::LeavesDecayEvent>(
        [this](endstone::LeavesDecayEvent &event) { handle_leaves_decay(event); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::ChunkLoadEvent>(
        [this](endstone::ChunkLoadEvent &event) { handle_chunk(event, "chunk-load"); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::ChunkUnloadEvent>(
        [this](endstone::ChunkUnloadEvent &event) { handle_chunk(event, "chunk-unload"); },
        endstone::EventPriority::Normal, false);
    plugin_.registerEvent<endstone::WeatherChangeEvent>(
        [this](endstone::WeatherChangeEvent &event) { handle_weather_change(event); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::ThunderChangeEvent>(
        [this](endstone::ThunderChangeEvent &event) { handle_thunder_change(event); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::ActorDeathEvent>(
        [this](endstone::ActorDeathEvent &event) { handle_actor_death(event); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::ActorExplodeEvent>(
        [this](endstone::ActorExplodeEvent &event) { handle_actor_explode(event); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::ActorKnockbackEvent>(
        [this](endstone::ActorKnockbackEvent &event) { handle_actor_knockback(event); },
        endstone::EventPriority::Normal, false);
    plugin_.registerEvent<endstone::ActorRemoveEvent>(
        [this](endstone::ActorRemoveEvent &event) { handle_actor_remove(event); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::ActorSpawnEvent>(
        [this](endstone::ActorSpawnEvent &event) { handle_actor_spawn(event); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::ActorTeleportEvent>(
        [this](endstone::ActorTeleportEvent &event) { handle_actor_teleport(event); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::PlayerDeathEvent>(
        [this](endstone::PlayerDeathEvent &event) { handle_player_death(event); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::BroadcastMessageEvent>(
        [this](endstone::BroadcastMessageEvent &event) { handle_broadcast_message(event); },
        endstone::EventPriority::Normal, false);
    plugin_.registerEvent<endstone::PacketSendEvent>(
        [this](endstone::PacketSendEvent &event) { handle_packet_send(event); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::PacketReceiveEvent>(
        [this](endstone::PacketReceiveEvent &event) { handle_packet_receive(event); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::MapInitializeEvent>(
        [this](endstone::MapInitializeEvent &event) { handle_map_initialize(event); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::ServerListPingEvent>(
        [this](endstone::ServerListPingEvent &event) { handle_server_list_ping(event); },
        endstone::EventPriority::Normal, false);
    plugin_.registerEvent<endstone::ScriptMessageEvent>(
        [this](endstone::ScriptMessageEvent &event) { handle_script_message(event); }, endstone::EventPriority::Normal,
        false);
    plugin_.registerEvent<endstone::PluginEnableEvent>(
        [this](endstone::PluginEnableEvent &event) { handle_plugin_lifecycle(event.getPlugin(), "plugin-enable"); },
        endstone::EventPriority::Normal, false);
    plugin_.registerEvent<endstone::PluginDisableEvent>(
        [this](endstone::PluginDisableEvent &event) { handle_plugin_lifecycle(event.getPlugin(), "plugin-disable"); },
        endstone::EventPriority::Normal, false);
    plugin_.registerEvent<endstone::ServerLoadEvent>(
        [this](endstone::ServerLoadEvent &event) { handle_server_load(event); }, endstone::EventPriority::Normal,
        false);
    registered_ = true;
}

void EventBridge::unregister_all() noexcept
{
    registered_ = false;
}

void EventBridge::handle_join(endstone::PlayerJoinEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "player-join")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_player_join_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::PlayerJoinEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_quit(endstone::PlayerQuitEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "player-quit")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_player_quit_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::PlayerQuitEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_chat(endstone::PlayerChatEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "player-chat")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_player_chat_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::PlayerChatEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_kick(endstone::PlayerKickEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "player-kick")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_player_kick_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::PlayerKickEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_command(endstone::PlayerCommandEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "player-command")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_player_command_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::PlayerCommandEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_game_mode_change(endstone::PlayerGameModeChangeEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "player-game-mode-change")) {
                continue;
            }
            static_cast<void>(aegilex::runtime::dispatch_player_game_mode_change_event(
                *runtime_->handle, plugin_id, context_.next_invocation_id(),
                std::make_unique<aegilex::native::endstone_binding::events::PlayerGameModeChangeEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_teleport(endstone::PlayerTeleportEvent &event) noexcept
{
    handle_move_like(event, "player-teleport");
}

template <typename MoveEvent> void EventBridge::handle_move_like(MoveEvent &event, const char *subscription) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, subscription)) {
                continue;
            }
            static_cast<void>(aegilex::runtime::dispatch_player_move_event(
                *runtime_->handle, plugin_id, context_.next_invocation_id(), subscription,
                std::make_unique<aegilex::native::endstone_binding::events::PlayerMoveEventFacade>(&event, &context_)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_drop_item(endstone::PlayerDropItemEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "player-drop-item")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_player_drop_item_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::PlayerDropItemEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_block_break(endstone::BlockBreakEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "block-break")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_block_break_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::BlockBreakEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_block_place(endstone::BlockPlaceEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "block-place")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_block_place_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::BlockPlaceEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_actor_damage(endstone::ActorDamageEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "actor-damage")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_actor_damage_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::ActorDamageEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_server_command(endstone::ServerCommandEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "server-command")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_server_command_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::ServerCommandEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_bed_enter(endstone::PlayerBedEnterEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "player-bed-enter")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_player_bed_enter_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::PlayerBedEnterEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_bed_leave(endstone::PlayerBedLeaveEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "player-bed-leave")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_player_bed_leave_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::PlayerBedLeaveEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_dimension_change(endstone::PlayerDimensionChangeEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "player-dimension-change")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_player_dimension_change_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::PlayerDimensionChangeEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_emote(endstone::PlayerEmoteEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "player-emote")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_player_emote_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::PlayerEmoteEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_interact(endstone::PlayerInteractEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "player-interact")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_player_interact_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::PlayerInteractEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_interact_actor(endstone::PlayerInteractActorEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "player-interact-actor")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_player_interact_actor_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::PlayerInteractActorEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_item_consume(endstone::PlayerItemConsumeEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "player-item-consume")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_player_item_consume_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::PlayerItemConsumeEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_item_held(endstone::PlayerItemHeldEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "player-item-held")) {
                continue;
            }
            static_cast<void>(aegilex::runtime::dispatch_player_item_held_event(
                *runtime_->handle, plugin_id, context_.next_invocation_id(),
                std::make_unique<aegilex::native::endstone_binding::events::PlayerItemHeldEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_login(endstone::PlayerLoginEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "player-login")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_player_login_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::PlayerLoginEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_pickup_item(endstone::PlayerPickupItemEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "player-pickup-item")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_player_pickup_item_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::PlayerPickupItemEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_respawn(endstone::PlayerRespawnEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "player-respawn")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_player_respawn_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::PlayerRespawnEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_skin_change(endstone::PlayerSkinChangeEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "player-skin-change")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_player_skin_change_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::PlayerSkinChangeEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_block_cook(endstone::BlockCookEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "block-cook")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_block_cook_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::BlockCookEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_block_explode(endstone::BlockExplodeEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "block-explode")) {
                continue;
            }
            static_cast<void>(aegilex::runtime::dispatch_block_explode_event(
                *runtime_->handle, plugin_id, context_.next_invocation_id(),
                std::make_unique<aegilex::native::endstone_binding::events::BlockExplodeEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

template <typename GrowEvent> void EventBridge::handle_grow_like(GrowEvent &event, const char *subscription) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (should_dispatch_event(runtime_, plugin_id, subscription)) {
                static_cast<void>(aegilex::runtime::dispatch_block_grow_event(
                    *runtime_->handle, plugin_id, context_.next_invocation_id(), subscription,
                    std::make_unique<aegilex::native::endstone_binding::events::BlockGrowEventFacade>(&event)));
            }
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_block_from_to(endstone::BlockFromToEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (should_dispatch_event(runtime_, plugin_id, "block-from-to")) {
                static_cast<void>(aegilex::runtime::dispatch_block_from_to_event(
                    *runtime_->handle, plugin_id, context_.next_invocation_id(),
                    std::make_unique<aegilex::native::endstone_binding::events::BlockFromToEventFacade>(&event)));
            }
        }
    }
    catch (...) {
    }
}

template <typename PistonEvent>
void EventBridge::handle_piston_like(PistonEvent &event, const char *subscription) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (should_dispatch_event(runtime_, plugin_id, subscription)) {
                static_cast<void>(aegilex::runtime::dispatch_block_piston_event(
                    *runtime_->handle, plugin_id, context_.next_invocation_id(), subscription,
                    std::make_unique<aegilex::native::endstone_binding::events::BlockPistonEventFacade>(&event)));
            }
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_leaves_decay(endstone::LeavesDecayEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (should_dispatch_event(runtime_, plugin_id, "leaves-decay")) {
                static_cast<void>(aegilex::runtime::dispatch_leaves_decay_event(
                    *runtime_->handle, plugin_id, context_.next_invocation_id(),
                    std::make_unique<aegilex::native::endstone_binding::events::LeavesDecayEventFacade>(&event)));
            }
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_chunk(endstone::ChunkEvent &event, const char *subscription) noexcept
{
    try {
        HostContext &context = context_;
        if (runtime_ == nullptr || context.server.native() == nullptr || !context.accepting_calls ||
            !context.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, subscription)) {
                continue;
            }
            static_cast<void>(aegilex::runtime::dispatch_chunk_event(
                *runtime_->handle, plugin_id, context.next_invocation_id(), subscription,
                std::make_unique<aegilex::native::endstone_binding::events::ChunkEventFacade>(event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_weather_change(endstone::WeatherChangeEvent &event) noexcept
{
    try {
        HostContext &context = context_;
        if (runtime_ == nullptr || context.server.native() == nullptr || !context.accepting_calls ||
            !context.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "weather-change")) {
                continue;
            }
            static_cast<void>(aegilex::runtime::dispatch_weather_change_event(
                *runtime_->handle, plugin_id, context.next_invocation_id(),
                std::make_unique<aegilex::native::endstone_binding::events::WeatherChangeEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_thunder_change(endstone::ThunderChangeEvent &event) noexcept
{
    try {
        HostContext &context = context_;
        if (runtime_ == nullptr || context.server.native() == nullptr || !context.accepting_calls ||
            !context.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "thunder-change")) {
                continue;
            }
            static_cast<void>(aegilex::runtime::dispatch_thunder_change_event(
                *runtime_->handle, plugin_id, context.next_invocation_id(),
                std::make_unique<aegilex::native::endstone_binding::events::ThunderChangeEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_actor_death(endstone::ActorDeathEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "actor-death")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_actor_death_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::ActorDeathEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_actor_explode(endstone::ActorExplodeEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "actor-explode")) {
                continue;
            }
            static_cast<void>(aegilex::runtime::dispatch_actor_explode_event(
                *runtime_->handle, plugin_id, context_.next_invocation_id(),
                std::make_unique<aegilex::native::endstone_binding::events::ActorExplodeEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_actor_knockback(endstone::ActorKnockbackEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "actor-knockback")) {
                continue;
            }
            static_cast<void>(aegilex::runtime::dispatch_actor_knockback_event(
                *runtime_->handle, plugin_id, context_.next_invocation_id(),
                std::make_unique<aegilex::native::endstone_binding::events::ActorKnockbackEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_actor_remove(endstone::ActorRemoveEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "actor-remove")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_actor_remove_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::ActorRemoveEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_actor_spawn(endstone::ActorSpawnEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "actor-spawn")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_actor_spawn_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::ActorSpawnEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_actor_teleport(endstone::ActorTeleportEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "actor-teleport")) {
                continue;
            }
            static_cast<void>(aegilex::runtime::dispatch_actor_teleport_event(
                *runtime_->handle, plugin_id, context_.next_invocation_id(),
                std::make_unique<aegilex::native::endstone_binding::events::ActorTeleportEventFacade>(&event,
                                                                                                      &context_)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_player_death(endstone::PlayerDeathEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "player-death")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_player_death_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::PlayerDeathEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_broadcast_message(endstone::BroadcastMessageEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "broadcast-message")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_broadcast_message_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::BroadcastMessageEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

template <typename EndstoneEvent, typename Facade, typename Dispatch>
void EventBridge::handle_packet_like(EndstoneEvent &event, const char *subscription, Dispatch &&dispatch) noexcept
{
    if (dispatching_packet_event_) {
        return;
    }
    dispatching_packet_event_ = true;
    struct ResetPacketDispatch {
        bool &dispatching;
        ~ResetPacketDispatch()
        {
            dispatching = false;
        }
    } reset{dispatching_packet_event_};

    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, subscription)) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(dispatch(*runtime_->handle, plugin_id, invocation_id, std::make_unique<Facade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_packet_send(endstone::PacketSendEvent &event) noexcept
{
    handle_packet_like<endstone::PacketSendEvent, aegilex::native::endstone_binding::events::PacketSendEventFacade>(
        event, "packet-send", aegilex::runtime::dispatch_packet_send_event);
}

void EventBridge::handle_packet_receive(endstone::PacketReceiveEvent &event) noexcept
{
    handle_packet_like<endstone::PacketReceiveEvent,
                       aegilex::native::endstone_binding::events::PacketReceiveEventFacade>(
        event, "packet-receive", aegilex::runtime::dispatch_packet_receive_event);
}

void EventBridge::handle_map_initialize(endstone::MapInitializeEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "map-initialize")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_map_initialize_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::MapInitializeEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_script_message(endstone::ScriptMessageEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "script-message")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_script_message_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::ScriptMessageEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_server_list_ping(endstone::ServerListPingEvent &event) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context_.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "server-list-ping")) {
                continue;
            }
            const auto invocation_id = context_.next_invocation_id();
            static_cast<void>(aegilex::runtime::dispatch_server_list_ping_event(
                *runtime_->handle, plugin_id, invocation_id,
                std::make_unique<aegilex::native::endstone_binding::events::ServerListPingEventFacade>(&event)));
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_plugin_lifecycle(endstone::Plugin &plugin, const char *subscription) noexcept
{
    try {
        HostContext &context = context_;
        if (runtime_ == nullptr || context.server.native() == nullptr || !context.accepting_calls ||
            !context.server.native()->isPrimaryThread()) {
            return;
        }
        for (const auto &plugin_id : context.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, subscription)) {
                continue;
            }
            const auto invocation_id = context.next_invocation_id();
            if (std::string_view(subscription) == "plugin-enable") {
                static_cast<void>(aegilex::runtime::dispatch_plugin_enable_event(
                    *runtime_->handle, plugin_id, invocation_id,
                    std::make_unique<aegilex::native::endstone_binding::events::PluginLifecycleEventFacade>(
                        plugin.getName())));
            }
            else {
                static_cast<void>(aegilex::runtime::dispatch_plugin_disable_event(
                    *runtime_->handle, plugin_id, invocation_id,
                    std::make_unique<aegilex::native::endstone_binding::events::PluginLifecycleEventFacade>(
                        plugin.getName())));
            }
        }
    }
    catch (...) {
    }
}

void EventBridge::handle_server_load(endstone::ServerLoadEvent &event) noexcept
{
    try {
        HostContext &context = context_;
        if (runtime_ == nullptr || context.server.native() == nullptr || !context.accepting_calls ||
            !context.server.native()->isPrimaryThread()) {
            return;
        }
        const auto load_type = static_cast<std::uint8_t>(event.getType());
        for (const auto &plugin_id : context.enabled_plugin_ids()) {
            if (!should_dispatch_event(runtime_, plugin_id, "server-load")) {
                continue;
            }
            static_cast<void>(aegilex::runtime::dispatch_server_load_event(
                *runtime_->handle, plugin_id, context.next_invocation_id(),
                std::make_unique<aegilex::native::endstone_binding::events::ServerLoadEventFacade>(load_type)));
        }
    }
    catch (...) {
    }
}

} // namespace aegilex::native
