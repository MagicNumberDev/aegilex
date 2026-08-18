#include "player_respawn_event_facade.h"

#include <endstone/event/player/player_respawn_event.h>

namespace aegilex::native::endstone_binding::events {

PlayerRespawnEventFacade::PlayerRespawnEventFacade(endstone::PlayerRespawnEvent *event) noexcept : event_(event)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerRespawnEventFacade::getPlayer() const noexcept
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

} // namespace aegilex::native::endstone_binding::events
