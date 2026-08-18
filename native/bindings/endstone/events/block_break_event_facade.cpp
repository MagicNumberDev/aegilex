#include "block_break_event_facade.h"

#include <endstone/event/block/block_break_event.h>

namespace aegilex::native::endstone_binding::events {

BlockBreakEventFacade::BlockBreakEventFacade(endstone::BlockBreakEvent *event) noexcept : event_(event)
{
}

std::unique_ptr<::aegilex::native::player::Player> BlockBreakEventFacade::getPlayer() const noexcept
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

std::unique_ptr<::aegilex::native::level::Block> BlockBreakEventFacade::getBlock() const noexcept
{
    if (event_ == nullptr) {
        return nullptr;
    }
    try {
        return std::make_unique<::aegilex::native::level::Block>(event_->getBlock());
    }
    catch (...) {
        return nullptr;
    }
}

bool BlockBreakEventFacade::isCancelled() const noexcept
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

bool BlockBreakEventFacade::setCancelled(const bool cancelled) noexcept
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
