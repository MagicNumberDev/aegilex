#include "broadcast_message_event_facade.h"

#include <endstone/event/server/broadcast_message_event.h>
#include <endstone/lang/translatable.h>

#include <variant>

namespace aegilex::native::endstone_binding::events {

BroadcastMessageEventFacade::BroadcastMessageEventFacade(endstone::BroadcastMessageEvent *event) noexcept
    : event_(event)
{
}

bool BroadcastMessageEventFacade::isCancelled() const noexcept
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

bool BroadcastMessageEventFacade::setCancelled(const bool cancelled) noexcept
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

std::string BroadcastMessageEventFacade::getMessage() const noexcept
{
    if (event_ == nullptr) {
        return {};
    }

    try {
        const auto &message = event_->getMessage();
        if (const auto *text = std::get_if<std::string>(&message)) {
            return *text;
        }
        if (const auto *translatable = std::get_if<endstone::Translatable>(&message)) {
            return translatable->getText();
        }
        return {};
    }
    catch (...) {
        return {};
    }
}

bool BroadcastMessageEventFacade::setMessage(const std::string_view message) noexcept
{
    if (event_ == nullptr) {
        return false;
    }
    try {
        event_->setMessage(endstone::Message{std::string(message)});
        return true;
    }
    catch (...) {
        return false;
    }
}

rust::String BroadcastMessageEventFacade::getMessageForRust() const noexcept
{
    try {
        return rust::String(getMessage());
    }
    catch (...) {
        return rust::String();
    }
}

bool BroadcastMessageEventFacade::setMessageForRust(const rust::Str message) noexcept
{
    return setMessage(std::string_view(message.data(), message.size()));
}

} // namespace aegilex::native::endstone_binding::events
