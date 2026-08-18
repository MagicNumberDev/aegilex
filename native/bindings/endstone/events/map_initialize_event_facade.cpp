#include "map_initialize_event_facade.h"

#include <endstone/event/server/map_initialize_event.h>
#include <endstone/map/map_view.h>

namespace aegilex::native::endstone_binding::events {

MapInitializeEventFacade::MapInitializeEventFacade(endstone::MapInitializeEvent *event) noexcept : event_(event)
{
}

std::int64_t MapInitializeEventFacade::getMapIdForRust() const noexcept
{
    if (event_ == nullptr) {
        return 0;
    }
    try {
        return event_->getMap().getId();
    }
    catch (...) {
        return 0;
    }
}

} // namespace aegilex::native::endstone_binding::events
