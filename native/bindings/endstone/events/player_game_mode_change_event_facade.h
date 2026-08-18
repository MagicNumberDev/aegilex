#pragma once

#include "bindings/endstone/actor/player.h"

#include <cstdint>
#include <memory>
#include <optional>

namespace endstone {
class PlayerGameModeChangeEvent;
}

namespace aegilex::native::endstone_binding::events {

// Non-owning, callback-scoped view of PlayerGameModeChangeEvent. Endstone
// exposes no new-game-mode setter; only cancellation is mutable.
class PlayerGameModeChangeEventFacade final {
  public:
    explicit PlayerGameModeChangeEventFacade(endstone::PlayerGameModeChangeEvent *event) noexcept;
    ~PlayerGameModeChangeEventFacade() noexcept = default;

    PlayerGameModeChangeEventFacade(const PlayerGameModeChangeEventFacade &) = delete;
    PlayerGameModeChangeEventFacade &operator=(const PlayerGameModeChangeEventFacade &) = delete;
    PlayerGameModeChangeEventFacade(PlayerGameModeChangeEventFacade &&) = delete;
    PlayerGameModeChangeEventFacade &operator=(PlayerGameModeChangeEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> getPlayer() const noexcept;
    [[nodiscard]] std::uint8_t getNewGameMode() const noexcept;
    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;

  private:
    endstone::PlayerGameModeChangeEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
