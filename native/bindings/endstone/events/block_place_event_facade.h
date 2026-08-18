#pragma once

#include "bindings/endstone/actor/player.h"
#include "bindings/endstone/level/block.h"

#include <memory>
#include <optional>

namespace endstone {
class BlockPlaceEvent;
}

namespace aegilex::native::endstone_binding::events {

class BlockPlaceEventFacade final {
  public:
    explicit BlockPlaceEventFacade(endstone::BlockPlaceEvent *event) noexcept;
    ~BlockPlaceEventFacade() noexcept = default;

    BlockPlaceEventFacade(const BlockPlaceEventFacade &) = delete;
    BlockPlaceEventFacade &operator=(const BlockPlaceEventFacade &) = delete;
    BlockPlaceEventFacade(BlockPlaceEventFacade &&) = delete;
    BlockPlaceEventFacade &operator=(BlockPlaceEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> getPlayer() const noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::level::Block> getBlockReplaced() const noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::level::Block> getBlockAgainst() const noexcept;
    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;

  private:
    endstone::BlockPlaceEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
