#include "host_context.h"

#include <aegilex-runtime/src/cxx_host.rs.h>
#include <aegilex-runtime/src/cxx_runtime.rs.h>

#include "bindings/endstone/validation.h"
#include "bridge/task_bridge.h"
#include "bridge/form_bridge.h"
#include "bridge/map_renderer_bridge.h"
#include "runtime_bridge.h"

#include <cstring>
#include "wasm_plugin_loader.h"

#include <string>
#include <string_view>
#include <vector>

namespace {

[[nodiscard]] std::uint32_t validate_host_context(aegilex::native::HostContext &context,
                                                  const std::string &plugin_id) noexcept
{
    aegilex::native::HostContext *validated = nullptr;
    return aegilex::native::endstone_binding::validate_context(&context, plugin_id, &validated);
}

[[nodiscard]] aegilex::native::TaskBridge *get_task_bridge(aegilex::native::HostContext &context) noexcept
{
    return context.task_bridge.get();
}

[[nodiscard]] aegilex::native::FormBridge *get_form_bridge(aegilex::native::HostContext &context) noexcept
{
    return context.form_bridge.get();
}

[[nodiscard]] aegilex::native::MapRendererBridge *
get_map_renderer_bridge(aegilex::native::HostContext &context) noexcept
{
    return context.map_renderer_bridge.get();
}

[[nodiscard]] aegilex::host::TaskId task_id_result(const std::uint32_t status, const std::uint64_t task_id) noexcept
{
    return {.status = status, .task_id = task_id};
}

[[nodiscard]] endstone::Logger *calling_logger(const aegilex::native::HostContext &context,
                                               const std::string_view plugin_id) noexcept
{
    if (context.wasm_loader != nullptr) {
        if (const auto *plugin = context.wasm_loader->find_plugin(plugin_id); plugin != nullptr) {
            // Endstone assigns the proxy logger after onLoad has completed.
            if (plugin->isEnabled()) {
                return &plugin->getLogger();
            }
        }
    }
    return context.logger;
}

} // namespace

namespace aegilex::native {

std::unique_ptr<LoggerResult> HostContext::getLogger(const std::string &plugin_id,
                                                     const std::uint64_t invocation_id) const noexcept
{
    static_cast<void>(invocation_id);
    auto &context = const_cast<HostContext &>(*this);
    const auto status = validate_host_context(context, plugin_id);
    if (status != aegilex::kOk) {
        return std::make_unique<LoggerResult>(status, nullptr);
    }
    try {
        auto *logger = calling_logger(context, plugin_id);
        if (logger == nullptr) {
            return std::make_unique<LoggerResult>(aegilex::kNotFound, nullptr);
        }
        return std::make_unique<LoggerResult>(aegilex::kOk, std::make_unique<Logger>(logger));
    }
    catch (...) {
        return std::make_unique<LoggerResult>(aegilex::kHostError, nullptr);
    }
}

aegilex::host::TaskId HostContext::scheduleTaskNow(const std::string &plugin_id,
                                                   const std::uint64_t invocation_id) const noexcept
{
    static_cast<void>(invocation_id);
    auto &context = const_cast<HostContext &>(*this);
    const auto status = validate_host_context(context, plugin_id);
    auto *bridge = get_task_bridge(context);
    if (status != aegilex::kOk || bridge == nullptr) {
        return task_id_result(status == aegilex::kOk ? aegilex::kInternalError : status, 0);
    }
    std::uint64_t task_id = 0;
    const auto task_status = bridge->schedule_now(plugin_id, &task_id);
    return task_id_result(task_status, task_id);
}

aegilex::host::TaskId HostContext::scheduleTaskAfter(const std::string &plugin_id, const std::uint64_t invocation_id,
                                                     const std::uint64_t delay_ticks) const noexcept
{
    static_cast<void>(invocation_id);
    auto &context = const_cast<HostContext &>(*this);
    const auto status = validate_host_context(context, plugin_id);
    auto *bridge = get_task_bridge(context);
    if (status != aegilex::kOk || bridge == nullptr) {
        return task_id_result(status == aegilex::kOk ? aegilex::kInternalError : status, 0);
    }
    std::uint64_t task_id = 0;
    const auto task_status = bridge->schedule_after(plugin_id, delay_ticks, &task_id);
    return task_id_result(task_status, task_id);
}

aegilex::host::TaskId HostContext::scheduleTaskEvery(const std::string &plugin_id, const std::uint64_t invocation_id,
                                                     const std::uint64_t initial_delay_ticks,
                                                     const std::uint64_t period_ticks) const noexcept
{
    static_cast<void>(invocation_id);
    auto &context = const_cast<HostContext &>(*this);
    const auto status = validate_host_context(context, plugin_id);
    auto *bridge = get_task_bridge(context);
    if (status != aegilex::kOk || bridge == nullptr) {
        return task_id_result(status == aegilex::kOk ? aegilex::kInternalError : status, 0);
    }
    std::uint64_t task_id = 0;
    const auto task_status = bridge->schedule_every(plugin_id, initial_delay_ticks, period_ticks, &task_id);
    return task_id_result(task_status, task_id);
}

std::uint32_t HostContext::cancelTask(const std::string &plugin_id, const std::uint64_t invocation_id,
                                      const std::uint64_t task_id) const noexcept
{
    static_cast<void>(invocation_id);
    auto &context = const_cast<HostContext &>(*this);
    const auto status = validate_host_context(context, plugin_id);
    auto *bridge = get_task_bridge(context);
    return status != aegilex::kOk ? status
           : bridge == nullptr    ? aegilex::kInternalError
                                  : bridge->cancel(plugin_id, task_id);
}

aegilex::host::TaskSummary HostContext::getTaskInfo(const std::string &plugin_id, const std::uint64_t invocation_id,
                                                    const std::uint64_t task_id) const noexcept
{
    static_cast<void>(invocation_id);
    auto &context = const_cast<HostContext &>(*this);
    const auto status = validate_host_context(context, plugin_id);
    auto *bridge = get_task_bridge(context);
    if (status != aegilex::kOk || bridge == nullptr) {
        return {.status = status == aegilex::kOk ? aegilex::kInternalError : status};
    }
    TaskBridge::TaskSummary result{};
    const auto result_status = bridge->get_info(plugin_id, task_id, &result);
    return {.status = result_status,
            .task_id = result.task_id,
            .owner = rust::String(result.owner),
            .is_sync = result.is_sync,
            .is_cancelled = result.is_cancelled};
}

aegilex::host::TaskState HostContext::isTaskRunning(const std::string &plugin_id, const std::uint64_t invocation_id,
                                                    const std::uint64_t task_id) const noexcept
{
    static_cast<void>(invocation_id);
    auto &context = const_cast<HostContext &>(*this);
    const auto status = validate_host_context(context, plugin_id);
    auto *bridge = get_task_bridge(context);
    if (status != aegilex::kOk || bridge == nullptr) {
        return {.status = status == aegilex::kOk ? aegilex::kInternalError : status};
    }
    bool value = false;
    return {.status = bridge->is_running(plugin_id, task_id, &value), .value = value};
}

aegilex::host::TaskState HostContext::isTaskQueued(const std::string &plugin_id, const std::uint64_t invocation_id,
                                                   const std::uint64_t task_id) const noexcept
{
    static_cast<void>(invocation_id);
    auto &context = const_cast<HostContext &>(*this);
    const auto status = validate_host_context(context, plugin_id);
    auto *bridge = get_task_bridge(context);
    if (status != aegilex::kOk || bridge == nullptr) {
        return {.status = status == aegilex::kOk ? aegilex::kInternalError : status};
    }
    bool value = false;
    return {.status = bridge->is_queued(plugin_id, task_id, &value), .value = value};
}

aegilex::host::TaskList HostContext::listPendingTasks(const std::string &plugin_id,
                                                      const std::uint64_t invocation_id) const noexcept
{
    static_cast<void>(invocation_id);
    auto &context = const_cast<HostContext &>(*this);
    const auto status = validate_host_context(context, plugin_id);
    auto *bridge = get_task_bridge(context);
    if (status != aegilex::kOk || bridge == nullptr) {
        return {.status = status == aegilex::kOk ? aegilex::kInternalError : status};
    }
    std::vector<TaskBridge::TaskSummary> pending;
    aegilex::host::TaskList result{.status = bridge->list_pending(plugin_id, &pending)};
    result.tasks.reserve(pending.size());
    for (auto &task : pending) {
        result.tasks.push_back({.status = aegilex::kOk,
                                .task_id = task.task_id,
                                .owner = rust::String(task.owner),
                                .is_sync = task.is_sync,
                                .is_cancelled = task.is_cancelled});
    }
    return result;
}

std::uint32_t HostContext::form_show(const std::string &plugin_id, const rust::Slice<const std::uint8_t> uuid,
                                     const aegilex::runtime::FormSpecData &spec,
                                     std::uint64_t &out_form_id) const noexcept
{
    auto &context = const_cast<HostContext &>(*this);
    const auto status = validate_host_context(context, plugin_id);
    auto *bridge = get_form_bridge(context);
    if (status != aegilex::kOk || bridge == nullptr) {
        return status == aegilex::kOk ? aegilex::kInternalError : status;
    }
    if (uuid.size() != 16) {
        return aegilex::kInvalidArgument;
    }
    std::array<std::uint8_t, 16> player_uuid{};
    std::memcpy(player_uuid.data(), uuid.data(), uuid.size());
    return bridge->show(plugin_id, player_uuid, spec, &out_form_id);
}

std::uint32_t HostContext::form_close(const rust::Slice<const std::uint8_t> uuid) const noexcept
{
    auto &context = const_cast<HostContext &>(*this);
    auto *bridge = get_form_bridge(context);
    if (bridge == nullptr) {
        return aegilex::kInternalError;
    }
    if (uuid.size() != 16) {
        return aegilex::kInvalidArgument;
    }
    std::array<std::uint8_t, 16> player_uuid{};
    std::memcpy(player_uuid.data(), uuid.data(), uuid.size());
    return bridge->close_form(player_uuid);
}

aegilex::runtime::PluginList HostContext::list_plugins() const noexcept
{
    aegilex::runtime::PluginList result{.status = aegilex::kInternalError, .plugins = {}};
    try {
        auto *runtime = wasm_loader == nullptr ? nullptr : wasm_loader->runtime();
        if (runtime == nullptr) {
            return result;
        }
        return aegilex::runtime::plugin_manager_list(*runtime->handle);
    }
    catch (...) {
        return result;
    }
}

std::uint32_t HostContext::get_plugin(const std::string &plugin_id,
                                      aegilex::runtime::PluginInfoData &out) const noexcept
{
    try {
        auto *runtime = wasm_loader == nullptr ? nullptr : wasm_loader->runtime();
        if (runtime == nullptr) {
            return aegilex::kInternalError;
        }
        out = aegilex::runtime::plugin_manager_get(*runtime->handle, plugin_id);
        return out.metadata.name.empty() ? aegilex::kNotFound : aegilex::kOk;
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

std::uint32_t HostContext::enable_plugin(const std::string &plugin_id) const noexcept
{
    try {
        auto *runtime = wasm_loader == nullptr ? nullptr : wasm_loader->runtime();
        if (runtime == nullptr) {
            return aegilex::kInternalError;
        }
        return aegilex::runtime::plugin_manager_enable(*runtime->handle, plugin_id);
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

std::uint32_t HostContext::disable_plugin(const std::string &plugin_id) const noexcept
{
    try {
        auto *runtime = wasm_loader == nullptr ? nullptr : wasm_loader->runtime();
        if (runtime == nullptr) {
            return aegilex::kInternalError;
        }
        return aegilex::runtime::plugin_manager_disable(*runtime->handle, plugin_id);
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

std::uint64_t HostContext::service_publish(const std::string &plugin_id, const rust::Str name, const rust::Str version,
                                           const rust::Vec<rust::String> &methods,
                                           const std::uint32_t priority) const noexcept
{
    try {
        auto *runtime = wasm_loader == nullptr ? nullptr : wasm_loader->runtime();
        if (runtime == nullptr) {
            return 0;
        }
        return aegilex::runtime::service_publish(*runtime->handle, plugin_id, name, version, methods, priority);
    }
    catch (...) {
        return 0;
    }
}

std::uint32_t HostContext::service_unpublish(const std::string &plugin_id,
                                             const std::uint64_t provider_id) const noexcept
{
    try {
        auto *runtime = wasm_loader == nullptr ? nullptr : wasm_loader->runtime();
        if (runtime == nullptr) {
            return aegilex::kInternalError;
        }
        return aegilex::runtime::service_unpublish(*runtime->handle, plugin_id, provider_id);
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

aegilex::runtime::ServiceListData HostContext::service_list(const rust::Str name) const noexcept
{
    aegilex::runtime::ServiceListData result{.status = aegilex::kInternalError, .providers = {}};
    try {
        auto *runtime = wasm_loader == nullptr ? nullptr : wasm_loader->runtime();
        if (runtime == nullptr) {
            return result;
        }
        return aegilex::runtime::service_list(*runtime->handle, name);
    }
    catch (...) {
        return result;
    }
}

std::uint64_t HostContext::service_call(const std::string &plugin_id, const std::uint64_t provider_id,
                                        const rust::Str method, const rust::Slice<const std::uint8_t> payload,
                                        const std::uint64_t deadline) const noexcept
{
    try {
        auto *runtime = wasm_loader == nullptr ? nullptr : wasm_loader->runtime();
        if (runtime == nullptr) {
            return 0;
        }
        return aegilex::runtime::service_call(*runtime->handle, plugin_id, provider_id, method, payload, deadline);
    }
    catch (...) {
        return 0;
    }
}

aegilex::runtime::ServiceCallStatusData HostContext::service_call_status(const std::uint64_t call_id) const noexcept
{
    aegilex::runtime::ServiceCallStatusData result{.status = aegilex::kInternalError, .call_status = 0};
    try {
        auto *runtime = wasm_loader == nullptr ? nullptr : wasm_loader->runtime();
        if (runtime == nullptr) {
            return result;
        }
        return aegilex::runtime::service_call_status(*runtime->handle, call_id);
    }
    catch (...) {
        return result;
    }
}

aegilex::runtime::ServiceResponseData HostContext::service_take_response(const std::uint64_t call_id) const noexcept
{
    aegilex::runtime::ServiceResponseData result{
        .status = aegilex::kInternalError, .kind = 0, .payload = {}, .error = {}};
    try {
        auto *runtime = wasm_loader == nullptr ? nullptr : wasm_loader->runtime();
        if (runtime == nullptr) {
            return result;
        }
        return aegilex::runtime::service_take_response(*runtime->handle, call_id);
    }
    catch (...) {
        return result;
    }
}

std::uint32_t HostContext::map_renderer_register(const std::string &plugin_id, const std::int64_t map_id,
                                                 const bool contextual, std::uint64_t &out_renderer_id) const noexcept
{
    auto &context = const_cast<HostContext &>(*this);
    const auto status = validate_host_context(context, plugin_id);
    auto *bridge = get_map_renderer_bridge(context);
    if (status != aegilex::kOk) {
        if (context.logger != nullptr) {
            context.logger->error("Aegilex rejected map renderer registration for '{}': host status {}", plugin_id,
                                  status);
        }
        return status;
    }
    if (bridge == nullptr) {
        if (context.logger != nullptr) {
            context.logger->error("Aegilex cannot register a map renderer for '{}': bridge is unavailable", plugin_id);
        }
        return aegilex::kInternalError;
    }
    const auto bridge_status = bridge->register_renderer(plugin_id, map_id, contextual, &out_renderer_id);
    if (bridge_status != aegilex::kOk || out_renderer_id == 0) {
        if (context.logger != nullptr) {
            context.logger->error("Aegilex map renderer registration for '{}' returned status {} and id {}", plugin_id,
                                  bridge_status, out_renderer_id);
        }
        return bridge_status == aegilex::kOk ? aegilex::kInternalError : bridge_status;
    }
    return aegilex::kOk;
}

std::uint32_t HostContext::map_renderer_unregister(const std::string &plugin_id,
                                                   const std::uint64_t renderer_id) const noexcept
{
    auto &context = const_cast<HostContext &>(*this);
    const auto status = validate_host_context(context, plugin_id);
    auto *bridge = get_map_renderer_bridge(context);
    if (status != aegilex::kOk || bridge == nullptr) {
        return status == aegilex::kOk ? aegilex::kInternalError : status;
    }
    return bridge->unregister_renderer(plugin_id, renderer_id);
}

std::uint32_t HostContext::service_cancel(const std::uint64_t call_id) const noexcept
{
    try {
        auto *runtime = wasm_loader == nullptr ? nullptr : wasm_loader->runtime();
        if (runtime == nullptr) {
            return aegilex::kInternalError;
        }
        return aegilex::runtime::service_cancel(*runtime->handle, call_id);
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

} // namespace aegilex::native
