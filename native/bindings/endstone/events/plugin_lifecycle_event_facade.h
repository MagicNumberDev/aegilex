#pragma once

#include "rust/cxx.h"

#include <string>

namespace aegilex::native::endstone_binding::events {

// Stores only the callback's copied plugin name; it never retains Plugin.
class PluginLifecycleEventFacade final {
  public:
    explicit PluginLifecycleEventFacade(std::string plugin_name) noexcept;
    ~PluginLifecycleEventFacade() noexcept = default;

    PluginLifecycleEventFacade(const PluginLifecycleEventFacade &) = delete;
    PluginLifecycleEventFacade &operator=(const PluginLifecycleEventFacade &) = delete;
    PluginLifecycleEventFacade(PluginLifecycleEventFacade &&) = delete;
    PluginLifecycleEventFacade &operator=(PluginLifecycleEventFacade &&) = delete;

    [[nodiscard]] rust::String getPluginNameForRust() const noexcept;

  private:
    std::string plugin_name_;
};

} // namespace aegilex::native::endstone_binding::events
