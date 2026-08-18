#pragma once

#include "bindings/endstone/actor/player.h"
#include "bindings/endstone/inventory/item_stack.h"

#include <memory>
#include <optional>

namespace endstone {
class PlayerDropItemEvent;
}

namespace aegilex::native::endstone_binding::events {

// Non-owning, callback-scoped view of PlayerDropItemEvent.
class PlayerDropItemEventFacade final {
  public:
    explicit PlayerDropItemEventFacade(endstone::PlayerDropItemEvent *event) noexcept;
    ~PlayerDropItemEventFacade() noexcept = default;

    PlayerDropItemEventFacade(const PlayerDropItemEventFacade &) = delete;
    PlayerDropItemEventFacade &operator=(const PlayerDropItemEventFacade &) = delete;
    PlayerDropItemEventFacade(PlayerDropItemEventFacade &&) = delete;
    PlayerDropItemEventFacade &operator=(PlayerDropItemEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> getPlayer() const noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::inventory::ItemStackRef> getItem() const noexcept;
    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;

  private:
    endstone::PlayerDropItemEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
