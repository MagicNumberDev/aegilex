#include "player_interact_event_facade.h"

#include <aegilex-runtime/src/cxx_runtime.rs.h>

#include <endstone/event/player/player_interact_event.h>

namespace aegilex::native::endstone_binding::events {

PlayerInteractEventFacade::PlayerInteractEventFacade(endstone::PlayerInteractEvent *event) noexcept : event_(event)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerInteractEventFacade::getPlayer() const noexcept
{
    if (event_ == nullptr) {
        return nullptr;
    }
    try {
        return std::make_unique<::aegilex::native::player::Player>(&event_->getPlayer());
    }
    catch (...) {
        return nullptr;
    }
}

std::uint8_t PlayerInteractEventFacade::getAction() const noexcept
{
    if (event_ == nullptr) {
        return 0;
    }
    try {
        return static_cast<std::uint8_t>(event_->getAction());
    }
    catch (...) {
        return 0;
    }
}

std::unique_ptr<::aegilex::native::inventory::ItemStackRef> PlayerInteractEventFacade::getItem() const noexcept
{
    if (event_ == nullptr) {
        return nullptr;
    }
    try {
        const auto &item = event_->getItem();
        return item ? std::make_unique<::aegilex::native::inventory::ItemStackRef>(&*item) : nullptr;
    }
    catch (...) {
        return nullptr;
    }
}

std::unique_ptr<::aegilex::native::level::Block> PlayerInteractEventFacade::getBlock() const noexcept
{
    if (event_ == nullptr) {
        return nullptr;
    }
    try {
        auto *block = event_->getBlock();
        return block == nullptr ? nullptr : std::make_unique<::aegilex::native::level::Block>(*block);
    }
    catch (...) {
        return nullptr;
    }
}

std::uint8_t PlayerInteractEventFacade::getBlockFace() const noexcept
{
    if (event_ == nullptr) {
        return 0;
    }
    try {
        return static_cast<std::uint8_t>(event_->getBlockFace());
    }
    catch (...) {
        return 0;
    }
}

bool PlayerInteractEventFacade::hasClickedPosition() const noexcept
{
    if (event_ == nullptr) {
        return false;
    }
    try {
        return event_->getClickedPosition().has_value();
    }
    catch (...) {
        return false;
    }
}

aegilex::runtime::VectorData PlayerInteractEventFacade::getClickedPosition() const noexcept
{
    aegilex::runtime::VectorData result{};
    if (event_ == nullptr) {
        return result;
    }
    try {
        const auto position = event_->getClickedPosition();
        if (position) {
            result.x = position->getX();
            result.y = position->getY();
            result.z = position->getZ();
        }
    }
    catch (...) {
    }
    return result;
}

bool PlayerInteractEventFacade::isCancelled() const noexcept
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

bool PlayerInteractEventFacade::setCancelled(const bool cancelled) noexcept
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
