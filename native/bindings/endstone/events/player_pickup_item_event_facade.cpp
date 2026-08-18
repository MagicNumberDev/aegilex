#include "player_pickup_item_event_facade.h"

#include <endstone/event/player/player_pickup_item_event.h>

namespace aegilex::native::endstone_binding::events {

PlayerPickupItemEventFacade::PlayerPickupItemEventFacade(endstone::PlayerPickupItemEvent *event) noexcept
    : event_(event)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerPickupItemEventFacade::getPlayer() const noexcept
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

std::unique_ptr<::aegilex::native::actor::Actor> PlayerPickupItemEventFacade::getItemActor() const noexcept
{
    if (event_ == nullptr) {
        return nullptr;
    }
    try {
        return std::make_unique<::aegilex::native::actor::Actor>(&event_->getItem());
    }
    catch (...) {
        return nullptr;
    }
}

bool PlayerPickupItemEventFacade::isCancelled() const noexcept
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

bool PlayerPickupItemEventFacade::setCancelled(const bool cancelled) noexcept
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
