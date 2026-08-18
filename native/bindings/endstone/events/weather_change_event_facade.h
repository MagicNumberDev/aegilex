#pragma once

#include <optional>

namespace endstone {
class WeatherChangeEvent;
}

namespace aegilex::native::endstone_binding::events {

class WeatherChangeEventFacade final {
  public:
    explicit WeatherChangeEventFacade(endstone::WeatherChangeEvent *event) noexcept;
    ~WeatherChangeEventFacade() noexcept = default;

    WeatherChangeEventFacade(const WeatherChangeEventFacade &) = delete;
    WeatherChangeEventFacade &operator=(const WeatherChangeEventFacade &) = delete;

    [[nodiscard]] bool getToWeather() const noexcept;
    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;

  private:
    endstone::WeatherChangeEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
