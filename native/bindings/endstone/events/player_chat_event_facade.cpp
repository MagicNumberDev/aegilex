#include "player_chat_event_facade.h"

#include <endstone/player.h>
#include <endstone/server.h>
#include <endstone/event/player/player_chat_event.h>

namespace aegilex::native::endstone_binding::events {

PlayerChatEventFacade::PlayerChatEventFacade(endstone::PlayerChatEvent *event) noexcept : event_(event)
{
}

bool PlayerChatEventFacade::isCancelled() const noexcept
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

bool PlayerChatEventFacade::setCancelled(const bool cancelled) noexcept
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

std::string PlayerChatEventFacade::getMessage() const noexcept
{
    if (event_ == nullptr) {
        return {};
    }

    try {
        return event_->getMessage();
    }
    catch (...) {
        return {};
    }
}

bool PlayerChatEventFacade::setMessage(const std::string_view message) noexcept
{
    if (event_ == nullptr) {
        return false;
    }
    try {
        event_->setMessage(std::string(message));
        return true;
    }
    catch (...) {
        return false;
    }
}

std::string PlayerChatEventFacade::getFormat() const noexcept
{
    if (event_ == nullptr) {
        return {};
    }

    try {
        return event_->getFormat();
    }
    catch (...) {
        return {};
    }
}

bool PlayerChatEventFacade::setFormat(const std::string_view format) noexcept
{
    if (event_ == nullptr) {
        return false;
    }
    try {
        event_->setFormat(std::string(format));
        return true;
    }
    catch (...) {
        return false;
    }
}

std::unique_ptr<::aegilex::native::player::Player> PlayerChatEventFacade::getPlayer() const noexcept
{
    if (event_ == nullptr) {
        return nullptr;
    }

    try {
        auto *player = &event_->getPlayer();
        return std::make_unique<::aegilex::native::player::Player>(player);
    }
    catch (...) {
        return nullptr;
    }
}

bool PlayerChatEventFacade::setPlayer(const ::aegilex::native::player::Player &player) noexcept
{
    if (event_ == nullptr || player.native() == nullptr) {
        return false;
    }
    try {
        event_->setPlayer(*player.native());
        return true;
    }
    catch (...) {
        return false;
    }
}

std::unique_ptr<::aegilex::native::server::PlayerCollection> PlayerChatEventFacade::getRecipients() const noexcept
{
    try {
        if (event_ == nullptr) {
            return std::unique_ptr<::aegilex::native::server::PlayerCollection>();
        }
        std::vector<std::unique_ptr<::aegilex::native::player::Player>> players;
        for (auto *player : event_->getRecipients()) {
            if (player != nullptr) {
                players.push_back(std::make_unique<::aegilex::native::player::Player>(player));
            }
        }
        return std::make_unique<::aegilex::native::server::PlayerCollection>(std::move(players));
    }
    catch (...) {
        return std::unique_ptr<::aegilex::native::server::PlayerCollection>();
    }
}

rust::String PlayerChatEventFacade::getMessageForRust() const noexcept
{
    try {
        return rust::String(getMessage());
    }
    catch (...) {
        return rust::String();
    }
}

bool PlayerChatEventFacade::setMessageForRust(const rust::Str message) noexcept
{
    return setMessage(std::string_view(message.data(), message.size()));
}

rust::String PlayerChatEventFacade::getFormatForRust() const noexcept
{
    try {
        return rust::String(getFormat());
    }
    catch (...) {
        return rust::String();
    }
}

bool PlayerChatEventFacade::setFormatForRust(const rust::Str format) noexcept
{
    return setFormat(std::string_view(format.data(), format.size()));
}

} // namespace aegilex::native::endstone_binding::events
