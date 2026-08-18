#pragma once

#include "rust/cxx.h"

#include <cstdint>
#include <memory>
#include <string>

namespace endstone {
class Plugin;
class PluginCommand;
} // namespace endstone

namespace aegilex::native::server {

class Plugin;
class PluginCommand;

// OOP/Pimpl facade over endstone::Plugin. The impl holds only a non-owning
// endstone::Plugin* (the plugin loader owns the plugin); mirrors
// endstone/plugin/plugin.h and endstone/plugin/plugin_description.h.
class Plugin {
  public:
    explicit Plugin(endstone::Plugin *plugin) noexcept;
    ~Plugin() noexcept = default;

    Plugin(const Plugin &) = delete;
    Plugin &operator=(const Plugin &) = delete;

    rust::String getName() const;
    rust::String getVersion() const;
    rust::String getFullName() const;
    rust::String getApiVersion() const;
    rust::String getDescription() const;
    std::uint8_t getLoadOrder() const;
    rust::Vec<rust::String> listAuthors() const;
    rust::Vec<rust::String> listContributors() const;
    rust::String getWebsite() const;
    rust::String getPrefix() const;
    rust::Vec<rust::String> listProvides() const;
    rust::Vec<rust::String> listDepend() const;
    rust::Vec<rust::String> listSoftDepend() const;
    rust::Vec<rust::String> listLoadBefore() const;
    std::uint8_t getDefaultPermission() const;
    rust::Vec<rust::String> listCommands() const;
    bool isEnabled() const;
    rust::String getDataFolder() const;
    rust::Vec<rust::String> listLoaderFileFilters() const;
    [[nodiscard]] std::unique_ptr<PluginCommand> getCommand(rust::Str name) const;
    [[nodiscard]] endstone::Plugin *native() const noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl_;
};

// OOP/Pimpl facade over endstone::PluginCommand (owned by the server's
// command map); mirrors endstone/command/plugin_command.h.
class PluginCommand {
  public:
    explicit PluginCommand(endstone::PluginCommand *command) noexcept;
    ~PluginCommand() noexcept = default;

    PluginCommand(const PluginCommand &) = delete;
    PluginCommand &operator=(const PluginCommand &) = delete;

    rust::String getName() const;
    rust::String getDescription() const;
    rust::Vec<rust::String> getAliases() const;
    rust::Vec<rust::String> getUsages() const;
    rust::Vec<rust::String> getPermissions() const;
    [[nodiscard]] endstone::PluginCommand *native() const noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl_;
};

} // namespace aegilex::native::server
