#include "player_bed_leave_event_facade.h"

#include <endstone/event/player/player_bed_leave_event.h>

namespace aegilex::native::endstone_binding::events {

PlayerBedLeaveEventFacade::PlayerBedLeaveEventFacade(endstone::PlayerBedLeaveEvent *event) noexcept : event_(event)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerBedLeaveEventFacade::getPlayer() const noexcept
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

std::unique_ptr<::aegilex::native::level::Block> PlayerBedLeaveEventFacade::getBed() const noexcept
{
    if (event_ == nullptr) {
        return nullptr;
    }
    try {
        return std::make_unique<::aegilex::native::level::Block>(event_->getBed());
    }
    catch (...) {
        return nullptr;
    }
}

} // namespace aegilex::native::endstone_binding::events
