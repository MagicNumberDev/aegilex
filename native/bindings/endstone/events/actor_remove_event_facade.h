#pragma once

#include "bindings/endstone/actor/actor.h"

#include <memory>

namespace endstone {
class ActorRemoveEvent;
}

namespace aegilex::native::endstone_binding::events {

// Non-owning, callback-scoped view of ActorRemoveEvent. v0.11.6 exposes only
// the inherited actor reference and no mutable or cancellable event state.
class ActorRemoveEventFacade final {
  public:
    explicit ActorRemoveEventFacade(endstone::ActorRemoveEvent *event) noexcept;
    ~ActorRemoveEventFacade() noexcept = default;

    ActorRemoveEventFacade(const ActorRemoveEventFacade &) = delete;
    ActorRemoveEventFacade &operator=(const ActorRemoveEventFacade &) = delete;
    ActorRemoveEventFacade(ActorRemoveEventFacade &&) = delete;
    ActorRemoveEventFacade &operator=(ActorRemoveEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::actor::Actor> getActor() const noexcept;

  private:
    endstone::ActorRemoveEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
