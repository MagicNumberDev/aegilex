#pragma once

#include "rust/cxx.h"

#include <cstdint>
#include <memory>

namespace endstone {
class Inventory;
class ItemStack;
} // namespace endstone

namespace aegilex::native::inventory {

class ItemStack;

// OOP/Pimpl facade over endstone::Inventory. The impl holds only a non-owning
// endstone::Inventory* (BDS owns the inventory); no VM semantics live here.
// Mirrors the layout of endstone/inventory/inventory.h.
class Inventory {
  public:
    explicit Inventory(endstone::Inventory *inventory) noexcept;
    ~Inventory() noexcept = default;

    Inventory(const Inventory &) = delete;
    Inventory &operator=(const Inventory &) = delete;

    std::int32_t getSize() const;
    std::int32_t getMaxStackSize() const;
    [[nodiscard]] std::unique_ptr<ItemStack> getItem(std::int32_t index) const;
    void setItem(std::int32_t index, const ItemStack &item) const;
    void clear() const;
    void clearIndex(std::int32_t index) const;
    [[nodiscard]] std::unique_ptr<ItemStack> addItem(const ItemStack &item) const;
    [[nodiscard]] std::unique_ptr<ItemStack> removeItem(const ItemStack &item) const;
    [[nodiscard]] bool containsType(rust::Str type_id) const;
    [[nodiscard]] bool containsStack(const ItemStack &item) const;
    [[nodiscard]] bool containsAtLeastType(rust::Str type_id, std::int32_t amount) const;
    [[nodiscard]] bool containsAtLeastStack(const ItemStack &item, std::int32_t amount) const;
    [[nodiscard]] std::int32_t firstType(rust::Str type_id) const;
    [[nodiscard]] std::int32_t firstStack(const ItemStack &item) const;
    std::int32_t firstEmpty() const;
    bool isEmpty() const;
    void removeType(rust::Str type_id) const;
    void removeStack(const ItemStack &item) const;
    [[nodiscard]] endstone::Inventory *native() const noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl;
};

} // namespace aegilex::native::inventory
