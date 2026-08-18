//! Core ABI implementation for `native/bindings/endstone/logger.h`.

use crate::core_host::imports::HostLogger;
use crate::core_host::{LoggerLogLevel, TypesHostError};
use crate::host::endstone::support::*;

fn native_log_level(level: LoggerLogLevel) -> crate::bindings::endstone::logger::LogLevel {
    use crate::bindings::endstone::logger::LogLevel as NativeLogLevel;
    match level {
        LoggerLogLevel::Trace => NativeLogLevel::Trace,
        LoggerLogLevel::Debug => NativeLogLevel::Debug,
        LoggerLogLevel::Info => NativeLogLevel::Info,
        LoggerLogLevel::Warning => NativeLogLevel::Warning,
        LoggerLogLevel::Error => NativeLogLevel::Error,
        LoggerLogLevel::Critical => NativeLogLevel::Critical,
        LoggerLogLevel::Off => NativeLogLevel::Off,
    }
}

fn wit_log_level(level: crate::bindings::endstone::logger::LogLevel) -> LoggerLogLevel {
    use crate::bindings::endstone::logger::LogLevel as NativeLogLevel;
    match level {
        NativeLogLevel::Trace => LoggerLogLevel::Trace,
        NativeLogLevel::Debug => LoggerLogLevel::Debug,
        NativeLogLevel::Info => LoggerLogLevel::Info,
        NativeLogLevel::Warning => LoggerLogLevel::Warning,
        NativeLogLevel::Error => LoggerLogLevel::Error,
        NativeLogLevel::Critical => LoggerLogLevel::Critical,
        NativeLogLevel::Off => LoggerLogLevel::Off,
    }
}

fn resolve_logger(
    state: &PluginStoreState,
    handle: u32,
) -> Result<&crate::cxx_host::ffi::Logger, HostError> {
    state
        .resource_slot(handle, ResourceKind::Logger)
        .and_then(|slot| {
            state
                .handles
                .logger(state.invocation_id, slot.handle)
                .ok_or_else(|| HostError::from_status(crate::abi::AEGILEX_NOT_FOUND))
        })
}

impl HostLogger for PluginStoreState {
    fn get_logger(&mut self) -> Result<Result<u32, TypesHostError>, String> {
        self.require_capability("logger.get-logger")
            .map_err(|error| format!("{error:?}"))?;
        Ok(
            native::get_logger(&self.host, &self.plugin_id, self.invocation_id)
                .and_then(|logger| self.insert_logger_resource(logger))
                .map_err(map_core_host_error),
        )
    }

    fn logger_log(
        &mut self,
        self_: u32,
        level: LoggerLogLevel,
        message: String,
    ) -> Result<Result<(), TypesHostError>, String> {
        self.require_capability("logger.logger.log")
            .map_err(|error| format!("{error:?}"))?;
        Ok(resolve_logger(self, self_)
            .and_then(|logger| native::logger_log(logger, native_log_level(level), &message))
            .map_err(map_core_host_error))
    }

    fn logger_get_name(&mut self, self_: u32) -> Result<Result<String, TypesHostError>, String> {
        self.require_capability("logger.logger.get-name")
            .map_err(|error| format!("{error:?}"))?;
        Ok(resolve_logger(self, self_)
            .map(native::logger_name)
            .map_err(map_core_host_error))
    }

    fn logger_get_level(
        &mut self,
        self_: u32,
    ) -> Result<Result<LoggerLogLevel, TypesHostError>, String> {
        self.require_capability("logger.logger.get-level")
            .map_err(|error| format!("{error:?}"))?;
        Ok(resolve_logger(self, self_)
            .and_then(native::logger_level)
            .map(wit_log_level)
            .map_err(map_core_host_error))
    }

    fn logger_set_level(
        &mut self,
        self_: u32,
        level: LoggerLogLevel,
    ) -> Result<Result<(), TypesHostError>, String> {
        self.require_capability("logger.logger.set-level")
            .map_err(|error| format!("{error:?}"))?;
        Ok(resolve_logger(self, self_)
            .and_then(|logger| native::logger_set_level(logger, native_log_level(level)))
            .map_err(map_core_host_error))
    }

    fn logger_is_enabled_for(
        &mut self,
        self_: u32,
        level: LoggerLogLevel,
    ) -> Result<Result<bool, TypesHostError>, String> {
        self.require_capability("logger.logger.is-enabled-for")
            .map_err(|error| format!("{error:?}"))?;
        Ok(resolve_logger(self, self_)
            .and_then(|logger| native::logger_is_enabled_for(logger, native_log_level(level)))
            .map_err(map_core_host_error))
    }
}
