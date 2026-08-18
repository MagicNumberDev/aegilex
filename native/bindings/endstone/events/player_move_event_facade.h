#pragma once

#include "bindings/endstone/actor/player.h"

#include <memory>
#include <optional>
#include <string>

namespace endstone {
class PlayerMoveEvent;
}

namespace aegilex::native {
class HostContext;
}

namespace aegilex::runtime {
struct LocationData;
}

namespace aegilex::native::endstone_binding::events {

// Shared callback-scoped facade for Endstone's player move event hierarchy.
class PlayerMoveEventFacade final {
  public:
    PlayerMoveEventFacade(endstone::PlayerMoveEvent *event, HostContext *context) noexcept;
    ~PlayerMoveEventFacade() noexcept = default;

    PlayerMoveEventFacade(const PlayerMoveEventFacade &) = delete;
    PlayerMoveEventFacade &operator=(const PlayerMoveEventFacade &) = delete;
    PlayerMoveEventFacade(PlayerMoveEventFacade &&) = delete;
    PlayerMoveEventFacade &operator=(PlayerMoveEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> getPlayer() const noexcept;
    [[nodiscard]] aegilex::runtime::LocationData getFrom() const noexcept;
    [[nodiscard]] aegilex::runtime::LocationData getTo() const noexcept;
    [[nodiscard]] bool setFrom(const aegilex::runtime::LocationData &location) noexcept;
    [[nodiscard]] bool setTo(const aegilex::runtime::LocationData &location) noexcept;
    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;

  private:
    struct Location {
        std::string dimension;
        float x{};
        float y{};
        float z{};
        float pitch{};
        float yaw{};
    };

    endstone::PlayerMoveEvent *event_;
    HostContext *context_;
};

} // namespace aegilex::native::endstone_binding::events
