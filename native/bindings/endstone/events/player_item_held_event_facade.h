#pragma once

#include "bindings/endstone/actor/player.h"

#include <cstdint>
#include <memory>
#include <optional>

namespace endstone {
class PlayerItemHeldEvent;
}

namespace aegilex::native::endstone_binding::events {

// Non-owning, callback-scoped view of PlayerItemHeldEvent. Slots are read-only;
// cancellation is mutable.
class PlayerItemHeldEventFacade final {
  public:
    explicit PlayerItemHeldEventFacade(endstone::PlayerItemHeldEvent *event) noexcept;
    ~PlayerItemHeldEventFacade() noexcept = default;

    PlayerItemHeldEventFacade(const PlayerItemHeldEventFacade &) = delete;
    PlayerItemHeldEventFacade &operator=(const PlayerItemHeldEventFacade &) = delete;
    PlayerItemHeldEventFacade(PlayerItemHeldEventFacade &&) = delete;
    PlayerItemHeldEventFacade &operator=(PlayerItemHeldEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> getPlayer() const noexcept;
    [[nodiscard]] std::int32_t getPreviousSlot() const noexcept;
    [[nodiscard]] std::int32_t getNewSlot() const noexcept;
    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;

  private:
    endstone::PlayerItemHeldEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
