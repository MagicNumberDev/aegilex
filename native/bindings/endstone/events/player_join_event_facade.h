#pragma once

#include "bindings/endstone/actor/player.h"

#include <memory>
#include <optional>
#include <string>

#include "rust/cxx.h"

namespace endstone {
class PlayerJoinEvent;
}

namespace aegilex::native::endstone_binding::events {

class PlayerJoinEventFacade final {
  public:
    explicit PlayerJoinEventFacade(endstone::PlayerJoinEvent *event) noexcept;
    ~PlayerJoinEventFacade() noexcept = default;

    PlayerJoinEventFacade(const PlayerJoinEventFacade &) = delete;
    PlayerJoinEventFacade &operator=(const PlayerJoinEventFacade &) = delete;
    PlayerJoinEventFacade(PlayerJoinEventFacade &&) = delete;
    PlayerJoinEventFacade &operator=(PlayerJoinEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> getPlayer() const noexcept;
    [[nodiscard]] bool hasJoinMessage() const noexcept;
    [[nodiscard]] rust::String getJoinMessageForRust() const noexcept;
    [[nodiscard]] bool setJoinMessageForRust(bool has_message, rust::Str message) noexcept;

  private:
    endstone::PlayerJoinEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
