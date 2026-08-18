#include "player_dimension_change_event_facade.h"

#include <endstone/event/player/player_dimension_change_event.h>
#include <endstone/level/dimension.h>

namespace aegilex::native::endstone_binding::events {

PlayerDimensionChangeEventFacade::PlayerDimensionChangeEventFacade(endstone::PlayerDimensionChangeEvent *event) noexcept
    : event_(event)
{
}

std::unique_ptr<::aegilex::native::player::Player> PlayerDimensionChangeEventFacade::getPlayer() const noexcept
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

rust::String PlayerDimensionChangeEventFacade::getFromForRust() const noexcept
{
    if (event_ == nullptr) {
        return {};
    }
    try {
        return rust::String(event_->getFrom().getName());
    }
    catch (...) {
        return {};
    }
}

rust::String PlayerDimensionChangeEventFacade::getToForRust() const noexcept
{
    if (event_ == nullptr) {
        return {};
    }
    try {
        return rust::String(event_->getTo().getName());
    }
    catch (...) {
        return {};
    }
}

} // namespace aegilex::native::endstone_binding::events
