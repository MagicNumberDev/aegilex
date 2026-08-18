#[cxx::bridge(namespace = "aegilex::host")]
pub(crate) mod ffi {
    struct TaskId {
        status: u32,
        task_id: u64,
    }

    struct TaskState {
        status: u32,
        value: bool,
    }

    struct TaskSummary {
        status: u32,
        task_id: u64,
        owner: String,
        is_sync: bool,
        is_cancelled: bool,
    }

    struct TaskList {
        status: u32,
        tasks: Vec<TaskSummary>,
    }

    unsafe extern "C++" {
        include!("host_context.h");

        #[namespace = "aegilex::native"]
        type HostContext;
        #[namespace = "aegilex::native::server"]
        type Server = crate::cxx_host_server::ffi::Server;

        #[namespace = "aegilex::native"]
        fn getServer(self: &HostContext) -> &Server;
        #[namespace = "aegilex::native"]
        fn nextInvocationId(self: &HostContext) -> u64;
        #[namespace = "aegilex::native"]
        #[namespace = "aegilex::native"]
        #[namespace = "aegilex::native"]
        #[allow(dead_code)]
        #[namespace = "aegilex::native"]
        type Logger;
        #[namespace = "aegilex::native"]
        type LoggerResult;
        #[namespace = "aegilex::native"]
        fn getLogger(
            self: &HostContext,
            plugin_id: &CxxString,
            invocation_id: u64,
        ) -> UniquePtr<LoggerResult>;
        #[namespace = "aegilex::native"]
        fn getStatus(self: &LoggerResult) -> u32;
        #[namespace = "aegilex::native"]
        fn takeLogger(self: Pin<&mut LoggerResult>) -> UniquePtr<Logger>;
        #[namespace = "aegilex::native"]
        fn log(self: &Logger, level: u32, message: &str) -> u32;
        #[namespace = "aegilex::native"]
        fn getName(self: &Logger) -> String;
        #[namespace = "aegilex::native"]
        fn getLevel(self: &Logger) -> u32;
        #[namespace = "aegilex::native"]
        fn setLevel(self: &Logger, level: u32) -> u32;
        #[namespace = "aegilex::native"]
        fn isEnabledFor(self: &Logger, level: u32) -> bool;
        #[namespace = "aegilex::native"]
        fn scheduleTaskNow(self: &HostContext, plugin_id: &CxxString, invocation_id: u64)
        -> TaskId;
        #[namespace = "aegilex::native"]
        fn scheduleTaskAfter(
            self: &HostContext,
            plugin_id: &CxxString,
            invocation_id: u64,
            delay_ticks: u64,
        ) -> TaskId;
        #[namespace = "aegilex::native"]
        fn scheduleTaskEvery(
            self: &HostContext,
            plugin_id: &CxxString,
            invocation_id: u64,
            initial_delay_ticks: u64,
            period_ticks: u64,
        ) -> TaskId;
        #[namespace = "aegilex::native"]
        fn cancelTask(
            self: &HostContext,
            plugin_id: &CxxString,
            invocation_id: u64,
            task_id: u64,
        ) -> u32;
        #[namespace = "aegilex::native"]
        fn getTaskInfo(
            self: &HostContext,
            plugin_id: &CxxString,
            invocation_id: u64,
            task_id: u64,
        ) -> TaskSummary;
        #[namespace = "aegilex::native"]
        fn isTaskRunning(
            self: &HostContext,
            plugin_id: &CxxString,
            invocation_id: u64,
            task_id: u64,
        ) -> TaskState;
        #[namespace = "aegilex::native"]
        fn isTaskQueued(
            self: &HostContext,
            plugin_id: &CxxString,
            invocation_id: u64,
            task_id: u64,
        ) -> TaskState;
        #[namespace = "aegilex::native"]
        fn listPendingTasks(
            self: &HostContext,
            plugin_id: &CxxString,
            invocation_id: u64,
        ) -> TaskList;

        #[namespace = "aegilex::native"]
        #[Self = "HostContext"]
        #[rust_name = "test_stub"]
        #[allow(dead_code)]
        fn testStub() -> SharedPtr<HostContext>;
    }
}
