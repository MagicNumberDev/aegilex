// Test-only player event facade stubs. Never linked into the plugin.

#include "bindings/endstone/events/player_chat_event_facade.h"
#include "bindings/endstone/events/player_join_event_facade.h"
#include "bindings/endstone/events/player_quit_event_facade.h"
#include "bindings/endstone/events/actor_damage_event_facade.h"
#include "bindings/endstone/events/script_message_event_facade.h"
#include "bindings/endstone/events/actor_death_event_facade.h"
#include "bindings/endstone/events/block_explode_event_facade.h"
#include "bindings/endstone/events/actor_explode_event_facade.h"
#include "bindings/endstone/events/actor_knockback_event_facade.h"
#include "bindings/endstone/events/actor_remove_event_facade.h"
#include "bindings/endstone/events/actor_spawn_event_facade.h"
#include "bindings/endstone/events/actor_teleport_event_facade.h"
#include "bindings/endstone/events/player_death_event_facade.h"
#include "bindings/endstone/events/player_command_event_facade.h"
#include "bindings/endstone/events/player_kick_event_facade.h"
#include "bindings/endstone/events/player_login_event_facade.h"
#include "bindings/endstone/events/player_drop_item_event_facade.h"
#include "bindings/endstone/events/block_break_event_facade.h"
#include "bindings/endstone/events/block_cook_event_facade.h"
#include "bindings/endstone/events/block_from_to_event_facade.h"
#include "bindings/endstone/events/block_grow_event_facade.h"
#include "bindings/endstone/events/block_piston_event_facade.h"
#include "bindings/endstone/events/block_place_event_facade.h"
#include "bindings/endstone/events/leaves_decay_event_facade.h"
#include "bindings/endstone/events/player_interact_event_facade.h"
#include "bindings/endstone/events/player_interact_actor_event_facade.h"
#include "bindings/endstone/events/player_item_consume_event_facade.h"
#include "bindings/endstone/events/player_game_mode_change_event_facade.h"
#include "bindings/endstone/events/player_emote_event_facade.h"
#include "bindings/endstone/events/player_skin_change_event_facade.h"
#include "bindings/endstone/events/player_dimension_change_event_facade.h"
#include "bindings/endstone/events/player_bed_enter_event_facade.h"
#include "bindings/endstone/events/player_bed_leave_event_facade.h"
#include "bindings/endstone/events/player_respawn_event_facade.h"
#include "bindings/endstone/events/player_item_held_event_facade.h"
#include "bindings/endstone/events/player_pickup_item_event_facade.h"
#include "bindings/endstone/events/player_move_event_facade.h"
#include "bindings/endstone/events/server_command_event_facade.h"
#include "bindings/endstone/events/server_list_ping_event_facade.h"
#include "bindings/endstone/events/weather_change_event_facade.h"
#include "bindings/endstone/events/thunder_change_event_facade.h"
#include "bindings/endstone/events/plugin_lifecycle_event_facade.h"
#include "bindings/endstone/events/server_load_event_facade.h"
#include "bindings/endstone/events/chunk_event_facade.h"
#include "test_stub_event.h"
#include "bindings/endstone/events/broadcast_message_event_facade.h"
#include "bindings/endstone/events/packet_send_event_facade.h"
#include "bindings/endstone/events/packet_receive_event_facade.h"
#include "bindings/endstone/events/map_initialize_event_facade.h"

#include <aegilex-runtime/src/cxx_runtime.rs.h>

#include <any>
#include <optional>
#include <string>
#include <type_traits>
#include <unordered_map>
#include <utility>
#include <vector>
namespace endstone {
class Block {};
} // namespace endstone

namespace aegilex::native::endstone_binding::events {

namespace {
bool actor_knockback_event_has_source = false;
endstone::Block event_block;
} // namespace

struct TestSlot {
    TestSlot &operator=(std::string_view value)
    {
        value_ = std::string(value);
        return *this;
    }

    template <typename T> TestSlot &operator=(T &&value)
    {
        value_ = std::forward<T>(value);
        return *this;
    }

    template <typename T> [[nodiscard]] T value_or(T fallback) const
    {
        using Value = std::remove_cvref_t<T>;
        if constexpr (std::is_same_v<Value, const char *>) {
            if (const auto *stored = std::any_cast<std::string>(&value_)) {
                return stored->c_str();
            }
        }
        else if (const auto *stored = std::any_cast<Value>(&value_)) {
            return *stored;
        }
        return fallback;
    }

  private:
    std::any value_;
};

struct TestLocation {
    std::string dimension;
    float x{};
    float y{};
    float z{};
    float pitch{};
    float yaw{};
};

struct TestVector {
    float x{};
    float y{};
    float z{};
};

struct TestEventState {
    TestSlot cancelled;
    TestSlot command;
    TestSlot damage;
    TestSlot death_message;
    TestSlot format;
    TestSlot join_message;
    TestSlot kick_message;
    TestSlot local_port;
    TestSlot local_port_v6;
    TestSlot message;
    TestSlot motd;
    TestSlot muted;
    TestSlot reason;
    TestSlot server_guid;
    TestSlot quit_message;
    std::optional<TestVector> knockback;
    std::optional<TestLocation> actor_teleport_from;
    std::optional<TestLocation> actor_teleport_to;
    std::optional<TestLocation> player_move_from;
    std::optional<TestLocation> player_move_to;
    bool has_cook_result{};
};

TestEventState &test_event_state(const void *facade)
{
    static std::unordered_map<const void *, TestEventState> states;
    return states[facade];
}

[[nodiscard]] aegilex::runtime::LocationData to_location_data(const TestLocation &location)
{
    return {.dimension = rust::String(location.dimension),
            .x = location.x,
            .y = location.y,
            .z = location.z,
            .pitch = location.pitch,
            .yaw = location.yaw};
}

// Native facades intentionally retain only their callback-scoped event pointer.
// The standalone test stub keeps its synthetic observations outside that ABI.
#define active() true
#define cancelled_ test_event_state(this).cancelled
#define command_ test_event_state(this).command
#define damage_ test_event_state(this).damage
#define death_message_ test_event_state(this).death_message
#define format_ test_event_state(this).format
#define join_message_ test_event_state(this).join_message
#define kick_message_ test_event_state(this).kick_message
#define local_port_ test_event_state(this).local_port
#define local_port_v6_ test_event_state(this).local_port_v6
#define message_ test_event_state(this).message
#define motd_ test_event_state(this).motd
#define muted_ test_event_state(this).muted
#define quit_message_ test_event_state(this).quit_message
#define reason_ test_event_state(this).reason
#define server_guid_ test_event_state(this).server_guid

BroadcastMessageEventFacade::BroadcastMessageEventFacade(endstone::BroadcastMessageEvent *) noexcept : event_(nullptr)
{
}

bool BroadcastMessageEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool BroadcastMessageEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

std::string BroadcastMessageEventFacade::getMessage() const noexcept
{
    return message_.value_or(std::string{});
}

bool BroadcastMessageEventFacade::setMessage(const std::string_view message) noexcept
{
    message_ = message;
    return true;
}

rust::String BroadcastMessageEventFacade::getMessageForRust() const noexcept
{
    return rust::String(getMessage());
}

bool BroadcastMessageEventFacade::setMessageForRust(const rust::Str message) noexcept
{
    return setMessage(std::string_view(message.data(), message.size()));
}

PacketSendEventFacade::PacketSendEventFacade(endstone::PacketSendEvent *) noexcept : event_(nullptr)
{
}

std::int32_t PacketSendEventFacade::getPacketId() const noexcept
{
    return 7;
}

rust::Vec<std::uint8_t> PacketSendEventFacade::getPayloadForRust() const noexcept
{
    rust::Vec<std::uint8_t> payload;
    payload.push_back(1);
    payload.push_back(2);
    payload.push_back(3);
    return payload;
}

bool PacketSendEventFacade::setPayloadForRust(const rust::Slice<const std::uint8_t>) noexcept
{
    return active();
}

std::unique_ptr<::aegilex::native::player::Player> PacketSendEventFacade::getPlayer() const noexcept
{
    return std::make_unique<::aegilex::native::player::Player>(nullptr);
}

aegilex::runtime::SocketAddress PacketSendEventFacade::getAddress() const noexcept
{
    return aegilex::runtime::SocketAddress{.hostname = rust::String("127.0.0.1"), .port = 19132};
}

std::uint8_t PacketSendEventFacade::getSubClientId() const noexcept
{
    return 0;
}

bool PacketSendEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool PacketSendEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

PacketReceiveEventFacade::PacketReceiveEventFacade(endstone::PacketReceiveEvent *) noexcept : event_(nullptr)
{
}

std::int32_t PacketReceiveEventFacade::getPacketId() const noexcept
{
    return 7;
}

rust::Vec<std::uint8_t> PacketReceiveEventFacade::getPayloadForRust() const noexcept
{
    rust::Vec<std::uint8_t> payload;
    payload.push_back(4);
    payload.push_back(5);
    payload.push_back(6);
    return payload;
}

bool PacketReceiveEventFacade::setPayloadForRust(const rust::Slice<const std::uint8_t>) noexcept
{
    return active();
}

std::unique_ptr<::aegilex::native::player::Player> PacketReceiveEventFacade::getPlayer() const noexcept
{
    return std::make_unique<::aegilex::native::player::Player>(nullptr);
}

aegilex::runtime::SocketAddress PacketReceiveEventFacade::getAddress() const noexcept
{
    return aegilex::runtime::SocketAddress{.hostname = rust::String("127.0.0.1"), .port = 19132};
}

std::uint8_t PacketReceiveEventFacade::getSubClientId() const noexcept
{
    return 0;
}

bool PacketReceiveEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool PacketReceiveEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

MapInitializeEventFacade::MapInitializeEventFacade(endstone::MapInitializeEvent *) noexcept : event_(nullptr)
{
}

std::int64_t MapInitializeEventFacade::getMapIdForRust() const noexcept
{
    return 7;
}

PlayerChatEventFacade::PlayerChatEventFacade(endstone::PlayerChatEvent *) noexcept : event_(nullptr)
{
}

ActorDamageEventFacade::ActorDamageEventFacade(endstone::ActorDamageEvent *) noexcept : event_(nullptr)
{
}

aegilex::runtime::DamageSourceData ActorDamageEventFacade::getDamageSource() const noexcept
{
    return aegilex::runtime::DamageSourceData{.type_id = rust::String("entity_attack"),
                                              .has_actor_id = true,
                                              .actor_id = 1,
                                              .has_damaging_actor_id = false,
                                              .damaging_actor_id = 0,
                                              .indirect = false};
}

bool PlayerChatEventFacade::setPlayer(const ::aegilex::native::player::Player &) noexcept
{
    return active();
}

std::unique_ptr<::aegilex::native::server::PlayerCollection> PlayerChatEventFacade::getRecipients() const noexcept
{
    return std::make_unique<::aegilex::native::server::PlayerCollection>(
        std::vector<std::unique_ptr<::aegilex::native::player::Player>>{});
}

rust::String PlayerChatEventFacade::getFormatForRust() const noexcept
{
    return rust::String(format_.value_or("<{0}> {1}"));
}

bool PlayerChatEventFacade::setFormatForRust(const rust::Str format) noexcept
{
    format_ = std::string(format.data(), format.size());
    return true;
}

ScriptMessageEventFacade::ScriptMessageEventFacade(endstone::ScriptMessageEvent *) noexcept : event_(nullptr)
{
}

rust::String ScriptMessageEventFacade::getMessageIdForRust() const noexcept
{
    return rust::String("aegilex:test");
}

rust::String ScriptMessageEventFacade::getMessageForRust() const noexcept
{
    return rust::String("payload");
}

std::unique_ptr<::aegilex::native::host::CommandSender> ScriptMessageEventFacade::getSender() const noexcept
{
    return std::make_unique<::aegilex::native::host::CommandSender>(nullptr);
}

bool ScriptMessageEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool ScriptMessageEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

PluginLifecycleEventFacade::PluginLifecycleEventFacade(std::string plugin_name) noexcept
    : plugin_name_(std::move(plugin_name))
{
}

rust::String PluginLifecycleEventFacade::getPluginNameForRust() const noexcept
{
    return rust::String(plugin_name_);
}

ServerLoadEventFacade::ServerLoadEventFacade(const std::uint8_t load_type) noexcept : load_type_(load_type)
{
}

std::uint8_t ServerLoadEventFacade::getLoadType() const noexcept
{
    return load_type_;
}

ChunkEventFacade::ChunkEventFacade(const endstone::ChunkEvent &) noexcept : ChunkEventFacade(3, -7, "overworld")
{
}

ChunkEventFacade::ChunkEventFacade(const std::int32_t chunk_x, const std::int32_t chunk_z,
                                   std::string dimension) noexcept
    : chunk_x_(chunk_x), chunk_z_(chunk_z), dimension_(std::move(dimension))
{
}

std::int32_t ChunkEventFacade::getChunkX() const noexcept
{
    return chunk_x_;
}

std::int32_t ChunkEventFacade::getChunkZ() const noexcept
{
    return chunk_z_;
}

rust::String ChunkEventFacade::getDimensionForRust() const noexcept
{
    return rust::String(dimension_);
}

ActorDeathEventFacade::ActorDeathEventFacade(endstone::ActorDeathEvent *) noexcept : event_(nullptr)
{
}

ActorExplodeEventFacade::ActorExplodeEventFacade() noexcept
    : event_(nullptr), location_{.dimension = "test", .x = 1.0F, .y = 2.0F, .z = 3.0F, .pitch = 4.0F, .yaw = 5.0F}
{
    blocks_.push_back(std::make_unique<::aegilex::native::level::Block>(event_block));
}

ActorExplodeEventFacade::ActorExplodeEventFacade(endstone::ActorExplodeEvent *) noexcept : ActorExplodeEventFacade()
{
}

std::unique_ptr<::aegilex::native::actor::Actor> ActorExplodeEventFacade::getActor() const noexcept
{
    return std::make_unique<::aegilex::native::actor::Actor>(nullptr);
}

aegilex::runtime::LocationData ActorExplodeEventFacade::getLocation() const noexcept
{
    return {.dimension = rust::String(location_.dimension),
            .x = location_.x,
            .y = location_.y,
            .z = location_.z,
            .pitch = location_.pitch,
            .yaw = location_.yaw};
}

std::uint64_t ActorExplodeEventFacade::getBlockCount() const noexcept
{
    return blocks_.size();
}

std::unique_ptr<::aegilex::native::level::Block> ActorExplodeEventFacade::getBlock(const std::uint64_t index) const noexcept
{
    if (index >= blocks_.size() || blocks_[index] == nullptr) {
        return nullptr;
    }
    return blocks_[index]->clone();
}

bool ActorExplodeEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool ActorExplodeEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

BlockExplodeEventFacade::BlockExplodeEventFacade() noexcept : block_(&event_block)
{
    affected_blocks_.push_back(std::make_unique<::aegilex::native::level::Block>(event_block));
}

BlockExplodeEventFacade::BlockExplodeEventFacade(endstone::BlockExplodeEvent *) noexcept : BlockExplodeEventFacade()
{
}

std::unique_ptr<::aegilex::native::level::Block> BlockExplodeEventFacade::getBlock() const noexcept
{
    return std::make_unique<::aegilex::native::level::Block>(block_ == nullptr ? event_block : *block_);
}

std::uint64_t BlockExplodeEventFacade::getBlockCount() const noexcept
{
    return affected_blocks_.size();
}

std::unique_ptr<::aegilex::native::level::Block> BlockExplodeEventFacade::getAffectedBlock(const std::uint64_t index) const noexcept
{
    if (index >= affected_blocks_.size() || affected_blocks_[index] == nullptr) {
        return nullptr;
    }
    return affected_blocks_[index]->clone();
}

bool BlockExplodeEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool BlockExplodeEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

ActorRemoveEventFacade::ActorRemoveEventFacade(endstone::ActorRemoveEvent *) noexcept : event_(nullptr)
{
}

std::unique_ptr<::aegilex::native::actor::Actor> ActorRemoveEventFacade::getActor() const noexcept
{
    return std::make_unique<::aegilex::native::actor::Actor>(nullptr);
}

ActorSpawnEventFacade::ActorSpawnEventFacade(endstone::ActorSpawnEvent *) noexcept : event_(nullptr)
{
}

std::unique_ptr<::aegilex::native::actor::Actor> ActorSpawnEventFacade::getActor() const noexcept
{
    return std::make_unique<::aegilex::native::actor::Actor>(nullptr);
}

bool ActorSpawnEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool ActorSpawnEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

std::unique_ptr<::aegilex::native::actor::Actor> ActorDeathEventFacade::getActor() const noexcept
{
    return std::make_unique<::aegilex::native::actor::Actor>(nullptr);
}

PlayerDeathEventFacade::PlayerDeathEventFacade(endstone::PlayerDeathEvent *) noexcept : event_(nullptr)
{
}

PlayerJoinEventFacade::PlayerJoinEventFacade(endstone::PlayerJoinEvent *) noexcept : event_(nullptr)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerJoinEventFacade::getPlayer() const noexcept
{
    return std::make_unique<::aegilex::native::player::Player>(nullptr);
}

bool PlayerJoinEventFacade::hasJoinMessage() const noexcept
{
    return join_message_.value_or(std::optional<std::string>{"test join"}).has_value();
}

rust::String PlayerJoinEventFacade::getJoinMessageForRust() const noexcept
{
    return rust::String(join_message_.value_or(std::optional<std::string>{"test join"}).value_or(""));
}

bool PlayerJoinEventFacade::setJoinMessageForRust(const bool has_message, const rust::Str message) noexcept
{
    join_message_ =
        has_message ? std::optional<std::string>(std::string(message.data(), message.size())) : std::nullopt;
    return true;
}

PlayerQuitEventFacade::PlayerQuitEventFacade(endstone::PlayerQuitEvent *) noexcept : event_(nullptr)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerQuitEventFacade::getPlayer() const noexcept
{
    return std::make_unique<::aegilex::native::player::Player>(nullptr);
}

bool PlayerQuitEventFacade::hasQuitMessage() const noexcept
{
    return quit_message_.value_or(std::optional<std::string>{"test quit"}).has_value();
}

rust::String PlayerQuitEventFacade::getQuitMessageForRust() const noexcept
{
    return rust::String(quit_message_.value_or(std::optional<std::string>{"test quit"}).value_or(""));
}

bool PlayerQuitEventFacade::setQuitMessageForRust(const bool has_message, const rust::Str message) noexcept
{
    quit_message_ =
        has_message ? std::optional<std::string>(std::string(message.data(), message.size())) : std::nullopt;
    return true;
}

std::unique_ptr<::aegilex::native::player::Player> PlayerDeathEventFacade::getPlayer() const noexcept
{
    return std::make_unique<::aegilex::native::player::Player>(nullptr);
}

bool PlayerDeathEventFacade::hasDeathMessage() const noexcept
{
    return death_message_.value_or(std::optional<std::string>{"test death"}).has_value();
}

rust::String PlayerDeathEventFacade::getDeathMessageForRust() const noexcept
{
    return rust::String(death_message_.value_or(std::optional<std::string>{"test death"}).value_or(""));
}

bool PlayerDeathEventFacade::setDeathMessageForRust(const bool has_message, const rust::Str message) noexcept
{
    death_message_ =
        has_message ? std::optional<std::string>(std::string(message.data(), message.size())) : std::nullopt;
    return true;
}

std::unique_ptr<::aegilex::native::actor::Actor> ActorDamageEventFacade::getActor() const noexcept
{
    return std::make_unique<::aegilex::native::actor::Actor>(nullptr);
}

float ActorDamageEventFacade::getDamage() const noexcept
{
    return damage_.value_or(2.5F);
}

bool ActorDamageEventFacade::setDamage(const float damage) noexcept
{
    damage_ = damage;
    return true;
}

bool ActorDamageEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool ActorDamageEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

ActorKnockbackEventFacade::ActorKnockbackEventFacade(endstone::ActorKnockbackEvent *) noexcept : event_(nullptr)
{
}

std::unique_ptr<::aegilex::native::actor::Actor> ActorKnockbackEventFacade::getActor() const noexcept
{
    return std::make_unique<::aegilex::native::actor::Actor>(nullptr);
}

std::unique_ptr<::aegilex::native::actor::Actor> ActorKnockbackEventFacade::getSource() const noexcept
{
    return actor_knockback_event_has_source ? std::make_unique<::aegilex::native::actor::Actor>(nullptr) : nullptr;
}

aegilex::runtime::VectorData ActorKnockbackEventFacade::getKnockback() const noexcept
{
    const auto &knockback = test_event_state(this).knockback;
    if (knockback) {
        return {.x = knockback->x, .y = knockback->y, .z = knockback->z};
    }
    return {.x = 1.0F, .y = 2.0F, .z = 3.0F};
}

bool ActorKnockbackEventFacade::setKnockback(const aegilex::runtime::VectorData &knockback) noexcept
{
    test_event_state(this).knockback = {.x = knockback.x, .y = knockback.y, .z = knockback.z};
    return true;
}

bool ActorKnockbackEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool ActorKnockbackEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

bool PlayerChatEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool PlayerChatEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

std::string PlayerChatEventFacade::getMessage() const noexcept
{
    return message_.value_or(std::string{});
}

bool PlayerChatEventFacade::setMessage(const std::string_view message) noexcept
{
    message_ = message;
    return true;
}

std::unique_ptr<::aegilex::native::player::Player> PlayerChatEventFacade::getPlayer() const noexcept
{
    return nullptr;
}

rust::String PlayerChatEventFacade::getMessageForRust() const noexcept
{
    return rust::String(getMessage());
}

bool PlayerChatEventFacade::setMessageForRust(const rust::Str message) noexcept
{
    return setMessage(std::string_view(message.data(), message.size()));
}

PlayerCommandEventFacade::PlayerCommandEventFacade(endstone::PlayerCommandEvent *) noexcept : event_(nullptr)
{
}

bool PlayerCommandEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool PlayerCommandEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

std::string PlayerCommandEventFacade::getCommand() const noexcept
{
    return command_.value_or(std::string{});
}

bool PlayerCommandEventFacade::setCommand(const std::string_view command) noexcept
{
    command_ = command;
    return true;
}

std::unique_ptr<::aegilex::native::player::Player> PlayerCommandEventFacade::getPlayer() const noexcept
{
    return nullptr;
}

rust::String PlayerCommandEventFacade::getCommandForRust() const noexcept
{
    return rust::String(getCommand());
}

bool PlayerCommandEventFacade::setCommandForRust(const rust::Str command) noexcept
{
    return setCommand(std::string_view(command.data(), command.size()));
}

PlayerKickEventFacade::PlayerKickEventFacade(endstone::PlayerKickEvent *) noexcept : event_(nullptr)
{
}

bool PlayerKickEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool PlayerKickEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

std::string PlayerKickEventFacade::getReason() const noexcept
{
    return reason_.value_or(std::string{});
}

bool PlayerKickEventFacade::setReason(const std::string_view reason) noexcept
{
    reason_ = reason;
    return true;
}

std::unique_ptr<::aegilex::native::player::Player> PlayerKickEventFacade::getPlayer() const noexcept
{
    return nullptr;
}

rust::String PlayerKickEventFacade::getReasonForRust() const noexcept
{
    return rust::String(getReason());
}

bool PlayerKickEventFacade::setReasonForRust(const rust::Str reason) noexcept
{
    return setReason(std::string_view(reason.data(), reason.size()));
}

PlayerLoginEventFacade::PlayerLoginEventFacade(endstone::PlayerLoginEvent *) noexcept : event_(nullptr)
{
}

bool PlayerLoginEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool PlayerLoginEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

std::string PlayerLoginEventFacade::getKickMessage() const noexcept
{
    return kick_message_.value_or(std::string{});
}

bool PlayerLoginEventFacade::setKickMessage(const std::string_view message) noexcept
{
    kick_message_ = message;
    return true;
}

std::unique_ptr<::aegilex::native::player::Player> PlayerLoginEventFacade::getPlayer() const noexcept
{
    return nullptr;
}

rust::String PlayerLoginEventFacade::getKickMessageForRust() const noexcept
{
    return rust::String(getKickMessage());
}

bool PlayerLoginEventFacade::setKickMessageForRust(const rust::Str message) noexcept
{
    return setKickMessage(std::string_view(message.data(), message.size()));
}

PlayerPickupItemEventFacade::PlayerPickupItemEventFacade(endstone::PlayerPickupItemEvent *) noexcept : event_(nullptr)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerPickupItemEventFacade::getPlayer() const noexcept
{
    return std::make_unique<::aegilex::native::player::Player>(nullptr);
}

std::unique_ptr<::aegilex::native::actor::Actor> PlayerPickupItemEventFacade::getItemActor() const noexcept
{
    return std::make_unique<::aegilex::native::actor::Actor>(nullptr);
}

bool PlayerPickupItemEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool PlayerPickupItemEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

PlayerDropItemEventFacade::PlayerDropItemEventFacade(endstone::PlayerDropItemEvent *) noexcept : event_(nullptr)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerDropItemEventFacade::getPlayer() const noexcept
{
    return std::make_unique<::aegilex::native::player::Player>(nullptr);
}

std::unique_ptr<::aegilex::native::inventory::ItemStackRef> PlayerDropItemEventFacade::getItem() const noexcept
{
    return std::make_unique<::aegilex::native::inventory::ItemStackRef>(nullptr);
}

bool PlayerDropItemEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool PlayerDropItemEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

BlockBreakEventFacade::BlockBreakEventFacade(endstone::BlockBreakEvent *) noexcept : event_(nullptr)
{
}

std::unique_ptr<::aegilex::native::player::Player> BlockBreakEventFacade::getPlayer() const noexcept
{
    return std::make_unique<::aegilex::native::player::Player>(nullptr);
}

std::unique_ptr<::aegilex::native::level::Block> BlockBreakEventFacade::getBlock() const noexcept
{
    return std::make_unique<::aegilex::native::level::Block>(event_block);
}

bool BlockBreakEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool BlockBreakEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

BlockCookEventFacade::BlockCookEventFacade(endstone::BlockCookEvent *) noexcept : event_(nullptr)
{
}

std::unique_ptr<::aegilex::native::level::Block> BlockCookEventFacade::getBlock() const noexcept
{
    return std::make_unique<::aegilex::native::level::Block>(event_block);
}

std::unique_ptr<::aegilex::native::inventory::ItemStackRef> BlockCookEventFacade::getSource() const noexcept
{
    return std::make_unique<::aegilex::native::inventory::ItemStackRef>(nullptr);
}

std::unique_ptr<::aegilex::native::inventory::ItemStackRef> BlockCookEventFacade::getResult() const noexcept
{
    return std::make_unique<::aegilex::native::inventory::ItemStackRef>(nullptr);
}

bool BlockCookEventFacade::setResult(const ::aegilex::native::inventory::ItemStack &) noexcept
{
    test_event_state(this).has_cook_result = true;
    return true;
}

bool BlockCookEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool BlockCookEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

LeavesDecayEventFacade::LeavesDecayEventFacade(endstone::LeavesDecayEvent *) noexcept : event_(nullptr)
{
}

std::unique_ptr<::aegilex::native::level::Block> LeavesDecayEventFacade::getBlock() const noexcept
{
    return std::make_unique<::aegilex::native::level::Block>(event_block);
}

bool LeavesDecayEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool LeavesDecayEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

BlockFromToEventFacade::BlockFromToEventFacade(endstone::BlockFromToEvent *) noexcept : event_(nullptr)
{
}

std::unique_ptr<::aegilex::native::level::Block> BlockFromToEventFacade::getBlock() const noexcept
{
    return std::make_unique<::aegilex::native::level::Block>(event_block);
}

std::unique_ptr<::aegilex::native::level::Block> BlockFromToEventFacade::getToBlock() const noexcept
{
    return std::make_unique<::aegilex::native::level::Block>(event_block);
}

bool BlockFromToEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool BlockFromToEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

BlockGrowEventFacade::BlockGrowEventFacade(endstone::BlockGrowEvent *) noexcept : event_(nullptr)
{
}

std::unique_ptr<::aegilex::native::level::Block> BlockGrowEventFacade::getBlock() const noexcept
{
    return std::make_unique<::aegilex::native::level::Block>(event_block);
}

bool BlockGrowEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool BlockGrowEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

BlockPistonEventFacade::BlockPistonEventFacade(endstone::BlockPistonEvent *) noexcept : event_(nullptr)
{
}

std::unique_ptr<::aegilex::native::level::Block> BlockPistonEventFacade::getBlock() const noexcept
{
    return std::make_unique<::aegilex::native::level::Block>(event_block);
}

std::uint8_t BlockPistonEventFacade::getDirection() const noexcept
{
    return 1;
}

bool BlockPistonEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool BlockPistonEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

BlockPlaceEventFacade::BlockPlaceEventFacade(endstone::BlockPlaceEvent *) noexcept : event_(nullptr)
{
}

std::unique_ptr<::aegilex::native::player::Player> BlockPlaceEventFacade::getPlayer() const noexcept
{
    return std::make_unique<::aegilex::native::player::Player>(nullptr);
}

std::unique_ptr<::aegilex::native::level::Block> BlockPlaceEventFacade::getBlockReplaced() const noexcept
{
    return std::make_unique<::aegilex::native::level::Block>(event_block);
}

std::unique_ptr<::aegilex::native::level::Block> BlockPlaceEventFacade::getBlockAgainst() const noexcept
{
    return std::make_unique<::aegilex::native::level::Block>(event_block);
}

bool BlockPlaceEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool BlockPlaceEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

PlayerItemConsumeEventFacade::PlayerItemConsumeEventFacade(endstone::PlayerItemConsumeEvent *) noexcept
    : event_(nullptr)
{
}

PlayerGameModeChangeEventFacade::PlayerGameModeChangeEventFacade(endstone::PlayerGameModeChangeEvent *) noexcept
    : event_(nullptr)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerGameModeChangeEventFacade::getPlayer() const noexcept
{
    return std::make_unique<::aegilex::native::player::Player>(nullptr);
}

std::uint8_t PlayerGameModeChangeEventFacade::getNewGameMode() const noexcept
{
    return 1;
}

bool PlayerGameModeChangeEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool PlayerGameModeChangeEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

PlayerEmoteEventFacade::PlayerEmoteEventFacade(endstone::PlayerEmoteEvent *) noexcept : event_(nullptr)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerEmoteEventFacade::getPlayer() const noexcept
{
    return std::make_unique<::aegilex::native::player::Player>(nullptr);
}

rust::String PlayerEmoteEventFacade::getEmoteIdForRust() const noexcept
{
    return rust::String("test-emote");
}

bool PlayerEmoteEventFacade::isMuted() const noexcept
{
    return muted_.value_or(false);
}

bool PlayerEmoteEventFacade::setMuted(const bool muted) noexcept
{
    muted_ = muted;
    return true;
}

bool PlayerEmoteEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool PlayerEmoteEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

PlayerSkinChangeEventFacade::PlayerSkinChangeEventFacade(endstone::PlayerSkinChangeEvent *) noexcept : event_(nullptr)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerSkinChangeEventFacade::getPlayer() const noexcept
{
    return std::make_unique<::aegilex::native::player::Player>(nullptr);
}

bool PlayerSkinChangeEventFacade::hasSkinChangeMessage() const noexcept
{
    return message_.value_or(std::optional<std::string>{}).has_value();
}

rust::String PlayerSkinChangeEventFacade::getSkinChangeMessageForRust() const noexcept
{
    const auto message = message_.value_or(std::optional<std::string>{});
    return message.has_value() ? rust::String(*message) : rust::String();
}

bool PlayerSkinChangeEventFacade::setSkinChangeMessageForRust(const bool has_message, const rust::Str message) noexcept
{
    try {
        message_ = has_message ? std::optional<std::string>(std::string(message.data(), message.size())) : std::nullopt;
        return true;
    }
    catch (...) {
        return false;
    }
}

bool PlayerSkinChangeEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool PlayerSkinChangeEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

PlayerDimensionChangeEventFacade::PlayerDimensionChangeEventFacade(endstone::PlayerDimensionChangeEvent *) noexcept
    : event_(nullptr)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerDimensionChangeEventFacade::getPlayer() const noexcept
{
    return std::make_unique<::aegilex::native::player::Player>(nullptr);
}

rust::String PlayerDimensionChangeEventFacade::getFromForRust() const noexcept
{
    return rust::String("overworld");
}

rust::String PlayerDimensionChangeEventFacade::getToForRust() const noexcept
{
    return rust::String("nether");
}

PlayerBedEnterEventFacade::PlayerBedEnterEventFacade(endstone::PlayerBedEnterEvent *) noexcept : event_(nullptr)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerBedEnterEventFacade::getPlayer() const noexcept
{
    return std::make_unique<::aegilex::native::player::Player>(nullptr);
}

std::unique_ptr<::aegilex::native::level::Block> PlayerBedEnterEventFacade::getBed() const noexcept
{
    return std::make_unique<::aegilex::native::level::Block>(event_block);
}

bool PlayerBedEnterEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool PlayerBedEnterEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

PlayerBedLeaveEventFacade::PlayerBedLeaveEventFacade(endstone::PlayerBedLeaveEvent *) noexcept : event_(nullptr)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerBedLeaveEventFacade::getPlayer() const noexcept
{
    return std::make_unique<::aegilex::native::player::Player>(nullptr);
}

std::unique_ptr<::aegilex::native::level::Block> PlayerBedLeaveEventFacade::getBed() const noexcept
{
    return std::make_unique<::aegilex::native::level::Block>(event_block);
}

PlayerRespawnEventFacade::PlayerRespawnEventFacade(endstone::PlayerRespawnEvent *) noexcept : event_(nullptr)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerRespawnEventFacade::getPlayer() const noexcept
{
    return std::make_unique<::aegilex::native::player::Player>(nullptr);
}

PlayerItemHeldEventFacade::PlayerItemHeldEventFacade(endstone::PlayerItemHeldEvent *) noexcept : event_(nullptr)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerItemHeldEventFacade::getPlayer() const noexcept
{
    return std::make_unique<::aegilex::native::player::Player>(nullptr);
}

std::int32_t PlayerItemHeldEventFacade::getPreviousSlot() const noexcept
{
    return 2;
}

std::int32_t PlayerItemHeldEventFacade::getNewSlot() const noexcept
{
    return 5;
}

bool PlayerItemHeldEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool PlayerItemHeldEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

PlayerInteractEventFacade::PlayerInteractEventFacade(endstone::PlayerInteractEvent *) noexcept : event_(nullptr)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerInteractEventFacade::getPlayer() const noexcept
{
    return std::make_unique<::aegilex::native::player::Player>(nullptr);
}

std::uint8_t PlayerInteractEventFacade::getAction() const noexcept
{
    return 0;
}

std::unique_ptr<::aegilex::native::inventory::ItemStackRef> PlayerInteractEventFacade::getItem() const noexcept
{
    return std::make_unique<::aegilex::native::inventory::ItemStackRef>(nullptr);
}

std::unique_ptr<::aegilex::native::level::Block> PlayerInteractEventFacade::getBlock() const noexcept
{
    static endstone::Block block;
    return std::make_unique<::aegilex::native::level::Block>(block);
}

std::uint8_t PlayerInteractEventFacade::getBlockFace() const noexcept
{
    return 0;
}

bool PlayerInteractEventFacade::hasClickedPosition() const noexcept
{
    return false;
}

aegilex::runtime::VectorData PlayerInteractEventFacade::getClickedPosition() const noexcept
{
    return {};
}

bool PlayerInteractEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool PlayerInteractEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

PlayerInteractActorEventFacade::PlayerInteractActorEventFacade(endstone::PlayerInteractActorEvent *) noexcept
    : event_(nullptr)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerInteractActorEventFacade::getPlayer() const noexcept
{
    return std::make_unique<::aegilex::native::player::Player>(nullptr);
}

std::unique_ptr<::aegilex::native::actor::Actor> PlayerInteractActorEventFacade::getActor() const noexcept
{
    return std::make_unique<::aegilex::native::actor::Actor>(nullptr);
}

bool PlayerInteractActorEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool PlayerInteractActorEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

std::unique_ptr<::aegilex::native::player::Player> PlayerItemConsumeEventFacade::getPlayer() const noexcept
{
    return std::make_unique<::aegilex::native::player::Player>(nullptr);
}

std::unique_ptr<::aegilex::native::inventory::ItemStackRef> PlayerItemConsumeEventFacade::getItem() const noexcept
{
    return std::make_unique<::aegilex::native::inventory::ItemStackRef>(nullptr);
}

std::uint8_t PlayerItemConsumeEventFacade::getHand() const noexcept
{
    return 0;
}

bool PlayerItemConsumeEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool PlayerItemConsumeEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

ServerCommandEventFacade::ServerCommandEventFacade(endstone::ServerCommandEvent *) noexcept : event_(nullptr)
{
}

bool ServerCommandEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool ServerCommandEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

std::string ServerCommandEventFacade::getCommand() const noexcept
{
    return command_.value_or(std::string{});
}

bool ServerCommandEventFacade::setCommand(const std::string_view command) noexcept
{
    command_ = command;
    return true;
}

rust::String ServerCommandEventFacade::getSenderNameForRust() const noexcept
{
    return rust::String();
}

rust::String ServerCommandEventFacade::getCommandForRust() const noexcept
{
    return rust::String(getCommand());
}

bool ServerCommandEventFacade::setCommandForRust(const rust::Str command) noexcept
{
    return setCommand(std::string_view(command.data(), command.size()));
}

ServerListPingEventFacade::ServerListPingEventFacade(endstone::ServerListPingEvent *) noexcept : event_(nullptr)
{
}

bool ServerListPingEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool ServerListPingEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

std::string ServerListPingEventFacade::getMotd() const noexcept
{
    return motd_.value_or(std::string{});
}

bool ServerListPingEventFacade::setMotd(const std::string_view motd) noexcept
{
    motd_ = motd;
    return true;
}

std::string ServerListPingEventFacade::getServerGuid() const noexcept
{
    return server_guid_.value_or(std::string{});
}

bool ServerListPingEventFacade::setServerGuid(const std::string_view guid) noexcept
{
    server_guid_ = guid;
    return true;
}

int ServerListPingEventFacade::getLocalPort() const noexcept
{
    return local_port_.value_or(0);
}

bool ServerListPingEventFacade::setLocalPort(const int port) noexcept
{
    if (port <= 0) {
        return false;
    }
    local_port_ = port;
    return true;
}

int ServerListPingEventFacade::getLocalPortV6() const noexcept
{
    return local_port_v6_.value_or(0);
}

bool ServerListPingEventFacade::setLocalPortV6(const int port) noexcept
{
    if (port <= 0) {
        return false;
    }
    local_port_v6_ = port;
    return true;
}

rust::String ServerListPingEventFacade::getMotdForRust() const noexcept
{
    return rust::String(getMotd());
}

bool ServerListPingEventFacade::setMotdForRust(const rust::Str motd) noexcept
{
    return setMotd(std::string_view(motd.data(), motd.size()));
}

rust::String ServerListPingEventFacade::getServerGuidForRust() const noexcept
{
    return rust::String(getServerGuid());
}

bool ServerListPingEventFacade::setServerGuidForRust(const rust::Str guid) noexcept
{
    return setServerGuid(std::string_view(guid.data(), guid.size()));
}

WeatherChangeEventFacade::WeatherChangeEventFacade(endstone::WeatherChangeEvent *) noexcept : event_(nullptr)
{
}

bool WeatherChangeEventFacade::getToWeather() const noexcept
{
    return false;
}

bool WeatherChangeEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool WeatherChangeEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

ThunderChangeEventFacade::ThunderChangeEventFacade(endstone::ThunderChangeEvent *) noexcept : event_(nullptr)
{
}

PlayerMoveEventFacade::PlayerMoveEventFacade(endstone::PlayerMoveEvent *, HostContext *) noexcept
    : event_(nullptr), context_(nullptr)
{
}

ActorTeleportEventFacade::ActorTeleportEventFacade(endstone::ActorTeleportEvent *, HostContext *) noexcept
    : event_(nullptr), context_(nullptr)
{
}

std::unique_ptr<::aegilex::native::actor::Actor> ActorTeleportEventFacade::getActor() const noexcept
{
    return std::make_unique<::aegilex::native::actor::Actor>(nullptr);
}

aegilex::runtime::LocationData ActorTeleportEventFacade::getFrom() const noexcept
{
    const auto &from = test_event_state(this).actor_teleport_from;
    return to_location_data(from.value_or(TestLocation{"overworld", 1.0F, 2.0F, 3.0F, 4.0F, 5.0F}));
}

aegilex::runtime::LocationData ActorTeleportEventFacade::getTo() const noexcept
{
    const auto &to = test_event_state(this).actor_teleport_to;
    return to_location_data(to.value_or(TestLocation{"overworld", 6.0F, 7.0F, 8.0F, 9.0F, 10.0F}));
}

bool ActorTeleportEventFacade::setFrom(const aegilex::runtime::LocationData &location) noexcept
{
    test_event_state(this).actor_teleport_from = {.dimension = std::string(location.dimension),
                                                  .x = location.x,
                                                  .y = location.y,
                                                  .z = location.z,
                                                  .pitch = location.pitch,
                                                  .yaw = location.yaw};
    return true;
}

bool ActorTeleportEventFacade::setTo(const aegilex::runtime::LocationData &location) noexcept
{
    test_event_state(this).actor_teleport_to = {.dimension = std::string(location.dimension),
                                                .x = location.x,
                                                .y = location.y,
                                                .z = location.z,
                                                .pitch = location.pitch,
                                                .yaw = location.yaw};
    return true;
}

bool ActorTeleportEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool ActorTeleportEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

std::unique_ptr<::aegilex::native::player::Player> PlayerMoveEventFacade::getPlayer() const noexcept
{
    return std::make_unique<::aegilex::native::player::Player>(nullptr);
}

aegilex::runtime::LocationData PlayerMoveEventFacade::getFrom() const noexcept
{
    const auto &from = test_event_state(this).player_move_from;
    return to_location_data(from.value_or(TestLocation{"overworld", 1.0F, 2.0F, 3.0F, 4.0F, 5.0F}));
}

aegilex::runtime::LocationData PlayerMoveEventFacade::getTo() const noexcept
{
    const auto &to = test_event_state(this).player_move_to;
    return to_location_data(to.value_or(TestLocation{"overworld", 6.0F, 7.0F, 8.0F, 9.0F, 10.0F}));
}

bool PlayerMoveEventFacade::setFrom(const aegilex::runtime::LocationData &location) noexcept
{
    test_event_state(this).player_move_from = {.dimension = std::string(location.dimension),
                                               .x = location.x,
                                               .y = location.y,
                                               .z = location.z,
                                               .pitch = location.pitch,
                                               .yaw = location.yaw};
    return true;
}

bool PlayerMoveEventFacade::setTo(const aegilex::runtime::LocationData &location) noexcept
{
    test_event_state(this).player_move_to = {.dimension = std::string(location.dimension),
                                             .x = location.x,
                                             .y = location.y,
                                             .z = location.z,
                                             .pitch = location.pitch,
                                             .yaw = location.yaw};
    return true;
}

bool PlayerMoveEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool PlayerMoveEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

static ThunderChangeEventFacade *make_test_thunder_change_event_facade() noexcept
{
    return std::make_unique<ThunderChangeEventFacade>(nullptr).release();
}

bool ThunderChangeEventFacade::getToThunder() const noexcept
{
    return false;
}

bool ThunderChangeEventFacade::isCancelled() const noexcept
{
    return cancelled_.value_or(false);
}

bool ThunderChangeEventFacade::setCancelled(const bool cancelled) noexcept
{
    cancelled_ = cancelled;
    return true;
}

static WeatherChangeEventFacade *make_test_weather_change_event_facade() noexcept
{
    return std::make_unique<WeatherChangeEventFacade>(nullptr).release();
}

std::unique_ptr<ServerListPingEventFacade> make_test_server_list_ping_event_facade() noexcept
{
    return std::make_unique<ServerListPingEventFacade>(nullptr);
}

std::unique_ptr<PlayerDropItemEventFacade> make_test_player_drop_item_event_facade() noexcept
{
    return std::make_unique<PlayerDropItemEventFacade>(nullptr);
}

std::unique_ptr<ActorDamageEventFacade> make_test_actor_damage_event_facade() noexcept
{
    return std::make_unique<ActorDamageEventFacade>(nullptr);
}

std::unique_ptr<ActorKnockbackEventFacade> make_test_actor_knockback_event_facade(const bool has_source) noexcept
{
    actor_knockback_event_has_source = has_source;
    return std::make_unique<ActorKnockbackEventFacade>(nullptr);
}

std::unique_ptr<ActorDeathEventFacade> make_test_actor_death_event_facade() noexcept
{
    return std::make_unique<ActorDeathEventFacade>(nullptr);
}

std::unique_ptr<ActorRemoveEventFacade> make_test_actor_remove_event_facade() noexcept
{
    return std::make_unique<ActorRemoveEventFacade>(nullptr);
}

std::unique_ptr<ActorSpawnEventFacade> make_test_actor_spawn_event_facade() noexcept
{
    return std::make_unique<ActorSpawnEventFacade>(nullptr);
}

std::unique_ptr<PlayerDeathEventFacade> make_test_player_death_event_facade() noexcept
{
    return std::make_unique<PlayerDeathEventFacade>(nullptr);
}

std::unique_ptr<PlayerJoinEventFacade> make_test_player_join_event_facade() noexcept
{
    return std::make_unique<PlayerJoinEventFacade>(nullptr);
}

std::unique_ptr<PlayerQuitEventFacade> make_test_player_quit_event_facade() noexcept
{
    return std::make_unique<PlayerQuitEventFacade>(nullptr);
}

std::unique_ptr<ActorExplodeEventFacade> make_test_actor_explode_event_facade() noexcept
{
    return std::make_unique<ActorExplodeEventFacade>();
}

std::unique_ptr<BlockExplodeEventFacade> make_test_block_explode_event_facade() noexcept
{
    return std::make_unique<BlockExplodeEventFacade>();
}

std::unique_ptr<PlayerInteractEventFacade> make_test_player_interact_event_facade() noexcept
{
    return std::make_unique<PlayerInteractEventFacade>(nullptr);
}

std::unique_ptr<PlayerInteractActorEventFacade> make_test_player_interact_actor_event_facade() noexcept
{
    return std::make_unique<PlayerInteractActorEventFacade>(nullptr);
}

std::unique_ptr<PlayerItemConsumeEventFacade> make_test_player_item_consume_event_facade() noexcept
{
    return std::make_unique<PlayerItemConsumeEventFacade>(nullptr);
}

std::unique_ptr<PlayerGameModeChangeEventFacade> make_test_player_game_mode_change_event_facade() noexcept
{
    return std::make_unique<PlayerGameModeChangeEventFacade>(nullptr);
}

std::unique_ptr<PlayerEmoteEventFacade> make_test_player_emote_event_facade() noexcept
{
    return std::make_unique<PlayerEmoteEventFacade>(nullptr);
}

std::unique_ptr<PlayerSkinChangeEventFacade> make_test_player_skin_change_event_facade() noexcept
{
    return std::make_unique<PlayerSkinChangeEventFacade>(nullptr);
}

std::unique_ptr<PlayerDimensionChangeEventFacade> make_test_player_dimension_change_event_facade() noexcept
{
    return std::make_unique<PlayerDimensionChangeEventFacade>(nullptr);
}

std::unique_ptr<PlayerBedEnterEventFacade> make_test_player_bed_enter_event_facade() noexcept
{
    auto facade = std::make_unique<PlayerBedEnterEventFacade>(nullptr);
    test_event_state(facade.get()) = TestEventState{};
    return facade;
}

std::unique_ptr<PlayerBedLeaveEventFacade> make_test_player_bed_leave_event_facade() noexcept
{
    return std::make_unique<PlayerBedLeaveEventFacade>(nullptr);
}

std::unique_ptr<PlayerRespawnEventFacade> make_test_player_respawn_event_facade() noexcept
{
    return std::make_unique<PlayerRespawnEventFacade>(nullptr);
}

std::unique_ptr<PlayerItemHeldEventFacade> make_test_player_item_held_event_facade() noexcept
{
    return std::make_unique<PlayerItemHeldEventFacade>(nullptr);
}

std::unique_ptr<PlayerPickupItemEventFacade> make_test_player_pickup_item_event_facade() noexcept
{
    return std::make_unique<PlayerPickupItemEventFacade>(nullptr);
}

std::unique_ptr<PlayerMoveEventFacade> make_test_player_move_event_facade() noexcept
{
    return std::make_unique<PlayerMoveEventFacade>(nullptr, nullptr);
}

std::unique_ptr<ActorTeleportEventFacade> make_test_actor_teleport_event_facade() noexcept
{
    return std::make_unique<ActorTeleportEventFacade>(nullptr, nullptr);
}

std::unique_ptr<BlockBreakEventFacade> make_test_block_break_event_facade() noexcept
{
    return std::make_unique<BlockBreakEventFacade>(nullptr);
}

std::unique_ptr<BlockCookEventFacade> make_test_block_cook_event_facade() noexcept
{
    return std::make_unique<BlockCookEventFacade>(nullptr);
}

std::unique_ptr<LeavesDecayEventFacade> make_test_leaves_decay_event_facade() noexcept
{
    return std::make_unique<LeavesDecayEventFacade>(nullptr);
}

std::unique_ptr<BlockFromToEventFacade> make_test_block_from_to_event_facade() noexcept
{
    return std::make_unique<BlockFromToEventFacade>(nullptr);
}

std::unique_ptr<BlockGrowEventFacade> make_test_block_grow_event_facade() noexcept
{
    return std::make_unique<BlockGrowEventFacade>(nullptr);
}

std::unique_ptr<BlockPistonEventFacade> make_test_block_piston_event_facade() noexcept
{
    return std::make_unique<BlockPistonEventFacade>(nullptr);
}

std::unique_ptr<BlockPlaceEventFacade> make_test_block_place_event_facade() noexcept
{
    return std::make_unique<BlockPlaceEventFacade>(nullptr);
}

std::unique_ptr<PluginLifecycleEventFacade> make_test_plugin_lifecycle_event_facade() noexcept
{
    return std::make_unique<PluginLifecycleEventFacade>("example-plugin");
}

std::unique_ptr<ServerLoadEventFacade> make_test_server_load_event_facade() noexcept
{
    return std::make_unique<ServerLoadEventFacade>(1);
}

std::unique_ptr<ChunkEventFacade> make_test_chunk_event_facade() noexcept
{
    return std::make_unique<ChunkEventFacade>(3, -7, "overworld");
}

} // namespace aegilex::native::endstone_binding::events

extern "C" {

aegilex::native::endstone_binding::events::PluginLifecycleEventFacade *
aegilex_test_make_plugin_lifecycle_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_plugin_lifecycle_event_facade().release();
}

aegilex::native::endstone_binding::events::ServerLoadEventFacade *aegilex_test_make_server_load_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_server_load_event_facade().release();
}

aegilex::native::endstone_binding::events::ChunkEventFacade *aegilex_test_make_chunk_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_chunk_event_facade().release();
}

aegilex::native::endstone_binding::events::ActorDeathEventFacade *aegilex_test_make_actor_death_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_actor_death_event_facade().release();
}

aegilex::native::endstone_binding::events::ActorRemoveEventFacade *
aegilex_test_make_actor_remove_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_actor_remove_event_facade().release();
}

aegilex::native::endstone_binding::events::ActorSpawnEventFacade *aegilex_test_make_actor_spawn_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_actor_spawn_event_facade().release();
}

aegilex::native::endstone_binding::events::ActorTeleportEventFacade *
aegilex_test_make_actor_teleport_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_actor_teleport_event_facade().release();
}

aegilex::native::endstone_binding::events::ServerListPingEventFacade *
aegilex_test_make_server_list_ping_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_server_list_ping_event_facade().release();
}

aegilex::native::endstone_binding::events::PlayerDropItemEventFacade *
aegilex_test_make_player_drop_item_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_player_drop_item_event_facade().release();
}

aegilex::native::endstone_binding::events::ActorDamageEventFacade *
aegilex_test_make_actor_damage_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_actor_damage_event_facade().release();
}

aegilex::native::endstone_binding::events::ActorExplodeEventFacade *
aegilex_test_make_actor_explode_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_actor_explode_event_facade().release();
}

aegilex::native::endstone_binding::events::BlockExplodeEventFacade *
aegilex_test_make_block_explode_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_block_explode_event_facade().release();
}

aegilex::native::endstone_binding::events::ActorKnockbackEventFacade *
aegilex_test_make_actor_knockback_event_facade(const bool has_source) noexcept
{
    return aegilex::native::endstone_binding::events::make_test_actor_knockback_event_facade(has_source).release();
}

aegilex::native::endstone_binding::events::PlayerDeathEventFacade *
aegilex_test_make_player_death_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_player_death_event_facade().release();
}

aegilex::native::endstone_binding::events::PlayerJoinEventFacade *aegilex_test_make_player_join_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_player_join_event_facade().release();
}

aegilex::native::endstone_binding::events::PlayerQuitEventFacade *aegilex_test_make_player_quit_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_player_quit_event_facade().release();
}

aegilex::native::endstone_binding::events::BlockBreakEventFacade *aegilex_test_make_block_break_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_block_break_event_facade().release();
}

aegilex::native::endstone_binding::events::BlockCookEventFacade *aegilex_test_make_block_cook_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_block_cook_event_facade().release();
}

aegilex::native::endstone_binding::events::LeavesDecayEventFacade *
aegilex_test_make_leaves_decay_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_leaves_decay_event_facade().release();
}

aegilex::native::endstone_binding::events::BlockFromToEventFacade *
aegilex_test_make_block_from_to_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_block_from_to_event_facade().release();
}

aegilex::native::endstone_binding::events::BlockGrowEventFacade *aegilex_test_make_block_grow_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_block_grow_event_facade().release();
}

aegilex::native::endstone_binding::events::BlockPistonEventFacade *
aegilex_test_make_block_piston_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_block_piston_event_facade().release();
}

aegilex::native::endstone_binding::events::BlockPlaceEventFacade *aegilex_test_make_block_place_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_block_place_event_facade().release();
}

aegilex::native::endstone_binding::events::PlayerInteractEventFacade *
aegilex_test_make_player_interact_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_player_interact_event_facade().release();
}

aegilex::native::endstone_binding::events::PlayerInteractActorEventFacade *
aegilex_test_make_player_interact_actor_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_player_interact_actor_event_facade().release();
}

aegilex::native::endstone_binding::events::PlayerItemConsumeEventFacade *
aegilex_test_make_player_item_consume_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_player_item_consume_event_facade().release();
}

aegilex::native::endstone_binding::events::PlayerGameModeChangeEventFacade *
aegilex_test_make_player_game_mode_change_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_player_game_mode_change_event_facade().release();
}

aegilex::native::endstone_binding::events::PlayerEmoteEventFacade *
aegilex_test_make_player_emote_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_player_emote_event_facade().release();
}

aegilex::native::endstone_binding::events::PlayerSkinChangeEventFacade *
aegilex_test_make_player_skin_change_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_player_skin_change_event_facade().release();
}

aegilex::native::endstone_binding::events::PlayerDimensionChangeEventFacade *
aegilex_test_make_player_dimension_change_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_player_dimension_change_event_facade().release();
}

aegilex::native::endstone_binding::events::PlayerBedEnterEventFacade *
aegilex_test_make_player_bed_enter_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_player_bed_enter_event_facade().release();
}

aegilex::native::endstone_binding::events::PlayerBedLeaveEventFacade *
aegilex_test_make_player_bed_leave_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_player_bed_leave_event_facade().release();
}

aegilex::native::endstone_binding::events::PlayerRespawnEventFacade *
aegilex_test_make_player_respawn_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_player_respawn_event_facade().release();
}

aegilex::native::endstone_binding::events::PlayerItemHeldEventFacade *
aegilex_test_make_player_item_held_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_player_item_held_event_facade().release();
}

aegilex::native::endstone_binding::events::PlayerPickupItemEventFacade *
aegilex_test_make_player_pickup_item_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_player_pickup_item_event_facade().release();
}

aegilex::native::endstone_binding::events::PlayerMoveEventFacade *aegilex_test_make_player_move_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_player_move_event_facade().release();
}

aegilex::native::endstone_binding::events::WeatherChangeEventFacade *
aegilex_test_make_weather_change_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_weather_change_event_facade();
}

aegilex::native::endstone_binding::events::ThunderChangeEventFacade *
aegilex_test_make_thunder_change_event_facade() noexcept
{
    return aegilex::native::endstone_binding::events::make_test_thunder_change_event_facade();
}

} // extern "C"
