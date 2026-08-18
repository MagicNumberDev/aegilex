#pragma once

#include "bindings/endstone/actor/player.h"
#include "rust/cxx.h"

#include <cstdint>
#include <memory>
#include <optional>
#include <string>
#include <string_view>
#include <vector>

namespace endstone {
class PacketReceiveEvent;
}

namespace aegilex::runtime {
struct SocketAddress;
}

namespace aegilex::native::endstone_binding::events {

// Non-owning, callback-scoped view of PacketReceiveEvent. Payload bytes cross
// the boundary by value; replacement and cancellation mutate the live event.
class PacketReceiveEventFacade final {
  public:
    explicit PacketReceiveEventFacade(endstone::PacketReceiveEvent *event) noexcept;
    ~PacketReceiveEventFacade() noexcept = default;

    PacketReceiveEventFacade(const PacketReceiveEventFacade &) = delete;
    PacketReceiveEventFacade &operator=(const PacketReceiveEventFacade &) = delete;

    [[nodiscard]] std::int32_t getPacketId() const noexcept;
    [[nodiscard]] rust::Vec<std::uint8_t> getPayloadForRust() const noexcept;
    [[nodiscard]] bool setPayloadForRust(rust::Slice<const std::uint8_t> payload) noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> getPlayer() const noexcept;
    [[nodiscard]] aegilex::runtime::SocketAddress getAddress() const noexcept;
    [[nodiscard]] std::uint8_t getSubClientId() const noexcept;
    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;

  private:
    endstone::PacketReceiveEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
