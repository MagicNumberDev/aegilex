//! Types in this module map to Endstone's scheduler `Task` API namespace.

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub(crate) struct TaskSummary {
    pub(crate) task_id: u64,
    pub(crate) owner: String,
    pub(crate) is_sync: bool,
    pub(crate) is_cancelled: bool,
}
