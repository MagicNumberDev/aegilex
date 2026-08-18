#pragma once

#include "bindings/endstone/level/block.h"

#include <memory>
#include <optional>

namespace endstone {
class BlockGrowEvent;
}

namespace aegilex::native::endstone_binding::events {

// BlockFormEvent adds no ABI-safe surface beyond BlockGrowEvent. Its BlockState&
// is intentionally not exposed because block-state has no resource contract.
class BlockGrowEventFacade final {
  public:
    explicit BlockGrowEventFacade(endstone::BlockGrowEvent *event) noexcept;
    ~BlockGrowEventFacade() noexcept = default;

    BlockGrowEventFacade(const BlockGrowEventFacade &) = delete;
    BlockGrowEventFacade &operator=(const BlockGrowEventFacade &) = delete;
    BlockGrowEventFacade(BlockGrowEventFacade &&) = delete;
    BlockGrowEventFacade &operator=(BlockGrowEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::level::Block> getBlock() const noexcept;
    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;

  private:
    endstone::BlockGrowEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
