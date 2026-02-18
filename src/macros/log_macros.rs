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
/// let level = LogLevel::INFO;
/// let component = "TestComponent";
/// let description = "Test description";
/// let format = LogFormat::CLF;
///
/// let log = macro_log_info!(&level, component, description, &format);
/// ```
#[macro_export]
macro_rules! macro_log_info {
    ($level:expr, $component:expr, $description:expr, $format:expr) => {{
        use dtt::datetime::DateTime;
        use rlg::log::Log;
        use vrd::random::Random;

        let date = DateTime::new();
        let mut rng = Random::default();
        let session_id = rng.rand().to_string();

        // Create the log and call `.ok()` to discard the unused result
        let _log = Log::new(
            &session_id,
            &date.to_string(),
            $level,
            $component,
            $description,
            $format,
        );
    }};
}

#[cfg(test)]
mod tests {
    use rlg::log_format::LogFormat;
    use rlg::log_level::LogLevel;

    #[test]
    fn test_macro_log_info_basic() {
        let level = LogLevel::INFO;
        let component = "TestComponent";
        let description = "Test description";
        let format = LogFormat::CLF;

        // This should compile and run without errors
        macro_log_info!(&level, component, description, &format);
    }

    #[test]
    fn test_macro_log_info_debug_level() {
        let level = LogLevel::DEBUG;
        let component = "Debug";
        let description = "Debug message";
        let format = LogFormat::JSON;

        macro_log_info!(&level, component, description, &format);
    }

    #[test]
    fn test_macro_log_info_error_level() {
        let level = LogLevel::ERROR;
        let component = "ErrorHandler";
        let description = "Error occurred";
        let format = LogFormat::CLF;

        macro_log_info!(&level, component, description, &format);
    }

    #[test]
    fn test_macro_log_info_warning_level() {
        let level = LogLevel::WARN;
        let component = "Warning";
        let description = "Warning message";
        let format = LogFormat::CLF;

        macro_log_info!(&level, component, description, &format);
    }
}
