#include "player_interact_actor_event_facade.h"

#include <endstone/event/player/player_interact_actor_event.h>

namespace aegilex::native::endstone_binding::events {

PlayerInteractActorEventFacade::PlayerInteractActorEventFacade(endstone::PlayerInteractActorEvent *event) noexcept
    : event_(event)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerInteractActorEventFacade::getPlayer() const noexcept
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

std::unique_ptr<::aegilex::native::actor::Actor> PlayerInteractActorEventFacade::getActor() const noexcept
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

bool PlayerInteractActorEventFacade::isCancelled() const noexcept
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

bool PlayerInteractActorEventFacade::setCancelled(const bool cancelled) noexcept
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
