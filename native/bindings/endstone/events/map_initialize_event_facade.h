#pragma once

#include "rust/cxx.h"

#include <cstdint>

namespace endstone {
class MapInitializeEvent;
}

namespace aegilex::native::endstone_binding::events {

// Non-owning, callback-scoped view of MapInitializeEvent. The event carries
// only the initialized map id.
class MapInitializeEventFacade final {
  public:
    explicit MapInitializeEventFacade(endstone::MapInitializeEvent *event) noexcept;
    ~MapInitializeEventFacade() noexcept = default;

    MapInitializeEventFacade(const MapInitializeEventFacade &) = delete;
    MapInitializeEventFacade &operator=(const MapInitializeEventFacade &) = delete;

    [[nodiscard]] std::int64_t getMapIdForRust() const noexcept;

  private:
    endstone::MapInitializeEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
