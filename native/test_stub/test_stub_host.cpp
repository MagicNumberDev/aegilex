// Test-only typed HostContext bridge stubs. Never linked into the plugin.

namespace aegilex::native {
class EventBridge {};
class CommandBridge {};
class TaskBridge {};
} // namespace aegilex::native

#include "host_context.h"
#include "bridge/form_bridge.h"
#include "bridge/map_renderer_bridge.h"

#include <aegilex-runtime/src/cxx_host.rs.h>
#include <aegilex-runtime/src/cxx_runtime.rs.h>

#include <memory>
#include <string>
#include <utility>

namespace aegilex::native {

HostContext::HostContext(endstone::Logger *logger, endstone::Server *native_server,
                         WasmPluginLoader *wasm_loader) noexcept
    : logger(logger), server(native_server, wasm_loader), wasm_loader(wasm_loader)
{
}

HostContext::~HostContext() noexcept = default;

void HostContext::remove_enabled_plugin_id(const std::string &) noexcept
{
}

std::shared_ptr<HostContext> HostContext::testStub()
{
    return std::make_shared<HostContext>(nullptr, nullptr, nullptr);
}

std::unique_ptr<LoggerResult> HostContext::getLogger(const std::string &, std::uint64_t) const noexcept
{
    return std::make_unique<LoggerResult>(0, std::make_unique<Logger>(nullptr));
}

aegilex::host::TaskId HostContext::scheduleTaskNow(const std::string &, std::uint64_t) const noexcept
{
    return {.status = 0, .task_id = 7};
}

aegilex::host::TaskId HostContext::scheduleTaskAfter(const std::string &, std::uint64_t, std::uint64_t) const noexcept
{
    return {.status = 0, .task_id = 7};
}

aegilex::host::TaskId HostContext::scheduleTaskEvery(const std::string &, std::uint64_t, std::uint64_t,
                                                     std::uint64_t) const noexcept
{
    return {.status = 0, .task_id = 7};
}

std::uint32_t HostContext::cancelTask(const std::string &, std::uint64_t, std::uint64_t) const noexcept
{
    return 0;
}

aegilex::host::TaskSummary HostContext::getTaskInfo(const std::string &plugin_id, std::uint64_t,
                                                    std::uint64_t task_id) const noexcept
{
    return {.status = 0, .task_id = task_id, .owner = rust::String(plugin_id), .is_sync = true, .is_cancelled = true};
}

aegilex::host::TaskState HostContext::isTaskRunning(const std::string &, std::uint64_t, std::uint64_t) const noexcept
{
    return {.status = 0, .value = true};
}

aegilex::host::TaskState HostContext::isTaskQueued(const std::string &, std::uint64_t, std::uint64_t) const noexcept
{
    return {.status = 0, .value = false};
}

aegilex::host::TaskList HostContext::listPendingTasks(const std::string &plugin_id, std::uint64_t) const noexcept
{
    aegilex::host::TaskList result{.status = 0};
    result.tasks.push_back({.task_id = 7, .owner = rust::String(plugin_id), .is_sync = true, .is_cancelled = false});
    return result;
}

std::uint32_t HostContext::form_show(const std::string &, const rust::Slice<const std::uint8_t>,
                                     const aegilex::runtime::FormSpecData &, std::uint64_t &out_form_id) const noexcept
{
    out_form_id = 7;
    return 0;
}

std::uint32_t HostContext::form_close(const rust::Slice<const std::uint8_t>) const noexcept
{
    return 0;
}

aegilex::runtime::PluginList HostContext::list_plugins() const noexcept
{
    aegilex::runtime::PluginList result{.status = 0, .plugins = {}};
    aegilex::runtime::PluginInfoData info{};
    info.metadata = {.name = rust::String("hello"),
                     .version = rust::String("1.0.0"),
                     .description = rust::String("A probe plugin"),
                     .load_order = 1,
                     .authors = {rust::String("Aegilex")},
                     .contributors = {},
                     .website = rust::String(),
                     .prefix = rust::String("Hello"),
                     .provides = {},
                     .depend = {},
                     .soft_depend = {},
                     .load_before = {},
                     .default_permission = 2,
                     .commands = {},
                     .permissions = {},
                     .subscriptions = {}};
    info.enabled = true;
    result.plugins.push_back(std::move(info));
    return result;
}

std::uint32_t HostContext::get_plugin(const std::string &, aegilex::runtime::PluginInfoData &out) const noexcept
{
    out = {};
    out.metadata = {.name = rust::String("hello"),
                    .version = rust::String("1.0.0"),
                    .description = rust::String("A probe plugin"),
                    .load_order = 1,
                    .authors = {rust::String("Aegilex")},
                    .contributors = {},
                    .website = rust::String(),
                    .prefix = rust::String("Hello"),
                    .provides = {},
                    .depend = {},
                    .soft_depend = {},
                    .load_before = {},
                    .default_permission = 2,
                    .commands = {},
                    .permissions = {},
                    .subscriptions = {}};
    out.enabled = true;
    return 0;
}

std::uint32_t HostContext::enable_plugin(const std::string &) const noexcept
{
    return 0;
}

std::uint32_t HostContext::disable_plugin(const std::string &) const noexcept
{
    return 0;
}

std::uint64_t HostContext::service_publish(const std::string &, const rust::Str, const rust::Str,
                                           const rust::Vec<rust::String> &, const std::uint32_t) const noexcept
{
    return 7;
}

std::uint32_t HostContext::service_unpublish(const std::string &, const std::uint64_t) const noexcept
{
    return 0;
}

aegilex::runtime::ServiceListData HostContext::service_list(const rust::Str) const noexcept
{
    aegilex::runtime::ServiceListData result{.status = 0, .providers = {}};
    result.providers.push_back({.id = 7,
                                .name = rust::String("echo"),
                                .version = rust::String("1.0.0"),
                                .methods = {rust::String("echo")},
                                .priority = 2});
    return result;
}

std::uint64_t HostContext::service_call(const std::string &, const std::uint64_t, const rust::Str,
                                        const rust::Slice<const std::uint8_t>, const std::uint64_t) const noexcept
{
    return 7;
}

aegilex::runtime::ServiceCallStatusData HostContext::service_call_status(const std::uint64_t) const noexcept
{
    return {.status = 0, .call_status = 1};
}

aegilex::runtime::ServiceResponseData HostContext::service_take_response(const std::uint64_t) const noexcept
{
    return {.status = 0, .kind = 1, .payload = {}, .error = {}};
}

std::uint32_t HostContext::service_cancel(const std::uint64_t) const noexcept
{
    return 0;
}

std::uint32_t HostContext::map_renderer_register(const std::string &, const std::int64_t, const bool,
                                                 std::uint64_t &out_renderer_id) const noexcept
{
    out_renderer_id = 7;
    return 0;
}

std::uint32_t HostContext::map_renderer_unregister(const std::string &, const std::uint64_t) const noexcept
{
    return 0;
}

} // namespace aegilex::native
