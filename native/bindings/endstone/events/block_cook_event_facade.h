#pragma once

#include "bindings/endstone/inventory/item_stack.h"
#include "bindings/endstone/level/block.h"

#include <memory>
#include <optional>

namespace endstone {
class BlockCookEvent;
}

namespace aegilex::native::endstone_binding::events {

class BlockCookEventFacade final {
  public:
    explicit BlockCookEventFacade(endstone::BlockCookEvent *event) noexcept;
    ~BlockCookEventFacade() noexcept = default;

    BlockCookEventFacade(const BlockCookEventFacade &) = delete;
    BlockCookEventFacade &operator=(const BlockCookEventFacade &) = delete;
    BlockCookEventFacade(BlockCookEventFacade &&) = delete;
    BlockCookEventFacade &operator=(BlockCookEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::level::Block> getBlock() const noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::inventory::ItemStackRef> getSource() const noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::inventory::ItemStackRef> getResult() const noexcept;
    [[nodiscard]] bool setResult(const ::aegilex::native::inventory::ItemStack &result) noexcept;
    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;

  private:
    endstone::BlockCookEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
