#include "item_actor.h"

#include "actor.h"

#include "../inventory/item_stack.h"

#include <endstone/actor/item.h>

namespace aegilex::native::actor {

class ItemActor::impl {
  public:
    explicit impl(endstone::Item *item) noexcept : item(item)
    {
    }

    endstone::Item *item;
};

ItemActor::ItemActor(endstone::Item *item) noexcept : impl(std::make_shared<class ItemActor::impl>(item))
{
}

std::unique_ptr<::aegilex::native::inventory::ItemStack> ItemActor::getItemStack() const
{
    try {
        if (impl == nullptr || impl->item == nullptr) {
            return {};
        }
        // Endstone declares this as ItemStack getItemStack() const: retain the value.
        return std::make_unique<::aegilex::native::inventory::ItemStack>(impl->item->getItemStack());
    }
    catch (...) {
        return {};
    }
}

bool ItemActor::setItemStack(const ::aegilex::native::inventory::ItemStack &stack) const
{
    try {
        if (impl == nullptr || impl->item == nullptr || stack.native() == nullptr) {
            return false;
        }
        // Endstone copies the const reference; the guest facade is never retained.
        impl->item->setItemStack(*stack.native());
        return true;
    }
    catch (...) {
        return false;
    }
}

std::int32_t ItemActor::getPickupDelay() const
{
    try {
        return impl == nullptr || impl->item == nullptr ? 0 : impl->item->getPickupDelay();
    }
    catch (...) {
        return 0;
    }
}

bool ItemActor::setPickupDelay(const std::int32_t delay) const
{
    if (delay < 0) {
        return false;
    }
    try {
        if (impl == nullptr || impl->item == nullptr) {
            return false;
        }
        impl->item->setPickupDelay(delay);
        return true;
    }
    catch (...) {
        return false;
    }
}

bool ItemActor::isUnlimitedLifetime() const
{
    try {
        return impl != nullptr && impl->item != nullptr && impl->item->isUnlimitedLifetime();
    }
    catch (...) {
        return false;
    }
}

bool ItemActor::setUnlimitedLifetime(const bool unlimited) const
{
    try {
        if (impl == nullptr || impl->item == nullptr) {
            return false;
        }
        impl->item->setUnlimitedLifetime(unlimited);
        return true;
    }
    catch (...) {
        return false;
    }
}

bool ItemActor::getThrower(bool &has_thrower, std::int64_t &thrower) const
{
    has_thrower = false;
    thrower = 0;
    try {
        if (impl == nullptr || impl->item == nullptr) {
            return false;
        }
        const auto value = impl->item->getThrower();
        has_thrower = value.has_value();
        thrower = value.value_or(0);
        return true;
    }
    catch (...) {
        return false;
    }
}

bool ItemActor::setThrower(const bool has_thrower, const std::int64_t thrower) const
{
    try {
        if (impl == nullptr || impl->item == nullptr) {
            return false;
        }
        impl->item->setThrower(has_thrower ? std::optional<std::int64_t>(thrower) : std::nullopt);
        return true;
    }
    catch (...) {
        return false;
    }
}

std::unique_ptr<Actor> ItemActor::asActor() const
{
    try {
        return impl == nullptr || impl->item == nullptr ? std::unique_ptr<Actor>()
                                                        : std::make_unique<Actor>(impl->item);
    }
    catch (...) {
        return {};
    }
}

} // namespace aegilex::native::actor
