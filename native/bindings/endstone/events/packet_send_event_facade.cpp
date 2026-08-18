#include "packet_send_event_facade.h"

#include <aegilex-runtime/src/cxx_runtime.rs.h>
#include <endstone/util/socket_address.h>
#include <endstone/event/server/packet_send_event.h>

#include <algorithm>

namespace aegilex::native::endstone_binding::events {

PacketSendEventFacade::PacketSendEventFacade(endstone::PacketSendEvent *event) noexcept : event_(event)
{
}

std::int32_t PacketSendEventFacade::getPacketId() const noexcept
{
    if (event_ == nullptr) {
        return 0;
    }
    try {
        return event_->getPacketId();
    }
    catch (...) {
        return 0;
    }
}

rust::Vec<std::uint8_t> PacketSendEventFacade::getPayloadForRust() const noexcept
{
    rust::Vec<std::uint8_t> payload;
    try {
        const std::string_view source = event_->getPayload();
        payload.reserve(source.size());
        std::copy(source.begin(), source.end(), std::back_inserter(payload));
    }
    catch (...) {
    }
    return payload;
}

bool PacketSendEventFacade::setPayloadForRust(const rust::Slice<const std::uint8_t> payload) noexcept
{
    if (event_ == nullptr) {
        return false;
    }
    try {
        event_->setPayload(std::string(reinterpret_cast<const char *>(payload.data()), payload.size()));
        return true;
    }
    catch (...) {
        return false;
    }
}

std::unique_ptr<::aegilex::native::player::Player> PacketSendEventFacade::getPlayer() const noexcept
{
    if (event_ == nullptr) {
        return nullptr;
    }
    try {
        auto *player = event_->getPlayer();
        return player == nullptr ? std::unique_ptr<::aegilex::native::player::Player>()
                                 : std::make_unique<::aegilex::native::player::Player>(player);
    }
    catch (...) {
        return nullptr;
    }
}

aegilex::runtime::SocketAddress PacketSendEventFacade::getAddress() const noexcept
{
    try {
        const auto address = event_->getAddress();
        return aegilex::runtime::SocketAddress{.hostname = rust::String(address.getHostname()),
                                               .port = address.getPort()};
    }
    catch (...) {
        return aegilex::runtime::SocketAddress{.hostname = rust::String(), .port = 0};
    }
}

std::uint8_t PacketSendEventFacade::getSubClientId() const noexcept
{
    if (event_ == nullptr) {
        return 0;
    }
    try {
        return static_cast<std::uint8_t>(event_->getSubClientId());
    }
    catch (...) {
        return 0;
    }
}

bool PacketSendEventFacade::isCancelled() const noexcept
{
    if (event_ == nullptr) {
        return false;
    }
    try {
        return event_->isCancelled();
    }
    catch (...) {
        return false;
    }
}

bool PacketSendEventFacade::setCancelled(const bool cancelled) noexcept
{
    if (event_ == nullptr) {
        return false;
    }
    try {
        event_->setCancelled(cancelled);
        return true;
    }
    catch (...) {
        return false;
    }
}

} // namespace aegilex::native::endstone_binding::events
