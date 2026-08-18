#pragma once

#include "bindings/endstone/actor/player.h"
#include "bindings/endstone/inventory/item_stack.h"

#include <cstdint>
#include <memory>
#include <optional>

namespace endstone {
class PlayerItemConsumeEvent;
}

namespace aegilex::native::endstone_binding::events {

// Non-owning, callback-scoped view of PlayerItemConsumeEvent.
class PlayerItemConsumeEventFacade final {
  public:
    explicit PlayerItemConsumeEventFacade(endstone::PlayerItemConsumeEvent *event) noexcept;
    ~PlayerItemConsumeEventFacade() noexcept = default;

    PlayerItemConsumeEventFacade(const PlayerItemConsumeEventFacade &) = delete;
    PlayerItemConsumeEventFacade &operator=(const PlayerItemConsumeEventFacade &) = delete;
    PlayerItemConsumeEventFacade(PlayerItemConsumeEventFacade &&) = delete;
    PlayerItemConsumeEventFacade &operator=(PlayerItemConsumeEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> getPlayer() const noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::inventory::ItemStackRef> getItem() const noexcept;
    [[nodiscard]] std::uint8_t getHand() const noexcept;
    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;

  private:
    endstone::PlayerItemConsumeEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
