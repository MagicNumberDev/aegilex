#pragma once

#include "aegilex_types.h"

#include "host_context.h"

#include <aegilex-runtime/src/cxx_runtime.rs.h>

#include <memory>
#include <string>
#include <string_view>
#include <vector>

namespace aegilex::native {

struct Runtime {
    Runtime(rust::Box<aegilex::runtime::RuntimeHandle> handle, HostContext *host_context) noexcept;

    rust::Box<aegilex::runtime::RuntimeHandle> handle;
    HostContext *host_context{};
};

struct WasmCommandSpec {
    std::string name;
    std::string description;
    std::vector<std::string> aliases;
    std::vector<std::string> usages;
    std::vector<std::string> permissions;
};

struct WasmPermissionChild {
    std::string name;
    bool value{};
};

struct WasmPermissionSpec {
    std::string name;
    std::string description;
    bool has_default_value{};
    std::uint32_t default_value{};
    std::vector<WasmPermissionChild> children;
};

struct WasmPluginMetadata {
    std::string name;
    std::string version;
    std::string description;
    std::uint32_t load_order{};
    std::vector<std::string> authors;
    std::vector<std::string> contributors;
    std::string website;
    std::string prefix;
    std::vector<std::string> provides;
    std::vector<std::string> depend;
    std::vector<std::string> soft_depend;
    std::vector<std::string> load_before;
    std::uint32_t default_permission{};
    std::vector<WasmCommandSpec> commands;
    std::vector<WasmPermissionSpec> permissions;
    std::vector<std::string> subscriptions;
};

[[nodiscard]] aegilex::runtime::RuntimeConfig default_runtime_config() noexcept;
[[nodiscard]] aegilex::status create_runtime(const std::shared_ptr<HostContext> &host_context,
                                             const aegilex::runtime::RuntimeConfig &config,
                                             std::unique_ptr<Runtime> *out_runtime) noexcept;
[[nodiscard]] aegilex::status inspect_plugin(Runtime *runtime, std::string_view component_path,
                                             WasmPluginMetadata *out_metadata, std::string *out_error) noexcept;
[[nodiscard]] aegilex::status prepare_plugin(Runtime *runtime, std::string_view component_path) noexcept;
[[nodiscard]] aegilex::status enable_plugin(Runtime *runtime, std::string_view plugin_id) noexcept;
[[nodiscard]] aegilex::status disable_plugin(Runtime *runtime, std::string_view plugin_id) noexcept;
[[nodiscard]] bool should_dispatch_event(Runtime *runtime, std::string_view plugin_id,
                                         std::string_view subscription) noexcept;
[[nodiscard]] aegilex::status dispatch_task(Runtime *runtime, std::string_view plugin_id,
                                            std::uint64_t task_id) noexcept;

} // namespace aegilex::native
