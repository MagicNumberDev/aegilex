// Typed status codes and Endstone enum encodings shared by the Rust host.

pub const AEGILEX_OK: u32 = 0;
pub const AEGILEX_INVALID_ARGUMENT: u32 = 1;
pub const AEGILEX_NOT_FOUND: u32 = 3;
pub const AEGILEX_DENIED: u32 = 4;
pub const AEGILEX_LIMIT_EXCEEDED: u32 = 6;
pub const AEGILEX_TRAP: u32 = 7;
pub const AEGILEX_INTERNAL_ERROR: u32 = 9;

pub const AEGILEX_LOG_TRACE: u32 = 0;
pub const AEGILEX_LOG_DEBUG: u32 = 1;
pub const AEGILEX_LOG_INFO: u32 = 2;
pub const AEGILEX_LOG_WARNING: u32 = 3;
pub const AEGILEX_LOG_ERROR: u32 = 4;
pub const AEGILEX_LOG_CRITICAL: u32 = 5;
pub const AEGILEX_LOG_OFF: u32 = 6;
