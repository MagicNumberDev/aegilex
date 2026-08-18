#include "task_bridge.h"
#include "../aegilex_types.h"

#include "../host_context.h"

#include <endstone/plugin/plugin.h>
#include <endstone/scheduler/scheduler.h>
#include <endstone/scheduler/task.h>
#include <endstone/server.h>

#include <algorithm>
#include <cstdint>
#include <string>
#include <string_view>
#include <vector>

namespace aegilex::native {

TaskBridge::TaskBridge(HostContext &context, endstone::Plugin &plugin, Runtime *runtime)
    : context_(context), plugin_(plugin), runtime_(runtime)
{
}

aegilex::status TaskBridge::schedule_now(const std::string_view plugin_id, std::uint64_t *out_task_id) noexcept
{
    try {
        if (plugin_id.empty() || out_task_id == nullptr || context_.server.native() == nullptr) {
            return aegilex::kInvalidArgument;
        }
        if (!context_.server.native()->isPrimaryThread()) {
            return aegilex::kWrongThread;
        }
        const auto logical_id = next_logical_id();
        if (logical_id == 0) {
            return aegilex::kLimitExceeded;
        }

        HostContext *context = &context_;
        const std::string owner(plugin_id);
        const auto task = context_.server.native()->getScheduler().runTask(plugin_, [context, owner, logical_id] {
            if (context->accepting_calls && context->task_bridge != nullptr) {
                context->task_bridge->dispatch(owner, logical_id);
            }
        });
        if (!task) {
            return aegilex::kHostError;
        }
        tasks_.emplace(logical_id, Record{task->getTaskId(), task, std::move(owner)});
        *out_task_id = logical_id;
        return aegilex::kOk;
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

aegilex::status TaskBridge::schedule_after(const std::string_view plugin_id, const std::uint64_t delay_ticks,
                                           std::uint64_t *out_task_id) noexcept
{
    return schedule(plugin_id, delay_ticks, 0, out_task_id);
}

aegilex::status TaskBridge::schedule_every(const std::string_view plugin_id, const std::uint64_t initial_delay_ticks,
                                           const std::uint64_t period_ticks, std::uint64_t *out_task_id) noexcept
{
    return schedule(plugin_id, initial_delay_ticks, period_ticks, out_task_id);
}

aegilex::status TaskBridge::schedule(const std::string_view plugin_id, const std::uint64_t delay_ticks,
                                     const std::uint64_t period_ticks, std::uint64_t *out_task_id) noexcept
{
    try {
        if (plugin_id.empty() || out_task_id == nullptr || context_.server.native() == nullptr) {
            return aegilex::kInvalidArgument;
        }
        if (!context_.server.native()->isPrimaryThread()) {
            return aegilex::kWrongThread;
        }
        const auto logical_id = next_logical_id();
        if (logical_id == 0) {
            return aegilex::kLimitExceeded;
        }

        HostContext *context = &context_;
        const std::string owner(plugin_id);
        const auto task = context_.server.native()->getScheduler().runTaskTimer(
            plugin_,
            [context, owner, logical_id] {
                if (context->accepting_calls && context->task_bridge != nullptr) {
                    context->task_bridge->dispatch(owner, logical_id);
                }
            },
            delay_ticks, period_ticks);
        if (!task) {
            return aegilex::kHostError;
        }
        tasks_.emplace(logical_id, Record{task->getTaskId(), task, std::move(owner)});
        *out_task_id = logical_id;
        return aegilex::kOk;
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

aegilex::status TaskBridge::cancel(const std::string_view plugin_id, const std::uint64_t task_id) noexcept
{
    try {
        if (plugin_id.empty() || task_id == 0) {
            return aegilex::kInvalidArgument;
        }
        const auto it = tasks_.find(task_id);
        if (it == tasks_.end()) {
            return aegilex::kNotFound;
        }
        if (it->second.plugin_id != plugin_id) {
            return aegilex::kDenied;
        }
        if (context_.server.native() != nullptr) {
            context_.server.native()->getScheduler().cancelTask(it->second.endstone_id);
        }
        return aegilex::kOk;
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

aegilex::status TaskBridge::get_info(const std::string_view plugin_id, const std::uint64_t task_id,
                                     TaskSummary *out) noexcept
{
    try {
        if (plugin_id.empty() || task_id == 0 || out == nullptr) {
            return aegilex::kInvalidArgument;
        }
        const Record *record = nullptr;
        if (const auto status = find_record(plugin_id, task_id, &record); status != aegilex::kOk) {
            return status;
        }

        *out = {.task_id = task_id,
                .owner = record->plugin_id,
                .is_sync = record->task->isSync(),
                .is_cancelled = record->task->isCancelled()};
        return aegilex::kOk;
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

aegilex::status TaskBridge::is_running(const std::string_view plugin_id, const std::uint64_t task_id,
                                       bool *out) noexcept
{
    try {
        if (plugin_id.empty() || task_id == 0 || out == nullptr) {
            return aegilex::kInvalidArgument;
        }
        if (context_.server.native() == nullptr) {
            return aegilex::kInternalError;
        }
        const Record *record = nullptr;
        if (const auto status = find_record(plugin_id, task_id, &record); status != aegilex::kOk) {
            return status;
        }

        *out = context_.server.native()->getScheduler().isRunning(record->endstone_id);
        return aegilex::kOk;
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

aegilex::status TaskBridge::is_queued(const std::string_view plugin_id, const std::uint64_t task_id, bool *out) noexcept
{
    try {
        if (plugin_id.empty() || task_id == 0 || out == nullptr) {
            return aegilex::kInvalidArgument;
        }
        if (context_.server.native() == nullptr) {
            return aegilex::kInternalError;
        }
        const Record *record = nullptr;
        if (const auto status = find_record(plugin_id, task_id, &record); status != aegilex::kOk) {
            return status;
        }

        *out = context_.server.native()->getScheduler().isQueued(record->endstone_id);
        return aegilex::kOk;
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

aegilex::status TaskBridge::list_pending(const std::string_view plugin_id, std::vector<TaskSummary> *out) noexcept
{
    try {
        if (plugin_id.empty() || out == nullptr) {
            return aegilex::kInvalidArgument;
        }
        if (context_.server.native() == nullptr) {
            return aegilex::kInternalError;
        }

        const auto pending = context_.server.native()->getScheduler().getPendingTasks();
        std::vector<TaskSummary> rows;
        for (const auto *task : pending) {
            if (task->getOwner() != &plugin_) {
                continue;
            }
            const std::uint32_t endstone_id = task->getTaskId();
            const Record *record = nullptr;
            std::uint64_t logical_id = 0;
            for (const auto &[candidate_id, candidate] : tasks_) {
                if (candidate.endstone_id == endstone_id) {
                    record = &candidate;
                    logical_id = candidate_id;
                    break;
                }
            }
            if (record == nullptr || record->plugin_id != plugin_id) {
                continue;
            }
            rows.push_back({.task_id = logical_id,
                            .owner = record->plugin_id,
                            .is_sync = task->isSync(),
                            .is_cancelled = task->isCancelled()});
        }
        *out = std::move(rows);
        return aegilex::kOk;
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

aegilex::status TaskBridge::find_record(const std::string_view plugin_id, const std::uint64_t task_id,
                                        const Record **out_record) const noexcept
{
    const auto it = tasks_.find(task_id);
    if (it == tasks_.end()) {
        return aegilex::kNotFound;
    }
    if (it->second.plugin_id != plugin_id) {
        return aegilex::kDenied;
    }
    *out_record = &it->second;
    return aegilex::kOk;
}

void TaskBridge::cancel_all_for_plugin(const std::string_view plugin_id) noexcept
{
    try {
        for (auto it = tasks_.begin(); it != tasks_.end();) {
            if (it->second.plugin_id != plugin_id) {
                ++it;
                continue;
            }
            if (context_.server.native() != nullptr) {
                context_.server.native()->getScheduler().cancelTask(it->second.endstone_id);
            }
            it = tasks_.erase(it);
        }
    }
    catch (...) {
    }
}

void TaskBridge::cancel_all() noexcept
{
    try {
        for (const auto &entry : tasks_) {
            if (context_.server.native() != nullptr) {
                context_.server.native()->getScheduler().cancelTask(entry.second.endstone_id);
            }
        }
        tasks_.clear();
    }
    catch (...) {
    }
}

std::uint64_t TaskBridge::next_logical_id() noexcept
{
    const auto id = next_task_id_++;
    if (id == 0) {
        next_task_id_ = 1;
    }
    return id;
}

void TaskBridge::dispatch(const std::string_view plugin_id, const std::uint64_t task_id) noexcept
{
    try {
        const auto it = tasks_.find(task_id);
        if (it == tasks_.end() || it->second.plugin_id != plugin_id || runtime_ == nullptr ||
            context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        const auto status = aegilex::native::dispatch_task(runtime_, plugin_id, task_id);
        if (status != aegilex::kOk && context_.logger != nullptr) {
            context_.logger->error("Aegilex task dispatch to '{}' failed (status {}).", plugin_id, status);
        }
    }
    catch (...) {
    }
}

} // namespace aegilex::native
