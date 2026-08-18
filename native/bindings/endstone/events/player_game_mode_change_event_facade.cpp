#include "player_game_mode_change_event_facade.h"

#include <endstone/event/player/player_game_mode_change_event.h>

namespace aegilex::native::endstone_binding::events {

PlayerGameModeChangeEventFacade::PlayerGameModeChangeEventFacade(endstone::PlayerGameModeChangeEvent *event) noexcept
    : event_(event)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerGameModeChangeEventFacade::getPlayer() const noexcept
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

std::uint8_t PlayerGameModeChangeEventFacade::getNewGameMode() const noexcept
{
    if (event_ == nullptr) {
        return 0;
    }
    try {
        return static_cast<std::uint8_t>(event_->getNewGameMode());
    }
    catch (...) {
        return 0;
    }
}

bool PlayerGameModeChangeEventFacade::isCancelled() const noexcept
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

bool PlayerGameModeChangeEventFacade::setCancelled(const bool cancelled) noexcept
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
