#include "player_kick_event_facade.h"

#include <endstone/server.h>
#include <endstone/event/player/player_kick_event.h>

namespace aegilex::native::endstone_binding::events {

PlayerKickEventFacade::PlayerKickEventFacade(endstone::PlayerKickEvent *event) noexcept : event_(event)
{
}

bool PlayerKickEventFacade::isCancelled() const noexcept
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

bool PlayerKickEventFacade::setCancelled(const bool cancelled) noexcept
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

std::string PlayerKickEventFacade::getReason() const noexcept
{
    if (event_ == nullptr) {
        return {};
    }

    try {
        return event_->getReason();
    }
    catch (...) {
        return {};
    }
}

bool PlayerKickEventFacade::setReason(const std::string_view reason) noexcept
{
    if (event_ == nullptr) {
        return false;
    }
    try {
        event_->setReason(std::string(reason));
        return true;
    }
    catch (...) {
        return false;
    }
}

std::unique_ptr<::aegilex::native::player::Player> PlayerKickEventFacade::getPlayer() const noexcept
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

rust::String PlayerKickEventFacade::getReasonForRust() const noexcept
{
    try {
        return rust::String(getReason());
    }
    catch (...) {
        return rust::String();
    }
}

bool PlayerKickEventFacade::setReasonForRust(const rust::Str reason) noexcept
{
    return setReason(std::string_view(reason.data(), reason.size()));
}

} // namespace aegilex::native::endstone_binding::events
