// Test-only typed item actor bridge stubs. Never linked into the plugin.

#include <aegilex-runtime/src/cxx_host_actor.rs.h>

#include "bindings/endstone/actor/item_actor.h"
#include "bindings/endstone/actor/actor.h"
#include "bindings/endstone/inventory/item_stack.h"

namespace aegilex::native::actor {

class ItemActor::impl {
  public:
    std::unique_ptr<::aegilex::native::inventory::ItemStack> item_stack{
        std::make_unique<::aegilex::native::inventory::ItemStack>(nullptr)};
    std::int32_t pickup_delay{0};
    bool unlimited_lifetime{false};
    bool has_thrower{false};
    std::int64_t thrower{0};
};

ItemActor::ItemActor(endstone::Item *) noexcept : impl(std::make_shared<class ItemActor::impl>())
{
}

std::unique_ptr<::aegilex::native::inventory::ItemStack> ItemActor::getItemStack() const
{
    return impl->item_stack->cloneItemStack();
}

bool ItemActor::setItemStack(const ::aegilex::native::inventory::ItemStack &stack) const
{
    auto copy = stack.cloneItemStack();
    if (copy == nullptr) {
        return false;
    }
    impl->item_stack = std::move(copy);
    return true;
}

std::int32_t ItemActor::getPickupDelay() const
{
    return impl->pickup_delay;
}

bool ItemActor::setPickupDelay(const std::int32_t delay) const
{
    if (delay < 0) {
        return false;
    }
    impl->pickup_delay = delay;
    return true;
}

bool ItemActor::isUnlimitedLifetime() const
{
    return impl->unlimited_lifetime;
}

bool ItemActor::setUnlimitedLifetime(const bool unlimited) const
{
    impl->unlimited_lifetime = unlimited;
    return true;
}

bool ItemActor::getThrower(bool &has_thrower, std::int64_t &thrower) const
{
    has_thrower = impl->has_thrower;
    thrower = impl->thrower;
    return true;
}

bool ItemActor::setThrower(const bool has_thrower, const std::int64_t thrower) const
{
    impl->has_thrower = has_thrower;
    impl->thrower = thrower;
    return true;
}

std::unique_ptr<Actor> ItemActor::asActor() const
{
    return std::unique_ptr<Actor>(new Actor(nullptr));
}

} // namespace aegilex::native::actor
