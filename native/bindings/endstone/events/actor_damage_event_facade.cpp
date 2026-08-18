#include "actor_damage_event_facade.h"

#include <aegilex-runtime/src/cxx_runtime.rs.h>

#include <endstone/actor/actor.h>
#include <endstone/actor/mob.h>
#include <endstone/damage/damage_source.h>
#include <endstone/event/actor/actor_damage_event.h>

namespace aegilex::native::endstone_binding::events {

ActorDamageEventFacade::ActorDamageEventFacade(endstone::ActorDamageEvent *event) noexcept : event_(event)
{
}

std::unique_ptr<::aegilex::native::actor::Actor> ActorDamageEventFacade::getActor() const noexcept
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

float ActorDamageEventFacade::getDamage() const noexcept
{
    if (event_ == nullptr) {
        return 0.0F;
    }
    try {
        return event_->getDamage();
    }
    catch (...) {
        return 0.0F;
    }
}

bool ActorDamageEventFacade::setDamage(const float damage) noexcept
{
    if (event_ == nullptr) {
        return false;
    }
    try {
        event_->setDamage(damage);
        return true;
    }
    catch (...) {
        return false;
    }
}

aegilex::runtime::DamageSourceData ActorDamageEventFacade::getDamageSource() const noexcept
{
    aegilex::runtime::DamageSourceData out{};
    if (event_ == nullptr) {
        return out;
    }
    try {
        const auto &source = event_->getDamageSource();
        out.type_id = rust::String(std::string(source.getType()));
        if (auto *actor = source.getActor()) {
            out.has_actor_id = true;
            out.actor_id = actor->getId();
        }
        if (auto *actor = source.getDamagingActor()) {
            out.has_damaging_actor_id = true;
            out.damaging_actor_id = actor->getId();
        }
        out.indirect = source.isIndirect();
        return out;
    }
    catch (...) {
        return aegilex::runtime::DamageSourceData{};
    }
}

bool ActorDamageEventFacade::isCancelled() const noexcept
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

bool ActorDamageEventFacade::setCancelled(const bool cancelled) noexcept
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
