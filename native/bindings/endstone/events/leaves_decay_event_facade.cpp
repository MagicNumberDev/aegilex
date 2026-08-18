#include "leaves_decay_event_facade.h"

#include <endstone/event/block/leaves_decay_event.h>

namespace aegilex::native::endstone_binding::events {

LeavesDecayEventFacade::LeavesDecayEventFacade(endstone::LeavesDecayEvent *event) noexcept : event_(event)
{
}

std::unique_ptr<::aegilex::native::level::Block> LeavesDecayEventFacade::getBlock() const noexcept
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

bool LeavesDecayEventFacade::isCancelled() const noexcept
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

bool LeavesDecayEventFacade::setCancelled(const bool cancelled) noexcept
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
