#pragma once

#include "aegilex_types.h"
#include <endstone/endstone.hpp>

#include <filesystem>
#include <memory>
#include <string>
#include <vector>

#include "host_context.h"
#include "runtime_bridge.h"

namespace aegilex::native {

class WasmPluginProxy final : public endstone::Plugin {
  public:
    WasmPluginProxy(Runtime *runtime, std::filesystem::path component_path, WasmPluginMetadata metadata);

    [[nodiscard]] const endstone::PluginDescription &getDescription() const override;
    void onLoad() override;
    void onEnable() override;
    void onDisable() override;
    bool onCommand(endstone::CommandSender &sender, const endstone::Command &command,
                   const std::vector<std::string> &args) override;

    [[nodiscard]] aegilex::status last_status() const noexcept;
    [[nodiscard]] aegilex::status prepare() noexcept;
    [[nodiscard]] aegilex::status activate();
    void reset_guest_state() noexcept;
    void set_runtime(Runtime *runtime) noexcept;
    void set_context(HostContext *context) noexcept;

  private:
    Runtime *runtime_{};
    HostContext *context_{};
    std::filesystem::path component_path_;
    endstone::PluginDescription description_;
    aegilex::status last_status_{aegilex::kOk};
    bool guest_enabled_{};
};

class WasmPluginLoader final : public endstone::PluginLoader {
  public:
    WasmPluginLoader(endstone::Server &server, std::filesystem::path plugin_root);
    ~WasmPluginLoader() override;

    [[nodiscard]] endstone::Plugin *loadPlugin(std::string file) override;
    [[nodiscard]] std::vector<std::string> getPluginFileFilters() const override;
    void enablePlugin(endstone::Plugin &plugin) const override;

    void set_runtime(Runtime *runtime) noexcept;
    void set_context(HostContext *context) noexcept;
    void clear_runtime() noexcept;
    void reset_guest_states() noexcept;
    [[nodiscard]] std::vector<std::string> component_paths() const;
    [[nodiscard]] aegilex::status prepare_loaded_plugins() noexcept;
    void enable_owned_plugins() const;
    void disable_owned_plugins() const;
    [[nodiscard]] WasmPluginProxy *find_plugin(std::string_view id) const noexcept;
    [[nodiscard]] Runtime *runtime() const noexcept
    {
        return runtime_;
    }

  private:
    [[nodiscard]] bool is_managed_component(const std::filesystem::path &component_path) const noexcept;

    std::filesystem::path plugin_root_;
    Runtime *runtime_{};
    HostContext *context_{};
    std::vector<std::unique_ptr<WasmPluginProxy>> plugins_;
};

} // namespace aegilex::native
