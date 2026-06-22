// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! This module contains macros related to logging messages at various log levels and formats.
//!
//! It includes a custom logging macro, `macro_log_info`, which logs messages
//! using the standard `log` crate with component and timestamp context.

/// Custom logging macro for structured log messages.
///
/// # Parameters
///
/// * `$level`: A log level string (e.g., `"INFO"`, `"DEBUG"`, `"ERROR"`, `"WARN"`).
/// * `$component`: The component where the log is coming from.
/// * `$description`: A description of the log message.
/// * `$format`: A format identifier string (e.g., `"CLF"`, `"JSON"`).
///
/// # Example
///
/// ```
/// use staticdatagen::macro_log_info;
///
/// macro_log_info!("INFO", "TestComponent", "Test message", "CLF");
/// ```
#[macro_export]
macro_rules! macro_log_info {
    ($level:expr, $component:expr, $description:expr, $format:expr) => {{
        log::info!(
            "[{}] [{}] {} (format={})",
            $component,
            $level,
            $description,
            $format
        );
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_macro_log_info_compiles() {
        macro_log_info!(
            "INFO",
            "TestComponent",
            "Test description",
            "CLF"
        );
    }

    #[test]
    fn test_macro_log_info_debug_level() {
        macro_log_info!("DEBUG", "Debug", "Debug message", "JSON");
    }

    #[test]
    fn test_macro_log_info_error_level() {
        macro_log_info!(
            "ERROR",
            "ErrorHandler",
            "Error occurred",
            "CLF"
        );
    }

    #[test]
    fn test_macro_log_info_warning_level() {
        macro_log_info!("WARN", "Warning", "Warning message", "CLF");
    }
}
