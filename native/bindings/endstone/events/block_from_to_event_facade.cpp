#include "block_from_to_event_facade.h"

#include <endstone/event/block/block_from_to_event.h>

namespace aegilex::native::endstone_binding::events {

BlockFromToEventFacade::BlockFromToEventFacade(endstone::BlockFromToEvent *event) noexcept : event_(event)
{
}

std::unique_ptr<::aegilex::native::level::Block> BlockFromToEventFacade::getBlock() const noexcept
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

std::unique_ptr<::aegilex::native::level::Block> BlockFromToEventFacade::getToBlock() const noexcept
{
    if (event_ == nullptr) {
        return nullptr;
    }
    try {
        return std::make_unique<::aegilex::native::level::Block>(event_->getToBlock());
    }
    catch (...) {
        return nullptr;
    }
}

bool BlockFromToEventFacade::isCancelled() const noexcept
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

bool BlockFromToEventFacade::setCancelled(const bool cancelled) noexcept
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
