#include "mob.h"

#include "actor.h"

#include <endstone/actor/mob.h>

#include <memory>

namespace aegilex::native::actor {

class Mob::impl {
  public:
    explicit impl(endstone::Mob *mob) noexcept : mob(mob)
    {
    }

    endstone::Mob *mob;
};

Mob::Mob(endstone::Mob *mob) noexcept : impl(std::make_shared<class Mob::impl>(mob))
{
}

endstone::Mob *Mob::native() const noexcept
{
    return impl == nullptr ? nullptr : impl->mob;
}

bool Mob::isGliding() const
{
    try {
        return impl->mob->isGliding();
    }
    catch (...) {
        return false;
    }
}

std::int32_t Mob::getHealth() const
{
    try {
        return impl->mob->getHealth();
    }
    catch (...) {
        return 0;
    }
}

std::int32_t Mob::getMaxHealth() const
{
    try {
        return impl->mob->getMaxHealth();
    }
    catch (...) {
        return 0;
    }
}

void Mob::setHealth(const std::int32_t health) const
{
    try {
        impl->mob->setHealth(health);
    }
    catch (...) {
    }
}

void Mob::setMaxHealth(const std::int32_t health) const
{
    try {
        impl->mob->setMaxHealth(health);
    }
    catch (...) {
    }
}

std::unique_ptr<Actor> Mob::asActor() const
{
    try {
        return std::unique_ptr<Actor>(new Actor(static_cast<endstone::Actor *>(impl->mob)));
    }
    catch (...) {
        return nullptr;
    }
}

} // namespace aegilex::native::actor
