#pragma once

#include "rust/cxx.h"

#include <cstdint>
#include <memory>
#include <string>

namespace endstone {
class Player;
}

namespace aegilex::native::actor {
class Actor;
}

namespace aegilex::native::inventory {
class Inventory;
class PlayerInventory;
} // namespace aegilex::native::inventory

namespace aegilex::native::ui {
class Map;
class Scoreboard;
} // namespace aegilex::native::ui

namespace aegilex::native::player {

struct Location;
struct SkinData;
struct SocketAddress;
enum class GameMode : ::std::uint8_t;

// OOP/Pimpl facade over endstone::Player. The impl holds only a non-owning
// endstone::Player* (BDS owns the player); no VM semantics live here.
// Mirrors the layout of endstone/player.h.
class Player {
  public:
    explicit Player(endstone::Player *player) noexcept;
    ~Player() noexcept = default;

    Player(const Player &) = delete;
    Player &operator=(const Player &) = delete;

    // Identity.
    rust::String getName() const;
    rust::Vec<std::uint8_t> getUniqueId() const;
    rust::String getXuid() const;
    bool isOp() const;
    void setOp(bool value) const;
    std::uint32_t getPing() const;
    rust::String getLocale() const;
    rust::String getGameVersion() const;
    rust::String getDeviceOS() const;
    rust::String getDeviceId() const;
    SocketAddress getAddress() const;
    void sendPacket(std::int32_t packet_id, rust::Slice<const std::uint8_t> payload) const;

    // Lifecycle.
    void transfer(rust::Str host, std::uint16_t port) const;
    void kick(rust::Str message) const;
    bool performCommand(rust::Str command) const;
    void updateCommands() const;

    // Movement.
    bool isSneaking() const;
    void setSneaking(bool sneaking) const;
    bool isSprinting() const;
    void setSprinting(bool sprinting) const;

    // Experience.
    float getExpProgress() const;
    void setExpProgress(float progress) const;
    std::int32_t getExpLevel() const;
    void setExpLevel(std::int32_t level) const;
    std::int32_t getTotalExp() const;
    void giveExp(std::int32_t amount) const;
    void giveExpLevels(std::int32_t amount) const;

    // Flight and movement speed.
    bool getAllowFlight() const;
    void setAllowFlight(bool allow) const;
    bool isFlying() const;
    void setFlying(bool flying) const;
    float getFlySpeed() const;
    void setFlySpeed(float speed) const;
    float getWalkSpeed() const;
    void setWalkSpeed(float speed) const;

    // Messaging.
    void sendMessage(rust::Str message) const;
    void sendPopup(rust::Str text) const;
    void sendTip(rust::Str text) const;
    void sendToast(rust::Str title, rust::Str content) const;
    void sendTitle(rust::Str title, rust::Str subtitle, std::int32_t fade_in, std::int32_t stay,
                   std::int32_t fade_out) const;
    void resetTitle() const;

    // Sound and particles. An empty molang_json means no molang variables.
    void playSound(const Location &location, rust::Str sound, float volume, float pitch) const;
    void stopSound(rust::Str sound) const;
    void stopAllSounds() const;
    void spawnParticle(rust::Str name, float x, float y, float z, rust::Str molang_json) const;

    // Game mode and skin.
    GameMode getGameMode() const;
    void setGameMode(GameMode mode) const;
    SkinData getSkin() const;

    // Rust resolves guest-visible IDs before combining facades.
    [[nodiscard]] std::unique_ptr<Player> clone() const;
    [[nodiscard]] std::unique_ptr<::aegilex::native::actor::Actor> asActor() const;
    std::unique_ptr<::aegilex::native::ui::Scoreboard> getScoreboard() const;
    void setScoreboard(const ::aegilex::native::ui::Scoreboard &scoreboard) const;
    void sendMap(const ::aegilex::native::ui::Map &map) const;

    std::unique_ptr<::aegilex::native::inventory::PlayerInventory> getInventory() const;
    std::unique_ptr<::aegilex::native::inventory::Inventory> getEnderChest() const;
    [[nodiscard]] endstone::Player *native() const noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl;
};

} // namespace aegilex::native::player
