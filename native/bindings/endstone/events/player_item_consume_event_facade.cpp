#include "player_item_consume_event_facade.h"

#include <endstone/event/player/player_item_consume_event.h>

namespace aegilex::native::endstone_binding::events {

PlayerItemConsumeEventFacade::PlayerItemConsumeEventFacade(endstone::PlayerItemConsumeEvent *event) noexcept
    : event_(event)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerItemConsumeEventFacade::getPlayer() const noexcept
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

std::unique_ptr<::aegilex::native::inventory::ItemStackRef> PlayerItemConsumeEventFacade::getItem() const noexcept
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

std::uint8_t PlayerItemConsumeEventFacade::getHand() const noexcept
{
    if (event_ == nullptr) {
        return 0;
    }
    try {
        return static_cast<std::uint8_t>(event_->getHand());
    }
    catch (...) {
        return 0;
    }
}

bool PlayerItemConsumeEventFacade::isCancelled() const noexcept
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

bool PlayerItemConsumeEventFacade::setCancelled(const bool cancelled) noexcept
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
