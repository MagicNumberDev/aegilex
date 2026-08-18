#pragma once

#include "bindings/endstone/actor/actor.h"
#include "bindings/endstone/actor/player.h"

#include <memory>
#include <optional>

namespace endstone {
class PlayerPickupItemEvent;
}

namespace aegilex::native::endstone_binding::events {

// Non-owning, callback-scoped view of PlayerPickupItemEvent. Player and item
// facades are independent views of live event references.
class PlayerPickupItemEventFacade final {
  public:
    explicit PlayerPickupItemEventFacade(endstone::PlayerPickupItemEvent *event) noexcept;
    ~PlayerPickupItemEventFacade() noexcept = default;

    PlayerPickupItemEventFacade(const PlayerPickupItemEventFacade &) = delete;
    PlayerPickupItemEventFacade &operator=(const PlayerPickupItemEventFacade &) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> getPlayer() const noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::actor::Actor> getItemActor() const noexcept;
    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;

  private:
    endstone::PlayerPickupItemEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
