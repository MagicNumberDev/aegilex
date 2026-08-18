#pragma once

#include "rust/cxx.h"

#include <cstdint>
#include <memory>

namespace endstone {
class Mob;
}

namespace aegilex::native::actor {

class Actor;

// OOP/Pimpl facade over endstone::Mob; non-owning, mirrors Endstone's Mob API.
// Mirrors the layout of endstone/actor/mob.h.
class Mob {
  public:
    explicit Mob(endstone::Mob *mob) noexcept;
    ~Mob() noexcept = default;

    Mob(const Mob &) = delete;
    Mob &operator=(const Mob &) = delete;

    bool isGliding() const;
    std::int32_t getHealth() const;
    std::int32_t getMaxHealth() const;
    void setHealth(std::int32_t health) const;
    void setMaxHealth(std::int32_t health) const;
    std::unique_ptr<Actor> asActor() const;
    [[nodiscard]] endstone::Mob *native() const noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl;
};

} // namespace aegilex::native::actor
