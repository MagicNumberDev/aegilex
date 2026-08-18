#include "actor_spawn_event_facade.h"

#include <endstone/event/actor/actor_spawn_event.h>

namespace aegilex::native::endstone_binding::events {

ActorSpawnEventFacade::ActorSpawnEventFacade(endstone::ActorSpawnEvent *event) noexcept : event_(event)
{
}

std::unique_ptr<::aegilex::native::actor::Actor> ActorSpawnEventFacade::getActor() const noexcept
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

bool ActorSpawnEventFacade::isCancelled() const noexcept
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

bool ActorSpawnEventFacade::setCancelled(const bool cancelled) noexcept
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
