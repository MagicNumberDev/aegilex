#pragma once

#include "bindings/endstone/level/block.h"

#include <memory>
#include <optional>

namespace endstone {
class LeavesDecayEvent;
}

namespace aegilex::native::endstone_binding::events {

class LeavesDecayEventFacade final {
  public:
    explicit LeavesDecayEventFacade(endstone::LeavesDecayEvent *event) noexcept;
    ~LeavesDecayEventFacade() noexcept = default;

    LeavesDecayEventFacade(const LeavesDecayEventFacade &) = delete;
    LeavesDecayEventFacade &operator=(const LeavesDecayEventFacade &) = delete;
    LeavesDecayEventFacade(LeavesDecayEventFacade &&) = delete;
    LeavesDecayEventFacade &operator=(LeavesDecayEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::level::Block> getBlock() const noexcept;
    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;

  private:
    endstone::LeavesDecayEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
