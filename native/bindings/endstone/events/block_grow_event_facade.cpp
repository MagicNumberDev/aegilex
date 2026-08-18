#include "block_grow_event_facade.h"

#include <endstone/block/block_state.h>
#include <endstone/event/block/block_grow_event.h>

namespace aegilex::native::endstone_binding::events {

BlockGrowEventFacade::BlockGrowEventFacade(endstone::BlockGrowEvent *event) noexcept : event_(event)
{
}

std::unique_ptr<::aegilex::native::level::Block> BlockGrowEventFacade::getBlock() const noexcept
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

bool BlockGrowEventFacade::isCancelled() const noexcept
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

bool BlockGrowEventFacade::setCancelled(const bool cancelled) noexcept
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
