#pragma once

#include "rust/cxx.h"

#include <cstdint>
#include <memory>

namespace endstone {
class ItemType;
}

namespace aegilex::native::inventory {

class ItemStack;

// OOP/Pimpl facade over endstone::ItemType. The impl holds only a non-owning
// endstone::ItemType* (the server registry owns the type). Mirrors the layout
// of endstone/inventory/item_type.h.
class ItemType {
  public:
    explicit ItemType(const endstone::ItemType *type) noexcept;
    ~ItemType() noexcept = default;

    ItemType(const ItemType &) = delete;
    ItemType &operator=(const ItemType &) = delete;

    rust::String getTypeId() const;
    rust::String getTranslationKey() const;
    std::int32_t getMaxStackSize() const;
    std::int32_t getMaxDurability() const;
    [[nodiscard]] std::unique_ptr<ItemStack> createItemStack(std::int32_t amount) const;
    [[nodiscard]] const endstone::ItemType *native() const noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl;
};

} // namespace aegilex::native::inventory
