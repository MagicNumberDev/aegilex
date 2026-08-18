#pragma once

#include "bindings/endstone/actor/player.h"

#include <memory>
#include <optional>
#include <string>

#include "rust/cxx.h"

namespace endstone {
class PlayerQuitEvent;
}

namespace aegilex::native::endstone_binding::events {

class PlayerQuitEventFacade final {
  public:
    explicit PlayerQuitEventFacade(endstone::PlayerQuitEvent *event) noexcept;
    ~PlayerQuitEventFacade() noexcept = default;

    PlayerQuitEventFacade(const PlayerQuitEventFacade &) = delete;
    PlayerQuitEventFacade &operator=(const PlayerQuitEventFacade &) = delete;
    PlayerQuitEventFacade(PlayerQuitEventFacade &&) = delete;
    PlayerQuitEventFacade &operator=(PlayerQuitEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> getPlayer() const noexcept;
    [[nodiscard]] bool hasQuitMessage() const noexcept;
    [[nodiscard]] rust::String getQuitMessageForRust() const noexcept;
    [[nodiscard]] bool setQuitMessageForRust(bool has_message, rust::Str message) noexcept;

  private:
    endstone::PlayerQuitEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
