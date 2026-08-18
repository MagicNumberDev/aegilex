#include "thunder_change_event_facade.h"

#include <endstone/event/weather/thunder_change_event.h>

namespace aegilex::native::endstone_binding::events {

ThunderChangeEventFacade::ThunderChangeEventFacade(endstone::ThunderChangeEvent *event) noexcept : event_(event)
{
}

bool ThunderChangeEventFacade::getToThunder() const noexcept
{
    if (event_ == nullptr) {
        return false;
    }

    try {
        return event_->toThunderState();
    }
    catch (...) {
        return false;
    }
}

bool ThunderChangeEventFacade::isCancelled() const noexcept
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

bool ThunderChangeEventFacade::setCancelled(const bool cancelled) noexcept
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
