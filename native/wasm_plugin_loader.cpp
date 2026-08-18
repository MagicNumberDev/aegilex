#include "wasm_plugin_loader.h"
#include "aegilex_types.h"

#include "bridge/command_bridge.h"
#include "bridge/task_bridge.h"
#include "bridge/form_bridge.h"
#include "runtime_bridge.h"

#include <algorithm>
#include <memory>
#include <stdexcept>
#include <system_error>
#include <unordered_map>
#include <utility>

namespace {

constexpr std::string_view kComponentName = "plugin.wasm";

[[nodiscard]] std::vector<endstone::Command> make_commands(const std::vector<aegilex::native::WasmCommandSpec> &specs)
{
    std::vector<endstone::Command> commands;
    commands.reserve(specs.size());
    for (const auto &spec : specs) {
        commands.emplace_back(spec.name, spec.description, spec.usages, spec.aliases, spec.permissions);
    }
    return commands;
}

[[nodiscard]] std::vector<endstone::Permission>
make_permissions(const std::vector<aegilex::native::WasmPermissionSpec> &specs)
{
    std::vector<endstone::Permission> permissions;
    permissions.reserve(specs.size());
    for (const auto &spec : specs) {
        const auto default_value = spec.has_default_value ? static_cast<endstone::PermissionDefault>(spec.default_value)
                                                          : endstone::Permission::DefaultPermission;
        std::unordered_map<std::string, bool> children;
        children.reserve(spec.children.size());
        for (const auto &child : spec.children) {
            children.emplace(child.name, child.value);
        }
        permissions.emplace_back(spec.name, spec.description, default_value, std::move(children));
    }
    return permissions;
}

void ensure_plugin_permissions(endstone::Plugin &plugin)
{
    auto &manager = plugin.getServer().getPluginManager();
    for (const auto &permission : plugin.getDescription().getPermissions()) {
        if (manager.getPermission(permission.getName()) == nullptr) {
            manager.addPermission(std::make_unique<endstone::Permission>(permission));
        }
    }
}

} // namespace

namespace aegilex::native {

WasmPluginProxy::WasmPluginProxy(Runtime *runtime, std::filesystem::path component_path, WasmPluginMetadata metadata)
    : runtime_(runtime), component_path_(std::move(component_path)),
      description_(std::move(metadata.name), std::move(metadata.version), std::move(metadata.description),
                   metadata.load_order == aegilex::kPluginLoadStartup ? endstone::PluginLoadOrder::Startup
                                                                      : endstone::PluginLoadOrder::PostWorld,
                   std::move(metadata.authors), std::move(metadata.contributors), std::move(metadata.website),
                   std::move(metadata.prefix), std::move(metadata.provides), std::move(metadata.depend),
                   std::move(metadata.soft_depend), std::move(metadata.load_before),
                   static_cast<endstone::PermissionDefault>(metadata.default_permission),
                   make_commands(metadata.commands), make_permissions(metadata.permissions))
{
}

const endstone::PluginDescription &WasmPluginProxy::getDescription() const
{
    return description_;
}

void WasmPluginProxy::onLoad()
{
    last_status_ = prepare();
    if (last_status_ != aegilex::kOk) {
        throw std::runtime_error("Aegilex could not prepare the Wasm component (status " +
                                 std::to_string(last_status_) + ").");
    }
}

void WasmPluginProxy::onEnable()
{
}

bool WasmPluginProxy::onCommand(endstone::CommandSender &sender, const endstone::Command &command,
                                const std::vector<std::string> &args)
{
    if (context_ == nullptr || context_->command_bridge == nullptr) {
        return false;
    }
    return context_->command_bridge->handle_guest_command(sender, getName(), command.getName(), args);
}

void WasmPluginProxy::onDisable()
{
    if (guest_enabled_) {
        last_status_ = disable_plugin(runtime_, getName());
        if (last_status_ != aegilex::kOk) {
            getLogger().error("Aegilex could not disable this Wasm component (status {}).", last_status_);
        }
        guest_enabled_ = false;
    }
    if (context_ != nullptr) {
        context_->remove_enabled_plugin_id(getName());
        if (context_->task_bridge != nullptr) {
            context_->task_bridge->cancel_all_for_plugin(getName());
        }
        if (context_->form_bridge != nullptr) {
            context_->form_bridge->clear_for_plugin(getName());
        }
    }
}

aegilex::status WasmPluginProxy::last_status() const noexcept
{
    return last_status_;
}

aegilex::status WasmPluginProxy::prepare() noexcept
{
    last_status_ = prepare_plugin(runtime_, component_path_.string());
    return last_status_;
}

aegilex::status WasmPluginProxy::activate()
{
    if (guest_enabled_) {
        return aegilex::kOk;
    }

    if (context_ != nullptr) {
        context_->record_enabled_plugin_id(getName());
    }
    last_status_ = enable_plugin(runtime_, getName());
    if (last_status_ != aegilex::kOk) {
        if (context_ != nullptr) {
            context_->remove_enabled_plugin_id(getName());
        }
        getLogger().error("Aegilex could not enable this Wasm component (status {}).", last_status_);
        return last_status_;
    }
    guest_enabled_ = true;
    return aegilex::kOk;
}

void WasmPluginProxy::reset_guest_state() noexcept
{
    guest_enabled_ = false;
    last_status_ = aegilex::kOk;
}

void WasmPluginProxy::set_runtime(Runtime *runtime) noexcept
{
    runtime_ = runtime;
}

void WasmPluginProxy::set_context(HostContext *context) noexcept
{
    context_ = context;
}

WasmPluginLoader::WasmPluginLoader(endstone::Server &server, std::filesystem::path plugin_root)
    : PluginLoader(server), plugin_root_(std::move(plugin_root))
{
}

WasmPluginLoader::~WasmPluginLoader()
{
    if (context_ != nullptr && context_->wasm_loader == this) {
        context_->wasm_loader = nullptr;
    }
}

endstone::Plugin *WasmPluginLoader::loadPlugin(const std::string file)
{
    if (runtime_ == nullptr) {
        server_.getLogger().error("Cannot load Wasm plugin '{}': Aegilex is not running.", file);
        return nullptr;
    }

    const std::filesystem::path component_path{file};
    if (!is_managed_component(component_path)) {
        server_.getLogger().error("Cannot load Wasm plugin '{}': component is outside Aegilex's plugin root.", file);
        return nullptr;
    }

    WasmPluginMetadata metadata;
    std::string error;
    const auto status = inspect_plugin(runtime_, component_path.string(), &metadata, &error);
    if (status != aegilex::kOk) {
        if (error.empty()) {
            server_.getLogger().error("Cannot inspect Wasm plugin '{}': Aegilex status {}.", file, status);
        }
        else {
            server_.getLogger().error("Cannot inspect Wasm plugin '{}': {}", file, error);
        }
        return nullptr;
    }
    if (std::any_of(plugins_.begin(), plugins_.end(),
                    [&metadata](const auto &plugin) { return plugin->getName() == metadata.name; })) {
        server_.getLogger().error("Cannot load Wasm plugin '{}': duplicate plugin id '{}'.", file, metadata.name);
        return nullptr;
    }

    auto plugin = std::make_unique<WasmPluginProxy>(runtime_, component_path, std::move(metadata));
    plugin->set_context(context_);
    auto *result = plugin.get();
    plugins_.push_back(std::move(plugin));
    return result;
}

std::vector<std::string> WasmPluginLoader::getPluginFileFilters() const
{
    return {"plugin\\.wasm$"};
}

void WasmPluginLoader::enablePlugin(endstone::Plugin &plugin) const
{
    auto &wasm_plugin = static_cast<WasmPluginProxy &>(plugin);
    if (wasm_plugin.last_status() != aegilex::kOk || context_ == nullptr || !context_->accepting_calls) {
        return;
    }
    ensure_plugin_permissions(plugin);
    endstone::PluginLoader::enablePlugin(plugin);
    if (!plugin.isEnabled()) {
        return;
    }
    if (wasm_plugin.activate() != aegilex::kOk) {
        endstone::PluginLoader::disablePlugin(plugin);
    }
}

void WasmPluginLoader::set_runtime(Runtime *runtime) noexcept
{
    runtime_ = runtime;
    for (const auto &plugin : plugins_) {
        plugin->set_runtime(runtime);
    }
}

void WasmPluginLoader::set_context(HostContext *context) noexcept
{
    if (context_ != nullptr && context_->wasm_loader == this) {
        context_->wasm_loader = nullptr;
    }
    context_ = context;
    if (context_ != nullptr) {
        context_->wasm_loader = this;
    }
    for (const auto &plugin : plugins_) {
        plugin->set_context(context);
    }
}

void WasmPluginLoader::clear_runtime() noexcept
{
    set_runtime(nullptr);
}

void WasmPluginLoader::reset_guest_states() noexcept
{
    for (const auto &plugin : plugins_) {
        plugin->reset_guest_state();
    }
}

std::vector<std::string> WasmPluginLoader::component_paths() const
{
    std::vector<std::string> components;
    std::error_code error;
    for (const auto &entry : std::filesystem::directory_iterator(plugin_root_, error)) {
        if (error) {
            return components;
        }
        if (!entry.is_directory(error) || error) {
            error.clear();
            continue;
        }
        const auto component = entry.path() / kComponentName;
        if (std::filesystem::is_regular_file(component, error) && !error) {
            components.push_back(component.string());
        }
        error.clear();
    }
    std::sort(components.begin(), components.end());
    return components;
}

aegilex::status WasmPluginLoader::prepare_loaded_plugins() noexcept
{
    for (const auto &plugin : plugins_) {
        if (const auto status = plugin->prepare(); status != aegilex::kOk) {
            return status;
        }
    }
    return aegilex::kOk;
}

void WasmPluginLoader::enable_owned_plugins() const
{
    for (const auto &plugin : plugins_) {
        if (!plugin->isEnabled()) {
            server_.getPluginManager().enablePlugin(*plugin);
        }
    }
}

void WasmPluginLoader::disable_owned_plugins() const
{
    for (auto it = plugins_.rbegin(); it != plugins_.rend(); ++it) {
        if ((*it)->isEnabled()) {
            server_.getPluginManager().disablePlugin(**it);
        }
    }
}

WasmPluginProxy *WasmPluginLoader::find_plugin(const std::string_view id) const noexcept
{
    const auto it =
        std::find_if(plugins_.begin(), plugins_.end(), [id](const auto &plugin) { return plugin->getName() == id; });
    return it == plugins_.end() ? nullptr : it->get();
}

bool WasmPluginLoader::is_managed_component(const std::filesystem::path &component_path) const noexcept
{
    std::error_code error;
    const auto root = std::filesystem::weakly_canonical(plugin_root_, error);
    if (error) {
        return false;
    }
    const auto component = std::filesystem::weakly_canonical(component_path, error);
    if (error || component.filename() != kComponentName) {
        return false;
    }
    const auto parent = component.parent_path();
    return parent.parent_path() == root && std::filesystem::is_regular_file(component, error) && !error;
}

} // namespace aegilex::native
