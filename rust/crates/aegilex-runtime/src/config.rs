pub(crate) const DEFAULT_MAX_NESTED_DISPATCH_DEPTH: u64 = 32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct RuntimeConfig {
    pub(crate) max_module_bytes: u64,
    /// Bounded synchronous callback nesting. A callback entered through an
    /// already-active host import consumes one level.
    pub(crate) max_nested_dispatch_depth: u64,
    // A zero quota is unlimited.
    pub(crate) max_nbt_depth: u64,
    pub(crate) max_nbt_nodes: u64,
    pub(crate) max_nbt_compound_entries: u64,
    pub(crate) max_nbt_string_bytes: u64,
    pub(crate) max_nbt_array_bytes: u64,
    pub(crate) max_invocation_native_resources: u64,
    pub(crate) max_plugin_resource_slots: u64,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            max_module_bytes: 0,
            max_nested_dispatch_depth: DEFAULT_MAX_NESTED_DISPATCH_DEPTH,
            max_nbt_depth: 0,
            max_nbt_nodes: 0,
            max_nbt_compound_entries: 0,
            max_nbt_string_bytes: 0,
            max_nbt_array_bytes: 0,
            max_invocation_native_resources: 0,
            max_plugin_resource_slots: 0,
        }
    }
}
