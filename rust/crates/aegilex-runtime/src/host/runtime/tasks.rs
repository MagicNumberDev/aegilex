//! Runtime scheduler ABI implementation.

use crate::host::endstone::support::*;

impl crate::core_host::imports::HostTasks for PluginStoreState {
    fn schedule_now(&mut self) -> Result<Result<u64, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "tasks.schedule-now")?;
            native::task_schedule_now(&self.host, &self.plugin_id, 0).map_err(map_core_host_error)
        })())
    }

    fn schedule_after(&mut self, delay_ticks: u64) -> Result<Result<u64, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "tasks.schedule-after")?;
            native::task_schedule_after(&self.host, &self.plugin_id, 0, delay_ticks)
                .map_err(map_core_host_error)
        })())
    }

    fn schedule_every(
        &mut self,
        initial_delay_ticks: u64,
        period_ticks: u64,
    ) -> Result<Result<u64, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "tasks.schedule-every")?;
            native::task_schedule_every(
                &self.host,
                &self.plugin_id,
                0,
                initial_delay_ticks,
                period_ticks,
            )
            .map_err(map_core_host_error)
        })())
    }

    fn cancel(&mut self, task_id: u64) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "tasks.cancel")?;
            native::task_cancel(&self.host, &self.plugin_id, 0, task_id)
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn get_task(&mut self, task_id: u64) -> Result<Result<TaskTask, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "tasks.get-task")?;
            let info = native::task_get_info(&self.host, &self.plugin_id, 0, task_id)
                .map_err(map_core_host_error)?;
            Ok(TaskTask {
                task_id: info.task_id,
                owner: info.owner,
                is_sync: info.is_sync,
                is_cancelled: info.is_cancelled,
            })
        })())
    }

    fn task_is_running(&mut self, task_id: u64) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "tasks.task-is-running")?;
            native::task_is_running(&self.host, &self.plugin_id, 0, task_id)
                .map_err(map_core_host_error)
        })())
    }

    fn task_is_queued(&mut self, task_id: u64) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "tasks.task-is-queued")?;
            native::task_is_queued(&self.host, &self.plugin_id, 0, task_id)
                .map_err(map_core_host_error)
        })())
    }

    fn task_list_pending(&mut self) -> Result<Result<Vec<TaskTask>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "tasks.task-list-pending")?;
            let summaries = native::task_list_pending(&self.host, &self.plugin_id, 0)
                .map_err(map_core_host_error)?;
            Ok(summaries
                .into_iter()
                .map(|summary| TaskTask {
                    task_id: summary.task_id,
                    owner: summary.owner,
                    is_sync: summary.is_sync,
                    is_cancelled: summary.is_cancelled,
                })
                .collect())
        })())
    }

    fn on_task(&mut self, _task_id: u64) -> Result<Result<(), String>, String> {
        unreachable!("guest callbacks are exported, not imported")
    }
}

