#include "player_death_event_facade.h"

#include <endstone/event/actor/player_death_event.h>
#include <endstone/lang/translatable.h>

#include <string>
#include <variant>

namespace aegilex::native::endstone_binding::events {

PlayerDeathEventFacade::PlayerDeathEventFacade(endstone::PlayerDeathEvent *event) noexcept : event_(event)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerDeathEventFacade::getPlayer() const noexcept
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

bool PlayerDeathEventFacade::hasDeathMessage() const noexcept
{
    if (event_ == nullptr) {
        return false;
    }
    try {
        return event_->getDeathMessage().has_value();
    }
    catch (...) {
        return false;
    }
}

rust::String PlayerDeathEventFacade::getDeathMessageForRust() const noexcept
{
    if (event_ == nullptr) {
        return rust::String();
    }
    try {
        const auto message = event_->getDeathMessage();
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

bool PlayerDeathEventFacade::setDeathMessageForRust(const bool has_message, const rust::Str message) noexcept
{
    if (event_ == nullptr) {
        return false;
    }
    try {
        if (has_message) {
            event_->setDeathMessage(endstone::Message{std::string(message.data(), message.size())});
        }
        else {
            event_->setDeathMessage(std::nullopt);
        }
        return true;
    }
    catch (...) {
        return false;
    }
}

} // namespace aegilex::native::endstone_binding::events
