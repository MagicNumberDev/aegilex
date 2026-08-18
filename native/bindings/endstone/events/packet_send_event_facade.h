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
class PacketSendEvent;
}

namespace aegilex::runtime {
struct SocketAddress;
}

namespace aegilex::native::endstone_binding::events {

// Non-owning, callback-scoped view of PacketSendEvent. Payload bytes cross
// the boundary by value; replacement and cancellation mutate the live event.
class PacketSendEventFacade final {
  public:
    explicit PacketSendEventFacade(endstone::PacketSendEvent *event) noexcept;
    ~PacketSendEventFacade() noexcept = default;

    PacketSendEventFacade(const PacketSendEventFacade &) = delete;
    PacketSendEventFacade &operator=(const PacketSendEventFacade &) = delete;

    [[nodiscard]] std::int32_t getPacketId() const noexcept;
    [[nodiscard]] rust::Vec<std::uint8_t> getPayloadForRust() const noexcept;
    [[nodiscard]] bool setPayloadForRust(rust::Slice<const std::uint8_t> payload) noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> getPlayer() const noexcept;
    [[nodiscard]] aegilex::runtime::SocketAddress getAddress() const noexcept;
    [[nodiscard]] std::uint8_t getSubClientId() const noexcept;
    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;

  private:
    endstone::PacketSendEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
