#pragma once

#include "bindings/endstone/level/block.h"

#include <cstdint>
#include <memory>
#include <optional>

namespace endstone {
class BlockPistonEvent;
}

namespace aegilex::native::endstone_binding::events {

// Extend and retract have the same inherited ABI-safe surface.
class BlockPistonEventFacade final {
  public:
    explicit BlockPistonEventFacade(endstone::BlockPistonEvent *event) noexcept;
    ~BlockPistonEventFacade() noexcept = default;

    BlockPistonEventFacade(const BlockPistonEventFacade &) = delete;
    BlockPistonEventFacade &operator=(const BlockPistonEventFacade &) = delete;
    BlockPistonEventFacade(BlockPistonEventFacade &&) = delete;
    BlockPistonEventFacade &operator=(BlockPistonEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::level::Block> getBlock() const noexcept;
    [[nodiscard]] std::uint8_t getDirection() const noexcept;
    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;

  private:
    endstone::BlockPistonEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
