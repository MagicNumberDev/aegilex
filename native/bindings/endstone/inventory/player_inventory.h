#pragma once

#include "rust/cxx.h"

#include <cstdint>
#include <memory>

namespace endstone {
class PlayerInventory;
}

namespace aegilex::native::inventory {

class Inventory;
class ItemStack;

// OOP/Pimpl facade over endstone::PlayerInventory. The impl holds only a
// non-owning endstone::PlayerInventory* (BDS owns the inventory). Mirrors the
// layout of endstone/inventory/player_inventory.h.
class PlayerInventory {
  public:
    explicit PlayerInventory(endstone::PlayerInventory *inventory) noexcept;
    ~PlayerInventory() noexcept = default;

    PlayerInventory(const PlayerInventory &) = delete;
    PlayerInventory &operator=(const PlayerInventory &) = delete;

    // Produces a non-owning base-inventory view of the same BDS-owned object.
    [[nodiscard]] std::unique_ptr<Inventory> asInventory() const;
    std::int32_t getHeldItemSlot() const;
    void setHeldItemSlot(std::int32_t slot) const;
    [[nodiscard]] std::unique_ptr<ItemStack> getHelmet() const;
    void setHelmet(const ItemStack &item) const;
    void clearHelmet() const;
    [[nodiscard]] std::unique_ptr<ItemStack> getChestplate() const;
    void setChestplate(const ItemStack &item) const;
    void clearChestplate() const;
    [[nodiscard]] std::unique_ptr<ItemStack> getLeggings() const;
    void setLeggings(const ItemStack &item) const;
    void clearLeggings() const;
    [[nodiscard]] std::unique_ptr<ItemStack> getBoots() const;
    void setBoots(const ItemStack &item) const;
    void clearBoots() const;
    [[nodiscard]] std::unique_ptr<ItemStack> getItemInMainHand() const;
    void setItemInMainHand(const ItemStack &item) const;
    void clearItemInMainHand() const;
    [[nodiscard]] std::unique_ptr<ItemStack> getItemInOffHand() const;
    void setItemInOffHand(const ItemStack &item) const;
    void clearItemInOffHand() const;
    [[nodiscard]] endstone::PlayerInventory *native() const noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl;
};

} // namespace aegilex::native::inventory
