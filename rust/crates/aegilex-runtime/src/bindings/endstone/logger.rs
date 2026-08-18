use crate::abi::{
    AEGILEX_LOG_CRITICAL, AEGILEX_LOG_DEBUG, AEGILEX_LOG_ERROR, AEGILEX_LOG_INFO, AEGILEX_LOG_OFF,
    AEGILEX_LOG_TRACE, AEGILEX_LOG_WARNING,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub(crate) enum LogLevel {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
    Critical,
    Off,
}

impl LogLevel {
    pub(crate) const fn as_raw(self) -> u32 {
        match self {
            Self::Trace => AEGILEX_LOG_TRACE,
            Self::Debug => AEGILEX_LOG_DEBUG,
            Self::Info => AEGILEX_LOG_INFO,
            Self::Warning => AEGILEX_LOG_WARNING,
            Self::Error => AEGILEX_LOG_ERROR,
            Self::Critical => AEGILEX_LOG_CRITICAL,
            Self::Off => AEGILEX_LOG_OFF,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub(crate) struct LoggerInfo {
    pub(crate) name: String,
    pub(crate) level: LogLevel,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_every_log_level_variant_to_its_abi_constant() {
        assert_eq!(LogLevel::Trace.as_raw(), AEGILEX_LOG_TRACE);
        assert_eq!(LogLevel::Debug.as_raw(), AEGILEX_LOG_DEBUG);
        assert_eq!(LogLevel::Info.as_raw(), AEGILEX_LOG_INFO);
        assert_eq!(LogLevel::Warning.as_raw(), AEGILEX_LOG_WARNING);
        assert_eq!(LogLevel::Error.as_raw(), AEGILEX_LOG_ERROR);
        assert_eq!(LogLevel::Critical.as_raw(), AEGILEX_LOG_CRITICAL);
        assert_eq!(LogLevel::Off.as_raw(), AEGILEX_LOG_OFF);
    }

    #[test]
    fn abi_levels_match_the_endstone_logger_level_enum_order() {
        assert_eq!(AEGILEX_LOG_CRITICAL, 5);
        assert_eq!(AEGILEX_LOG_OFF, 6);
    }
}
