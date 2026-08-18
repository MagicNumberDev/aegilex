#pragma once

#include "bindings/endstone/actor/player.h"
#include "rust/cxx.h"

#include <optional>
#include <string>
#include <string_view>

namespace endstone {
class PlayerLoginEvent;
}

namespace aegilex::native::endstone_binding::events {

class PlayerLoginEventFacade final {
  public:
    explicit PlayerLoginEventFacade(endstone::PlayerLoginEvent *event) noexcept;
    ~PlayerLoginEventFacade() noexcept = default;

    PlayerLoginEventFacade(const PlayerLoginEventFacade &) = delete;
    PlayerLoginEventFacade &operator=(const PlayerLoginEventFacade &) = delete;

    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;
    [[nodiscard]] std::string getKickMessage() const noexcept;
    [[nodiscard]] bool setKickMessage(std::string_view message) noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> getPlayer() const noexcept;

    // CXX bridge adapters preserve the native string/string_view facade API.
    [[nodiscard]] rust::String getKickMessageForRust() const noexcept;
    [[nodiscard]] bool setKickMessageForRust(rust::Str message) noexcept;

  private:
    endstone::PlayerLoginEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
