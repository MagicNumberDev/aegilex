#include "server_list_ping_event_facade.h"

#include <endstone/event/server/server_list_ping_event.h>

namespace aegilex::native::endstone_binding::events {

ServerListPingEventFacade::ServerListPingEventFacade(endstone::ServerListPingEvent *event) noexcept : event_(event)
{
}

bool ServerListPingEventFacade::isCancelled() const noexcept
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

bool ServerListPingEventFacade::setCancelled(const bool cancelled) noexcept
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

std::string ServerListPingEventFacade::getMotd() const noexcept
{
    if (event_ == nullptr) {
        return {};
    }

    try {
        return event_->getMotd();
    }
    catch (...) {
        return {};
    }
}

bool ServerListPingEventFacade::setMotd(const std::string_view motd) noexcept
{
    if (event_ == nullptr) {
        return false;
    }
    try {
        event_->setMotd(std::string(motd));
        return true;
    }
    catch (...) {
        return false;
    }
}

std::string ServerListPingEventFacade::getServerGuid() const noexcept
{
    if (event_ == nullptr) {
        return {};
    }

    try {
        return event_->getServerGuid();
    }
    catch (...) {
        return {};
    }
}

bool ServerListPingEventFacade::setServerGuid(const std::string_view guid) noexcept
{
    if (event_ == nullptr) {
        return false;
    }
    try {
        event_->setServerGuid(std::string(guid));
        return true;
    }
    catch (...) {
        return false;
    }
}

int ServerListPingEventFacade::getLocalPort() const noexcept
{
    if (event_ == nullptr) {
        return 0;
    }

    try {
        return event_->getLocalPort();
    }
    catch (...) {
        return 0;
    }
}

bool ServerListPingEventFacade::setLocalPort(const int port) noexcept
{
    if (event_ == nullptr || port <= 0) {
        return false;
    }
    try {
        event_->setLocalPort(port);
        return true;
    }
    catch (...) {
        return false;
    }
}

int ServerListPingEventFacade::getLocalPortV6() const noexcept
{
    if (event_ == nullptr) {
        return 0;
    }

    try {
        return event_->getLocalPortV6();
    }
    catch (...) {
        return 0;
    }
}

bool ServerListPingEventFacade::setLocalPortV6(const int port) noexcept
{
    if (event_ == nullptr || port <= 0) {
        return false;
    }
    try {
        event_->setLocalPortV6(port);
        return true;
    }
    catch (...) {
        return false;
    }
}

rust::String ServerListPingEventFacade::getMotdForRust() const noexcept
{
    try {
        return rust::String(getMotd());
    }
    catch (...) {
        return rust::String();
    }
}

bool ServerListPingEventFacade::setMotdForRust(const rust::Str motd) noexcept
{
    return setMotd(std::string_view(motd.data(), motd.size()));
}

rust::String ServerListPingEventFacade::getServerGuidForRust() const noexcept
{
    try {
        return rust::String(getServerGuid());
    }
    catch (...) {
        return rust::String();
    }
}

bool ServerListPingEventFacade::setServerGuidForRust(const rust::Str guid) noexcept
{
    return setServerGuid(std::string_view(guid.data(), guid.size()));
}

} // namespace aegilex::native::endstone_binding::events
