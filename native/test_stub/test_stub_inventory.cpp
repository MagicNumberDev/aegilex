// Test-only typed inventory/item stack bridge stubs. Never linked into the plugin.

#include <aegilex-runtime/src/cxx_host_inventory.rs.h>

#include "bindings/endstone/inventory/inventory.h"
#include "bindings/endstone/inventory/item_meta.h"
#include "bindings/endstone/inventory/item_stack.h"
#include "bindings/endstone/inventory/item_type.h"
#include "bindings/endstone/inventory/player_inventory.h"

#include <cstddef>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

namespace aegilex::native::inventory {

class ItemStack::impl {
  public:
    std::int32_t amount{1};
    std::int32_t data{0};
    std::string type_id{"minecraft:apple"};
    Nbt nbt;
};

class ItemType::impl {
  public:
    impl() noexcept = default;
};

class Inventory::impl {
  public:
    impl() noexcept = default;
};

class PlayerInventory::impl {
  public:
    impl() noexcept = default;
};

ItemStack::ItemStack(endstone::ItemStack *) noexcept : impl(std::make_shared<class ItemStack::impl>())
{
    impl->nbt.root_index = 0;
    auto root = NbtNode{};
    root.kind = 10;
    impl->nbt.nodes.push_back(std::move(root));
}

rust::String ItemStack::getType() const
{
    return rust::String(impl->type_id.empty() ? "minecraft:apple" : impl->type_id);
}

bool ItemStack::setType(const rust::Str type_id) const
{
    if (type_id.empty()) {
        return false;
    }
    impl->type_id = std::string(type_id.data(), type_id.size());
    return true;
}

std::int32_t ItemStack::getAmount() const
{
    return impl->amount;
}

void ItemStack::setAmount(const std::int32_t amount) const
{
    impl->amount = amount;
}

std::int32_t ItemStack::getData() const
{
    return impl->data;
}

void ItemStack::setData(const std::int32_t data) const
{
    impl->data = data;
}

rust::String ItemStack::getTranslationKey() const
{
    return rust::String("item.apple.name");
}

std::int32_t ItemStack::getMaxStackSize() const
{
    return 64;
}

std::unique_ptr<ItemStack> ItemStack::cloneItemStack() const
{
    auto copy = std::unique_ptr<ItemStack>(new ItemStack(nullptr));
    copy->impl->amount = impl->amount;
    copy->impl->data = impl->data;
    copy->impl->type_id = impl->type_id;
    copy->impl->nbt = impl->nbt;
    return copy;
}

bool ItemStack::hasItemMeta() const
{
    return false;
}

bool ItemStack::equals(const ItemStack &) const
{
    return true;
}

bool ItemStack::isSimilar(const ItemStack &) const
{
    return true;
}

bool ItemStack::getMeta(ItemMeta &out) const
{
    out = ItemMeta{};
    return true;
}

std::unique_ptr<ItemStackCollection> ItemStack::getChargedProjectiles() const
{
    return ItemStackCollection::create();
}

bool ItemStack::setMeta(const ::aegilex::native::server::Server &, const ItemMeta &, const ItemStackCollection &,
                        bool &out_success) const
{
    out_success = true;
    return true;
}

bool ItemStack::getNbt(Nbt &out) const
{
    out = impl->nbt;
    return true;
}

bool ItemStack::setNbt(const Nbt &value) const
{
    impl->nbt = value;
    return true;
}

ItemStackRef::ItemStackRef(const endstone::ItemStack *stack) noexcept : stack_(stack)
{
}

rust::String ItemStackRef::getType() const
{
    return rust::String("minecraft:apple");
}

std::int32_t ItemStackRef::getAmount() const
{
    return 1;
}

rust::String ItemStackRef::getTranslationKey() const
{
    return rust::String("item.apple.name");
}

std::int32_t ItemStackRef::getMaxStackSize() const
{
    return 64;
}

std::int32_t ItemStackRef::getData() const
{
    return 0;
}

std::unique_ptr<ItemStack> ItemStackRef::cloneItemStack() const
{
    return std::unique_ptr<ItemStack>(new ItemStack(nullptr));
}

bool ItemStackRef::hasItemMeta() const
{
    return false;
}

bool ItemStackRef::isSimilar(const ItemStack &) const
{
    return true;
}

bool ItemStackRef::getMeta(ItemMeta &out) const
{
    out = ItemMeta{};
    return true;
}

std::unique_ptr<ItemStackCollection> ItemStackRef::getChargedProjectiles() const
{
    return ItemStackCollection::create();
}

bool ItemStackRef::getNbt(Nbt &out) const
{
    out.root_index = 0;
    auto root = NbtNode{};
    root.kind = 10;
    out.nodes = {std::move(root)};
    return true;
}

std::unique_ptr<ItemStackRef> borrow_item_stack(const ItemStack &stack) noexcept
{
    return std::make_unique<ItemStackRef>(nullptr);
}

std::unique_ptr<ItemStackCollection> ItemStackCollection::create() noexcept
{
    return std::make_unique<ItemStackCollection>();
}

void ItemStackCollection::push(const ItemStack &)
{
}

std::size_t ItemStackCollection::len() const noexcept
{
    return 0;
}

std::unique_ptr<ItemStack> ItemStackCollection::get(std::size_t) const
{
    return std::unique_ptr<ItemStack>();
}

ItemType::ItemType(const endstone::ItemType *) noexcept : impl(std::make_shared<class ItemType::impl>())
{
}

rust::String ItemType::getTypeId() const
{
    return rust::String("minecraft:apple");
}

rust::String ItemType::getTranslationKey() const
{
    return rust::String("item.apple");
}

std::int32_t ItemType::getMaxStackSize() const
{
    return 64;
}

std::int32_t ItemType::getMaxDurability() const
{
    return 0;
}

std::unique_ptr<ItemStack> ItemType::createItemStack(std::int32_t) const
{
    return std::unique_ptr<ItemStack>(new ItemStack(nullptr));
}

Inventory::Inventory(endstone::Inventory *) noexcept : impl(std::make_shared<class Inventory::impl>())
{
}

std::int32_t Inventory::getSize() const
{
    return 36;
}

std::int32_t Inventory::getMaxStackSize() const
{
    return 64;
}

std::unique_ptr<ItemStack> Inventory::getItem(std::int32_t) const
{
    return std::unique_ptr<ItemStack>(new ItemStack(nullptr));
}

void Inventory::setItem(std::int32_t, const ItemStack &) const
{
}

void Inventory::clear() const
{
}

void Inventory::clearIndex(std::int32_t) const
{
}

std::unique_ptr<ItemStack> Inventory::addItem(const ItemStack &) const
{
    return std::unique_ptr<ItemStack>();
}

std::unique_ptr<ItemStack> Inventory::removeItem(const ItemStack &) const
{
    return std::unique_ptr<ItemStack>();
}

bool Inventory::containsType(rust::Str) const
{
    return true;
}

bool Inventory::containsStack(const ItemStack &) const
{
    return true;
}

bool Inventory::containsAtLeastType(rust::Str, std::int32_t) const
{
    return true;
}

bool Inventory::containsAtLeastStack(const ItemStack &, std::int32_t) const
{
    return true;
}

std::int32_t Inventory::firstType(rust::Str) const
{
    return 0;
}

std::int32_t Inventory::firstStack(const ItemStack &) const
{
    return 0;
}

std::int32_t Inventory::firstEmpty() const
{
    return 0;
}

bool Inventory::isEmpty() const
{
    return true;
}

void Inventory::removeType(rust::Str) const
{
}

void Inventory::removeStack(const ItemStack &) const
{
}

PlayerInventory::PlayerInventory(endstone::PlayerInventory *) noexcept
    : impl(std::make_shared<class PlayerInventory::impl>())
{
}

std::unique_ptr<Inventory> PlayerInventory::asInventory() const
{
    return std::make_unique<Inventory>(nullptr);
}

std::int32_t PlayerInventory::getHeldItemSlot() const
{
    return 0;
}

void PlayerInventory::setHeldItemSlot(std::int32_t) const
{
}

std::unique_ptr<ItemStack> PlayerInventory::getHelmet() const
{
    return std::unique_ptr<ItemStack>();
}

void PlayerInventory::setHelmet(const ItemStack &) const
{
}

void PlayerInventory::clearHelmet() const
{
}

std::unique_ptr<ItemStack> PlayerInventory::getChestplate() const
{
    return std::unique_ptr<ItemStack>();
}

void PlayerInventory::setChestplate(const ItemStack &) const
{
}

void PlayerInventory::clearChestplate() const
{
}

std::unique_ptr<ItemStack> PlayerInventory::getLeggings() const
{
    return std::unique_ptr<ItemStack>();
}

void PlayerInventory::setLeggings(const ItemStack &) const
{
}

void PlayerInventory::clearLeggings() const
{
}

std::unique_ptr<ItemStack> PlayerInventory::getBoots() const
{
    return std::unique_ptr<ItemStack>();
}

void PlayerInventory::setBoots(const ItemStack &) const
{
}

void PlayerInventory::clearBoots() const
{
}

std::unique_ptr<ItemStack> PlayerInventory::getItemInMainHand() const
{
    return std::unique_ptr<ItemStack>(new ItemStack(nullptr));
}

void PlayerInventory::setItemInMainHand(const ItemStack &) const
{
}

void PlayerInventory::clearItemInMainHand() const
{
}

std::unique_ptr<ItemStack> PlayerInventory::getItemInOffHand() const
{
    return std::unique_ptr<ItemStack>();
}

void PlayerInventory::setItemInOffHand(const ItemStack &) const
{
}

void PlayerInventory::clearItemInOffHand() const
{
}

} // namespace aegilex::native::inventory
