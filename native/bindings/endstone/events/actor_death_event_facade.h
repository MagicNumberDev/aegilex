#pragma once

#include "bindings/endstone/actor/actor.h"
#include "rust/cxx.h"

#include <memory>

namespace endstone {
class ActorDeathEvent;
}

namespace aegilex::native::endstone_binding::events {

// Non-owning, callback-scoped view of ActorDeathEvent. v0.11.6 exposes no
// mutable or cancellable death-event state.
class ActorDeathEventFacade final {
  public:
    explicit ActorDeathEventFacade(endstone::ActorDeathEvent *event) noexcept;
    ~ActorDeathEventFacade() noexcept = default;

    ActorDeathEventFacade(const ActorDeathEventFacade &) = delete;
    ActorDeathEventFacade &operator=(const ActorDeathEventFacade &) = delete;
    ActorDeathEventFacade(ActorDeathEventFacade &&) = delete;
    ActorDeathEventFacade &operator=(ActorDeathEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::actor::Actor> getActor() const noexcept;

  private:
    endstone::ActorDeathEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
