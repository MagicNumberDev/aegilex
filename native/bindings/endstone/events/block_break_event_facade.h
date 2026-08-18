#pragma once

#include "bindings/endstone/actor/player.h"
#include "bindings/endstone/level/block.h"

#include <memory>
#include <optional>

namespace endstone {
class BlockBreakEvent;
}

namespace aegilex::native::endstone_binding::events {

class BlockBreakEventFacade final {
  public:
    explicit BlockBreakEventFacade(endstone::BlockBreakEvent *event) noexcept;
    ~BlockBreakEventFacade() noexcept = default;

    BlockBreakEventFacade(const BlockBreakEventFacade &) = delete;
    BlockBreakEventFacade &operator=(const BlockBreakEventFacade &) = delete;
    BlockBreakEventFacade(BlockBreakEventFacade &&) = delete;
    BlockBreakEventFacade &operator=(BlockBreakEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> getPlayer() const noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::level::Block> getBlock() const noexcept;
    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;

  private:
    endstone::BlockBreakEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
