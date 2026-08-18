#include "player_move_event_facade.h"

#include "host_context.h"

#include <aegilex-runtime/src/cxx_runtime.rs.h>
#include <endstone/event/player/player_move_event.h>
#include <endstone/level/dimension.h>
#include <endstone/level/level.h>

#include <cmath>

namespace aegilex::native::endstone_binding::events {

namespace {

[[nodiscard]] aegilex::runtime::LocationData copy_location(const endstone::Location &location) noexcept
{
    try {
        return {.dimension = rust::String(location.getDimension().getName()),
                .x = location.getX(),
                .y = location.getY(),
                .z = location.getZ(),
                .pitch = location.getPitch(),
                .yaw = location.getYaw()};
    }
    catch (...) {
        return {};
    }
}

} // namespace

PlayerMoveEventFacade::PlayerMoveEventFacade(endstone::PlayerMoveEvent *event, HostContext *context) noexcept
    : event_(event), context_(context)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerMoveEventFacade::getPlayer() const noexcept
{
    if (event_ == nullptr) {
        return nullptr;
    }
    try {
        return std::make_unique<::aegilex::native::player::Player>(&event_->getPlayer());
    }
    catch (...) {
        return nullptr;
    }
}

aegilex::runtime::LocationData PlayerMoveEventFacade::getFrom() const noexcept
{
    if (event_ == nullptr) {
        return {};
    }
    return copy_location(event_->getFrom());
}

aegilex::runtime::LocationData PlayerMoveEventFacade::getTo() const noexcept
{
    if (event_ == nullptr) {
        return {};
    }
    return copy_location(event_->getTo());
}

bool PlayerMoveEventFacade::setFrom(const aegilex::runtime::LocationData &location) noexcept
{
    if (event_ == nullptr || context_ == nullptr || context_->server.native() == nullptr ||
        location.dimension.empty() || !std::isfinite(location.x) || !std::isfinite(location.y) ||
        !std::isfinite(location.z) || !std::isfinite(location.pitch) || !std::isfinite(location.yaw)) {
        return false;
    }
    try {
        auto *level = context_->server.native()->getLevel();
        auto *dimension = level == nullptr ? nullptr : level->getDimension(std::string(location.dimension));
        if (dimension == nullptr) {
            return false;
        }
        event_->setFrom(
            endstone::Location{*dimension, location.x, location.y, location.z, location.pitch, location.yaw});
        return true;
    }
    catch (...) {
        return false;
    }
}

bool PlayerMoveEventFacade::setTo(const aegilex::runtime::LocationData &location) noexcept
{
    if (event_ == nullptr || context_ == nullptr || context_->server.native() == nullptr ||
        location.dimension.empty() || !std::isfinite(location.x) || !std::isfinite(location.y) ||
        !std::isfinite(location.z) || !std::isfinite(location.pitch) || !std::isfinite(location.yaw)) {
        return false;
    }
    try {
        auto *level = context_->server.native()->getLevel();
        auto *dimension = level == nullptr ? nullptr : level->getDimension(std::string(location.dimension));
        if (dimension == nullptr) {
            return false;
        }
        event_->setTo(endstone::Location{*dimension, location.x, location.y, location.z, location.pitch, location.yaw});
        return true;
    }
    catch (...) {
        return false;
    }
}

bool PlayerMoveEventFacade::isCancelled() const noexcept
{
    if (event_ == nullptr) {
        return false;
    }
    try {
        return event_->isCancelled();
    }
    catch (...) {
        return false;
    }
}

bool PlayerMoveEventFacade::setCancelled(const bool cancelled) noexcept
{
    if (event_ == nullptr) {
        return false;
    }
    try {
        event_->setCancelled(cancelled);
        return true;
    }
    catch (...) {
        return false;
    }
}

} // namespace aegilex::native::endstone_binding::events
