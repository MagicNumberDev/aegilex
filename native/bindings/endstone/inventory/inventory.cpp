#include "inventory.h"

#include "item_stack.h"

#include <endstone/inventory/inventory.h>
#include <endstone/inventory/item_stack.h>

#include <optional>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

namespace aegilex::native::inventory {

namespace {

[[nodiscard]] bool valid_inventory_type_id(const std::string_view type_id) noexcept
{
    if (type_id.empty()) {
        return false;
    }
    std::size_t colons = 0;
    for (const auto byte : type_id) {
        const auto value = static_cast<std::uint8_t>(byte);
        const auto valid = (value >= static_cast<std::uint8_t>('a') && value <= static_cast<std::uint8_t>('z')) ||
                           (value >= static_cast<std::uint8_t>('0') && value <= static_cast<std::uint8_t>('9')) ||
                           value == static_cast<std::uint8_t>('_') || value == static_cast<std::uint8_t>(':');
        if (!valid) {
            return false;
        }
        colons += value == static_cast<std::uint8_t>(':') ? 1U : 0U;
    }
    return colons <= 1 && type_id.front() != ':' && type_id.back() != ':';
}

} // namespace

class Inventory::impl {
  public:
    explicit impl(endstone::Inventory *inventory) noexcept : inventory(inventory)
    {
    }

    endstone::Inventory *inventory;
};

Inventory::Inventory(endstone::Inventory *inventory) noexcept : impl(std::make_shared<class Inventory::impl>(inventory))
{
}

endstone::Inventory *Inventory::native() const noexcept
{
    return impl == nullptr ? nullptr : impl->inventory;
}

std::int32_t Inventory::getSize() const
{
    try {
        return impl->inventory->getSize();
    }
    catch (...) {
        return 0;
    }
}

std::int32_t Inventory::getMaxStackSize() const
{
    try {
        return impl->inventory->getMaxStackSize();
    }
    catch (...) {
        return 0;
    }
}

std::unique_ptr<ItemStack> Inventory::getItem(const std::int32_t index) const
{
    try {
        if (index < 0) {
            return std::unique_ptr<ItemStack>();
        }
        const auto item = impl->inventory->getItem(index);
        return item.has_value() ? std::unique_ptr<ItemStack>(new ItemStack(*item)) : std::unique_ptr<ItemStack>();
    }
    catch (...) {
        return std::unique_ptr<ItemStack>();
    }
}

void Inventory::setItem(const std::int32_t index, const ItemStack &item) const
{
    try {
        if (index < 0 || item.native() == nullptr) {
            return;
        }
        impl->inventory->setItem(index, std::optional<endstone::ItemStack>{*item.native()});
    }
    catch (...) {
    }
}

void Inventory::clear() const
{
    try {
        impl->inventory->clear();
    }
    catch (...) {
    }
}

void Inventory::clearIndex(const std::int32_t index) const
{
    try {
        if (index < 0) {
            return;
        }
        impl->inventory->clear(index);
    }
    catch (...) {
    }
}

std::unique_ptr<ItemStack> Inventory::addItem(const ItemStack &item) const
{
    try {
        if (item.native() == nullptr) {
            return std::unique_ptr<ItemStack>();
        }
        auto leftovers = impl->inventory->addItem(std::vector<endstone::ItemStack>{*item.native()});
        const auto leftover = leftovers.find(0);
        return leftover != leftovers.end() ? std::unique_ptr<ItemStack>(new ItemStack(leftover->second))
                                           : std::unique_ptr<ItemStack>();
    }
    catch (...) {
        return std::unique_ptr<ItemStack>();
    }
}

std::unique_ptr<ItemStack> Inventory::removeItem(const ItemStack &item) const
{
    try {
        if (item.native() == nullptr) {
            return std::unique_ptr<ItemStack>();
        }
        auto leftovers = impl->inventory->removeItem(std::vector<endstone::ItemStack>{*item.native()});
        const auto leftover = leftovers.find(0);
        return leftover != leftovers.end() ? std::unique_ptr<ItemStack>(new ItemStack(leftover->second))
                                           : std::unique_ptr<ItemStack>();
    }
    catch (...) {
        return std::unique_ptr<ItemStack>();
    }
}

bool Inventory::containsType(const rust::Str type_id) const
{
    try {
        const std::string_view identifier(type_id.data(), type_id.size());
        return valid_inventory_type_id(identifier) && impl->inventory->contains(std::string(identifier));
    }
    catch (...) {
        return false;
    }
}

bool Inventory::containsStack(const ItemStack &item) const
{
    try {
        return item.native() != nullptr && impl->inventory->contains(*item.native());
    }
    catch (...) {
        return false;
    }
}

bool Inventory::containsAtLeastType(const rust::Str type_id, const std::int32_t amount) const
{
    try {
        const std::string_view identifier(type_id.data(), type_id.size());
        return valid_inventory_type_id(identifier) && impl->inventory->containsAtLeast(std::string(identifier), amount);
    }
    catch (...) {
        return false;
    }
}

bool Inventory::containsAtLeastStack(const ItemStack &item, const std::int32_t amount) const
{
    try {
        return item.native() != nullptr && impl->inventory->containsAtLeast(*item.native(), amount);
    }
    catch (...) {
        return false;
    }
}

std::int32_t Inventory::firstType(const rust::Str type_id) const
{
    try {
        const std::string_view identifier(type_id.data(), type_id.size());
        return valid_inventory_type_id(identifier) ? impl->inventory->first(std::string(identifier)) : -1;
    }
    catch (...) {
        return -1;
    }
}

std::int32_t Inventory::firstStack(const ItemStack &item) const
{
    try {
        return item.native() != nullptr ? impl->inventory->first(*item.native()) : -1;
    }
    catch (...) {
        return -1;
    }
}

std::int32_t Inventory::firstEmpty() const
{
    try {
        return impl->inventory->firstEmpty();
    }
    catch (...) {
        return -1;
    }
}

bool Inventory::isEmpty() const
{
    try {
        return impl->inventory->isEmpty();
    }
    catch (...) {
        return false;
    }
}

void Inventory::removeType(const rust::Str type_id) const
{
    try {
        const std::string_view identifier(type_id.data(), type_id.size());
        if (valid_inventory_type_id(identifier)) {
            impl->inventory->remove(std::string(identifier));
        }
    }
    catch (...) {
    }
}

void Inventory::removeStack(const ItemStack &item) const
{
    try {
        if (item.native() != nullptr) {
            impl->inventory->remove(*item.native());
        }
    }
    catch (...) {
    }
}

} // namespace aegilex::native::inventory
