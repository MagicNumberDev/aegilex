#pragma once

#include "bindings/endstone/actor/player.h"
#include "rust/cxx.h"

#include <optional>
#include <string>
#include <string_view>

namespace endstone {
class PlayerCommandEvent;
}

namespace aegilex::native::endstone_binding::events {

class PlayerCommandEventFacade final {
  public:
    explicit PlayerCommandEventFacade(endstone::PlayerCommandEvent *event) noexcept;
    ~PlayerCommandEventFacade() noexcept = default;

    PlayerCommandEventFacade(const PlayerCommandEventFacade &) = delete;
    PlayerCommandEventFacade &operator=(const PlayerCommandEventFacade &) = delete;

    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;
    [[nodiscard]] std::string getCommand() const noexcept;
    [[nodiscard]] bool setCommand(std::string_view command) noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> getPlayer() const noexcept;

    // CXX bridge adapters preserve the native string/string_view facade API.
    [[nodiscard]] rust::String getCommandForRust() const noexcept;
    [[nodiscard]] bool setCommandForRust(rust::Str command) noexcept;

  private:
    endstone::PlayerCommandEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
