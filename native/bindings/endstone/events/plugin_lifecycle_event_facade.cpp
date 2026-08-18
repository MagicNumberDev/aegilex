#include "plugin_lifecycle_event_facade.h"

#include <utility>

namespace aegilex::native::endstone_binding::events {

PluginLifecycleEventFacade::PluginLifecycleEventFacade(std::string plugin_name) noexcept
    : plugin_name_(std::move(plugin_name))
{
}

rust::String PluginLifecycleEventFacade::getPluginNameForRust() const noexcept
{
    return rust::String(plugin_name_);
}

} // namespace aegilex::native::endstone_binding::events
