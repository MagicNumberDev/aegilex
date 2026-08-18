#pragma once

#include "bindings/endstone/actor/player.h"

#include <memory>
#include <optional>
#include <string>

#include "rust/cxx.h"

namespace endstone {
class PlayerEmoteEvent;
}

namespace aegilex::native::endstone_binding::events {

// Non-owning, callback-scoped view of PlayerEmoteEvent.
class PlayerEmoteEventFacade final {
  public:
    explicit PlayerEmoteEventFacade(endstone::PlayerEmoteEvent *event) noexcept;
    ~PlayerEmoteEventFacade() noexcept = default;

    PlayerEmoteEventFacade(const PlayerEmoteEventFacade &) = delete;
    PlayerEmoteEventFacade &operator=(const PlayerEmoteEventFacade &) = delete;
    PlayerEmoteEventFacade(PlayerEmoteEventFacade &&) = delete;
    PlayerEmoteEventFacade &operator=(PlayerEmoteEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> getPlayer() const noexcept;
    [[nodiscard]] rust::String getEmoteIdForRust() const noexcept;
    [[nodiscard]] bool isMuted() const noexcept;
    [[nodiscard]] bool setMuted(bool muted) noexcept;
    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;

  private:
    endstone::PlayerEmoteEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
