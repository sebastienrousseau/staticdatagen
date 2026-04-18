// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! This module contains macros related to logging messages at various log levels and formats.
//!
//! It includes a custom logging macro, `macro_log_info`, which allows logging messages with
//! specified log levels, components, descriptions, and formats.

/// Custom logging macro for various log levels and formats.
///
/// # Parameters
///
/// * `$level`: The log level of the message.
/// * `$component`: The component where the log is coming from.
/// * `$description`: A description of the log message.
/// * `$format`: The format of the log message.
///
/// # Example
///
/// ```
/// use staticdatagen::macro_log_info;
/// use rlg::log_level::LogLevel;
/// use rlg::log_format::LogFormat;
///
/// let level = &LogLevel::INFO;
/// let component = "TestComponent";
/// let description = "Test description";
/// let format = &LogFormat::CLF;
///
/// let log = macro_log_info!(level, component, description, format);
/// ```
#[macro_export]
macro_rules! macro_log_info {
    ($level:expr, $component:expr, $description:expr, $format:expr) => {{
        use dtt::datetime::DateTime;
        use rlg::log::Log;

        let date = DateTime::new();
        let mut entry = Log::build(*$level, $description)
            .time(&date.to_string())
            .component($component);
        entry.format = *$format;
        let _log = entry;
    }};
}

#[cfg(test)]
mod tests {
    use rlg::log::Log;
    use rlg::log_format::LogFormat;
    use rlg::log_level::LogLevel;

    /// Verify log construction at each level without firing the rlg
    /// flusher (which triggers a macOS os_log assertion on process exit).
    #[test]
    fn test_log_build_info() {
        let log = Log::build(LogLevel::INFO, "info message")
            .component("TestComponent")
            .format(LogFormat::CLF);
        assert_eq!(log.level, LogLevel::INFO);
    }

    #[test]
    fn test_log_build_debug() {
        let log = Log::build(LogLevel::DEBUG, "debug message")
            .component("Debug")
            .format(LogFormat::JSON);
        assert_eq!(log.level, LogLevel::DEBUG);
    }

    #[test]
    fn test_log_build_error() {
        let log = Log::build(LogLevel::ERROR, "error occurred")
            .component("ErrorHandler")
            .format(LogFormat::CLF);
        assert_eq!(log.level, LogLevel::ERROR);
    }

    #[test]
    fn test_log_build_warning() {
        let log = Log::build(LogLevel::WARN, "warning message")
            .component("Warning")
            .format(LogFormat::CLF);
        assert_eq!(log.level, LogLevel::WARN);
    }
}
