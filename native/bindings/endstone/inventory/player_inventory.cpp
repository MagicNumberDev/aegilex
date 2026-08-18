#include "player_inventory.h"

#include "inventory.h"
#include "item_stack.h"

#include <endstone/inventory/player_inventory.h>
#include <endstone/player.h>

#include <optional>
#include <string>

namespace aegilex::native::inventory {

class PlayerInventory::impl {
  public:
    explicit impl(endstone::PlayerInventory *inventory) noexcept : inventory(inventory)
    {
    }

    endstone::PlayerInventory *inventory;
};

PlayerInventory::PlayerInventory(endstone::PlayerInventory *inventory) noexcept
    : impl(std::make_shared<class PlayerInventory::impl>(inventory))
{
}

endstone::PlayerInventory *PlayerInventory::native() const noexcept
{
    return impl == nullptr ? nullptr : impl->inventory;
}

std::unique_ptr<Inventory> PlayerInventory::asInventory() const
{
    try {
        return native() == nullptr ? std::unique_ptr<Inventory>()
                                   : std::make_unique<Inventory>(static_cast<endstone::Inventory *>(native()));
    }
    catch (...) {
        return {};
    }
}

std::int32_t PlayerInventory::getHeldItemSlot() const
{
    try {
        return impl->inventory->getHeldItemSlot();
    }
    catch (...) {
        return 0;
    }
}

void PlayerInventory::setHeldItemSlot(const std::int32_t slot) const
{
    try {
        if (slot >= 0 && slot <= 8) {
            impl->inventory->setHeldItemSlot(slot);
        }
    }
    catch (...) {
    }
}

namespace {

std::unique_ptr<ItemStack> get_stack(const std::optional<endstone::ItemStack> &item) noexcept
{
    try {
        return item.has_value() ? std::unique_ptr<ItemStack>(new ItemStack(*item)) : std::unique_ptr<ItemStack>();
    }
    catch (...) {
        return std::unique_ptr<ItemStack>();
    }
}

void clear_stack(endstone::PlayerInventory &inventory,
                 void (endstone::PlayerInventory::*setter)(std::optional<endstone::ItemStack>)) noexcept
{
    try {
        (inventory.*setter)(std::nullopt);
    }
    catch (...) {
    }
}

} // namespace

std::unique_ptr<ItemStack> PlayerInventory::getHelmet() const
{
    try {
        return get_stack(impl->inventory->getHelmet());
    }
    catch (...) {
        return std::unique_ptr<ItemStack>();
    }
}

void PlayerInventory::setHelmet(const ItemStack &item) const
{
    try {
        if (item.native() != nullptr) {
            impl->inventory->setHelmet(std::optional<endstone::ItemStack>{*item.native()});
        }
    }
    catch (...) {
    }
}

void PlayerInventory::clearHelmet() const
{
    clear_stack(*impl->inventory, &endstone::PlayerInventory::setHelmet);
}

std::unique_ptr<ItemStack> PlayerInventory::getChestplate() const
{
    try {
        return get_stack(impl->inventory->getChestplate());
    }
    catch (...) {
        return std::unique_ptr<ItemStack>();
    }
}

void PlayerInventory::setChestplate(const ItemStack &item) const
{
    try {
        if (item.native() != nullptr) {
            impl->inventory->setChestplate(std::optional<endstone::ItemStack>{*item.native()});
        }
    }
    catch (...) {
    }
}

void PlayerInventory::clearChestplate() const
{
    clear_stack(*impl->inventory, &endstone::PlayerInventory::setChestplate);
}

std::unique_ptr<ItemStack> PlayerInventory::getLeggings() const
{
    try {
        return get_stack(impl->inventory->getLeggings());
    }
    catch (...) {
        return std::unique_ptr<ItemStack>();
    }
}

void PlayerInventory::setLeggings(const ItemStack &item) const
{
    try {
        if (item.native() != nullptr) {
            impl->inventory->setLeggings(std::optional<endstone::ItemStack>{*item.native()});
        }
    }
    catch (...) {
    }
}

void PlayerInventory::clearLeggings() const
{
    clear_stack(*impl->inventory, &endstone::PlayerInventory::setLeggings);
}

std::unique_ptr<ItemStack> PlayerInventory::getBoots() const
{
    try {
        return get_stack(impl->inventory->getBoots());
    }
    catch (...) {
        return std::unique_ptr<ItemStack>();
    }
}

void PlayerInventory::setBoots(const ItemStack &item) const
{
    try {
        if (item.native() != nullptr) {
            impl->inventory->setBoots(std::optional<endstone::ItemStack>{*item.native()});
        }
    }
    catch (...) {
    }
}

void PlayerInventory::clearBoots() const
{
    clear_stack(*impl->inventory, &endstone::PlayerInventory::setBoots);
}

std::unique_ptr<ItemStack> PlayerInventory::getItemInMainHand() const
{
    try {
        return get_stack(impl->inventory->getItemInMainHand());
    }
    catch (...) {
        return std::unique_ptr<ItemStack>();
    }
}

void PlayerInventory::setItemInMainHand(const ItemStack &item) const
{
    try {
        if (item.native() != nullptr) {
            impl->inventory->setItemInMainHand(std::optional<endstone::ItemStack>{*item.native()});
        }
    }
    catch (...) {
    }
}

void PlayerInventory::clearItemInMainHand() const
{
    clear_stack(*impl->inventory, &endstone::PlayerInventory::setItemInMainHand);
}

std::unique_ptr<ItemStack> PlayerInventory::getItemInOffHand() const
{
    try {
        return get_stack(impl->inventory->getItemInOffHand());
    }
    catch (...) {
        return std::unique_ptr<ItemStack>();
    }
}

void PlayerInventory::setItemInOffHand(const ItemStack &item) const
{
    try {
        if (item.native() != nullptr) {
            impl->inventory->setItemInOffHand(std::optional<endstone::ItemStack>{*item.native()});
        }
    }
    catch (...) {
    }
}

void PlayerInventory::clearItemInOffHand() const
{
    clear_stack(*impl->inventory, &endstone::PlayerInventory::setItemInOffHand);
}

} // namespace aegilex::native::inventory
