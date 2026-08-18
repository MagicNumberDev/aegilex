#pragma once

#include "bindings/endstone/actor/actor.h"
#include "bindings/endstone/actor/player.h"

#include <memory>
#include <optional>

namespace endstone {
class PlayerInteractActorEvent;
}

namespace aegilex::native::endstone_binding::events {

// Non-owning, callback-scoped view of PlayerInteractActorEvent.
class PlayerInteractActorEventFacade final {
  public:
    explicit PlayerInteractActorEventFacade(endstone::PlayerInteractActorEvent *event) noexcept;
    ~PlayerInteractActorEventFacade() noexcept = default;

    PlayerInteractActorEventFacade(const PlayerInteractActorEventFacade &) = delete;
    PlayerInteractActorEventFacade &operator=(const PlayerInteractActorEventFacade &) = delete;
    PlayerInteractActorEventFacade(PlayerInteractActorEventFacade &&) = delete;
    PlayerInteractActorEventFacade &operator=(PlayerInteractActorEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> getPlayer() const noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::actor::Actor> getActor() const noexcept;
    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;

  private:
    endstone::PlayerInteractActorEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
