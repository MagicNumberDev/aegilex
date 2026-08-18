#pragma once

#include "rust/cxx.h"

#include <cstddef>
#include <cstdint>
#include <memory>
#include <string>

namespace endstone {
class Actor;
class Mob;
class Player;
} // namespace endstone

namespace aegilex::native::player {
class Player;
}

namespace aegilex::native::actor {

struct Location;
struct Vector;

class Actor;
class Mob;       // defined in mob.h
class ItemActor; // defined in item_actor.h

// OOP/Pimpl facade over endstone::Actor. The impl holds only a non-owning
// endstone::Actor* (BDS owns the actor); no VM semantics live here.
class Actor {
  public:
    explicit Actor(endstone::Actor *actor) noexcept;
    ~Actor() noexcept = default;

    Actor(const Actor &) = delete;
    Actor &operator=(const Actor &) = delete;

    rust::String getName() const;
    rust::String getType() const;
    std::uint64_t getRuntimeId() const;
    std::int64_t getId() const;
    bool isValid() const;
    bool isDead() const;
    bool isOnGround() const;
    bool isInWater() const;
    bool isInLava() const;
    Location getLocation() const;
    Location getDimensionLocation() const;
    Vector getVelocity() const;
    rust::String getLevelName() const;
    bool isNameTagVisible() const;
    bool isNameTagAlwaysVisible() const;
    rust::String getNameTag() const;
    rust::String getScoreTag() const;
    rust::Vec<rust::String> getScoreboardTags() const;
    bool addScoreboardTag(rust::Str tag) const;
    bool removeScoreboardTag(rust::Str tag) const;
    void setRotation(float yaw, float pitch) const;
    void setNameTagVisible(bool visible) const;
    void setNameTagAlwaysVisible(bool always_visible) const;
    void setNameTag(rust::Str name_tag) const;
    void setScoreTag(rust::Str score_tag) const;
    bool teleport(const Location &location) const;
    bool teleportToActor(const Actor &target) const;
    void remove() const;
    std::unique_ptr<Mob> asMob() const;
    std::unique_ptr<ItemActor> asItemActor() const;
    std::unique_ptr<::aegilex::native::player::Player> asPlayer() const;
    [[nodiscard]] std::unique_ptr<Actor> clone() const;
    [[nodiscard]] endstone::Actor *native() const noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl;
};

} // namespace aegilex::native::actor
