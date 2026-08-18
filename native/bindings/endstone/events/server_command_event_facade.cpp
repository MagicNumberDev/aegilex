#include "server_command_event_facade.h"

#include <endstone/event/server/server_command_event.h>

namespace aegilex::native::endstone_binding::events {

ServerCommandEventFacade::ServerCommandEventFacade(endstone::ServerCommandEvent *event) noexcept : event_(event)
{
}

bool ServerCommandEventFacade::isCancelled() const noexcept
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

bool ServerCommandEventFacade::setCancelled(const bool cancelled) noexcept
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

std::string ServerCommandEventFacade::getCommand() const noexcept
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

bool ServerCommandEventFacade::setCommand(const std::string_view command) noexcept
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

rust::String ServerCommandEventFacade::getSenderNameForRust() const noexcept
{
    if (event_ == nullptr) {
        return rust::String();
    }

    try {
        return rust::String(event_->getSender().getName());
    }
    catch (...) {
        return rust::String();
    }
}

rust::String ServerCommandEventFacade::getCommandForRust() const noexcept
{
    try {
        return rust::String(getCommand());
    }
    catch (...) {
        return rust::String();
    }
}

bool ServerCommandEventFacade::setCommandForRust(const rust::Str command) noexcept
{
    return setCommand(std::string_view(command.data(), command.size()));
}

} // namespace aegilex::native::endstone_binding::events
