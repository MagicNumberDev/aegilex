#include "runtime_bridge.h"

#include "aegilex_types.h"

#include <memory>
#include <string>
#include <string_view>
#include <vector>

namespace {

[[nodiscard]] std::string to_string(const rust::String &value)
{
    return static_cast<std::string>(value);
}

template <typename Values> [[nodiscard]] std::vector<std::string> to_strings(const Values &values)
{
    std::vector<std::string> out;
    out.reserve(values.size());
    for (const auto &value : values) {
        out.push_back(to_string(value));
    }
    return out;
}

} // namespace

namespace aegilex::native {

Runtime::Runtime(rust::Box<aegilex::runtime::RuntimeHandle> handle, HostContext *host_context) noexcept
    : handle(std::move(handle)), host_context(host_context)
{
}

aegilex::runtime::RuntimeConfig default_runtime_config() noexcept
{
    return aegilex::runtime::default_runtime_config();
}

aegilex::status create_runtime(const std::shared_ptr<HostContext> &host_context,
                               const aegilex::runtime::RuntimeConfig &config,
                               std::unique_ptr<Runtime> *out_runtime) noexcept
{
    try {
        if (out_runtime == nullptr) {
            return aegilex::kInvalidArgument;
        }
        if (host_context == nullptr) {
            return aegilex::kInvalidArgument;
        }
        auto handle = aegilex::runtime::create_runtime(host_context, config);
        *out_runtime = std::make_unique<Runtime>(std::move(handle), host_context.get());
        return aegilex::kOk;
    }
    catch (...) {
        if (out_runtime != nullptr) {
            out_runtime->reset();
        }
        return aegilex::kInternalError;
    }
}

aegilex::status inspect_plugin(Runtime *runtime, const std::string_view component_path,
                               WasmPluginMetadata *out_metadata, std::string *out_error) noexcept
{
    try {
        if (runtime == nullptr || out_metadata == nullptr || out_error == nullptr || component_path.empty()) {
            return aegilex::kInvalidArgument;
        }
        out_error->clear();
        const auto result = aegilex::runtime::inspect_plugin(*runtime->handle, std::string(component_path));
        if (result.status != aegilex::kOk) {
            *out_error = to_string(result.error);
            return result.status;
        }

        WasmPluginMetadata metadata;
        metadata.name = to_string(result.metadata.name);
        metadata.version = to_string(result.metadata.version);
        metadata.description = to_string(result.metadata.description);
        metadata.load_order = result.metadata.load_order;
        metadata.authors = to_strings(result.metadata.authors);
        metadata.contributors = to_strings(result.metadata.contributors);
        metadata.website = to_string(result.metadata.website);
        metadata.prefix = to_string(result.metadata.prefix);
        metadata.provides = to_strings(result.metadata.provides);
        metadata.depend = to_strings(result.metadata.depend);
        metadata.soft_depend = to_strings(result.metadata.soft_depend);
        metadata.load_before = to_strings(result.metadata.load_before);
        metadata.default_permission = result.metadata.default_permission;
        metadata.subscriptions = to_strings(result.metadata.subscriptions);
        metadata.commands.reserve(result.metadata.commands.size());
        for (const auto &command : result.metadata.commands) {
            metadata.commands.push_back(WasmCommandSpec{.name = to_string(command.name),
                                                        .description = to_string(command.description),
                                                        .aliases = to_strings(command.aliases),
                                                        .usages = to_strings(command.usages),
                                                        .permissions = to_strings(command.permissions)});
        }
        metadata.permissions.reserve(result.metadata.permissions.size());
        for (const auto &permission : result.metadata.permissions) {
            std::vector<WasmPermissionChild> children;
            children.reserve(permission.children.size());
            for (const auto &child : permission.children) {
                children.push_back(WasmPermissionChild{.name = to_string(child.name), .value = child.value});
            }
            metadata.permissions.push_back(WasmPermissionSpec{.name = to_string(permission.name),
                                                              .description = to_string(permission.description),
                                                              .has_default_value = permission.has_default_value,
                                                              .default_value = permission.default_value,
                                                              .children = std::move(children)});
        }
        *out_metadata = std::move(metadata);
        return aegilex::kOk;
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

aegilex::status prepare_plugin(Runtime *runtime, const std::string_view component_path) noexcept
{
    try {
        if (runtime == nullptr || component_path.empty()) {
            return aegilex::kInvalidArgument;
        }
        return aegilex::runtime::prepare_plugin(*runtime->handle, std::string(component_path));
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

aegilex::status enable_plugin(Runtime *runtime, const std::string_view plugin_id) noexcept
{
    try {
        if (runtime == nullptr || plugin_id.empty()) {
            return aegilex::kInvalidArgument;
        }
        return aegilex::runtime::enable_plugin(*runtime->handle, std::string(plugin_id));
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

aegilex::status disable_plugin(Runtime *runtime, const std::string_view plugin_id) noexcept
{
    try {
        if (runtime == nullptr || plugin_id.empty()) {
            return aegilex::kInvalidArgument;
        }
        return aegilex::runtime::disable_plugin(*runtime->handle, std::string(plugin_id));
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

bool should_dispatch_event(Runtime *runtime, const std::string_view plugin_id,
                           const std::string_view subscription) noexcept
{
    try {
        if (runtime == nullptr || runtime->host_context == nullptr || plugin_id.empty() || subscription.empty()) {
            return false;
        }
        return aegilex::runtime::should_dispatch_event(*runtime->handle, std::string(plugin_id),
                                                       std::string(subscription));
    }
    catch (...) {
        return false;
    }
}

aegilex::status dispatch_task(Runtime *runtime, const std::string_view plugin_id, const std::uint64_t task_id) noexcept
{
    try {
        if (runtime == nullptr || plugin_id.empty()) {
            return aegilex::kInvalidArgument;
        }
        return aegilex::runtime::dispatch_task(*runtime->handle, std::string(plugin_id), task_id);
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

} // namespace aegilex::native
