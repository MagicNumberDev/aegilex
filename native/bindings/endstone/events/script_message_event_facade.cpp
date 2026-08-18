#include "script_message_event_facade.h"

#include <endstone/command/command_sender.h>
#include <endstone/event/server/script_message_event.h>

namespace aegilex::native::endstone_binding::events {

ScriptMessageEventFacade::ScriptMessageEventFacade(endstone::ScriptMessageEvent *event) noexcept : event_(event)
{
}

rust::String ScriptMessageEventFacade::getMessageIdForRust() const noexcept
{
    try {
        return event_ == nullptr ? rust::String() : rust::String(event_->getMessageId());
    }
    catch (...) {
        return rust::String();
    }
}

rust::String ScriptMessageEventFacade::getMessageForRust() const noexcept
{
    try {
        return event_ == nullptr ? rust::String() : rust::String(event_->getMessage());
    }
    catch (...) {
        return rust::String();
    }
}

std::unique_ptr<::aegilex::native::host::CommandSender> ScriptMessageEventFacade::getSender() const noexcept
{
    if (event_ == nullptr) {
        return nullptr;
    }
    try {
        return std::make_unique<::aegilex::native::host::CommandSender>(
            const_cast<endstone::CommandSender *>(&event_->getSender()));
    }
    catch (...) {
        return nullptr;
    }
}

bool ScriptMessageEventFacade::isCancelled() const noexcept
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

bool ScriptMessageEventFacade::setCancelled(const bool cancelled) noexcept
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
