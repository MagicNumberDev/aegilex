#pragma once

#include "../aegilex_types.h"

#include "../runtime_bridge.h"

#include <cstdint>
#include <memory>
#include <string>
#include <string_view>
#include <unordered_map>
#include <vector>

namespace endstone {
class Plugin;
class Task;
} // namespace endstone

namespace aegilex::native {

class HostContext;

// Maps Aegilex logical task ids (1-based, 0 invalid) to Endstone scheduler
// tasks registered under the Aegilex plugin. Guest callbacks run on the
// primary thread only.
class TaskBridge {
  public:
    struct TaskSummary {
        std::uint64_t task_id{};
        std::string owner;
        bool is_sync{};
        bool is_cancelled{};
    };

    TaskBridge(HostContext &context, endstone::Plugin &plugin, Runtime *runtime);
    ~TaskBridge() = default;

    TaskBridge(const TaskBridge &) = delete;
    TaskBridge &operator=(const TaskBridge &) = delete;

    [[nodiscard]] aegilex::status schedule_now(std::string_view plugin_id, std::uint64_t *out_task_id) noexcept;
    [[nodiscard]] aegilex::status schedule_after(std::string_view plugin_id, std::uint64_t delay_ticks,
                                                 std::uint64_t *out_task_id) noexcept;
    [[nodiscard]] aegilex::status schedule_every(std::string_view plugin_id, std::uint64_t initial_delay_ticks,
                                                 std::uint64_t period_ticks, std::uint64_t *out_task_id) noexcept;
    [[nodiscard]] aegilex::status cancel(std::string_view plugin_id, std::uint64_t task_id) noexcept;
    [[nodiscard]] aegilex::status get_info(std::string_view plugin_id, std::uint64_t task_id,
                                           TaskSummary *out) noexcept;
    [[nodiscard]] aegilex::status is_running(std::string_view plugin_id, std::uint64_t task_id, bool *out) noexcept;
    [[nodiscard]] aegilex::status is_queued(std::string_view plugin_id, std::uint64_t task_id, bool *out) noexcept;
    [[nodiscard]] aegilex::status list_pending(std::string_view plugin_id, std::vector<TaskSummary> *out) noexcept;
    void cancel_all_for_plugin(std::string_view plugin_id) noexcept;
    void cancel_all() noexcept;

  private:
    struct Record {
        std::uint32_t endstone_id{};
        std::shared_ptr<endstone::Task> task;
        std::string plugin_id;
    };

    [[nodiscard]] aegilex::status schedule(std::string_view plugin_id, std::uint64_t delay_ticks,
                                           std::uint64_t period_ticks, std::uint64_t *out_task_id) noexcept;
    [[nodiscard]] aegilex::status find_record(std::string_view plugin_id, std::uint64_t task_id,
                                              const Record **out_record) const noexcept;
    [[nodiscard]] std::uint64_t next_logical_id() noexcept;
    void dispatch(std::string_view plugin_id, std::uint64_t task_id) noexcept;

    HostContext &context_;
    endstone::Plugin &plugin_;
    Runtime *runtime_;
    std::unordered_map<std::uint64_t, Record> tasks_;
    std::uint64_t next_task_id_{1};
};

} // namespace aegilex::native
