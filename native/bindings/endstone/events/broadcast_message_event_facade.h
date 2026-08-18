#pragma once

#include "rust/cxx.h"

#include <optional>
#include <string>
#include <string_view>

namespace endstone {
class BroadcastMessageEvent;
}

namespace aegilex::native::endstone_binding::events {

class BroadcastMessageEventFacade final {
  public:
    explicit BroadcastMessageEventFacade(endstone::BroadcastMessageEvent *event) noexcept;
    ~BroadcastMessageEventFacade() noexcept = default;

    BroadcastMessageEventFacade(const BroadcastMessageEventFacade &) = delete;
    BroadcastMessageEventFacade &operator=(const BroadcastMessageEventFacade &) = delete;

    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;
    [[nodiscard]] std::string getMessage() const noexcept;
    [[nodiscard]] bool setMessage(std::string_view message) noexcept;

    // CXX bridge adapters preserve the native string/string_view facade API.
    [[nodiscard]] rust::String getMessageForRust() const noexcept;
    [[nodiscard]] bool setMessageForRust(rust::Str message) noexcept;

  private:
    endstone::BroadcastMessageEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
