#include "plugin.h"

#include <aegilex-runtime/src/cxx_host_server.rs.h>

#include <endstone/command/plugin_command.h>
#include <endstone/plugin/plugin.h>
#include <endstone/plugin/plugin_load_order.h>
#include <endstone/plugin/plugin_loader.h>

#include <string>
#include <string_view>
#include <vector>

namespace aegilex::native::server {

class Plugin::impl {
  public:
    explicit impl(endstone::Plugin *plugin) noexcept : plugin(plugin)
    {
    }

    endstone::Plugin *plugin;
};

class PluginCommand::impl {
  public:
    explicit impl(endstone::PluginCommand *command) noexcept : command(command)
    {
    }

    endstone::PluginCommand *command;
};

Plugin::Plugin(endstone::Plugin *plugin) noexcept : impl_(std::make_shared<class Plugin::impl>(plugin))
{
}

endstone::Plugin *Plugin::native() const noexcept
{
    return impl_ == nullptr ? nullptr : impl_->plugin;
}

rust::String Plugin::getName() const
{
    try {
        return rust::String(impl_->plugin->getName());
    }
    catch (...) {
        return rust::String();
    }
}

rust::String Plugin::getVersion() const
{
    try {
        return rust::String(impl_->plugin->getDescription().getVersion());
    }
    catch (...) {
        return rust::String();
    }
}

rust::String Plugin::getFullName() const
{
    try {
        return rust::String(impl_->plugin->getDescription().getFullName());
    }
    catch (...) {
        return rust::String();
    }
}

rust::String Plugin::getApiVersion() const
{
    try {
        return rust::String(impl_->plugin->getDescription().getAPIVersion());
    }
    catch (...) {
        return rust::String();
    }
}

rust::String Plugin::getDescription() const
{
    try {
        return rust::String(impl_->plugin->getDescription().getDescription());
    }
    catch (...) {
        return rust::String();
    }
}

std::uint8_t Plugin::getLoadOrder() const
{
    try {
        switch (impl_->plugin->getDescription().getLoad()) {
        case endstone::PluginLoadOrder::Startup:
            return 0;
        case endstone::PluginLoadOrder::PostWorld:
            return 1;
        }
    }
    catch (...) {
    }
    return 1;
}

rust::Vec<rust::String> Plugin::listAuthors() const
{
    rust::Vec<rust::String> authors;
    try {
        for (const auto &author : impl_->plugin->getDescription().getAuthors()) {
            authors.push_back(rust::String(author));
        }
    }
    catch (...) {
    }
    return authors;
}

rust::Vec<rust::String> Plugin::listContributors() const
{
    rust::Vec<rust::String> contributors;
    try {
        for (const auto &contributor : impl_->plugin->getDescription().getContributors()) {
            contributors.push_back(rust::String(contributor));
        }
    }
    catch (...) {
    }
    return contributors;
}

rust::String Plugin::getWebsite() const
{
    try {
        return rust::String(impl_->plugin->getDescription().getWebsite());
    }
    catch (...) {
        return rust::String();
    }
}

rust::String Plugin::getPrefix() const
{
    try {
        return rust::String(impl_->plugin->getDescription().getPrefix());
    }
    catch (...) {
        return rust::String();
    }
}

rust::Vec<rust::String> Plugin::listProvides() const
{
    rust::Vec<rust::String> provides;
    try {
        for (const auto &value : impl_->plugin->getDescription().getProvides()) {
            provides.push_back(rust::String(value));
        }
    }
    catch (...) {
    }
    return provides;
}

rust::Vec<rust::String> Plugin::listDepend() const
{
    rust::Vec<rust::String> depend;
    try {
        for (const auto &value : impl_->plugin->getDescription().getDepend()) {
            depend.push_back(rust::String(value));
        }
    }
    catch (...) {
    }
    return depend;
}

rust::Vec<rust::String> Plugin::listSoftDepend() const
{
    rust::Vec<rust::String> soft_depend;
    try {
        for (const auto &value : impl_->plugin->getDescription().getSoftDepend()) {
            soft_depend.push_back(rust::String(value));
        }
    }
    catch (...) {
    }
    return soft_depend;
}

rust::Vec<rust::String> Plugin::listLoadBefore() const
{
    rust::Vec<rust::String> load_before;
    try {
        for (const auto &value : impl_->plugin->getDescription().getLoadBefore()) {
            load_before.push_back(rust::String(value));
        }
    }
    catch (...) {
    }
    return load_before;
}

std::uint8_t Plugin::getDefaultPermission() const
{
    try {
        return static_cast<std::uint8_t>(impl_->plugin->getDescription().getDefaultPermission());
    }
    catch (...) {
        return 0;
    }
}

rust::Vec<rust::String> Plugin::listCommands() const
{
    rust::Vec<rust::String> commands;
    try {
        for (const auto &command : impl_->plugin->getDescription().getCommands()) {
            commands.push_back(rust::String(command.getName()));
        }
    }
    catch (...) {
    }
    return commands;
}

bool Plugin::isEnabled() const
{
    try {
        return impl_->plugin->isEnabled();
    }
    catch (...) {
        return false;
    }
}

rust::String Plugin::getDataFolder() const
{
    try {
        return rust::String(impl_->plugin->getDataFolder().string());
    }
    catch (...) {
        return rust::String();
    }
}

rust::Vec<rust::String> Plugin::listLoaderFileFilters() const
{
    rust::Vec<rust::String> filters;
    try {
        for (const auto &filter : impl_->plugin->getPluginLoader().getPluginFileFilters()) {
            filters.push_back(rust::String(filter));
        }
    }
    catch (...) {
    }
    return filters;
}

std::unique_ptr<PluginCommand> Plugin::getCommand(const rust::Str name) const
{
    try {
        if (native() == nullptr) {
            return std::unique_ptr<PluginCommand>();
        }
        auto *command = native()->getCommand(std::string(name.data(), name.size()));
        return command == nullptr || &command->getPlugin() != native() ? std::unique_ptr<PluginCommand>()
                                                                       : std::make_unique<PluginCommand>(command);
    }
    catch (...) {
        return std::unique_ptr<PluginCommand>();
    }
}

PluginCommand::PluginCommand(endstone::PluginCommand *command) noexcept
    : impl_(std::make_shared<class PluginCommand::impl>(command))
{
}

endstone::PluginCommand *PluginCommand::native() const noexcept
{
    return impl_ == nullptr ? nullptr : impl_->command;
}

rust::String PluginCommand::getName() const
{
    try {
        return rust::String(impl_->command->getName());
    }
    catch (...) {
        return rust::String();
    }
}

rust::String PluginCommand::getDescription() const
{
    try {
        return rust::String(impl_->command->getDescription());
    }
    catch (...) {
        return rust::String();
    }
}

rust::Vec<rust::String> PluginCommand::getAliases() const
{
    rust::Vec<rust::String> aliases;
    try {
        for (const auto &alias : impl_->command->getAliases()) {
            aliases.push_back(rust::String(alias));
        }
    }
    catch (...) {
    }
    return aliases;
}

rust::Vec<rust::String> PluginCommand::getUsages() const
{
    rust::Vec<rust::String> usages;
    try {
        for (const auto &usage : impl_->command->getUsages()) {
            usages.push_back(rust::String(usage));
        }
    }
    catch (...) {
    }
    return usages;
}

rust::Vec<rust::String> PluginCommand::getPermissions() const
{
    rust::Vec<rust::String> permissions;
    try {
        for (const auto &permission : impl_->command->getPermissions()) {
            permissions.push_back(rust::String(permission));
        }
    }
    catch (...) {
    }
    return permissions;
}

} // namespace aegilex::native::server
