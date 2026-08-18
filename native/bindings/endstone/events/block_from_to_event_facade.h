#pragma once

#include "bindings/endstone/level/block.h"

#include <memory>
#include <optional>

namespace endstone {
class BlockFromToEvent;
}

namespace aegilex::native::endstone_binding::events {

class BlockFromToEventFacade final {
  public:
    explicit BlockFromToEventFacade(endstone::BlockFromToEvent *event) noexcept;
    ~BlockFromToEventFacade() noexcept = default;

    BlockFromToEventFacade(const BlockFromToEventFacade &) = delete;
    BlockFromToEventFacade &operator=(const BlockFromToEventFacade &) = delete;
    BlockFromToEventFacade(BlockFromToEventFacade &&) = delete;
    BlockFromToEventFacade &operator=(BlockFromToEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::level::Block> getBlock() const noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::level::Block> getToBlock() const noexcept;
    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;

  private:
    endstone::BlockFromToEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
