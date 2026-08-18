#pragma once

#include "bindings/endstone/actor/actor.h"

#include <memory>
#include <optional>

namespace endstone {
class ActorKnockbackEvent;
}

namespace aegilex::runtime {
struct VectorData;
}

namespace aegilex::native::endstone_binding::events {

// Non-owning, callback-scoped view of the live native event.
class ActorKnockbackEventFacade final {
  public:
    explicit ActorKnockbackEventFacade(endstone::ActorKnockbackEvent *event) noexcept;
    ~ActorKnockbackEventFacade() noexcept = default;

    ActorKnockbackEventFacade(const ActorKnockbackEventFacade &) = delete;
    ActorKnockbackEventFacade &operator=(const ActorKnockbackEventFacade &) = delete;
    ActorKnockbackEventFacade(ActorKnockbackEventFacade &&) = delete;
    ActorKnockbackEventFacade &operator=(ActorKnockbackEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::actor::Actor> getActor() const noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::actor::Actor> getSource() const noexcept;
    [[nodiscard]] aegilex::runtime::VectorData getKnockback() const noexcept;
    [[nodiscard]] bool setKnockback(const aegilex::runtime::VectorData &knockback) noexcept;
    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;

  private:
    struct Vector {
        float x{};
        float y{};
        float z{};
    };

    endstone::ActorKnockbackEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
