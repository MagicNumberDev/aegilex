#pragma once

#include "bindings/endstone/actor/actor.h"

#include <memory>
#include <optional>
#include <string>

namespace endstone {
class ActorTeleportEvent;
}

namespace aegilex::native {
class HostContext;
}

namespace aegilex::runtime {
struct LocationData;
}

namespace aegilex::native::endstone_binding::events {

class ActorTeleportEventFacade final {
  public:
    ActorTeleportEventFacade(endstone::ActorTeleportEvent *event, HostContext *context) noexcept;
    ~ActorTeleportEventFacade() noexcept = default;

    ActorTeleportEventFacade(const ActorTeleportEventFacade &) = delete;
    ActorTeleportEventFacade &operator=(const ActorTeleportEventFacade &) = delete;
    ActorTeleportEventFacade(ActorTeleportEventFacade &&) = delete;
    ActorTeleportEventFacade &operator=(ActorTeleportEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::actor::Actor> getActor() const noexcept;
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

    endstone::ActorTeleportEvent *event_;
    HostContext *context_;
};

} // namespace aegilex::native::endstone_binding::events
