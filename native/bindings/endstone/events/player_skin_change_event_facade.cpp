#include "player_skin_change_event_facade.h"

#include <endstone/event/player/player_skin_change_event.h>
#include <endstone/lang/translatable.h>

#include <variant>

namespace aegilex::native::endstone_binding::events {

PlayerSkinChangeEventFacade::PlayerSkinChangeEventFacade(endstone::PlayerSkinChangeEvent *event) noexcept
    : event_(event)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerSkinChangeEventFacade::getPlayer() const noexcept
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

bool PlayerSkinChangeEventFacade::hasSkinChangeMessage() const noexcept
{
    if (event_ == nullptr) {
        return false;
    }
    try {
        return event_->getSkinChangeMessage().has_value();
    }
    catch (...) {
        return false;
    }
}

rust::String PlayerSkinChangeEventFacade::getSkinChangeMessageForRust() const noexcept
{
    if (event_ == nullptr) {
        return rust::String();
    }
    try {
        const auto message = event_->getSkinChangeMessage();
        if (!message.has_value()) {
            return rust::String();
        }
        if (const auto *text = std::get_if<std::string>(&*message)) {
            return rust::String(*text);
        }
        if (const auto *translatable = std::get_if<endstone::Translatable>(&*message)) {
            return rust::String(translatable->getText());
        }
        return rust::String();
    }
    catch (...) {
        return rust::String();
    }
}

bool PlayerSkinChangeEventFacade::setSkinChangeMessageForRust(const bool has_message, const rust::Str message) noexcept
{
    if (event_ == nullptr) {
        return false;
    }
    try {
        if (has_message) {
            event_->setSkinChangeMessage(endstone::Message{std::string(message.data(), message.size())});
        }
        else {
            event_->setSkinChangeMessage(std::nullopt);
        }
        return true;
    }
    catch (...) {
        return false;
    }
}

bool PlayerSkinChangeEventFacade::isCancelled() const noexcept
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

bool PlayerSkinChangeEventFacade::setCancelled(const bool cancelled) noexcept
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
