#pragma once

#include "rust/cxx.h"

#include <optional>
#include <string>
#include <string_view>

namespace endstone {
class ServerListPingEvent;
}

namespace aegilex::native::endstone_binding::events {

class ServerListPingEventFacade final {
  public:
    explicit ServerListPingEventFacade(endstone::ServerListPingEvent *event) noexcept;
    ~ServerListPingEventFacade() noexcept = default;

    ServerListPingEventFacade(const ServerListPingEventFacade &) = delete;
    ServerListPingEventFacade &operator=(const ServerListPingEventFacade &) = delete;

    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;
    [[nodiscard]] std::string getMotd() const noexcept;
    [[nodiscard]] bool setMotd(std::string_view motd) noexcept;
    [[nodiscard]] std::string getServerGuid() const noexcept;
    [[nodiscard]] bool setServerGuid(std::string_view guid) noexcept;
    [[nodiscard]] int getLocalPort() const noexcept;
    [[nodiscard]] bool setLocalPort(int port) noexcept;
    [[nodiscard]] int getLocalPortV6() const noexcept;
    [[nodiscard]] bool setLocalPortV6(int port) noexcept;

    // CXX bridge adapters preserve the native string/string_view facade API.
    [[nodiscard]] rust::String getMotdForRust() const noexcept;
    [[nodiscard]] bool setMotdForRust(rust::Str motd) noexcept;
    [[nodiscard]] rust::String getServerGuidForRust() const noexcept;
    [[nodiscard]] bool setServerGuidForRust(rust::Str guid) noexcept;

  private:
    endstone::ServerListPingEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
