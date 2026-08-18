//! Canonical-ABI error and capability helpers shared by host modules.

use crate::abi::{
    AEGILEX_DENIED, AEGILEX_INVALID_ARGUMENT, AEGILEX_LIMIT_EXCEEDED, AEGILEX_NOT_FOUND,
};
use crate::core_host::TypesHostError;
use crate::host::runtime::native::HostError;
use crate::runtime::PluginStoreState;

/// Maps a native `HostError` status to the generated host-error enum.
pub(crate) fn map_core_host_error(error: HostError) -> TypesHostError {
    match error.status() {
        AEGILEX_INVALID_ARGUMENT => TypesHostError::InvalidInput,
        AEGILEX_LIMIT_EXCEEDED => TypesHostError::LimitExceeded,
        AEGILEX_DENIED => TypesHostError::Denied,
        AEGILEX_NOT_FOUND => TypesHostError::NotFound,
        _ => TypesHostError::Unavailable,
    }
}

/// Checks a capability and maps the rejection to the generated error enum.
pub(crate) fn check_capability(
    this: &PluginStoreState,
    capability: &str,
) -> Result<(), TypesHostError> {
    this.require_capability(capability)
}
