#pragma once

#include "bindings/endstone/level/block.h"

#include <cstdint>
#include <memory>
#include <optional>
#include <string>
#include <vector>

namespace aegilex::runtime {
}

namespace endstone {
class Block;
class BlockExplodeEvent;
} // namespace endstone

namespace aegilex::native::endstone_binding::events {

// BlockExplodeEvent owns native blocks. Expose independent block clones.
class BlockExplodeEventFacade final {
  public:
    // Test-only fixture constructor. Production dispatch always supplies an event.
    BlockExplodeEventFacade() noexcept;
    explicit BlockExplodeEventFacade(endstone::BlockExplodeEvent *event) noexcept;
    ~BlockExplodeEventFacade() noexcept = default;

    BlockExplodeEventFacade(const BlockExplodeEventFacade &) = delete;
    BlockExplodeEventFacade &operator=(const BlockExplodeEventFacade &) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::level::Block> getBlock() const noexcept;
    [[nodiscard]] std::uint64_t getBlockCount() const noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::level::Block> getAffectedBlock(std::uint64_t index) const noexcept;
    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;
  private:
    endstone::BlockExplodeEvent *event_{};
    endstone::Block *block_{};
    std::vector<std::unique_ptr<::aegilex::native::level::Block>> affected_blocks_;
};

} // namespace aegilex::native::endstone_binding::events
