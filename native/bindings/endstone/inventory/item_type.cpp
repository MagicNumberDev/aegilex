#include "item_type.h"

#include "item_stack.h"

#include <endstone/inventory/item_type.h>
#include <endstone/server.h>

#include <memory>
#include <string>
#include <string_view>

namespace aegilex::native::inventory {

class ItemType::impl {
  public:
    explicit impl(const endstone::ItemType *type) noexcept : type(type)
    {
    }

    const endstone::ItemType *type;
};

ItemType::ItemType(const endstone::ItemType *type) noexcept : impl(std::make_shared<class ItemType::impl>(type))
{
}

const endstone::ItemType *ItemType::native() const noexcept
{
    return impl == nullptr ? nullptr : impl->type;
}

rust::String ItemType::getTypeId() const
{
    try {
        return rust::String(std::string(impl->type->getId()));
    }
    catch (...) {
        return rust::String();
    }
}

rust::String ItemType::getTranslationKey() const
{
    try {
        return rust::String(impl->type->getTranslationKey());
    }
    catch (...) {
        return rust::String();
    }
}

std::int32_t ItemType::getMaxStackSize() const
{
    try {
        return impl->type->getMaxStackSize();
    }
    catch (...) {
        return 0;
    }
}

std::int32_t ItemType::getMaxDurability() const
{
    try {
        return impl->type->getMaxDurability();
    }
    catch (...) {
        return 0;
    }
}

std::unique_ptr<ItemStack> ItemType::createItemStack(const std::int32_t amount) const
{
    try {
        if (amount < 1 || amount > impl->type->getMaxStackSize()) {
            return std::unique_ptr<ItemStack>();
        }
        return std::unique_ptr<ItemStack>(new ItemStack(impl->type->createItemStack(amount)));
    }
    catch (...) {
        return std::unique_ptr<ItemStack>();
    }
}

} // namespace aegilex::native::inventory
