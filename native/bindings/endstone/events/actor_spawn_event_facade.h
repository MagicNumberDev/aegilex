#pragma once

#include "bindings/endstone/actor/actor.h"

#include <memory>
#include <optional>

namespace endstone {
class ActorSpawnEvent;
}

namespace aegilex::native::endstone_binding::events {

// Non-owning, callback-scoped view of ActorSpawnEvent. v0.11.6 exposes the
// inherited actor reference plus Cancellable<ActorEvent<Actor>> state.
class ActorSpawnEventFacade final {
  public:
    explicit ActorSpawnEventFacade(endstone::ActorSpawnEvent *event) noexcept;
    ~ActorSpawnEventFacade() noexcept = default;

    ActorSpawnEventFacade(const ActorSpawnEventFacade &) = delete;
    ActorSpawnEventFacade &operator=(const ActorSpawnEventFacade &) = delete;
    ActorSpawnEventFacade(ActorSpawnEventFacade &&) = delete;
    ActorSpawnEventFacade &operator=(ActorSpawnEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::actor::Actor> getActor() const noexcept;
    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;

  private:
    endstone::ActorSpawnEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
