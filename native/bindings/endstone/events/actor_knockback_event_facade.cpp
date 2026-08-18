#include "actor_knockback_event_facade.h"

#include <aegilex-runtime/src/cxx_runtime.rs.h>

#include <endstone/event/actor/actor_knockback_event.h>

#include <cmath>

namespace aegilex::native::endstone_binding::events {

ActorKnockbackEventFacade::ActorKnockbackEventFacade(endstone::ActorKnockbackEvent *event) noexcept : event_(event)
{
}

std::unique_ptr<::aegilex::native::actor::Actor> ActorKnockbackEventFacade::getActor() const noexcept
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

std::unique_ptr<::aegilex::native::actor::Actor> ActorKnockbackEventFacade::getSource() const noexcept
{
    if (event_ == nullptr) {
        return nullptr;
    }
    try {
        auto *source = event_->getSource();
        return source == nullptr ? nullptr : std::make_unique<::aegilex::native::actor::Actor>(source);
    }
    catch (...) {
        return nullptr;
    }
}

aegilex::runtime::VectorData ActorKnockbackEventFacade::getKnockback() const noexcept
{
    if (event_ == nullptr) {
        return {};
    }
    try {
        const auto knockback = event_->getKnockback();
        return {.x = knockback.getX(), .y = knockback.getY(), .z = knockback.getZ()};
    }
    catch (...) {
        return {};
    }
}

bool ActorKnockbackEventFacade::setKnockback(const aegilex::runtime::VectorData &knockback) noexcept
{
    if (event_ == nullptr || !std::isfinite(knockback.x) || !std::isfinite(knockback.y) ||
        !std::isfinite(knockback.z)) {
        return false;
    }
    try {
        event_->setKnockback(endstone::Vector{knockback.x, knockback.y, knockback.z});
        return true;
    }
    catch (...) {
        return false;
    }
}

bool ActorKnockbackEventFacade::isCancelled() const noexcept
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

bool ActorKnockbackEventFacade::setCancelled(const bool cancelled) noexcept
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
