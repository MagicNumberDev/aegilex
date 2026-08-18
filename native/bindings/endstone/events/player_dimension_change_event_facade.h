#pragma once

#include "bindings/endstone/actor/player.h"
#include "rust/cxx.h"

#include <memory>

namespace endstone {
class PlayerDimensionChangeEvent;
}

namespace aegilex::native::endstone_binding::events {

// Non-owning, callback-scoped view. Dimension names are copied for the guest;
// v0.11.6 exposes no cancellation or mutation on this event.
class PlayerDimensionChangeEventFacade final {
  public:
    explicit PlayerDimensionChangeEventFacade(endstone::PlayerDimensionChangeEvent *event) noexcept;
    ~PlayerDimensionChangeEventFacade() noexcept = default;

    PlayerDimensionChangeEventFacade(const PlayerDimensionChangeEventFacade &) = delete;
    PlayerDimensionChangeEventFacade &operator=(const PlayerDimensionChangeEventFacade &) = delete;
    PlayerDimensionChangeEventFacade(PlayerDimensionChangeEventFacade &&) = delete;
    PlayerDimensionChangeEventFacade &operator=(PlayerDimensionChangeEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> getPlayer() const noexcept;
    [[nodiscard]] rust::String getFromForRust() const noexcept;
    [[nodiscard]] rust::String getToForRust() const noexcept;

  private:
    endstone::PlayerDimensionChangeEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
