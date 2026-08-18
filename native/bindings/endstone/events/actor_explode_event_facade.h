#pragma once

#include "bindings/endstone/actor/actor.h"
#include "bindings/endstone/level/block.h"

#include <cstdint>
#include <memory>
#include <optional>
#include <string>
#include <vector>

namespace aegilex::runtime {
struct LocationData;
} // namespace aegilex::runtime

namespace endstone {
class ActorExplodeEvent;
}

namespace aegilex::native::endstone_binding::events {

// ActorExplodeEvent owns native blocks. Expose independent block clones.
class ActorExplodeEventFacade final {
  public:
    // Test-only fixture constructor. Production dispatch always supplies an event.
    ActorExplodeEventFacade() noexcept;
    explicit ActorExplodeEventFacade(endstone::ActorExplodeEvent *event) noexcept;
    ~ActorExplodeEventFacade() noexcept = default;

    ActorExplodeEventFacade(const ActorExplodeEventFacade &) = delete;
    ActorExplodeEventFacade &operator=(const ActorExplodeEventFacade &) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::actor::Actor> getActor() const noexcept;
    [[nodiscard]] aegilex::runtime::LocationData getLocation() const noexcept;
    [[nodiscard]] std::uint64_t getBlockCount() const noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::level::Block> getBlock(std::uint64_t index) const noexcept;
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

    endstone::ActorExplodeEvent *event_;
    Location location_;
    std::vector<std::unique_ptr<::aegilex::native::level::Block>> blocks_;
};

} // namespace aegilex::native::endstone_binding::events
