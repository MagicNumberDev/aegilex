#include "plugin.h"
#include "aegilex_types.h"
#include "version.h"

#include "bridge/command_bridge.h"
#include "bridge/event_bridge.h"
#include "bridge/form_bridge.h"
#include "bridge/map_renderer_bridge.h"
#include "bridge/task_bridge.h"
#include "runtime_bridge.h"
#include "wasm_plugin_loader.h"

#include <algorithm>
#include <filesystem>
#include <memory>
#include <stdexcept>
#include <system_error>

namespace {

constexpr std::string_view kGuestDirectoryName = "plugins";

} // namespace

AegilexPlugin::~AegilexPlugin()
{
    if (host_context_) {
        host_context_->close_bridges();
    }
}

void AegilexPlugin::onLoad()
{
    const auto plugin_root = std::filesystem::current_path() / kGuestDirectoryName;
    std::error_code error;
    std::filesystem::create_directories(plugin_root, error);
    if (error) {
        getLogger().error("Cannot create the Endstone plugin directory: {}", error.message());
        return;
    }

    auto loader = std::make_unique<aegilex::native::WasmPluginLoader>(getServer(), plugin_root);
    host_context_ = std::make_shared<aegilex::native::HostContext>(&getLogger(), &getServer(), loader.get());
    loader->set_context(host_context_.get());
    wasm_loader_ = loader.get();
    getServer().getPluginManager().registerLoader(std::move(loader));

    if (!start_runtime()) {
        throw std::runtime_error("Aegilex could not start its Wasm runtime.");
    }
    wasm_loader_->set_runtime(runtime_.get());

    const auto components = wasm_loader_->component_paths();
    getServer().getPluginManager().loadPlugins(components);
    getLogger().info("Aegilex loader registered; guest directory is {}.", plugin_root.string());
}

void AegilexPlugin::onEnable()
{
    if (!start_runtime()) {
        return;
    }

    wasm_loader_->set_runtime(runtime_.get());

    const auto status = wasm_loader_->prepare_loaded_plugins();
    if (status != aegilex::kOk) {
        getLogger().error("Failed to prepare Aegilex Wasm plugins (status {}).", status);
        return;
    }

    host_context_->event_bridge = std::make_unique<aegilex::native::EventBridge>(*host_context_, *this, runtime_.get());
    host_context_->event_bridge->register_listeners();
    host_context_->command_bridge = std::make_unique<aegilex::native::CommandBridge>(*host_context_, runtime_.get());
    host_context_->task_bridge = std::make_unique<aegilex::native::TaskBridge>(*host_context_, *this, runtime_.get());
    host_context_->form_bridge = std::make_unique<aegilex::native::FormBridge>(*host_context_, *this, runtime_.get());
    host_context_->map_renderer_bridge =
        std::make_unique<aegilex::native::MapRendererBridge>(*host_context_, runtime_.get());

    host_context_->accepting_calls = true;
    wasm_loader_->enable_owned_plugins();
    getLogger().info("Aegilex runtime enabled.");
}

void AegilexPlugin::onDisable()
{
    if (wasm_loader_ != nullptr) {
        wasm_loader_->disable_owned_plugins();
    }
    if (host_context_) {
        host_context_->accepting_calls = false;
        host_context_->close_bridges();
    }

    getLogger().info("Aegilex runtime disabled.");
}

bool AegilexPlugin::start_runtime()
{
    if (runtime_ != nullptr) {
        return true;
    }
    if (!host_context_) {
        getLogger().error("Cannot start the Aegilex runtime before plugin load.");
        return false;
    }

    const auto runtime_config = aegilex::native::default_runtime_config();
    const auto status = aegilex::native::create_runtime(host_context_, runtime_config, &runtime_);
    if (status != aegilex::kOk) {
        runtime_.reset();
        getLogger().error("Failed to start the Aegilex runtime (status {}).", status);
        return false;
    }
    return true;
}

bool AegilexPlugin::onCommand(endstone::CommandSender &sender, const endstone::Command &command,
                              const std::vector<std::string> &args)
{
    if (command.getName() != "aegilex") {
        return false;
    }

    if (args.empty()) {
        sender.sendMessage("Aegilex runtime is {}.", runtime_ == nullptr ? "unavailable" : "running");
        return true;
    }

    const auto &subcommand = args[0];
    const bool admin_subcommand = subcommand == "list" || subcommand == "info" || subcommand == "enable" ||
                                  subcommand == "disable" || subcommand == "reload" || subcommand == "load" ||
                                  subcommand == "unload";
    if (admin_subcommand) {
        sender.sendMessage("Aegilex runtime is {}.", runtime_ == nullptr ? "unavailable" : "running");
        return true;
    }

    sender.sendErrorMessage("Usage: /aegilex");
    return false;
}

ENDSTONE_PLUGIN("aegilex", AEGILEX_VERSION, AegilexPlugin)
{
    prefix = "Aegilex";
    description = "Wasm plugin loader for Endstone";
    authors = {"Aegilex Contributors"};
    load = endstone::PluginLoadOrder::PostWorld;

    command("aegilex")
        .description("Show the Aegilex runtime status")
        .usages("/aegilex")
        .permissions("aegilex.command.manage");

    permission("aegilex.command.manage")
        .description("Allow managing the Aegilex runtime.")
        .default_(endstone::PermissionDefault::Operator);
}

namespace aegilex::native {

HostContext::HostContext(endstone::Logger *logger, endstone::Server *native_server,
                         WasmPluginLoader *wasm_loader) noexcept
    : logger(logger), server(native_server, wasm_loader), wasm_loader(wasm_loader)
{
}

HostContext::~HostContext() noexcept = default;

std::shared_ptr<HostContext> HostContext::testStub()
{
    return {};
}

void HostContext::close_bridges() noexcept
{
    if (event_bridge != nullptr) {
        event_bridge->unregister_all();
    }
    if (task_bridge != nullptr) {
        task_bridge->cancel_all();
    }
    if (form_bridge != nullptr) {
        form_bridge->clear_all();
    }
    if (map_renderer_bridge != nullptr) {
        map_renderer_bridge->clear_all();
    }
    server.clearRegistryCaches();
    event_bridge.reset();
    command_bridge.reset();
    task_bridge.reset();
    form_bridge.reset();
    map_renderer_bridge.reset();
    enabled_plugin_ids_.clear();
}

void HostContext::remove_enabled_plugin_id(const std::string &plugin_id) noexcept
{
    enabled_plugin_ids_.erase(std::remove(enabled_plugin_ids_.begin(), enabled_plugin_ids_.end(), plugin_id),
                              enabled_plugin_ids_.end());
}

} // namespace aegilex::native
