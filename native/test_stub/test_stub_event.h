#pragma once

#include <memory>

namespace aegilex::native::endstone_binding::events {

class ServerListPingEventFacade;
class ActorDamageEventFacade;
class ActorDeathEventFacade;
class ActorExplodeEventFacade;
class BlockExplodeEventFacade;
class ActorKnockbackEventFacade;
class ActorRemoveEventFacade;
class ActorSpawnEventFacade;
class ActorTeleportEventFacade;
class PlayerDeathEventFacade;
class PlayerJoinEventFacade;
class PlayerQuitEventFacade;
class PlayerDropItemEventFacade;
class BlockBreakEventFacade;
class BlockCookEventFacade;
class BlockFromToEventFacade;
class BlockGrowEventFacade;
class BlockPistonEventFacade;
class BlockPlaceEventFacade;
class LeavesDecayEventFacade;
class PlayerInteractEventFacade;
class PlayerInteractActorEventFacade;
class PlayerItemConsumeEventFacade;
class PlayerGameModeChangeEventFacade;
class PlayerEmoteEventFacade;
class PlayerSkinChangeEventFacade;
class PlayerDimensionChangeEventFacade;
class PlayerBedEnterEventFacade;
class PlayerBedLeaveEventFacade;
class PlayerRespawnEventFacade;
class PlayerItemHeldEventFacade;
class PlayerPickupItemEventFacade;
class PlayerMoveEventFacade;
class PluginLifecycleEventFacade;
class ServerLoadEventFacade;
class ChunkEventFacade;

std::unique_ptr<ServerListPingEventFacade> make_test_server_list_ping_event_facade() noexcept;

std::unique_ptr<PlayerDropItemEventFacade> make_test_player_drop_item_event_facade() noexcept;

std::unique_ptr<ActorDamageEventFacade> make_test_actor_damage_event_facade() noexcept;

std::unique_ptr<ActorKnockbackEventFacade> make_test_actor_knockback_event_facade(bool has_source) noexcept;

std::unique_ptr<ActorDeathEventFacade> make_test_actor_death_event_facade() noexcept;

std::unique_ptr<ActorExplodeEventFacade> make_test_actor_explode_event_facade() noexcept;

std::unique_ptr<BlockExplodeEventFacade> make_test_block_explode_event_facade() noexcept;

std::unique_ptr<ActorRemoveEventFacade> make_test_actor_remove_event_facade() noexcept;

std::unique_ptr<ActorSpawnEventFacade> make_test_actor_spawn_event_facade() noexcept;

std::unique_ptr<ActorTeleportEventFacade> make_test_actor_teleport_event_facade() noexcept;

std::unique_ptr<PlayerDeathEventFacade> make_test_player_death_event_facade() noexcept;

std::unique_ptr<PlayerJoinEventFacade> make_test_player_join_event_facade() noexcept;

std::unique_ptr<PlayerQuitEventFacade> make_test_player_quit_event_facade() noexcept;

std::unique_ptr<BlockBreakEventFacade> make_test_block_break_event_facade() noexcept;

std::unique_ptr<BlockCookEventFacade> make_test_block_cook_event_facade() noexcept;

std::unique_ptr<LeavesDecayEventFacade> make_test_leaves_decay_event_facade() noexcept;

std::unique_ptr<BlockFromToEventFacade> make_test_block_from_to_event_facade() noexcept;

std::unique_ptr<BlockGrowEventFacade> make_test_block_grow_event_facade() noexcept;

std::unique_ptr<BlockPistonEventFacade> make_test_block_piston_event_facade() noexcept;

std::unique_ptr<BlockPlaceEventFacade> make_test_block_place_event_facade() noexcept;

std::unique_ptr<PlayerInteractEventFacade> make_test_player_interact_event_facade() noexcept;

std::unique_ptr<PlayerInteractActorEventFacade> make_test_player_interact_actor_event_facade() noexcept;

std::unique_ptr<PlayerItemConsumeEventFacade> make_test_player_item_consume_event_facade() noexcept;

std::unique_ptr<PlayerGameModeChangeEventFacade> make_test_player_game_mode_change_event_facade() noexcept;

std::unique_ptr<PlayerEmoteEventFacade> make_test_player_emote_event_facade() noexcept;

std::unique_ptr<PlayerSkinChangeEventFacade> make_test_player_skin_change_event_facade() noexcept;

std::unique_ptr<PlayerDimensionChangeEventFacade> make_test_player_dimension_change_event_facade() noexcept;

std::unique_ptr<PlayerBedEnterEventFacade> make_test_player_bed_enter_event_facade() noexcept;

std::unique_ptr<PlayerBedLeaveEventFacade> make_test_player_bed_leave_event_facade() noexcept;

std::unique_ptr<PlayerRespawnEventFacade> make_test_player_respawn_event_facade() noexcept;

std::unique_ptr<PlayerItemHeldEventFacade> make_test_player_item_held_event_facade() noexcept;

std::unique_ptr<PlayerPickupItemEventFacade> make_test_player_pickup_item_event_facade() noexcept;

std::unique_ptr<PlayerMoveEventFacade> make_test_player_move_event_facade() noexcept;

std::unique_ptr<PluginLifecycleEventFacade> make_test_plugin_lifecycle_event_facade() noexcept;

std::unique_ptr<ServerLoadEventFacade> make_test_server_load_event_facade() noexcept;

std::unique_ptr<ChunkEventFacade> make_test_chunk_event_facade() noexcept;

} // namespace aegilex::native::endstone_binding::events
