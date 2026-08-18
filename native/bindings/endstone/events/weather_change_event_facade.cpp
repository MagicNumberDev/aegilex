#include "weather_change_event_facade.h"

#include <endstone/event/weather/weather_change_event.h>

namespace aegilex::native::endstone_binding::events {

WeatherChangeEventFacade::WeatherChangeEventFacade(endstone::WeatherChangeEvent *event) noexcept : event_(event)
{
}

bool WeatherChangeEventFacade::getToWeather() const noexcept
{
    if (event_ == nullptr) {
        return false;
    }

    try {
        return event_->toWeatherState();
    }
    catch (...) {
        return false;
    }
}

bool WeatherChangeEventFacade::isCancelled() const noexcept
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

bool WeatherChangeEventFacade::setCancelled(const bool cancelled) noexcept
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
