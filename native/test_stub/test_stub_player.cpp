// Test-only typed player bridge stubs. Never linked into the plugin.

#include <aegilex-runtime/src/cxx_host_player.rs.h>

#include "bindings/endstone/actor/player.h"
#include "bindings/endstone/actor/actor.h"
#include "bindings/endstone/inventory/inventory.h"
#include "bindings/endstone/inventory/player_inventory.h"
#include "bindings/endstone/map/map.h"
#include "bindings/endstone/scoreboard/scoreboard.h"

#include <string>

namespace aegilex::native::player {

class Player::impl {
  public:
    impl() noexcept = default;
};

Player::Player(endstone::Player *) noexcept : impl(std::make_shared<class Player::impl>())
{
}

rust::String Player::getName() const
{
    return rust::String("Aegilex");
}

rust::Vec<std::uint8_t> Player::getUniqueId() const
{
    rust::Vec<std::uint8_t> uuid;
    for (std::uint8_t byte = 1; byte <= 16; ++byte) {
        uuid.push_back(byte);
    }
    return uuid;
}

rust::String Player::getXuid() const
{
    return rust::String("2535400000000001");
}

bool Player::isOp() const
{
    return false;
}

void Player::setOp(bool) const
{
}

std::uint32_t Player::getPing() const
{
    return 7;
}

rust::String Player::getLocale() const
{
    return rust::String("en_US");
}

rust::String Player::getGameVersion() const
{
    return rust::String("1.21.0");
}

rust::String Player::getDeviceOS() const
{
    return rust::String("Windows");
}

rust::String Player::getDeviceId() const
{
    return rust::String("device-1");
}

SocketAddress Player::getAddress() const
{
    return SocketAddress{.hostname = rust::String("127.0.0.1"), .port = 19132};
}

void Player::sendPacket(const std::int32_t, const rust::Slice<const std::uint8_t>) const
{
}

void Player::transfer(rust::Str, std::uint16_t) const
{
}

void Player::kick(rust::Str) const
{
}

bool Player::performCommand(rust::Str) const
{
    return true;
}

void Player::updateCommands() const
{
}

bool Player::isSneaking() const
{
    return false;
}

void Player::setSneaking(bool) const
{
}

bool Player::isSprinting() const
{
    return false;
}

void Player::setSprinting(bool) const
{
}

float Player::getExpProgress() const
{
    return 0.5F;
}

void Player::setExpProgress(float) const
{
}

std::int32_t Player::getExpLevel() const
{
    return 7;
}

void Player::setExpLevel(std::int32_t) const
{
}

std::int32_t Player::getTotalExp() const
{
    return 100;
}

void Player::giveExp(std::int32_t) const
{
}

void Player::giveExpLevels(std::int32_t) const
{
}

bool Player::getAllowFlight() const
{
    return false;
}

void Player::setAllowFlight(bool) const
{
}

bool Player::isFlying() const
{
    return false;
}

void Player::setFlying(bool) const
{
}

float Player::getFlySpeed() const
{
    return 0.05F;
}

void Player::setFlySpeed(float) const
{
}

float Player::getWalkSpeed() const
{
    return 0.1F;
}

void Player::setWalkSpeed(float) const
{
}

void Player::sendMessage(rust::Str) const
{
}

void Player::sendPopup(rust::Str) const
{
}

void Player::sendTip(rust::Str) const
{
}

void Player::sendToast(rust::Str, rust::Str) const
{
}

void Player::sendTitle(rust::Str, rust::Str, std::int32_t, std::int32_t, std::int32_t) const
{
}

void Player::resetTitle() const
{
}

void Player::playSound(const Location &, rust::Str, float, float) const
{
}

void Player::stopSound(rust::Str) const
{
}

void Player::stopAllSounds() const
{
}

void Player::spawnParticle(rust::Str, float, float, float, rust::Str) const
{
}

GameMode Player::getGameMode() const
{
    return GameMode::Survival;
}

void Player::setGameMode(GameMode) const
{
}

SkinData Player::getSkin() const
{
    SkinData skin;
    skin.id = rust::String("test-skin");
    skin.width = 64;
    skin.height = 64;
    skin.pixels.push_back(1);
    skin.pixels.push_back(2);
    skin.pixels.push_back(3);
    skin.pixels.push_back(4);
    return skin;
}

std::unique_ptr<::aegilex::native::inventory::PlayerInventory> Player::getInventory() const
{
    return std::unique_ptr<::aegilex::native::inventory::PlayerInventory>(
        new ::aegilex::native::inventory::PlayerInventory(nullptr));
}

std::unique_ptr<::aegilex::native::inventory::Inventory> Player::getEnderChest() const
{
    return std::unique_ptr<::aegilex::native::inventory::Inventory>(
        new ::aegilex::native::inventory::Inventory(nullptr));
}

std::unique_ptr<Player> Player::clone() const
{
    return std::unique_ptr<Player>(new Player(nullptr));
}

std::unique_ptr<::aegilex::native::actor::Actor> Player::asActor() const
{
    return std::unique_ptr<::aegilex::native::actor::Actor>(new ::aegilex::native::actor::Actor(nullptr));
}

std::unique_ptr<::aegilex::native::ui::Scoreboard> Player::getScoreboard() const
{
    return std::unique_ptr<::aegilex::native::ui::Scoreboard>(new ::aegilex::native::ui::Scoreboard(nullptr));
}

void Player::setScoreboard(const ::aegilex::native::ui::Scoreboard &) const
{
}

void Player::sendMap(const ::aegilex::native::ui::Map &) const
{
}

} // namespace aegilex::native::player
