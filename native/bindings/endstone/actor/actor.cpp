#include "actor.h"
#include "item_actor.h"
#include "mob.h"
#include "player.h"
#include "../inventory/item_stack.h"

#include <aegilex-runtime/src/cxx_host_actor.rs.h>

#include <endstone/actor/actor.h>
#include <endstone/actor/item.h>
#include <endstone/actor/mob.h>
#include <endstone/level/dimension.h>
#include <endstone/level/level.h>
#include <endstone/player.h>

#include <optional>
#include <string>

namespace aegilex::native::actor {

class Actor::impl {
  public:
    explicit impl(endstone::Actor *actor) noexcept : actor(actor)
    {
    }

    endstone::Actor *actor;
};

Actor::Actor(endstone::Actor *actor) noexcept : impl(std::make_shared<class Actor::impl>(actor))
{
}

endstone::Actor *Actor::native() const noexcept
{
    return impl == nullptr ? nullptr : impl->actor;
}

rust::String Actor::getName() const
{
    try {
        return rust::String(impl->actor->getName());
    }
    catch (...) {
        return rust::String();
    }
}

rust::String Actor::getType() const
{
    try {
        return rust::String(impl->actor->getType());
    }
    catch (...) {
        return rust::String();
    }
}

std::uint64_t Actor::getRuntimeId() const
{
    try {
        return impl->actor->getRuntimeId();
    }
    catch (...) {
        return 0;
    }
}

std::int64_t Actor::getId() const
{
    try {
        return impl->actor->getId();
    }
    catch (...) {
        return 0;
    }
}

bool Actor::isValid() const
{
    try {
        return impl->actor->isValid();
    }
    catch (...) {
        return false;
    }
}

bool Actor::isDead() const
{
    try {
        return impl->actor->isDead();
    }
    catch (...) {
        return false;
    }
}

bool Actor::isOnGround() const
{
    try {
        return impl->actor->isOnGround();
    }
    catch (...) {
        return false;
    }
}

bool Actor::isInWater() const
{
    try {
        return impl->actor->isInWater();
    }
    catch (...) {
        return false;
    }
}

bool Actor::isInLava() const
{
    try {
        return impl->actor->isInLava();
    }
    catch (...) {
        return false;
    }
}

Location Actor::getLocation() const
{
    try {
        const auto &location = impl->actor->getLocation();
        return Location{.x = location.getX(),
                        .y = location.getY(),
                        .z = location.getZ(),
                        .pitch = location.getPitch(),
                        .yaw = location.getYaw(),
                        .dimension = rust::String(location.getDimension().getName())};
    }
    catch (...) {
        return Location{};
    }
}

Location Actor::getDimensionLocation() const
{
    return getLocation();
}

Vector Actor::getVelocity() const
{
    try {
        const auto &velocity = impl->actor->getVelocity();
        return Vector{.x = velocity.getX(), .y = velocity.getY(), .z = velocity.getZ()};
    }
    catch (...) {
        return Vector{};
    }
}

rust::String Actor::getLevelName() const
{
    try {
        return rust::String(impl->actor->getLevel().getName());
    }
    catch (...) {
        return rust::String();
    }
}

bool Actor::isNameTagVisible() const
{
    try {
        return impl->actor->isNameTagVisible();
    }
    catch (...) {
        return false;
    }
}

bool Actor::isNameTagAlwaysVisible() const
{
    try {
        return impl->actor->isNameTagAlwaysVisible();
    }
    catch (...) {
        return false;
    }
}

rust::String Actor::getNameTag() const
{
    try {
        return rust::String(impl->actor->getNameTag());
    }
    catch (...) {
        return rust::String();
    }
}

rust::String Actor::getScoreTag() const
{
    try {
        return rust::String(impl->actor->getScoreTag());
    }
    catch (...) {
        return rust::String();
    }
}

rust::Vec<rust::String> Actor::getScoreboardTags() const
{
    rust::Vec<rust::String> tags;
    try {
        for (const auto &tag : impl->actor->getScoreboardTags()) {
            tags.push_back(rust::String(tag));
        }
    }
    catch (...) {
    }
    return tags;
}

bool Actor::addScoreboardTag(const rust::Str tag) const
{
    try {
        return impl->actor->addScoreboardTag(std::string(tag));
    }
    catch (...) {
        return false;
    }
}

bool Actor::removeScoreboardTag(const rust::Str tag) const
{
    try {
        return impl->actor->removeScoreboardTag(std::string(tag));
    }
    catch (...) {
        return false;
    }
}

void Actor::setRotation(const float yaw, const float pitch) const
{
    try {
        impl->actor->setRotation(yaw, pitch);
    }
    catch (...) {
    }
}

void Actor::setNameTagVisible(const bool visible) const
{
    try {
        impl->actor->setNameTagVisible(visible);
    }
    catch (...) {
    }
}

void Actor::setNameTagAlwaysVisible(const bool always_visible) const
{
    try {
        impl->actor->setNameTagAlwaysVisible(always_visible);
    }
    catch (...) {
    }
}

void Actor::setNameTag(const rust::Str name_tag) const
{
    try {
        impl->actor->setNameTag(std::string(name_tag));
    }
    catch (...) {
    }
}

void Actor::setScoreTag(const rust::Str score_tag) const
{
    try {
        impl->actor->setScoreTag(std::string(score_tag));
    }
    catch (...) {
    }
}

bool Actor::teleport(const Location &location) const
{
    try {
        auto *dimension = impl->actor->getLevel().getDimension(std::string(location.dimension));
        if (dimension == nullptr) {
            return false;
        }
        const endstone::Location target{*dimension, location.x, location.y, location.z, location.pitch, location.yaw};
        return impl->actor->teleport(target);
    }
    catch (...) {
        return false;
    }
}

bool Actor::teleportToActor(const Actor &target) const
{
    try {
        return impl->actor->teleport(*target.impl->actor);
    }
    catch (...) {
        return false;
    }
}

void Actor::remove() const
{
    try {
        impl->actor->remove();
    }
    catch (...) {
    }
}

std::unique_ptr<Mob> Actor::asMob() const
{
    try {
        auto *mob = impl->actor->asMob();
        if (mob == nullptr) {
            return std::unique_ptr<Mob>();
        }
        return std::unique_ptr<Mob>(new Mob(mob));
    }
    catch (...) {
        return std::unique_ptr<Mob>();
    }
}

std::unique_ptr<ItemActor> Actor::asItemActor() const
{
    try {
        auto *item = impl->actor->asItem();
        if (item == nullptr) {
            return {};
        }
        return std::make_unique<ItemActor>(item);
    }
    catch (...) {
        return {};
    }
}

std::unique_ptr<::aegilex::native::player::Player> Actor::asPlayer() const
{
    try {
        if (impl == nullptr || impl->actor == nullptr) {
            return {};
        }
        auto *player = dynamic_cast<endstone::Player *>(impl->actor);
        return player == nullptr ? std::unique_ptr<::aegilex::native::player::Player>()
                                 : std::make_unique<::aegilex::native::player::Player>(player);
    }
    catch (...) {
        return {};
    }
}

std::unique_ptr<Actor> Actor::clone() const
{
    try {
        return std::make_unique<Actor>(impl->actor);
    }
    catch (...) {
        return std::unique_ptr<Actor>();
    }
}

} // namespace aegilex::native::actor
