#pragma once

#include "bindings/endstone/actor/player.h"

#include <memory>
#include <optional>
#include <string>

#include "rust/cxx.h"

namespace endstone {
class PlayerDeathEvent;
}

namespace aegilex::native::endstone_binding::events {

class PlayerDeathEventFacade final {
  public:
    explicit PlayerDeathEventFacade(endstone::PlayerDeathEvent *event) noexcept;
    ~PlayerDeathEventFacade() noexcept = default;

    PlayerDeathEventFacade(const PlayerDeathEventFacade &) = delete;
    PlayerDeathEventFacade &operator=(const PlayerDeathEventFacade &) = delete;
    PlayerDeathEventFacade(PlayerDeathEventFacade &&) = delete;
    PlayerDeathEventFacade &operator=(PlayerDeathEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> getPlayer() const noexcept;
    [[nodiscard]] bool hasDeathMessage() const noexcept;
    [[nodiscard]] rust::String getDeathMessageForRust() const noexcept;
    [[nodiscard]] bool setDeathMessageForRust(bool has_message, rust::Str message) noexcept;

  private:
    endstone::PlayerDeathEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
