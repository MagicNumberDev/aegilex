#include "block_piston_event_facade.h"

#include <endstone/event/block/block_piston_event.h>

namespace aegilex::native::endstone_binding::events {

BlockPistonEventFacade::BlockPistonEventFacade(endstone::BlockPistonEvent *event) noexcept : event_(event)
{
}

std::unique_ptr<::aegilex::native::level::Block> BlockPistonEventFacade::getBlock() const noexcept
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

std::uint8_t BlockPistonEventFacade::getDirection() const noexcept
{
    if (event_ == nullptr) {
        return 0;
    }
    try {
        return static_cast<std::uint8_t>(event_->getDirection());
    }
    catch (...) {
        return 0;
    }
}

bool BlockPistonEventFacade::isCancelled() const noexcept
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

bool BlockPistonEventFacade::setCancelled(const bool cancelled) noexcept
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
