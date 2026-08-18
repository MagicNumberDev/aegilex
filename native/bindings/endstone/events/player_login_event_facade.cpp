#include "player_login_event_facade.h"

#include <endstone/server.h>
#include <endstone/event/player/player_login_event.h>

namespace aegilex::native::endstone_binding::events {

PlayerLoginEventFacade::PlayerLoginEventFacade(endstone::PlayerLoginEvent *event) noexcept : event_(event)
{
}

bool PlayerLoginEventFacade::isCancelled() const noexcept
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

bool PlayerLoginEventFacade::setCancelled(const bool cancelled) noexcept
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

std::string PlayerLoginEventFacade::getKickMessage() const noexcept
{
    if (event_ == nullptr) {
        return {};
    }

    try {
        return event_->getKickMessage();
    }
    catch (...) {
        return {};
    }
}

bool PlayerLoginEventFacade::setKickMessage(const std::string_view message) noexcept
{
    if (event_ == nullptr) {
        return false;
    }
    try {
        event_->setKickMessage(std::string(message));
        return true;
    }
    catch (...) {
        return false;
    }
}

std::unique_ptr<::aegilex::native::player::Player> PlayerLoginEventFacade::getPlayer() const noexcept
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

rust::String PlayerLoginEventFacade::getKickMessageForRust() const noexcept
{
    try {
        return rust::String(getKickMessage());
    }
    catch (...) {
        return rust::String();
    }
}

bool PlayerLoginEventFacade::setKickMessageForRust(const rust::Str message) noexcept
{
    return setKickMessage(std::string_view(message.data(), message.size()));
}

} // namespace aegilex::native::endstone_binding::events
