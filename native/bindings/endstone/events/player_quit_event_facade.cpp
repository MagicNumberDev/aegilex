#include "player_quit_event_facade.h"

#include <endstone/event/player/player_quit_event.h>
#include <endstone/lang/translatable.h>

#include <string>
#include <variant>

namespace aegilex::native::endstone_binding::events {

PlayerQuitEventFacade::PlayerQuitEventFacade(endstone::PlayerQuitEvent *event) noexcept : event_(event)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerQuitEventFacade::getPlayer() const noexcept
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

bool PlayerQuitEventFacade::hasQuitMessage() const noexcept
{
    if (event_ == nullptr) {
        return false;
    }
    try {
        return event_->getQuitMessage().has_value();
    }
    catch (...) {
        return false;
    }
}

rust::String PlayerQuitEventFacade::getQuitMessageForRust() const noexcept
{
    if (event_ == nullptr) {
        return rust::String();
    }
    try {
        const auto message = event_->getQuitMessage();
        if (!message.has_value()) {
            return rust::String();
        }
        if (const auto *text = std::get_if<std::string>(&*message)) {
            return rust::String(*text);
        }
        if (const auto *translatable = std::get_if<endstone::Translatable>(&*message)) {
            return rust::String(translatable->getText());
        }
    }
    catch (...) {
    }
    return rust::String();
}

bool PlayerQuitEventFacade::setQuitMessageForRust(const bool has_message, const rust::Str message) noexcept
{
    if (event_ == nullptr) {
        return false;
    }
    try {
        if (has_message) {
            event_->setQuitMessage(endstone::Message{std::string(message.data(), message.size())});
        }
        else {
            event_->setQuitMessage(std::nullopt);
        }
        return true;
    }
    catch (...) {
        return false;
    }
}

} // namespace aegilex::native::endstone_binding::events
