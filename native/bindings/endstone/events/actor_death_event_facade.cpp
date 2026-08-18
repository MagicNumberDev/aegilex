#include "actor_death_event_facade.h"

#include <endstone/actor/mob.h>
#include <endstone/event/actor/actor_death_event.h>

namespace aegilex::native::endstone_binding::events {

ActorDeathEventFacade::ActorDeathEventFacade(endstone::ActorDeathEvent *event) noexcept : event_(event)
{
}

std::unique_ptr<::aegilex::native::actor::Actor> ActorDeathEventFacade::getActor() const noexcept
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
