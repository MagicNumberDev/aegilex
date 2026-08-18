#pragma once

#include "bindings/endstone/actor/actor.h"
#include "rust/cxx.h"

#include <cstdint>
#include <memory>
#include <optional>
#include <string>

namespace endstone {
class ActorDamageEvent;
}

namespace aegilex::runtime {
struct DamageSourceData;
}

namespace aegilex::native::endstone_binding::events {

class ActorDamageEventFacade final {
  public:
    explicit ActorDamageEventFacade(endstone::ActorDamageEvent *event) noexcept;
    ~ActorDamageEventFacade() noexcept = default;

    ActorDamageEventFacade(const ActorDamageEventFacade &) = delete;
    ActorDamageEventFacade &operator=(const ActorDamageEventFacade &) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::actor::Actor> getActor() const noexcept;
    [[nodiscard]] float getDamage() const noexcept;
    [[nodiscard]] bool setDamage(float damage) noexcept;
    [[nodiscard]] aegilex::runtime::DamageSourceData getDamageSource() const noexcept;
    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;

  private:
    endstone::ActorDamageEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
