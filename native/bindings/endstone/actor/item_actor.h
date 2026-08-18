#pragma once

#include "rust/cxx.h"
#include "../inventory/item_stack.h"

#include <cstdint>
#include <memory>
#include <optional>

namespace endstone {
class Item;
}

namespace aegilex::native::actor {

class Actor;

// Non-owning facade over endstone::Item. Its native pointer is owned by the
// Actor facade's underlying Endstone actor and is valid only for that scope.
class ItemActor {
  public:
    explicit ItemActor(endstone::Item *item) noexcept;
    ~ItemActor() noexcept = default;

    ItemActor(const ItemActor &) = delete;
    ItemActor &operator=(const ItemActor &) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::inventory::ItemStack> getItemStack() const;
    bool setItemStack(const ::aegilex::native::inventory::ItemStack &stack) const;
    std::int32_t getPickupDelay() const;
    bool setPickupDelay(std::int32_t delay) const;
    bool isUnlimitedLifetime() const;
    bool setUnlimitedLifetime(bool unlimited) const;
    [[nodiscard]] bool getThrower(bool &has_thrower, std::int64_t &thrower) const;
    bool setThrower(bool has_thrower, std::int64_t thrower) const;
    [[nodiscard]] std::unique_ptr<Actor> asActor() const;

  private:
    class impl;
    std::shared_ptr<impl> impl;
};

} // namespace aegilex::native::actor
