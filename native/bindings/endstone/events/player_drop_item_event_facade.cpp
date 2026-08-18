#include "player_drop_item_event_facade.h"

#include <endstone/event/player/player_drop_item_event.h>

namespace aegilex::native::endstone_binding::events {

PlayerDropItemEventFacade::PlayerDropItemEventFacade(endstone::PlayerDropItemEvent *event) noexcept : event_(event)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerDropItemEventFacade::getPlayer() const noexcept
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

std::unique_ptr<::aegilex::native::inventory::ItemStackRef> PlayerDropItemEventFacade::getItem() const noexcept
{
    if (event_ == nullptr) {
        return nullptr;
    }
    try {
        return std::make_unique<::aegilex::native::inventory::ItemStackRef>(&event_->getItem());
    }
    catch (...) {
        return nullptr;
    }
}

bool PlayerDropItemEventFacade::isCancelled() const noexcept
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

bool PlayerDropItemEventFacade::setCancelled(const bool cancelled) noexcept
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
