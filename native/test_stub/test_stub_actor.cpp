// Test-only typed actor/mob bridge stubs. Never linked into the plugin.

#include <aegilex-runtime/src/cxx_host_actor.rs.h>

#include "bindings/endstone/actor/actor.h"
#include "bindings/endstone/actor/mob.h"
#include "bindings/endstone/actor/item_actor.h"
#include "bindings/endstone/actor/player.h"
#include "bindings/endstone/inventory/item_stack.h"

#include <string>

namespace aegilex::native::actor {

class Actor::impl {
  public:
    explicit impl(const bool item) noexcept : item(item)
    {
    }

    bool item;
};

class Mob::impl {
  public:
    impl() noexcept = default;
};

Actor::Actor(endstone::Actor *actor) noexcept : impl(std::make_shared<class Actor::impl>(actor != nullptr))
{
}

rust::String Actor::getName() const
{
    return rust::String("Aegilex");
}

rust::String Actor::getType() const
{
    return rust::String("minecraft:test");
}

std::uint64_t Actor::getRuntimeId() const
{
    return 7;
}

std::int64_t Actor::getId() const
{
    return 7;
}

bool Actor::isValid() const
{
    return true;
}

bool Actor::isDead() const
{
    return false;
}

bool Actor::isOnGround() const
{
    return true;
}

bool Actor::isInWater() const
{
    return false;
}

bool Actor::isInLava() const
{
    return false;
}

Location Actor::getLocation() const
{
    return Location{
        .x = 1.0F, .y = 2.0F, .z = 3.0F, .pitch = 0.0F, .yaw = 0.0F, .dimension = rust::String("overworld")};
}

Location Actor::getDimensionLocation() const
{
    return getLocation();
}

Vector Actor::getVelocity() const
{
    return Vector{.x = 1.0F, .y = 0.0F, .z = 0.0F};
}

rust::String Actor::getLevelName() const
{
    return rust::String("world");
}

bool Actor::isNameTagVisible() const
{
    return true;
}

bool Actor::isNameTagAlwaysVisible() const
{
    return true;
}

rust::String Actor::getNameTag() const
{
    return rust::String("tag");
}

rust::String Actor::getScoreTag() const
{
    return rust::String("score");
}

rust::Vec<rust::String> Actor::getScoreboardTags() const
{
    rust::Vec<rust::String> tags;
    tags.push_back(rust::String("first"));
    return tags;
}

bool Actor::addScoreboardTag(rust::Str) const
{
    return true;
}

bool Actor::removeScoreboardTag(rust::Str) const
{
    return false;
}

void Actor::setRotation(float, float) const
{
}

void Actor::setNameTagVisible(bool) const
{
}

void Actor::setNameTagAlwaysVisible(bool) const
{
}

void Actor::setNameTag(rust::Str) const
{
}

void Actor::setScoreTag(rust::Str) const
{
}

bool Actor::teleport(const Location &) const
{
    return true;
}

bool Actor::teleportToActor(const Actor &) const
{
    return true;
}

void Actor::remove() const
{
}

std::unique_ptr<Mob> Actor::asMob() const
{
    return std::unique_ptr<Mob>(new Mob(nullptr));
}

std::unique_ptr<ItemActor> Actor::asItemActor() const
{
    return impl->item ? std::unique_ptr<ItemActor>(new ItemActor(nullptr)) : nullptr;
}

std::unique_ptr<::aegilex::native::player::Player> Actor::asPlayer() const
{
    return std::unique_ptr<::aegilex::native::player::Player>(new ::aegilex::native::player::Player(nullptr));
}

std::unique_ptr<Actor> Actor::clone() const
{
    return std::unique_ptr<Actor>(new Actor(impl->item ? reinterpret_cast<endstone::Actor *>(1) : nullptr));
}

Mob::Mob(endstone::Mob *) noexcept : impl(std::make_shared<class Mob::impl>())
{
}

bool Mob::isGliding() const
{
    return true;
}

std::int32_t Mob::getHealth() const
{
    return 20;
}

std::int32_t Mob::getMaxHealth() const
{
    return 20;
}

void Mob::setHealth(std::int32_t) const
{
}

void Mob::setMaxHealth(std::int32_t) const
{
}

std::unique_ptr<Actor> Mob::asActor() const
{
    return std::unique_ptr<Actor>(new Actor(nullptr));
}

} // namespace aegilex::native::actor
