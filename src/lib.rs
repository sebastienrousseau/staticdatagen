// Copyright © 2024 StaticDataGen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// src/lib.rs

#![doc = include_str!("../README.md")]
#![doc(
    html_favicon_url = "https://kura.pro/staticdatagen/images/favicon.ico",
    html_logo_url = "https://kura.pro/staticdatagen/images/logos/staticdatagen.svg",
    html_root_url = "https://docs.rs/staticdatagen"
)]

/// Generator modules for creating static content.
pub mod generators {
    /// CNAME Record Generation Module
    pub mod cname;
    /// Humans.txt Generation Module
    pub mod humans;
    /// Manifest Generation Module
    pub mod manifest;
}

/// Compiler module for processing and generating static site content.
pub mod compiler;

/// Locales module for language-specific translations and templates.
pub mod locales;

/// Macro definitions for common operations.
pub mod macros;

/// Data models and structures used throughout the crate.
pub mod models;

/// Various modules for specific functionalities (e.g., HTML generation, RSS feeds).
pub mod modules;

/// Utility functions and helpers.
pub mod utilities;

// Re-export commonly used items for easier access
pub use compiler::service::compile;
pub use http_handle::Server;
pub use utilities::uuid::generate_unique_string;

/// Version of the staticdatagen library.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Error type for the staticdatagen library
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Custom error type for the staticdatagen library
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// IO operation errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Content processing errors
    #[error("Content processing error: {0}")]
    ContentProcessing(String),

    /// Template rendering errors
    #[error("Template error: {0}")]
    Template(String),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),
}

#[cfg(test)]
mod tests {

    use crate::Error;
    use crate::VERSION;
    use std::error::Error as StdError;

    #[test]
    #[allow(clippy::const_is_empty)]
    fn test_version() {
        assert!(!VERSION.is_empty(), "Version should not be empty");
    }

    #[test]
    fn test_version_format() {
        assert!(
            VERSION.contains('.'),
            "Version should be in semver format"
        );
        assert!(
            VERSION.split('.').count() >= 2,
            "Version should have at least major.minor"
        );
    }

    #[test]
    fn test_error_conversion() {
        let io_err = std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "file not found",
        );
        let err: Error = Error::Io(io_err);
        assert!(err.source().is_some());
    }

    #[test]
    fn test_error_display() {
        let err =
            Error::ContentProcessing("invalid content".to_string());
        assert_eq!(
            err.to_string(),
            "Content processing error: invalid content"
        );

        let err = Error::Template("template error".to_string());
        assert_eq!(err.to_string(), "Template error: template error");

        let err = Error::Config("config error".to_string());
        assert_eq!(
            err.to_string(),
            "Configuration error: config error"
        );
    }

    #[test]
    fn test_error_debug() {
        let err = Error::Config("test".to_string());
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("Config"));
        assert!(debug_str.contains("test"));
    }

    #[test]
    fn test_error_source() {
        // Test IO error source
        let io_err =
            std::io::Error::new(std::io::ErrorKind::Other, "test");
        let err = Error::Io(io_err);
        assert!(err.source().is_some());

        // Test other variants have no source
        let err = Error::Config("test".to_string());
        assert!(err.source().is_none());

        let err = Error::ContentProcessing("test".to_string());
        assert!(err.source().is_none());

        let err = Error::Template("test".to_string());
        assert!(err.source().is_none());
    }

    #[test]
    fn test_error_from_io() {
        let io_err =
            std::io::Error::new(std::io::ErrorKind::Other, "io error");
        let err: Error = io_err.into();
        assert!(matches!(err, Error::Io(_)));
    }

    #[test]
    fn test_result_type() {
        let result: Result<i32, Error> = Ok(42);
        assert!(result.is_ok());

        let err_result: Result<(), Error> =
            Err(Error::Config("test".into()));
        assert!(err_result.is_err());
    }

    #[test]
    fn test_error_chaining() {
        let io_err = std::io::Error::new(
            std::io::ErrorKind::Other,
            "base error",
        );
        let err = Error::Io(io_err);
        let mut count = 0;
        let mut source = err.source();
        while let Some(err) = source {
            count += 1;
            source = err.source();
        }
        assert_eq!(count, 1);
    }

    #[test]
    fn test_version_components() {
        let parts: Vec<&str> = VERSION.split('.').collect();
        assert!(
            parts.len() >= 2,
            "Version should have at least major and minor components"
        );
        // Verify each component is a valid number
        for part in parts {
            assert!(
                part.parse::<u32>().is_ok(),
                "Version component should be a valid number"
            );
        }
    }

    #[test]
    fn test_version_semver_format() {
        let semver_regex = regex::Regex::new(r"^\d+\.\d+(\.\d+)?(-[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?(\+[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?$").unwrap();
        assert!(
            semver_regex.is_match(VERSION),
            "Version should follow semver format"
        );
    }

    // Error Tests
    #[test]
    fn test_error_variants() {
        let errors = vec![
            Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "not found",
            )),
            Error::ContentProcessing("test error".to_string()),
            Error::Template("template error".to_string()),
            Error::Config("config error".to_string()),
        ];

        for error in errors {
            // Check that display message is non-empty
            assert!(
                !error.to_string().is_empty(),
                "Error should have a non-empty display message"
            );

            // Check debug format contains the variant name
            let debug_str = format!("{:?}", error);
            match error {
                Error::Io(_) => assert!(debug_str.contains("Io")),
                Error::ContentProcessing(_) => {
                    assert!(debug_str.contains("ContentProcessing"))
                }
                Error::Template(_) => {
                    assert!(debug_str.contains("Template"))
                }
                Error::Config(_) => {
                    assert!(debug_str.contains("Config"))
                }
            }
        }
    }

    #[test]
    fn test_error_nesting() {
        let io_error = std::io::Error::new(
            std::io::ErrorKind::Other,
            "root cause",
        );
        let error = Error::Io(io_error);

        let mut source_opt = error.source();
        let mut depth = 0;
        while let Some(source) = source_opt {
            depth += 1;
            source_opt = source.source();
        }
        assert_eq!(
            depth, 1,
            "Error should have exactly one level of nesting"
        );
    }

    #[test]
    fn test_error_conversion_chain() {
        let io_error = std::io::Error::new(
            std::io::ErrorKind::Other,
            "base error",
        );
        let error: Error = io_error.into();

        match error {
            Error::Io(inner) => {
                assert_eq!(inner.kind(), std::io::ErrorKind::Other);
                assert_eq!(inner.to_string(), "base error");
            }
            _ => panic!("Expected Io error variant"),
        }
    }

    // Result Type Tests
    #[test]
    fn test_result_type_mapping() {
        let success: Result<i32, Error> = Ok(42);
        let mapped = success.map(|x| x * 2);
        assert_eq!(mapped.unwrap(), 84);

        let failure: Result<(), Error> =
            Err(Error::Config("test".into()));
        let mapped_err = failure.map_err(|e| format!("Wrapped: {}", e));
        assert!(mapped_err.is_err());
    }

    #[test]
    fn test_result_type_composition() {
        fn inner_operation() -> std::io::Result<i32> {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "inner error",
            ))
        }

        fn outer_operation() -> Result<i32, std::io::Error> {
            let result = inner_operation()?;
            Ok(result)
        }

        assert!(outer_operation().is_err());
    }

    #[test]
    fn test_error_display_formatting() {
        let errors = [
            (
                Error::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "missing",
                )),
                "IO error: missing",
            ),
            (
                Error::ContentProcessing("invalid".to_string()),
                "Content processing error: invalid",
            ),
            (
                Error::Template("bad template".to_string()),
                "Template error: bad template",
            ),
            (
                Error::Config("bad config".to_string()),
                "Configuration error: bad config",
            ),
        ];

        for (error, expected) in errors.iter() {
            assert_eq!(error.to_string(), *expected);
        }
    }

    #[test]
    fn test_error_conversions() {
        // Test From implementation for std::io::Error
        let io_err =
            std::io::Error::new(std::io::ErrorKind::Other, "io error");
        let err: Error = Error::from(io_err);
        assert!(matches!(err, Error::Io(_)));

        // Test Into implementation
        let io_err =
            std::io::Error::new(std::io::ErrorKind::Other, "io error");
        let err: Error = io_err.into();
        assert!(matches!(err, Error::Io(_)));
    }

    #[test]
    fn test_error_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Error>();
    }

    // Test error message consistency
    #[test]
    fn test_error_message_consistency() {
        let msg = "test message";
        let errors = [
            (
                Error::ContentProcessing(msg.into()),
                "Content processing error: test message",
            ),
            (
                Error::Template(msg.into()),
                "Template error: test message",
            ),
            (
                Error::Config(msg.into()),
                "Configuration error: test message",
            ),
        ];

        for (error, expected) in errors {
            assert_eq!(error.to_string(), expected);
        }
    }

    // Test Result type combinators
    #[test]
    fn test_result_combinators() {
        let mut ok_result: Result<_, Error> = Ok(42);
        let err_result = Error::Config("test".into());

        assert_eq!(*ok_result.as_mut().unwrap_or(&mut 0), 42);
        assert_eq!(err_result.to_string(), "Configuration error: test");

        let mapped = ok_result.map(|n| n * 2);
        assert_eq!(mapped.unwrap(), 84);
    }

    // Test error downcasting
    #[test]
    fn test_error_downcasting() {
        let io_err =
            std::io::Error::new(std::io::ErrorKind::Other, "test");
        let err = Error::Io(io_err);

        let std_error: &dyn StdError = &err;
        assert!(std_error.downcast_ref::<Error>().is_some());
    }

    // Test error cloning behavior with io::Error
    #[test]
    fn test_io_error_cloning() {
        let io_err =
            std::io::Error::new(std::io::ErrorKind::Other, "original");
        let err = Error::Io(io_err);

        // Convert to string and verify the content is preserved
        let err_string = err.to_string();
        assert!(err_string.contains("original"));
    }

    // Test error pattern matching
    #[test]
    fn test_error_pattern_matching() {
        let errors = vec![
            Error::Config("config".into()),
            Error::Template("template".into()),
            Error::ContentProcessing("content".into()),
            Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "io",
            )),
        ];

        let mut counts = (0, 0, 0, 0);
        for err in errors {
            match err {
                Error::Config(_) => counts.0 += 1,
                Error::Template(_) => counts.1 += 1,
                Error::ContentProcessing(_) => counts.2 += 1,
                Error::Io(_) => counts.3 += 1,
            }
        }

        assert_eq!(counts, (1, 1, 1, 1));
    }

    // Test error context preservation
    #[test]
    fn test_error_context_preservation() {
        let context = "important context";
        let io_err =
            std::io::Error::new(std::io::ErrorKind::Other, context);
        let err = Error::Io(io_err);

        assert!(err.to_string().contains(context));
    }

    #[test]
    fn test_custom_error_conversion() {
        // Test converting from String
        let err = Error::ContentProcessing(String::from("test"));
        assert_eq!(err.to_string(), "Content processing error: test");

        // Test converting from &str
        let err = Error::ContentProcessing("test".into());
        assert_eq!(err.to_string(), "Content processing error: test");
    }

    #[test]
    fn test_error_empty_messages() {
        let err = Error::Config(String::new());
        assert_eq!(err.to_string(), "Configuration error: ");

        let err = Error::Template(String::new());
        assert_eq!(err.to_string(), "Template error: ");
    }

    #[test]
    fn test_error_composition() {
        fn fallible_operation() -> Result<(), Error> {
            Err(Error::Config("inner error".into()))
        }

        let result = fallible_operation().map_err(|e| {
            Error::ContentProcessing(format!("outer: {}", e))
        });

        assert!(matches!(result, Err(Error::ContentProcessing(_))));
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("inner error"));
    }
}
