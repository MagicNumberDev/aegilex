#pragma once

#include <cstdint>

namespace aegilex::native::endstone_binding::events {

// Stores only the callback's copied LoadType discriminant.
class ServerLoadEventFacade final {
  public:
    explicit ServerLoadEventFacade(std::uint8_t load_type) noexcept;
    ~ServerLoadEventFacade() noexcept = default;

    ServerLoadEventFacade(const ServerLoadEventFacade &) = delete;
    ServerLoadEventFacade &operator=(const ServerLoadEventFacade &) = delete;
    ServerLoadEventFacade(ServerLoadEventFacade &&) = delete;
    ServerLoadEventFacade &operator=(ServerLoadEventFacade &&) = delete;

    [[nodiscard]] std::uint8_t getLoadType() const noexcept;

  private:
    std::uint8_t load_type_;
};

} // namespace aegilex::native::endstone_binding::events
