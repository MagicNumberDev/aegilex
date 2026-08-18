#pragma once

#include "rust/cxx.h"

#include <optional>
#include <string>
#include <string_view>

namespace endstone {
class ServerCommandEvent;
}

namespace aegilex::native::endstone_binding::events {

class ServerCommandEventFacade final {
  public:
    explicit ServerCommandEventFacade(endstone::ServerCommandEvent *event) noexcept;
    ~ServerCommandEventFacade() noexcept = default;

    ServerCommandEventFacade(const ServerCommandEventFacade &) = delete;
    ServerCommandEventFacade &operator=(const ServerCommandEventFacade &) = delete;

    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;
    [[nodiscard]] std::string getCommand() const noexcept;
    [[nodiscard]] bool setCommand(std::string_view command) noexcept;

    // CXX bridge adapters preserve the native string/string_view facade API.
    [[nodiscard]] rust::String getSenderNameForRust() const noexcept;
    [[nodiscard]] rust::String getCommandForRust() const noexcept;
    [[nodiscard]] bool setCommandForRust(rust::Str command) noexcept;

  private:
    endstone::ServerCommandEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
