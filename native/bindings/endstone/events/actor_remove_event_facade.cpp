#include "actor_remove_event_facade.h"

#include <endstone/event/actor/actor_remove_event.h>

namespace aegilex::native::endstone_binding::events {

ActorRemoveEventFacade::ActorRemoveEventFacade(endstone::ActorRemoveEvent *event) noexcept : event_(event)
{
}

std::unique_ptr<::aegilex::native::actor::Actor> ActorRemoveEventFacade::getActor() const noexcept
{
    if (event_ == nullptr) {
        return nullptr;
    }
    try {
        return std::make_unique<::aegilex::native::actor::Actor>(&event_->getActor());
    }
    catch (...) {
        return nullptr;
    }
}

} // namespace aegilex::native::endstone_binding::events
