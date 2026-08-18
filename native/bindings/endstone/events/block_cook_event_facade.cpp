#include "block_cook_event_facade.h"

#include <endstone/event/block/block_cook_event.h>

#include <utility>

namespace aegilex::native::endstone_binding::events {

BlockCookEventFacade::BlockCookEventFacade(endstone::BlockCookEvent *event) noexcept : event_(event)
{
}

std::unique_ptr<::aegilex::native::level::Block> BlockCookEventFacade::getBlock() const noexcept
{
    if (event_ == nullptr) {
        return nullptr;
    }
    try {
        return std::make_unique<::aegilex::native::level::Block>(event_->getBlock());
    }
    catch (...) {
        return nullptr;
    }
}

std::unique_ptr<::aegilex::native::inventory::ItemStackRef> BlockCookEventFacade::getSource() const noexcept
{
    if (event_ == nullptr) {
        return nullptr;
    }
    try {
        return std::make_unique<::aegilex::native::inventory::ItemStackRef>(&event_->getSource());
    }
    catch (...) {
        return nullptr;
    }
}

std::unique_ptr<::aegilex::native::inventory::ItemStackRef> BlockCookEventFacade::getResult() const noexcept
{
    if (event_ == nullptr) {
        return nullptr;
    }
    try {
        return std::make_unique<::aegilex::native::inventory::ItemStackRef>(&event_->getResult());
    }
    catch (...) {
        return nullptr;
    }
}

bool BlockCookEventFacade::setResult(const ::aegilex::native::inventory::ItemStack &result) noexcept
{
    if (event_ == nullptr || result.native() == nullptr) {
        return false;
    }
    try {
        auto native = result.cloneItemStack()->takeNative();
        if (native == nullptr) {
            return false;
        }
        event_->setResult(std::move(*native));
        return true;
    }
    catch (...) {
        return false;
    }
}

bool BlockCookEventFacade::isCancelled() const noexcept
{
    if (event_ == nullptr) {
        return false;
    }
    try {
        return event_->isCancelled();
    }
    catch (...) {
        return false;
    }
}

bool BlockCookEventFacade::setCancelled(const bool cancelled) noexcept
{
    if (event_ == nullptr) {
        return false;
    }
    try {
        event_->setCancelled(cancelled);
        return true;
    }
    catch (...) {
        return false;
    }
}

} // namespace aegilex::native::endstone_binding::events
