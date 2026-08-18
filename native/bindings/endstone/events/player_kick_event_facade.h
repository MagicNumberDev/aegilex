#pragma once

#include "bindings/endstone/actor/player.h"
#include "rust/cxx.h"

#include <optional>
#include <string>
#include <string_view>

namespace endstone {
class PlayerKickEvent;
}

namespace aegilex::native::endstone_binding::events {

class PlayerKickEventFacade final {
  public:
    explicit PlayerKickEventFacade(endstone::PlayerKickEvent *event) noexcept;
    ~PlayerKickEventFacade() noexcept = default;

    PlayerKickEventFacade(const PlayerKickEventFacade &) = delete;
    PlayerKickEventFacade &operator=(const PlayerKickEventFacade &) = delete;

    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;
    [[nodiscard]] std::string getReason() const noexcept;
    [[nodiscard]] bool setReason(std::string_view reason) noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> getPlayer() const noexcept;

    // CXX bridge adapters preserve the native string/string_view facade API.
    [[nodiscard]] rust::String getReasonForRust() const noexcept;
    [[nodiscard]] bool setReasonForRust(rust::Str reason) noexcept;

  private:
    endstone::PlayerKickEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
