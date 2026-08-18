#pragma once

#include "aegilex_types.h"
#include "bindings/endstone/logger.h"
#include "bindings/endstone/server.h"

#include <array>
#include <cstring>
#include <cstdint>
#include <memory>
#include <string>
#include <string_view>
#include <vector>

namespace endstone {
class Logger;
}

namespace aegilex::host {
struct TaskId;
struct TaskSummary;
struct TaskState;
struct TaskList;
} // namespace aegilex::host

namespace aegilex::runtime {
struct FormSpecData;
struct PluginList;
struct PluginInfoData;
struct ServiceCallStatusData;
struct ServiceListData;
struct ServiceResponseData;
} // namespace aegilex::runtime

namespace aegilex::native {

class EventBridge;
class CommandBridge;
class TaskBridge;
class FormBridge;
class MapRendererBridge;
class WasmPluginLoader;

struct HostContext {
    HostContext(endstone::Logger *logger, endstone::Server *native_server, WasmPluginLoader *wasm_loader) noexcept;
    ~HostContext() noexcept;

    [[nodiscard]] static std::shared_ptr<HostContext> testStub();

    endstone::Logger *logger{};
    server::Server server;
    bool accepting_calls{};
    std::unique_ptr<EventBridge> event_bridge;
    std::unique_ptr<CommandBridge> command_bridge;
    std::unique_ptr<TaskBridge> task_bridge;
    std::unique_ptr<FormBridge> form_bridge;
    std::unique_ptr<MapRendererBridge> map_renderer_bridge;
    WasmPluginLoader *wasm_loader{};

    [[nodiscard]] std::uint64_t next_invocation_id() noexcept
    {
        return next_invocation_id_++;
    }
    [[nodiscard]] std::uint64_t nextInvocationId() const noexcept
    {
        return const_cast<HostContext *>(this)->next_invocation_id();
    }

    [[nodiscard]] const std::vector<std::string> &enabled_plugin_ids() const noexcept
    {
        return enabled_plugin_ids_;
    }

    void record_enabled_plugin_id(const std::string &plugin_id) noexcept
    {
        if (std::find(enabled_plugin_ids_.begin(), enabled_plugin_ids_.end(), plugin_id) == enabled_plugin_ids_.end()) {
            enabled_plugin_ids_.push_back(plugin_id);
        }
    }

    void remove_enabled_plugin_id(const std::string &plugin_id) noexcept;

    void close_bridges() noexcept;

    [[nodiscard]] const server::Server &getServer() const noexcept
    {
        return server;
    }
    [[nodiscard]] std::unique_ptr<LoggerResult> getLogger(const std::string &plugin_id,
                                                          std::uint64_t invocation_id) const noexcept;
    [[nodiscard]] aegilex::host::TaskId scheduleTaskNow(const std::string &plugin_id,
                                                        std::uint64_t invocation_id) const noexcept;
    [[nodiscard]] aegilex::host::TaskId scheduleTaskAfter(const std::string &plugin_id, std::uint64_t invocation_id,
                                                          std::uint64_t delay_ticks) const noexcept;
    [[nodiscard]] aegilex::host::TaskId scheduleTaskEvery(const std::string &plugin_id, std::uint64_t invocation_id,
                                                          std::uint64_t initial_delay_ticks,
                                                          std::uint64_t period_ticks) const noexcept;
    [[nodiscard]] std::uint32_t cancelTask(const std::string &plugin_id, std::uint64_t invocation_id,
                                           std::uint64_t task_id) const noexcept;
    [[nodiscard]] aegilex::host::TaskSummary getTaskInfo(const std::string &plugin_id, std::uint64_t invocation_id,
                                                         std::uint64_t task_id) const noexcept;
    [[nodiscard]] aegilex::host::TaskState isTaskRunning(const std::string &plugin_id, std::uint64_t invocation_id,
                                                         std::uint64_t task_id) const noexcept;
    [[nodiscard]] aegilex::host::TaskState isTaskQueued(const std::string &plugin_id, std::uint64_t invocation_id,
                                                        std::uint64_t task_id) const noexcept;
    [[nodiscard]] aegilex::host::TaskList listPendingTasks(const std::string &plugin_id,
                                                           std::uint64_t invocation_id) const noexcept;

    [[nodiscard]] std::uint32_t form_show(const std::string &plugin_id, rust::Slice<const std::uint8_t> uuid,
                                          const aegilex::runtime::FormSpecData &spec,
                                          std::uint64_t &out_form_id) const noexcept;
    [[nodiscard]] std::uint32_t form_close(rust::Slice<const std::uint8_t> uuid) const noexcept;

    [[nodiscard]] aegilex::runtime::PluginList list_plugins() const noexcept;
    [[nodiscard]] std::uint32_t get_plugin(const std::string &plugin_id,
                                           aegilex::runtime::PluginInfoData &out) const noexcept;
    [[nodiscard]] std::uint32_t enable_plugin(const std::string &plugin_id) const noexcept;
    [[nodiscard]] std::uint32_t disable_plugin(const std::string &plugin_id) const noexcept;

    [[nodiscard]] std::uint64_t service_publish(const std::string &plugin_id, rust::Str name, rust::Str version,
                                                const rust::Vec<rust::String> &methods,
                                                std::uint32_t priority) const noexcept;
    [[nodiscard]] std::uint32_t service_unpublish(const std::string &plugin_id,
                                                  std::uint64_t provider_id) const noexcept;
    [[nodiscard]] aegilex::runtime::ServiceListData service_list(rust::Str name) const noexcept;
    [[nodiscard]] std::uint64_t service_call(const std::string &plugin_id, std::uint64_t provider_id, rust::Str method,
                                             rust::Slice<const std::uint8_t> payload,
                                             std::uint64_t deadline) const noexcept;
    [[nodiscard]] aegilex::runtime::ServiceCallStatusData service_call_status(std::uint64_t call_id) const noexcept;
    [[nodiscard]] aegilex::runtime::ServiceResponseData service_take_response(std::uint64_t call_id) const noexcept;
    [[nodiscard]] std::uint32_t service_cancel(std::uint64_t call_id) const noexcept;

    [[nodiscard]] std::uint32_t map_renderer_register(const std::string &plugin_id, std::int64_t map_id,
                                                      bool contextual, std::uint64_t &out_renderer_id) const noexcept;
    [[nodiscard]] std::uint32_t map_renderer_unregister(const std::string &plugin_id,
                                                        std::uint64_t renderer_id) const noexcept;

    std::uint64_t next_invocation_id_{1};
    std::vector<std::string> enabled_plugin_ids_;
    // One primary-thread counter shared by every guest instance and native bridge.
};

} // namespace aegilex::native
