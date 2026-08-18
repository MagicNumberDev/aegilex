#include "block_place_event_facade.h"

#include <endstone/event/block/block_place_event.h>

namespace aegilex::native::endstone_binding::events {

BlockPlaceEventFacade::BlockPlaceEventFacade(endstone::BlockPlaceEvent *event) noexcept : event_(event)
{
}

std::unique_ptr<::aegilex::native::player::Player> BlockPlaceEventFacade::getPlayer() const noexcept
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

std::unique_ptr<::aegilex::native::level::Block> BlockPlaceEventFacade::getBlockReplaced() const noexcept
{
    if (event_ == nullptr) {
        return nullptr;
    }
    try {
        return std::make_unique<::aegilex::native::level::Block>(event_->getBlockReplaced());
    }
    catch (...) {
        return nullptr;
    }
}

std::unique_ptr<::aegilex::native::level::Block> BlockPlaceEventFacade::getBlockAgainst() const noexcept
{
    if (event_ == nullptr) {
        return nullptr;
    }
    try {
        return std::make_unique<::aegilex::native::level::Block>(event_->getBlockAgainst());
    }
    catch (...) {
        return nullptr;
    }
}

bool BlockPlaceEventFacade::isCancelled() const noexcept
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

bool BlockPlaceEventFacade::setCancelled(const bool cancelled) noexcept
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
