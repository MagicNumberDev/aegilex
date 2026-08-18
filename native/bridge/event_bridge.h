#pragma once

#include "../runtime_bridge.h"

#include <cstdint>
#include <string>

namespace endstone {
class Plugin;
class PlayerJoinEvent;
class PlayerQuitEvent;
class PlayerChatEvent;
class PlayerKickEvent;
class PlayerCommandEvent;
class PlayerGameModeChangeEvent;
class PlayerTeleportEvent;
class PlayerDropItemEvent;
class BlockBreakEvent;
class BlockPlaceEvent;
class ActorDamageEvent;
class ServerCommandEvent;
class PlayerBedEnterEvent;
class PlayerBedLeaveEvent;
class PlayerDimensionChangeEvent;
class PlayerEmoteEvent;
class PlayerInteractEvent;
class PlayerInteractActorEvent;
class PlayerItemConsumeEvent;
class PlayerItemHeldEvent;
class PlayerJumpEvent;
class PlayerLoginEvent;
class PlayerMoveEvent;
class PlayerPickupItemEvent;
class PlayerPortalEvent;
class PlayerRespawnEvent;
class PlayerSkinChangeEvent;
class PlayerDeathEvent;
class BlockCookEvent;
class BlockExplodeEvent;
class BlockFormEvent;
class BlockFromToEvent;
class BlockGrowEvent;
class BlockPistonExtendEvent;
class BlockPistonRetractEvent;
class LeavesDecayEvent;
class ChunkEvent;
class ChunkLoadEvent;
class ChunkUnloadEvent;
class WeatherChangeEvent;
class ThunderChangeEvent;
class ActorDeathEvent;
class ActorExplodeEvent;
class ActorKnockbackEvent;
class ActorRemoveEvent;
class ActorSpawnEvent;
class ActorTeleportEvent;
class BroadcastMessageEvent;
class ServerListPingEvent;
class ServerLoadEvent;
class PacketSendEvent;
class PacketReceiveEvent;
class MapInitializeEvent;
class ScriptMessageEvent;
} // namespace endstone

namespace aegilex::native {

class HostContext;

// Dispatches Endstone events through callback-scoped typed facades, one guest
// invocation per enabled plugin on the primary thread. Facade setters apply
// permitted mutations to the live native event before the callback returns.
class EventBridge {
  public:
    EventBridge(HostContext &context, endstone::Plugin &plugin, Runtime *runtime);
    ~EventBridge();

    EventBridge(const EventBridge &) = delete;
    EventBridge &operator=(const EventBridge &) = delete;

    void register_listeners();
    void unregister_all() noexcept;

  private:
    template <typename MoveEvent> void handle_move_like(MoveEvent &event, const char *subscription) noexcept;
    template <typename GrowEvent> void handle_grow_like(GrowEvent &event, const char *subscription) noexcept;
    template <typename PistonEvent> void handle_piston_like(PistonEvent &event, const char *subscription) noexcept;
    template <typename EndstoneEvent, typename Facade, typename Dispatch>
    void handle_packet_like(EndstoneEvent &event, const char *subscription, Dispatch &&dispatch) noexcept;
    void handle_join(endstone::PlayerJoinEvent &event) noexcept;
    void handle_quit(endstone::PlayerQuitEvent &event) noexcept;
    void handle_chat(endstone::PlayerChatEvent &event) noexcept;
    void handle_kick(endstone::PlayerKickEvent &event) noexcept;
    void handle_command(endstone::PlayerCommandEvent &event) noexcept;
    void handle_game_mode_change(endstone::PlayerGameModeChangeEvent &event) noexcept;
    void handle_teleport(endstone::PlayerTeleportEvent &event) noexcept;
    void handle_drop_item(endstone::PlayerDropItemEvent &event) noexcept;
    void handle_block_break(endstone::BlockBreakEvent &event) noexcept;
    void handle_block_place(endstone::BlockPlaceEvent &event) noexcept;
    void handle_actor_damage(endstone::ActorDamageEvent &event) noexcept;
    void handle_server_command(endstone::ServerCommandEvent &event) noexcept;
    void handle_bed_enter(endstone::PlayerBedEnterEvent &event) noexcept;
    void handle_bed_leave(endstone::PlayerBedLeaveEvent &event) noexcept;
    void handle_dimension_change(endstone::PlayerDimensionChangeEvent &event) noexcept;
    void handle_emote(endstone::PlayerEmoteEvent &event) noexcept;
    void handle_interact(endstone::PlayerInteractEvent &event) noexcept;
    void handle_interact_actor(endstone::PlayerInteractActorEvent &event) noexcept;
    void handle_item_consume(endstone::PlayerItemConsumeEvent &event) noexcept;
    void handle_item_held(endstone::PlayerItemHeldEvent &event) noexcept;
    void handle_login(endstone::PlayerLoginEvent &event) noexcept;
    void handle_pickup_item(endstone::PlayerPickupItemEvent &event) noexcept;
    void handle_respawn(endstone::PlayerRespawnEvent &event) noexcept;
    void handle_skin_change(endstone::PlayerSkinChangeEvent &event) noexcept;
    void handle_block_cook(endstone::BlockCookEvent &event) noexcept;
    void handle_block_explode(endstone::BlockExplodeEvent &event) noexcept;
    void handle_block_from_to(endstone::BlockFromToEvent &event) noexcept;
    void handle_leaves_decay(endstone::LeavesDecayEvent &event) noexcept;
    void handle_chunk(endstone::ChunkEvent &event, const char *subscription) noexcept;
    void handle_weather_change(endstone::WeatherChangeEvent &event) noexcept;
    void handle_thunder_change(endstone::ThunderChangeEvent &event) noexcept;
    void handle_actor_death(endstone::ActorDeathEvent &event) noexcept;
    void handle_actor_explode(endstone::ActorExplodeEvent &event) noexcept;
    void handle_actor_knockback(endstone::ActorKnockbackEvent &event) noexcept;
    void handle_actor_remove(endstone::ActorRemoveEvent &event) noexcept;
    void handle_actor_spawn(endstone::ActorSpawnEvent &event) noexcept;
    void handle_actor_teleport(endstone::ActorTeleportEvent &event) noexcept;
    void handle_player_death(endstone::PlayerDeathEvent &event) noexcept;
    void handle_broadcast_message(endstone::BroadcastMessageEvent &event) noexcept;
    void handle_packet_send(endstone::PacketSendEvent &event) noexcept;
    void handle_packet_receive(endstone::PacketReceiveEvent &event) noexcept;
    void handle_map_initialize(endstone::MapInitializeEvent &event) noexcept;
    void handle_script_message(endstone::ScriptMessageEvent &event) noexcept;
    void handle_server_list_ping(endstone::ServerListPingEvent &event) noexcept;
    void handle_plugin_lifecycle(endstone::Plugin &plugin, const char *subscription) noexcept;
    void handle_server_load(endstone::ServerLoadEvent &event) noexcept;

    HostContext &context_;
    endstone::Plugin &plugin_;
    Runtime *runtime_;
    bool registered_{false};
    bool dispatching_packet_event_{false};
};

} // namespace aegilex::native
