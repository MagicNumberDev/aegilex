use crate::abi::{
    AEGILEX_INTERNAL_ERROR, AEGILEX_INVALID_ARGUMENT, AEGILEX_LOG_CRITICAL, AEGILEX_LOG_DEBUG,
    AEGILEX_LOG_ERROR, AEGILEX_LOG_INFO, AEGILEX_LOG_OFF, AEGILEX_LOG_TRACE, AEGILEX_LOG_WARNING,
    AEGILEX_OK,
};
use crate::bindings::endstone::logger::LogLevel;
use crate::bindings::endstone::task::TaskSummary;
use crate::cxx_host::ffi as cxx_host;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct HostError(u32);

impl HostError {
    pub(crate) const fn status(self) -> u32 {
        self.0
    }

    pub(crate) const fn from_status(status: u32) -> Self {
        Self(status)
    }
}

#[derive(Clone)]
pub(crate) struct HostContext {
    inner: cxx::SharedPtr<cxx_host::HostContext>,
}

// Runtime calls are confined to Endstone's primary thread.
unsafe impl Send for HostContext {}

impl HostContext {
    pub(crate) fn new(inner: cxx::SharedPtr<cxx_host::HostContext>) -> Result<Self, HostError> {
        (!inner.is_null())
            .then_some(Self { inner })
            .ok_or(HostError(AEGILEX_INVALID_ARGUMENT))
    }

    pub(crate) fn server(&self) -> Result<&crate::cxx_host_server::ffi::Server, HostError> {
        self.inner
            .as_ref()
            .map(cxx_host::HostContext::getServer)
            .ok_or(HostError(AEGILEX_INVALID_ARGUMENT))
    }

    fn as_ref(&self) -> Result<&cxx_host::HostContext, HostError> {
        self.inner
            .as_ref()
            .ok_or(HostError(AEGILEX_INVALID_ARGUMENT))
    }

    pub(crate) fn next_invocation_id(&self) -> u64 {
        self.as_ref()
            .map(cxx_host::HostContext::nextInvocationId)
            .unwrap_or(0)
    }
}

pub(crate) fn status_result(status: u32) -> Result<(), HostError> {
    if status == AEGILEX_OK {
        Ok(())
    } else {
        Err(HostError(status))
    }
}

pub(crate) fn get_logger(
    host: &HostContext,
    plugin_id: &str,
    invocation_id: u64,
) -> Result<cxx::UniquePtr<cxx_host::Logger>, HostError> {
    cxx::let_cxx_string!(plugin_id = plugin_id);
    let mut result = host.as_ref()?.getLogger(&plugin_id, invocation_id);
    let result = result.as_mut().ok_or(HostError(AEGILEX_INTERNAL_ERROR))?;
    status_result(result.getStatus())?;
    let logger = result.takeLogger();
    (!logger.is_null())
        .then_some(logger)
        .ok_or(HostError(AEGILEX_INTERNAL_ERROR))
}

pub(crate) fn logger_log(
    logger: &cxx_host::Logger,
    level: LogLevel,
    message: &str,
) -> Result<(), HostError> {
    status_result(logger.log(level.as_raw(), message))
}

pub(crate) fn logger_name(logger: &cxx_host::Logger) -> String {
    logger.getName()
}

pub(crate) fn logger_level(logger: &cxx_host::Logger) -> Result<LogLevel, HostError> {
    log_level_from_raw(logger.getLevel())
}

pub(crate) fn logger_set_level(
    logger: &cxx_host::Logger,
    level: LogLevel,
) -> Result<(), HostError> {
    status_result(logger.setLevel(level.as_raw()))
}

pub(crate) fn logger_is_enabled_for(
    logger: &cxx_host::Logger,
    level: LogLevel,
) -> Result<bool, HostError> {
    Ok(logger.isEnabledFor(level.as_raw()))
}

fn log_level_from_raw(level: u32) -> Result<LogLevel, HostError> {
    match level {
        AEGILEX_LOG_TRACE => Ok(LogLevel::Trace),
        AEGILEX_LOG_DEBUG => Ok(LogLevel::Debug),
        AEGILEX_LOG_INFO => Ok(LogLevel::Info),
        AEGILEX_LOG_WARNING => Ok(LogLevel::Warning),
        AEGILEX_LOG_ERROR => Ok(LogLevel::Error),
        AEGILEX_LOG_CRITICAL => Ok(LogLevel::Critical),
        AEGILEX_LOG_OFF => Ok(LogLevel::Off),
        _ => Err(HostError(AEGILEX_INVALID_ARGUMENT)),
    }
}

pub(crate) fn task_schedule_now(
    host: &HostContext,
    plugin_id: &str,
    invocation_id: u64,
) -> Result<u64, HostError> {
    cxx::let_cxx_string!(plugin_id = plugin_id);
    let result = host.as_ref()?.scheduleTaskNow(&plugin_id, invocation_id);
    status_result(result.status)?;
    nonzero_task_id(result.task_id)
}

pub(crate) fn form_show(
    host: &HostContext,
    plugin_id: &str,
    uuid: &[u8],
    spec: &crate::cxx_runtime::ffi::FormSpecData,
) -> Result<u64, HostError> {
    cxx::let_cxx_string!(plugin_id = plugin_id);
    let mut form_id = 0u64;
    let status = host
        .as_ref()?
        .form_show(&plugin_id, uuid, spec, std::pin::Pin::new(&mut form_id));
    status_result(status)?;
    if form_id == 0 {
        return Err(HostError(AEGILEX_INTERNAL_ERROR));
    }
    Ok(form_id)
}

pub(crate) fn form_close(host: &HostContext, uuid: &[u8]) -> Result<(), HostError> {
    status_result(host.as_ref()?.form_close(uuid))
}

pub(crate) fn plugin_manager_list(
    host: &HostContext,
) -> Result<crate::cxx_runtime::ffi::PluginList, HostError> {
    let result = host.as_ref()?.list_plugins();
    status_result(result.status)?;
    Ok(result)
}

pub(crate) fn plugin_manager_get(
    host: &HostContext,
    plugin_id: &str,
) -> Result<crate::cxx_runtime::ffi::PluginInfoData, HostError> {
    cxx::let_cxx_string!(plugin_id = plugin_id);
    let mut out = crate::cxx_runtime::ffi::PluginInfoData {
        metadata: crate::cxx_runtime::ffi::RuntimePluginMetadata {
            name: String::new(),
            version: String::new(),
            description: String::new(),
            load_order: 0,
            authors: Vec::new(),
            contributors: Vec::new(),
            website: String::new(),
            prefix: String::new(),
            provides: Vec::new(),
            depend: Vec::new(),
            soft_depend: Vec::new(),
            load_before: Vec::new(),
            default_permission: 0,
            commands: Vec::new(),
            permissions: Vec::new(),
            subscriptions: Vec::new(),
        },
        enabled: false,
    };
    status_result(host.as_ref()?.get_plugin(&plugin_id, &mut out))?;
    Ok(out)
}

pub(crate) fn plugin_manager_enable(host: &HostContext, plugin_id: &str) -> Result<(), HostError> {
    cxx::let_cxx_string!(plugin_id = plugin_id);
    status_result(host.as_ref()?.enable_plugin(&plugin_id))
}

pub(crate) fn plugin_manager_disable(host: &HostContext, plugin_id: &str) -> Result<(), HostError> {
    cxx::let_cxx_string!(plugin_id = plugin_id);
    status_result(host.as_ref()?.disable_plugin(&plugin_id))
}

pub(crate) fn service_publish(
    host: &HostContext,
    plugin_id: &str,
    name: &str,
    version: &str,
    methods: &[String],
    priority: u32,
) -> Result<u64, HostError> {
    cxx::let_cxx_string!(plugin_id = plugin_id);
    let methods = methods.to_vec();
    let id = host
        .as_ref()?
        .service_publish(&plugin_id, name, version, &methods, priority);
    if id == 0 {
        return Err(HostError(AEGILEX_INTERNAL_ERROR));
    }
    Ok(id)
}

pub(crate) fn service_unpublish(
    host: &HostContext,
    plugin_id: &str,
    provider_id: u64,
) -> Result<(), HostError> {
    cxx::let_cxx_string!(plugin_id = plugin_id);
    status_result(host.as_ref()?.service_unpublish(&plugin_id, provider_id))
}

pub(crate) fn service_list(
    host: &HostContext,
    name: &str,
) -> Result<crate::cxx_runtime::ffi::ServiceListData, HostError> {
    let result = host.as_ref()?.service_list(name);
    status_result(result.status)?;
    Ok(result)
}

pub(crate) fn service_call(
    host: &HostContext,
    plugin_id: &str,
    provider_id: u64,
    method: &str,
    payload: &[u8],
    deadline: u64,
) -> Result<u64, HostError> {
    cxx::let_cxx_string!(plugin_id = plugin_id);
    let id = host
        .as_ref()?
        .service_call(&plugin_id, provider_id, method, payload, deadline);
    if id == 0 {
        return Err(HostError(AEGILEX_INTERNAL_ERROR));
    }
    Ok(id)
}

pub(crate) fn service_call_status(
    host: &HostContext,
    call_id: u64,
) -> Result<crate::cxx_runtime::ffi::ServiceCallStatusData, HostError> {
    let result = host.as_ref()?.service_call_status(call_id);
    status_result(result.status)?;
    Ok(result)
}

pub(crate) fn service_take_response(
    host: &HostContext,
    call_id: u64,
) -> Result<crate::cxx_runtime::ffi::ServiceResponseData, HostError> {
    let result = host.as_ref()?.service_take_response(call_id);
    status_result(result.status)?;
    Ok(result)
}

pub(crate) fn service_cancel(host: &HostContext, call_id: u64) -> Result<(), HostError> {
    status_result(host.as_ref()?.service_cancel(call_id))
}

pub(crate) fn map_renderer_register(
    host: &HostContext,
    plugin_id: &str,
    map_id: i64,
    contextual: bool,
) -> Result<u64, HostError> {
    cxx::let_cxx_string!(plugin_id = plugin_id);
    let mut renderer_id = 0u64;
    let status = host.as_ref()?.map_renderer_register(
        &plugin_id,
        map_id,
        contextual,
        std::pin::Pin::new(&mut renderer_id),
    );
    status_result(status)?;
    if renderer_id == 0 {
        return Err(HostError(AEGILEX_INTERNAL_ERROR));
    }
    Ok(renderer_id)
}

pub(crate) fn map_renderer_unregister(
    host: &HostContext,
    plugin_id: &str,
    renderer_id: u64,
) -> Result<(), HostError> {
    cxx::let_cxx_string!(plugin_id = plugin_id);
    status_result(
        host.as_ref()?
            .map_renderer_unregister(&plugin_id, renderer_id),
    )
}

pub(crate) fn task_schedule_after(
    host: &HostContext,
    plugin_id: &str,
    invocation_id: u64,
    delay_ticks: u64,
) -> Result<u64, HostError> {
    cxx::let_cxx_string!(plugin_id = plugin_id);
    let result = host
        .as_ref()?
        .scheduleTaskAfter(&plugin_id, invocation_id, delay_ticks);
    status_result(result.status)?;
    nonzero_task_id(result.task_id)
}

pub(crate) fn task_schedule_every(
    host: &HostContext,
    plugin_id: &str,
    invocation_id: u64,
    initial_delay_ticks: u64,
    period_ticks: u64,
) -> Result<u64, HostError> {
    cxx::let_cxx_string!(plugin_id = plugin_id);
    let result = host.as_ref()?.scheduleTaskEvery(
        &plugin_id,
        invocation_id,
        initial_delay_ticks,
        period_ticks,
    );
    status_result(result.status)?;
    nonzero_task_id(result.task_id)
}

pub(crate) fn task_cancel(
    host: &HostContext,
    plugin_id: &str,
    invocation_id: u64,
    task_id: u64,
) -> Result<(), HostError> {
    cxx::let_cxx_string!(plugin_id = plugin_id);
    status_result(
        host.as_ref()?
            .cancelTask(&plugin_id, invocation_id, task_id),
    )
}

pub(crate) fn task_get_info(
    host: &HostContext,
    plugin_id: &str,
    invocation_id: u64,
    task_id: u64,
) -> Result<TaskSummary, HostError> {
    cxx::let_cxx_string!(plugin_id = plugin_id);
    let result = host
        .as_ref()?
        .getTaskInfo(&plugin_id, invocation_id, task_id);
    status_result(result.status)?;
    Ok(TaskSummary {
        task_id: result.task_id,
        owner: result.owner,
        is_sync: result.is_sync,
        is_cancelled: result.is_cancelled,
    })
}

pub(crate) fn task_is_running(
    host: &HostContext,
    plugin_id: &str,
    invocation_id: u64,
    task_id: u64,
) -> Result<bool, HostError> {
    cxx::let_cxx_string!(plugin_id = plugin_id);
    let result = host
        .as_ref()?
        .isTaskRunning(&plugin_id, invocation_id, task_id);
    status_result(result.status)?;
    Ok(result.value)
}

pub(crate) fn task_is_queued(
    host: &HostContext,
    plugin_id: &str,
    invocation_id: u64,
    task_id: u64,
) -> Result<bool, HostError> {
    cxx::let_cxx_string!(plugin_id = plugin_id);
    let result = host
        .as_ref()?
        .isTaskQueued(&plugin_id, invocation_id, task_id);
    status_result(result.status)?;
    Ok(result.value)
}

pub(crate) fn task_list_pending(
    host: &HostContext,
    plugin_id: &str,
    invocation_id: u64,
) -> Result<Vec<TaskSummary>, HostError> {
    cxx::let_cxx_string!(plugin_id = plugin_id);
    let result = host.as_ref()?.listPendingTasks(&plugin_id, invocation_id);
    status_result(result.status)?;
    Ok(result
        .tasks
        .into_iter()
        .map(|task| TaskSummary {
            task_id: task.task_id,
            owner: task.owner,
            is_sync: task.is_sync,
            is_cancelled: task.is_cancelled,
        })
        .collect())
}

fn nonzero_task_id(task_id: u64) -> Result<u64, HostError> {
    if task_id == 0 {
        Err(HostError(AEGILEX_INTERNAL_ERROR))
    } else {
        Ok(task_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::cxx_host::ffi as cxx_host;

    use super::{
        HostContext, LogLevel, get_logger, logger_level, logger_log, logger_name, logger_set_level,
        task_cancel, task_get_info, task_is_queued, task_is_running, task_list_pending,
        task_schedule_after, task_schedule_every, task_schedule_now,
    };

    #[test]
    fn logger_uses_the_typed_cxx_bridge() {
        let host = HostContext::new(cxx_host::HostContext::test_stub()).unwrap();
        let logger = get_logger(&host, "test_plugin", 42).unwrap();
        assert_eq!(logger_name(&logger), "Aegilex");
        assert_eq!(logger_level(&logger), Ok(LogLevel::Info));
        logger_log(&logger, LogLevel::Info, "typed logger").unwrap();
        logger_set_level(&logger, LogLevel::Off).unwrap();
    }

    #[test]
    fn tasks_use_the_typed_cxx_bridge() {
        let host = HostContext::new(cxx_host::HostContext::test_stub()).unwrap();
        assert_eq!(task_schedule_now(&host, "test_plugin", 42).unwrap(), 7);
        assert_eq!(
            task_schedule_after(&host, "test_plugin", 42, 10).unwrap(),
            7
        );
        assert_eq!(
            task_schedule_every(&host, "test_plugin", 42, 10, 20).unwrap(),
            7
        );
        task_cancel(&host, "test_plugin", 42, 7).unwrap();

        let info = task_get_info(&host, "test_plugin", 42, 7).unwrap();
        assert_eq!(info.task_id, 7);
        assert_eq!(info.owner, "test_plugin");
        assert!(info.is_sync);
        assert!(info.is_cancelled);
        assert!(task_is_running(&host, "test_plugin", 42, 7).unwrap());
        assert!(!task_is_queued(&host, "test_plugin", 42, 7).unwrap());

        let pending = task_list_pending(&host, "test_plugin", 42).unwrap();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].task_id, 7);
        assert_eq!(pending[0].owner, "test_plugin");
    }
}
