#include "bindings/endstone/actor/player.h"
#include "bindings/endstone/actor/actor.h"
#include "../../../aegilex_types.h"

#include "bindings/endstone/inventory/inventory.h"
#include "bindings/endstone/inventory/player_inventory.h"
#include "bindings/endstone/map/map.h"
#include "bindings/endstone/scoreboard/scoreboard.h"

#include <aegilex-runtime/src/cxx_host_player.rs.h>

#include <endstone/game_mode.h>
#include <endstone/level/dimension.h>
#include <endstone/level/level.h>
#include <endstone/map/map_view.h>
#include <endstone/message.h>
#include <endstone/player.h>
#include <endstone/scoreboard/scoreboard.h>
#include <endstone/skin.h>

#include <optional>
#include <string>
#include <string_view>

namespace aegilex::native::player {

class Player::impl {
  public:
    explicit impl(endstone::Player *player) noexcept : player(player)
    {
    }

    endstone::Player *player;
};

namespace {
[[nodiscard]] GameMode to_facade_game_mode(const endstone::GameMode mode) noexcept
{
    switch (mode) {
    case endstone::GameMode::Survival:
        return GameMode::Survival;
    case endstone::GameMode::Creative:
        return GameMode::Creative;
    case endstone::GameMode::Adventure:
        return GameMode::Adventure;
    case endstone::GameMode::Spectator:
        return GameMode::Spectator;
    }
    return GameMode::Survival;
}

[[nodiscard]] endstone::GameMode to_endstone_game_mode(const GameMode mode) noexcept
{
    switch (mode) {
    case GameMode::Survival:
        return endstone::GameMode::Survival;
    case GameMode::Creative:
        return endstone::GameMode::Creative;
    case GameMode::Adventure:
        return endstone::GameMode::Adventure;
    case GameMode::Spectator:
        return endstone::GameMode::Spectator;
    }
    return endstone::GameMode::Survival;
}

} // namespace

Player::Player(endstone::Player *player) noexcept : impl(std::make_shared<class Player::impl>(player))
{
}

rust::String Player::getName() const
{
    try {
        return rust::String(impl->player->getName());
    }
    catch (...) {
        return rust::String();
    }
}

rust::Vec<std::uint8_t> Player::getUniqueId() const
{
    rust::Vec<std::uint8_t> uuid;
    try {
        for (const auto byte : impl->player->getUniqueId()) {
            uuid.push_back(byte);
        }
    }
    catch (...) {
    }
    return uuid;
}

rust::String Player::getXuid() const
{
    try {
        return rust::String(impl->player->getXuid());
    }
    catch (...) {
        return rust::String();
    }
}

bool Player::isOp() const
{
    try {
        return impl->player->isOp();
    }
    catch (...) {
        return false;
    }
}

void Player::setOp(const bool value) const
{
    try {
        impl->player->setOp(value);
    }
    catch (...) {
    }
}

std::uint32_t Player::getPing() const
{
    try {
        return static_cast<std::uint32_t>(impl->player->getPing().count());
    }
    catch (...) {
        return 0;
    }
}

rust::String Player::getLocale() const
{
    try {
        return rust::String(impl->player->getLocale());
    }
    catch (...) {
        return rust::String();
    }
}

rust::String Player::getGameVersion() const
{
    try {
        return rust::String(impl->player->getGameVersion());
    }
    catch (...) {
        return rust::String();
    }
}

rust::String Player::getDeviceOS() const
{
    try {
        return rust::String(impl->player->getDeviceOS());
    }
    catch (...) {
        return rust::String();
    }
}

rust::String Player::getDeviceId() const
{
    try {
        return rust::String(impl->player->getDeviceId());
    }
    catch (...) {
        return rust::String();
    }
}

SocketAddress Player::getAddress() const
{
    try {
        const auto address = impl->player->getAddress();
        return SocketAddress{.hostname = rust::String(address.getHostname()), .port = address.getPort()};
    }
    catch (...) {
        return SocketAddress{.hostname = rust::String(), .port = 0};
    }
}

void Player::sendPacket(const std::int32_t packet_id, const rust::Slice<const std::uint8_t> payload) const
{
    try {
        if (impl->player != nullptr) {
            impl->player->sendPacket(packet_id,
                                     std::string_view(reinterpret_cast<const char *>(payload.data()), payload.size()));
        }
    }
    catch (...) {
    }
}

void Player::transfer(const rust::Str host, const std::uint16_t port) const
{
    try {
        impl->player->transfer(std::string(host), port);
    }
    catch (...) {
    }
}

void Player::kick(const rust::Str message) const
{
    try {
        impl->player->kick(std::string(message));
    }
    catch (...) {
    }
}

bool Player::performCommand(const rust::Str command) const
{
    try {
        return impl->player->performCommand(std::string(command));
    }
    catch (...) {
        return false;
    }
}

void Player::updateCommands() const
{
    try {
        impl->player->updateCommands();
    }
    catch (...) {
    }
}

bool Player::isSneaking() const
{
    try {
        return impl->player->isSneaking();
    }
    catch (...) {
        return false;
    }
}

void Player::setSneaking(const bool sneaking) const
{
    try {
        impl->player->setSneaking(sneaking);
    }
    catch (...) {
    }
}

bool Player::isSprinting() const
{
    try {
        return impl->player->isSprinting();
    }
    catch (...) {
        return false;
    }
}

void Player::setSprinting(const bool sprinting) const
{
    try {
        impl->player->setSprinting(sprinting);
    }
    catch (...) {
    }
}

float Player::getExpProgress() const
{
    try {
        return impl->player->getExpProgress();
    }
    catch (...) {
        return 0.0F;
    }
}

void Player::setExpProgress(const float progress) const
{
    try {
        impl->player->setExpProgress(progress);
    }
    catch (...) {
    }
}

std::int32_t Player::getExpLevel() const
{
    try {
        return impl->player->getExpLevel();
    }
    catch (...) {
        return 0;
    }
}

void Player::setExpLevel(const std::int32_t level) const
{
    try {
        impl->player->setExpLevel(level);
    }
    catch (...) {
    }
}

std::int32_t Player::getTotalExp() const
{
    try {
        return impl->player->getTotalExp();
    }
    catch (...) {
        return 0;
    }
}

void Player::giveExp(const std::int32_t amount) const
{
    try {
        impl->player->giveExp(amount);
    }
    catch (...) {
    }
}

void Player::giveExpLevels(const std::int32_t amount) const
{
    try {
        impl->player->giveExpLevels(amount);
    }
    catch (...) {
    }
}

bool Player::getAllowFlight() const
{
    try {
        return impl->player->getAllowFlight();
    }
    catch (...) {
        return false;
    }
}

void Player::setAllowFlight(const bool allow) const
{
    try {
        impl->player->setAllowFlight(allow);
    }
    catch (...) {
    }
}

bool Player::isFlying() const
{
    try {
        return impl->player->isFlying();
    }
    catch (...) {
        return false;
    }
}

void Player::setFlying(const bool flying) const
{
    try {
        impl->player->setFlying(flying);
    }
    catch (...) {
    }
}

float Player::getFlySpeed() const
{
    try {
        return impl->player->getFlySpeed();
    }
    catch (...) {
        return 0.0F;
    }
}

void Player::setFlySpeed(const float speed) const
{
    try {
        impl->player->setFlySpeed(speed);
    }
    catch (...) {
    }
}

float Player::getWalkSpeed() const
{
    try {
        return impl->player->getWalkSpeed();
    }
    catch (...) {
        return 0.0F;
    }
}

void Player::setWalkSpeed(const float speed) const
{
    try {
        impl->player->setWalkSpeed(speed);
    }
    catch (...) {
    }
}

void Player::sendMessage(const rust::Str message) const
{
    try {
        impl->player->sendMessage(endstone::Message{std::string(message)});
    }
    catch (...) {
    }
}

void Player::sendPopup(const rust::Str text) const
{
    try {
        impl->player->sendPopup(std::string(text));
    }
    catch (...) {
    }
}

void Player::sendTip(const rust::Str text) const
{
    try {
        impl->player->sendTip(std::string(text));
    }
    catch (...) {
    }
}

void Player::sendToast(const rust::Str title, const rust::Str content) const
{
    try {
        impl->player->sendToast(std::string(title), std::string(content));
    }
    catch (...) {
    }
}

void Player::sendTitle(const rust::Str title, const rust::Str subtitle, const std::int32_t fade_in,
                       const std::int32_t stay, const std::int32_t fade_out) const
{
    try {
        if (fade_in < 0) {
            impl->player->sendTitle(std::string(title), std::string(subtitle));
        }
        else {
            impl->player->sendTitle(std::string(title), std::string(subtitle), fade_in, stay, fade_out);
        }
    }
    catch (...) {
    }
}

void Player::resetTitle() const
{
    try {
        impl->player->resetTitle();
    }
    catch (...) {
    }
}

void Player::playSound(const Location &location, const rust::Str sound, const float volume, const float pitch) const
{
    try {
        auto *dimension = location.dimension.empty()
                              ? &impl->player->getLocation().getDimension()
                              : impl->player->getLevel().getDimension(std::string(location.dimension));
        if (dimension == nullptr) {
            return;
        }
        const endstone::Location target{*dimension, location.x, location.y, location.z, location.pitch, location.yaw};
        impl->player->playSound(target, std::string(sound), volume, pitch);
    }
    catch (...) {
    }
}

void Player::stopSound(const rust::Str sound) const
{
    try {
        impl->player->stopSound(std::string(sound));
    }
    catch (...) {
    }
}

void Player::stopAllSounds() const
{
    try {
        impl->player->stopAllSounds();
    }
    catch (...) {
    }
}

void Player::spawnParticle(const rust::Str name, const float x, const float y, const float z,
                           const rust::Str molang_json) const
{
    try {
        auto &dimension = impl->player->getLocation().getDimension();
        const endstone::Location location(dimension, x, y, z);
        if (molang_json.empty()) {
            impl->player->spawnParticle(std::string(name), location);
        }
        else {
            impl->player->spawnParticle(std::string(name), location,
                                        std::optional<std::string>{std::string(molang_json)});
        }
    }
    catch (...) {
    }
}

GameMode Player::getGameMode() const
{
    try {
        return to_facade_game_mode(impl->player->getGameMode());
    }
    catch (...) {
        return GameMode::Survival;
    }
}

void Player::setGameMode(const GameMode mode) const
{
    try {
        impl->player->setGameMode(to_endstone_game_mode(mode));
    }
    catch (...) {
    }
}

SkinData Player::getSkin() const
{
    try {
        const auto &skin = impl->player->getSkin();
        const auto &image = skin.getImage();
        const auto &pixels = image.getData();
        SkinData out{};
        out.id = rust::String(skin.getId());
        out.width = static_cast<std::uint32_t>(image.getWidth());
        out.height = static_cast<std::uint32_t>(image.getHeight());
        for (const auto byte : pixels) {
            out.pixels.push_back(static_cast<std::uint8_t>(byte));
        }
        return out;
    }
    catch (...) {
        return SkinData{};
    }
}

std::unique_ptr<::aegilex::native::inventory::PlayerInventory> Player::getInventory() const
{
    try {
        return std::make_unique<::aegilex::native::inventory::PlayerInventory>(&impl->player->getInventory());
    }
    catch (...) {
        return std::unique_ptr<::aegilex::native::inventory::PlayerInventory>();
    }
}

std::unique_ptr<::aegilex::native::inventory::Inventory> Player::getEnderChest() const
{
    try {
        return std::make_unique<::aegilex::native::inventory::Inventory>(&impl->player->getEnderChest());
    }
    catch (...) {
        return std::unique_ptr<::aegilex::native::inventory::Inventory>();
    }
}

std::unique_ptr<Player> Player::clone() const
{
    try {
        return impl == nullptr || impl->player == nullptr ? std::unique_ptr<Player>()
                                                          : std::make_unique<Player>(impl->player);
    }
    catch (...) {
        return std::unique_ptr<Player>();
    }
}

std::unique_ptr<::aegilex::native::actor::Actor> Player::asActor() const
{
    try {
        return impl == nullptr || impl->player == nullptr
                   ? std::unique_ptr<::aegilex::native::actor::Actor>()
                   : std::make_unique<::aegilex::native::actor::Actor>(impl->player);
    }
    catch (...) {
        return {};
    }
}

endstone::Player *Player::native() const noexcept
{
    return impl == nullptr ? nullptr : impl->player;
}

std::unique_ptr<::aegilex::native::ui::Scoreboard> Player::getScoreboard() const
{
    try {
        if (impl == nullptr || impl->player == nullptr) {
            return std::unique_ptr<::aegilex::native::ui::Scoreboard>();
        }
        return std::make_unique<::aegilex::native::ui::Scoreboard>(&impl->player->getScoreboard());
    }
    catch (...) {
        return std::unique_ptr<::aegilex::native::ui::Scoreboard>();
    }
}

void Player::setScoreboard(const ::aegilex::native::ui::Scoreboard &scoreboard) const
{
    try {
        if (impl == nullptr || impl->player == nullptr || scoreboard.native() == nullptr) {
            return;
        }
        impl->player->setScoreboard(*scoreboard.native());
    }
    catch (...) {
    }
}

void Player::sendMap(const ::aegilex::native::ui::Map &map) const
{
    try {
        if (impl == nullptr || impl->player == nullptr || map.native() == nullptr) {
            return;
        }
        impl->player->sendMap(*map.native());
    }
    catch (...) {
    }
}

} // namespace aegilex::native::player
