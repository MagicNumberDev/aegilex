#pragma once

#include "bindings/endstone/actor/player.h"
#include "bindings/endstone/inventory/item_stack.h"
#include "bindings/endstone/level/block.h"

#include <cstdint>
#include <memory>
#include <optional>

namespace endstone {
class PlayerInteractEvent;
}

namespace aegilex::runtime {
struct VectorData;
}

namespace aegilex::native::endstone_binding::events {

// Non-owning, callback-scoped view of PlayerInteractEvent.
class PlayerInteractEventFacade final {
  public:
    explicit PlayerInteractEventFacade(endstone::PlayerInteractEvent *event) noexcept;
    ~PlayerInteractEventFacade() noexcept = default;

    PlayerInteractEventFacade(const PlayerInteractEventFacade &) = delete;
    PlayerInteractEventFacade &operator=(const PlayerInteractEventFacade &) = delete;
    PlayerInteractEventFacade(PlayerInteractEventFacade &&) = delete;
    PlayerInteractEventFacade &operator=(PlayerInteractEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> getPlayer() const noexcept;
    [[nodiscard]] std::uint8_t getAction() const noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::inventory::ItemStackRef> getItem() const noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::level::Block> getBlock() const noexcept;
    [[nodiscard]] std::uint8_t getBlockFace() const noexcept;
    [[nodiscard]] bool hasClickedPosition() const noexcept;
    [[nodiscard]] aegilex::runtime::VectorData getClickedPosition() const noexcept;
    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;

  private:
    endstone::PlayerInteractEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
