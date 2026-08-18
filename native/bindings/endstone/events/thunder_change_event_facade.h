#pragma once

#include <optional>

namespace endstone {
class ThunderChangeEvent;
}

namespace aegilex::native::endstone_binding::events {

class ThunderChangeEventFacade final {
  public:
    explicit ThunderChangeEventFacade(endstone::ThunderChangeEvent *event) noexcept;
    ~ThunderChangeEventFacade() noexcept = default;

    ThunderChangeEventFacade(const ThunderChangeEventFacade &) = delete;
    ThunderChangeEventFacade &operator=(const ThunderChangeEventFacade &) = delete;

    [[nodiscard]] bool getToThunder() const noexcept;
    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;

  private:
    endstone::ThunderChangeEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
