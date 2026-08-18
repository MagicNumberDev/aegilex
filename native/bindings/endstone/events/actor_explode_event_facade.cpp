#include "actor_explode_event_facade.h"

#include <aegilex-runtime/src/cxx_runtime.rs.h>

#include <endstone/event/actor/actor_explode_event.h>
#include <endstone/level/dimension.h>

namespace aegilex::native::endstone_binding::events {

ActorExplodeEventFacade::ActorExplodeEventFacade() noexcept
    : event_(nullptr), location_{.dimension = "test", .x = 1.0F, .y = 2.0F, .z = 3.0F, .pitch = 4.0F, .yaw = 5.0F}
{
    blocks_.push_back(std::make_unique<::aegilex::native::level::Block>(std::unique_ptr<endstone::Block>()));
}

ActorExplodeEventFacade::ActorExplodeEventFacade(endstone::ActorExplodeEvent *event) noexcept : event_(event)
{
    if (event_ == nullptr) {
        return;
    }
    try {
        const auto &location = event_->getLocation();
        location_ = {.dimension = location.getDimension().getName(),
                     .x = location.getX(),
                     .y = location.getY(),
                     .z = location.getZ(),
                     .pitch = location.getPitch(),
                     .yaw = location.getYaw()};
        for (const auto &block : event_->getBlockList()) {
            if (block != nullptr) {
                blocks_.push_back(std::make_unique<::aegilex::native::level::Block>(*block));
            }
        }
    }
    catch (...) {
        location_ = {};
        blocks_.clear();
    }
}

std::unique_ptr<::aegilex::native::actor::Actor> ActorExplodeEventFacade::getActor() const noexcept
{
    if (event_ == nullptr) {
        return nullptr;
    }
    try {
        return std::make_unique<::aegilex::native::actor::Actor>(&event_->getActor());
    }
    catch (...) {
        return nullptr;
    }
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

bool ActorExplodeEventFacade::setCancelled(const bool cancelled) noexcept
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
