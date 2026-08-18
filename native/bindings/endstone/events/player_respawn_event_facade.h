#pragma once

#include "bindings/endstone/actor/player.h"

#include <memory>

namespace endstone {
class PlayerRespawnEvent;
}

namespace aegilex::native::endstone_binding::events {

// Non-owning, callback-scoped view. v0.11.6 exposes only PlayerEvent::getPlayer().
class PlayerRespawnEventFacade final {
  public:
    explicit PlayerRespawnEventFacade(endstone::PlayerRespawnEvent *event) noexcept;
    ~PlayerRespawnEventFacade() noexcept = default;

    PlayerRespawnEventFacade(const PlayerRespawnEventFacade &) = delete;
    PlayerRespawnEventFacade &operator=(const PlayerRespawnEventFacade &) = delete;
    PlayerRespawnEventFacade(PlayerRespawnEventFacade &&) = delete;
    PlayerRespawnEventFacade &operator=(PlayerRespawnEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> getPlayer() const noexcept;

  private:
    endstone::PlayerRespawnEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
