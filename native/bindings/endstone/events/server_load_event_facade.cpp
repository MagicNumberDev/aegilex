#include "server_load_event_facade.h"

namespace aegilex::native::endstone_binding::events {

ServerLoadEventFacade::ServerLoadEventFacade(const std::uint8_t load_type) noexcept : load_type_(load_type)
{
}

std::uint8_t ServerLoadEventFacade::getLoadType() const noexcept
{
    return load_type_;
}

} // namespace aegilex::native::endstone_binding::events
