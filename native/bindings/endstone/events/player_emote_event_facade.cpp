#include "player_emote_event_facade.h"

#include <endstone/event/player/player_emote_event.h>

namespace aegilex::native::endstone_binding::events {

PlayerEmoteEventFacade::PlayerEmoteEventFacade(endstone::PlayerEmoteEvent *event) noexcept : event_(event)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerEmoteEventFacade::getPlayer() const noexcept
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

rust::String PlayerEmoteEventFacade::getEmoteIdForRust() const noexcept
{
    if (event_ == nullptr) {
        return rust::String();
    }
    try {
        return rust::String(event_->getEmoteId());
    }
    catch (...) {
        return rust::String();
    }
}

bool PlayerEmoteEventFacade::isMuted() const noexcept
{
    if (event_ == nullptr) {
        return false;
    }
    try {
        return event_->isMuted();
    }
    catch (...) {
        return false;
    }
}

bool PlayerEmoteEventFacade::setMuted(const bool muted) noexcept
{
    if (event_ == nullptr) {
        return false;
    }
    try {
        event_->setMuted(muted);
        return true;
    }
    catch (...) {
        return false;
    }
}

bool PlayerEmoteEventFacade::isCancelled() const noexcept
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

bool PlayerEmoteEventFacade::setCancelled(const bool cancelled) noexcept
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
