#include "player_command_event_facade.h"

#include <endstone/server.h>
#include <endstone/event/player/player_command_event.h>

namespace aegilex::native::endstone_binding::events {

PlayerCommandEventFacade::PlayerCommandEventFacade(endstone::PlayerCommandEvent *event) noexcept : event_(event)
{
}

bool PlayerCommandEventFacade::isCancelled() const noexcept
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

bool PlayerCommandEventFacade::setCancelled(const bool cancelled) noexcept
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

std::string PlayerCommandEventFacade::getCommand() const noexcept
{
    if (event_ == nullptr) {
        return {};
    }

    try {
        return event_->getCommand();
    }
    catch (...) {
        return {};
    }
}

bool PlayerCommandEventFacade::setCommand(const std::string_view command) noexcept
{
    if (event_ == nullptr) {
        return false;
    }
    try {
        event_->setCommand(std::string(command));
        return true;
    }
    catch (...) {
        return false;
    }
}

std::unique_ptr<::aegilex::native::player::Player> PlayerCommandEventFacade::getPlayer() const noexcept
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

rust::String PlayerCommandEventFacade::getCommandForRust() const noexcept
{
    try {
        return rust::String(getCommand());
    }
    catch (...) {
        return rust::String();
    }
}

bool PlayerCommandEventFacade::setCommandForRust(const rust::Str command) noexcept
{
    return setCommand(std::string_view(command.data(), command.size()));
}

} // namespace aegilex::native::endstone_binding::events
