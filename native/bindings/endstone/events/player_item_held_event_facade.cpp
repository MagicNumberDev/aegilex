#include "player_item_held_event_facade.h"

#include <endstone/event/player/player_item_held_event.h>

namespace aegilex::native::endstone_binding::events {

PlayerItemHeldEventFacade::PlayerItemHeldEventFacade(endstone::PlayerItemHeldEvent *event) noexcept : event_(event)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerItemHeldEventFacade::getPlayer() const noexcept
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

std::int32_t PlayerItemHeldEventFacade::getPreviousSlot() const noexcept
{
    if (event_ == nullptr) {
        return 0;
    }
    try {
        return event_->getPreviousSlot();
    }
    catch (...) {
        return 0;
    }
}

std::int32_t PlayerItemHeldEventFacade::getNewSlot() const noexcept
{
    if (event_ == nullptr) {
        return 0;
    }
    try {
        return event_->getNewSlot();
    }
    catch (...) {
        return 0;
    }
}

bool PlayerItemHeldEventFacade::isCancelled() const noexcept
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

bool PlayerItemHeldEventFacade::setCancelled(const bool cancelled) noexcept
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
