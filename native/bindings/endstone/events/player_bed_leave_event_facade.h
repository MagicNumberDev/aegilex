#pragma once

#include "bindings/endstone/actor/player.h"
#include "bindings/endstone/level/block.h"

#include <memory>

namespace endstone {
class PlayerBedLeaveEvent;
}

namespace aegilex::native::endstone_binding::events {

class PlayerBedLeaveEventFacade final {
  public:
    explicit PlayerBedLeaveEventFacade(endstone::PlayerBedLeaveEvent *event) noexcept;
    ~PlayerBedLeaveEventFacade() noexcept = default;

    PlayerBedLeaveEventFacade(const PlayerBedLeaveEventFacade &) = delete;
    PlayerBedLeaveEventFacade &operator=(const PlayerBedLeaveEventFacade &) = delete;
    PlayerBedLeaveEventFacade(PlayerBedLeaveEventFacade &&) = delete;
    PlayerBedLeaveEventFacade &operator=(PlayerBedLeaveEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> getPlayer() const noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::level::Block> getBed() const noexcept;

  private:
    endstone::PlayerBedLeaveEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
