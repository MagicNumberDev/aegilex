#pragma once

#include "bindings/endstone/actor/player.h"
#include "bindings/endstone/level/block.h"

#include <memory>
#include <optional>

namespace endstone {
class PlayerBedEnterEvent;
}

namespace aegilex::native::endstone_binding::events {

class PlayerBedEnterEventFacade final {
  public:
    explicit PlayerBedEnterEventFacade(endstone::PlayerBedEnterEvent *event) noexcept;
    ~PlayerBedEnterEventFacade() noexcept = default;

    PlayerBedEnterEventFacade(const PlayerBedEnterEventFacade &) = delete;
    PlayerBedEnterEventFacade &operator=(const PlayerBedEnterEventFacade &) = delete;
    PlayerBedEnterEventFacade(PlayerBedEnterEventFacade &&) = delete;
    PlayerBedEnterEventFacade &operator=(PlayerBedEnterEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> getPlayer() const noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::level::Block> getBed() const noexcept;
    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;

  private:
    endstone::PlayerBedEnterEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
