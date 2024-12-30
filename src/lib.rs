// Copyright © 2025 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![doc = include_str!("../README.md")]
#![doc(
    html_favicon_url = "https://kura.pro/staticdatagen/images/favicon.ico",
    html_logo_url = "https://kura.pro/staticdatagen/images/logos/staticdatagen.svg",
    html_root_url = "https://docs.rs/staticdatagen"
)]

use std::error::Error as StdError;
use thiserror::Error;

/// The `compiler` module contains routines that parse and transform input data into
/// static site outputs. It also holds mechanisms to streamline content transformations
/// and validations, ensuring robust operation even under heavy loads.
pub mod compiler;

/// The `generators` module contains generators for creating various forms of static assets.
/// These may include text files, images, or custom formats tailored to your application's
/// needs. Each generator is designed with consistency and security in mind.
pub mod generators;

/// The `models` module defines core data structures and types used throughout the library.
/// By centralising these models in one place, you ensure a consistent contract for data
/// across your application.
pub mod models;

/// The `modules` module comprises discrete functionalities that augment the core features
/// of this library. For instance, it might include HTML generation, RSS feeds, or other
/// structured content creation tasks. This segmentation keeps the library modular and
/// maintainable.
pub mod modules;

/// The `utilities` module provides additional helpers and convenience functions, from
/// string manipulation to generating unique identifiers. Designed to reduce boilerplate
/// code, these utilities help you write concise, clear, and error-resilient Rust.
pub mod utilities;

/// The `locales` module contains functionalities for handling multiple languages,
/// including translations and localised templates. It assists with localised content
/// generation tasks, ensuring that your static site can be adapted for various audiences.
pub mod locales;

/// Macro definitions for repetitive or boilerplate-heavy operations reside in the
/// `macros` module. By abstracting these patterns into macros, your codebase can
/// remain succinct and easier to maintain.
#[macro_use]
pub mod macros;

/// Re-exports the `compile` function from [`compiler::service`].
///
/// This function is central for parsing, transforming, and validating
/// input data into static site assets, ensuring secure handling of
/// content during the process.
pub use compiler::service::compile;

/// Re-exports the `Server` type from `http_handle`.
///
/// This server structure can be employed to host or serve generated
/// static content. It is designed for performance and robustness,
/// making it suitable for production environments.
pub use http_handle::Server;

/// Re-exports the `generate_unique_string` function from [`utilities::uuid`].
///
/// This utility function produces a randomised UUID-like string for
/// scenarios requiring unique identifiers. It helps maintain data integrity
/// and guarantees uniqueness across operations.
pub use utilities::uuid::generate_unique_string;

/// Specifies the version of the **staticdatagen** library.
///
/// This constant is automatically aligned with the crate’s version defined in
/// `Cargo.toml`. It is commonly referenced in diagnostic logs, user-facing messages,
/// and documentation outputs, ensuring consistent visibility of the library's
/// release status.
///
/// # Examples
///
/// ```
/// // Retrieve and print the library version:
/// println!("Current staticdatagen version: {}", staticdatagen::VERSION);
/// ```
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// A crate-specific `Result` type that defaults to using the library's own
/// [`Error`] variant. This reduces verbosity by eliminating repeated
/// `std::result::Result<T, Error>` notation in function signatures.
///
/// # Examples
///
/// ```
/// use staticdatagen::{Error, Result};
///
/// fn do_something() -> Result<()> {
///     // Implement logic here
///     Ok(())
/// }
/// ```
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Builder for constructing [`Error::ContentProcessing`] variants.
///
/// This builder offers a fluent API for incrementally specifying optional fields like
/// custom error messages, a source error, additional context details, and severity levels.
///
/// # Examples
///
/// ```
/// use staticdatagen::{ContentProcessingErrorBuilder, ErrorSeverity};
/// use std::error::Error as StdError;
///
/// fn process_content() -> Result<(), Box<dyn StdError>> {
///     // Emulate a scenario where content parsing fails
///     let err = ContentProcessingErrorBuilder::new()
///         .message("Invalid content structure")
///         .context("Failed to parse JSON")
///         .severity(ErrorSeverity::Error)
///         .build();
///
///     Err(Box::new(err))
/// }
/// ```
#[derive(Debug, Default)]
pub struct ContentProcessingErrorBuilder {
    message: Option<String>,
    source: Option<Box<dyn StdError + Send + Sync>>,
    additional_context: Vec<String>,
    severity: Option<ErrorSeverity>,
}

/// Represents the severity level of an error.
///
/// This enumeration allows developers to categorise errors by their
/// importance or impact, facilitating structured error handling and logging.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorSeverity {
    /// Informational messages that do not indicate a critical problem.
    Info,
    /// Problems that merit attention but do not obstruct operation.
    Warning,
    /// Serious errors that impede normal operation and may require intervention.
    Error,
    /// Critical failures that demand immediate action to restore functionality.
    Critical,
}

impl ContentProcessingErrorBuilder {
    /// Creates a new `ContentProcessingErrorBuilder` instance with default values.
    ///
    /// All fields are initially:
    /// - `message`: `None`
    /// - `source`: `None`
    /// - `additional_context`: empty vector
    /// - `severity`: `None`
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the primary error message.
    ///
    /// This message should concisely describe what went wrong.
    pub fn message(mut self, msg: impl Into<String>) -> Self {
        self.message = Some(msg.into());
        self
    }

    /// Sets the underlying source error, if any.
    ///
    /// Including the source error can help diagnose the root cause of a failure.
    /// This version allows callers to pass either a concrete type implementing
    /// `StdError + Send + Sync + 'static` or a `Box<dyn StdError + Send + Sync>`.
    pub fn source<T>(mut self, err: T) -> Self
    where
        T: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    {
        self.source = Some(err.into());
        self
    }

    /// Adds an additional context message describing the error scenario.
    ///
    /// You can call this method multiple times to append different context snippets.
    pub fn context(mut self, ctx: impl Into<String>) -> Self {
        self.additional_context.push(ctx.into());
        self
    }

    /// Sets the severity level of the error for more granular categorisation.
    ///
    /// Refer to [`ErrorSeverity`] for possible variants and their definitions.
    pub fn severity(mut self, severity: ErrorSeverity) -> Self {
        self.severity = Some(severity);
        self
    }

    /// Builds and returns a fully constructed [`Error::ContentProcessing`] variant.
    ///
    /// If no message has been set, a default of `"Unknown error"` is used. All context
    /// strings and severity indicators are appended to the final error message.
    pub fn build(self) -> Error {
        let mut message =
            self.message.unwrap_or_else(|| "Unknown error".to_string());

        // Consolidate additional context.
        if !self.additional_context.is_empty() {
            message = format!(
                "{} (Context: {})",
                message,
                self.additional_context.join(", ")
            );
        }

        // Display the severity if specified.
        if let Some(severity) = self.severity {
            message = format!("[{:?}] {}", severity, message);
        }

        Error::ContentProcessing {
            message,
            source: self.source,
        }
    }
}

/// Represents all possible errors within the **StaticDataGen** library.
///
/// Each variant addresses a distinct domain of failure, facilitating targeted
/// handling and user feedback. They range from configuration issues through
/// content processing errors to unhandled, custom exceptions.
#[derive(Debug, Error)]
pub enum Error {
    /// Indicates issues in configuration or missing options that prevent the
    /// library from functioning correctly.
    #[error("Configuration Error: {0}")]
    Config(String),

    /// Raised when a content processing step fails due to invalid data,
    /// unsupportable formats, or template-related misconfiguration.
    ///
    /// Includes an optional source to track deeper causes.
    #[error("Content Processing Error: {message}")]
    ContentProcessing {
        /// A concise, user-facing description of the error.
        message: String,
        /// Optional reference to an underlying cause for diagnostic purposes.
        source: Option<Box<dyn StdError + Send + Sync>>,
    },

    /// Denotes input/output errors, such as inaccessible files, permission
    /// denials, or filesystem-related failures. Embeds a context string
    /// detailing the operation in question.
    #[error("IO Error: {context} - {source}")]
    Io {
        /// Original I/O error providing low-level information.
        source: std::io::Error,
        /// Descriptive context to highlight the specific file or operation.
        context: String,
    },

    /// A catch-all variant for unexpected or specialised errors. Errors of this
    /// kind typically lack a more specific classification and will be stored
    /// as simple strings.
    #[error("Unhandled Error: {0}")]
    Other(String),

    /// Highlights issues arising from template creation, parsing, or rendering.
    /// Typically, this variant addresses syntax errors, unknown placeholders,
    /// or incomplete sections in a template.
    #[error("Template Error: {0}")]
    Template(String),
}

/// Builder for constructing [`Error::Io`] variants.
///
/// This builder offers a flexible approach to specifying I/O errors,
/// enabling you to supply custom context, operations, and file paths.
#[derive(Debug, Default)]
pub struct IoErrorBuilder {
    /// Holds the underlying I/O error.
    source: Option<std::io::Error>,
    /// A string capturing additional context describing the error scenario.
    context: Option<String>,
    /// Identifies the operation being performed (e.g., "Reading" or "Writing").
    operation: Option<String>,
    /// Specifies the file path involved in the erroneous operation.
    path: Option<String>,
}

impl IoErrorBuilder {
    /// Creates a new `IoErrorBuilder`, starting with no supplied fields.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the base I/O error (`source`) for this builder.
    pub fn source(mut self, err: std::io::Error) -> Self {
        self.source = Some(err);
        self
    }

    /// Supplies a custom contextual message to better explain the failure.
    pub fn context(mut self, ctx: impl Into<String>) -> Self {
        self.context = Some(ctx.into());
        self
    }

    /// Declares the operation being performed at the time of error.
    pub fn operation(mut self, op: impl Into<String>) -> Self {
        self.operation = Some(op.into());
        self
    }

    /// Specifies the file path or resource that caused the I/O error.
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    /// Conveniently sets both operation and path in a single method call.
    pub fn with_operation_and_path(
        mut self,
        operation: impl Into<String>,
        path: impl Into<String>,
    ) -> Self {
        self.operation = Some(operation.into());
        self.path = Some(path.into());
        self
    }

    /// Builds and returns a fully assembled [`Error::Io`] variant.
    ///
    /// If no `source` error was provided, a default `std::io::ErrorKind::Other` will be used.
    /// Similarly, if no meaningful context is found, a generic placeholder is supplied.
    pub fn build(self) -> Error {
        let source = self.source.unwrap_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unknown IO error",
            )
        });

        let mut context_parts = Vec::new();

        if let Some(op) = self.operation {
            context_parts.push(format!("Operation: {}", op));
        }
        if let Some(path) = self.path {
            context_parts.push(format!("Path: {}", path));
        }
        if let Some(ctx) = self.context {
            context_parts.push(ctx);
        }

        let context = if context_parts.is_empty() {
            "No additional context".to_string()
        } else {
            context_parts.join(" | ")
        };

        Error::Io { source, context }
    }
}

impl Error {
    /// Creates a new [`ContentProcessingErrorBuilder`] instance to construct
    /// [`Error::ContentProcessing`] variants more intuitively.
    ///
    /// # Examples
    ///
    /// ```
    /// use staticdatagen::{Error, ContentProcessingErrorBuilder};
    /// use std::error::Error as StdError;
    /// use staticdatagen::ErrorSeverity;
    ///
    /// fn process_data() -> Result<(), Box<dyn StdError>> {
    ///     // Simulate a processing error scenario
    ///     let err = Error::content_processing_builder()
    ///         .message("Inconsistent data format")
    ///         .context("JSON deserialisation failure")
    ///         .severity(ErrorSeverity::Error)
    ///         .build();
    ///
    ///     Err(Box::new(err))
    /// }
    /// ```
    pub fn content_processing_builder() -> ContentProcessingErrorBuilder
    {
        ContentProcessingErrorBuilder::new()
    }

    /// Creates a new [`IoErrorBuilder`] to construct [`Error::Io`] variants
    /// with detailed context.
    ///
    /// # Examples
    ///
    /// ```
    /// use staticdatagen::{Error, IoErrorBuilder};
    /// use std::io::{self, ErrorKind};
    ///
    /// fn example_io_operation() -> Result<(), Error> {
    ///     let io_err = io::Error::new(ErrorKind::NotFound, "file missing");
    ///     // Apply a custom context message:
    ///     Err(
    ///         Error::io_builder()
    ///             .source(io_err)
    ///             .context("Failed opening config file")
    ///             .build()
    ///     )
    /// }
    /// ```
    pub fn io_builder() -> IoErrorBuilder {
        IoErrorBuilder::new()
    }

    /// Constructs an [`Error::Io`] variant using a standard I/O error
    /// and a descriptive context string.
    ///
    /// This method is especially useful when needing to store extra details
    /// about the failing operation, such as file paths or function names.
    ///
    /// # Examples
    ///
    /// ```
    /// use staticdatagen::Error;
    /// use std::io::{self, ErrorKind};
    ///
    /// fn example_io_operation() -> Result<(), Error> {
    ///     let io_err = io::Error::new(ErrorKind::NotFound, "file missing");
    ///     // Provide a descriptive message for clarity:
    ///     Err(Error::io(io_err, "Failed to open configuration file"))
    /// }
    /// ```
    pub fn io(source: std::io::Error, context: impl ToString) -> Self {
        Error::Io {
            source,
            context: context.to_string(),
        }
    }

    /// Constructs an [`Error::ContentProcessing`] variant using a message and
    /// optionally a source error. This is typically raised when content
    /// transformation fails due to malformed data or an incompatible input format.
    ///
    /// # Examples
    ///
    /// ```
    /// use staticdatagen::{Error, Result};
    /// use std::error::Error as StdError;
    ///
    /// fn example_processing() -> Result<()> {
    ///     let root_cause: Option<Box<dyn StdError + Send + Sync>> = None;
    ///     // Summarise the error in a short message:
    ///     Err(Error::content_processing("Malformed content", root_cause))
    /// }
    /// ```
    pub fn content_processing(
        message: impl ToString,
        source: Option<Box<dyn StdError + Send + Sync>>,
    ) -> Self {
        Error::ContentProcessing {
            message: message.to_string(),
            source,
        }
    }
}

/// Converts a standard I/O error into an [`Error::Io`] variant, providing a
/// default context of `"Unexpected IO error"`. This trait enables the `?`
/// operator to automatically transform `std::io::Error` into `staticdatagen::Error`.
///
/// # Examples
///
/// ```
/// use staticdatagen::{Error, Result};
/// use std::fs::File;
///
/// fn read_file(path: &str) -> Result<String> {
///     // The `?` operator auto-converts `std::io::Error` into `Error::Io`.
///     let _file = File::open(path)?;
///     Ok("File read successfully.".into())
/// }
/// ```
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io {
            source: err,
            context: "Unexpected IO error".to_string(),
        }
    }
}

/// Converts a string slice (`&str`) into an [`Error::Other`] variant. This
/// capability allows quick creation of a basic error from a string literal.
///
/// # Examples
///
/// ```
/// use staticdatagen::Error;
///
/// fn example_conversion() -> Error {
///     // Creates an `Error::Other("Generic error")`
///     "Generic error".into()
/// }
/// ```
impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Error::Other(msg.to_string())
    }
}

/// Converts a `String` into an [`Error::Other`] variant. This enables the
/// creation of a generic error from a dynamically owned string.
///
/// # Examples
///
/// ```
/// use staticdatagen::Error;
///
/// fn example_conversion() -> Error {
///     // Produces an `Error::Other("Misc error")`
///     "Misc error".to_string().into()
/// }
/// ```
impl From<String> for Error {
    fn from(msg: String) -> Self {
        Error::Other(msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as StdError;
    use std::io;
    use std::io::ErrorKind;

    /// Verifies that the `VERSION` string is not empty.
    #[test]
    #[allow(clippy::const_is_empty)]
    fn test_version() {
        // Uses Clippy’s recommended check, but we allow `const_is_empty`.
        assert!(!VERSION.is_empty(), "VERSION should not be empty");
    }

    /// Ensures that the `VERSION` string appears to follow a semver-like format with a dot.
    #[test]
    fn test_version_format() {
        assert!(
            VERSION.contains('.'),
            "VERSION should use a dotted semver format"
        );
        assert!(
            VERSION.split('.').count() >= 2,
            "VERSION should have at least major.minor components"
        );
    }

    /// Checks conversion from `std::io::Error` to our crate's `Error::Io`.
    #[test]
    fn test_error_conversions() {
        let io_error =
            io::Error::new(ErrorKind::NotFound, "File not found");
        let error: Error = io_error.into();
        assert!(matches!(error, Error::Io { .. }));
    }

    /// Validates the `Display` implementation for different variants.
    #[test]
    fn test_error_display() {
        // ContentProcessing with no source
        let err = Error::ContentProcessing {
            message: "invalid content".to_string(),
            source: None,
        };
        assert_eq!(
            err.to_string(),
            "Content Processing Error: invalid content"
        );

        // Template
        let err = Error::Template("template error".to_string());
        assert_eq!(err.to_string(), "Template Error: template error");

        // Config
        let err = Error::Config("config error".to_string());
        assert_eq!(
            err.to_string(),
            "Configuration Error: config error"
        );
    }

    /// Confirms that Rust's `Debug` output reveals variant names and fields.
    #[test]
    fn test_error_debug() {
        let err = Error::Config("test".to_string());
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("Config"));
        assert!(debug_str.contains("test"));
    }

    /// Ensures `.source()` returns the underlying error if it exists.
    #[test]
    fn test_error_source() {
        // Test IO error source
        let io_err = io::Error::new(ErrorKind::Other, "test");
        let err = Error::Io {
            source: io_err,
            context: "test context".to_string(),
        };
        assert!(err.source().is_some());

        // Variants without an underlying source
        let err = Error::Config("test".to_string());
        assert!(err.source().is_none());

        let err = Error::ContentProcessing {
            message: "test".to_string(),
            source: None,
        };
        assert!(err.source().is_none());

        let err = Error::Template("test".to_string());
        assert!(err.source().is_none());
    }

    /// Ensures the `?` operator converts `std::io::Error` to `Error::Io`.
    #[test]
    fn test_error_from_io() {
        let io_err = io::Error::new(ErrorKind::Other, "io error");
        let err: Error = io_err.into();
        assert!(matches!(err, Error::Io { .. }));
    }

    /// Ensures the crate-specific `Result` type behaves as expected.
    #[test]
    fn test_result_type() {
        let result: Result<i32, Error> = Ok(42);
        assert!(result.is_ok());

        let err_result: Result<(), Error> =
            Err(Error::Config("test".into()));
        assert!(err_result.is_err());
    }

    /// Verifies that nested I/O errors have a `.source()` chain.
    #[test]
    fn test_error_chaining() {
        let io_err = io::Error::new(ErrorKind::Other, "base error");
        let err = Error::Io {
            source: io_err,
            context: "test".to_string(),
        };
        let mut count = 0;
        let mut source = err.source();
        while let Some(s) = source {
            count += 1;
            source = s.source();
        }
        assert_eq!(count, 1);
    }

    /// Checks that `VERSION` string can be split into multiple numeric components.
    #[test]
    fn test_version_components() {
        let parts: Vec<&str> = VERSION.split('.').collect();
        assert!(
            parts.len() >= 2,
            "VERSION should have at least major and minor components"
        );
        for part in parts {
            assert!(
                part.parse::<u32>().is_ok(),
                "Each version component should be a valid number"
            );
        }
    }

    /// Uses a regular expression to test whether `VERSION` follows a typical semver structure.
    #[test]
    fn test_version_semver_format() {
        let semver_regex = regex::Regex::new(
            r"^\d+\.\d+(\.\d+)?(-[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?(\+[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?$"
        ).unwrap();

        assert!(
            semver_regex.is_match(VERSION),
            "VERSION should follow semver conventions"
        );
    }

    /// Ensures multiple error variants produce non-empty display outputs and recognisable `Debug` formats.
    #[test]
    fn test_error_variants() {
        let errors = vec![
            Error::io(
                io::Error::new(ErrorKind::NotFound, "not found"),
                "File operation failed",
            ),
            Error::ContentProcessing {
                message: "test error".to_string(),
                source: None,
            },
            Error::Template("template error".to_string()),
            Error::Config("config error".to_string()),
        ];

        for error in errors {
            // Check that display message is non-empty
            assert!(
                !error.to_string().is_empty(),
                "All errors must have non-empty display messages"
            );

            // Check debug format includes the correct variant
            let debug_str = format!("{:?}", error);
            match error {
                Error::Io { .. } => assert!(debug_str.contains("Io")),
                Error::ContentProcessing { .. } => {
                    assert!(debug_str.contains("ContentProcessing"))
                }
                Error::Template(_) => {
                    assert!(debug_str.contains("Template"))
                }
                Error::Config(_) => {
                    assert!(debug_str.contains("Config"))
                }
                Error::Other(_) => {
                    // Optional: handle or log other variant
                }
            }
        }
    }

    /// Tests that I/O errors carry exactly one level of nesting in the cause chain.
    #[test]
    fn test_error_nesting() {
        let io_error = io::Error::new(ErrorKind::Other, "root cause");
        let error = Error::Io {
            source: io_error,
            context: "test context".to_string(),
        };

        let mut source_opt = error.source();
        let mut depth = 0;
        while let Some(s) = source_opt {
            depth += 1;
            source_opt = s.source();
        }
        assert_eq!(
            depth, 1,
            "I/O error should have a single level of nesting"
        );
    }

    /// Checks `From<std::io::Error>` conversion retains default context.
    #[test]
    fn test_error_conversion_chain() {
        let io_error = io::Error::new(ErrorKind::Other, "base error");
        let error: Error = io_error.into();

        match error {
            Error::Io { source, context } => {
                assert_eq!(source.kind(), ErrorKind::Other);
                assert!(
                    context.contains("Unexpected IO error")
                        || context.is_empty(),
                    "Context should match default or be handled"
                );
            }
            _ => panic!("Expected an IO error variant"),
        }
    }

    /// Demonstrates how `.map` and `.map_err` can be used on the crate's `Result` type.
    #[test]
    fn test_result_type_mapping() {
        let success: Result<i32, Error> = Ok(42);
        let doubled = success.map(|x| x * 2);
        assert_eq!(doubled.unwrap(), 84);

        let failure: Result<(), Error> =
            Err(Error::Config("test".into()));
        let mapped_err = failure.map_err(|e| format!("Wrapped: {}", e));
        assert!(mapped_err.is_err());
    }

    /// Illustrates that multiple `?` operators can chain errors in Rust, automatically
    /// converting them to the crate's `Error` type when the function’s signature uses it.
    #[test]
    fn test_result_type_composition() {
        fn inner_operation() -> io::Result<i32> {
            Err(io::Error::new(ErrorKind::Other, "inner error"))
        }

        fn outer_operation() -> Result<i32, io::Error> {
            let result = inner_operation()?;
            Ok(result)
        }

        assert!(outer_operation().is_err());
    }

    /// Validates correct formatting of stringified errors, especially with custom context.
    #[test]
    fn test_error_display_formatting() {
        let errors = [
            (
                Error::Io {
                    source: io::Error::new(
                        ErrorKind::NotFound,
                        "missing",
                    ),
                    context: "missing context".to_string(),
                },
                "IO Error: missing context - missing",
            ),
            (
                Error::ContentProcessing {
                    message: "invalid".to_string(),
                    source: None,
                },
                "Content Processing Error: invalid",
            ),
            (
                Error::Template("bad template".to_string()),
                "Template Error: bad template",
            ),
            (
                Error::Config("bad config".to_string()),
                "Configuration Error: bad config",
            ),
        ];

        for (error, expected) in errors.iter() {
            assert_eq!(error.to_string(), *expected);
        }
    }

    /// Ensures that both `From` and `Into` conversions yield `Error::Io`.
    #[test]
    fn test_error_conversions_again() {
        // `From` implementation
        let io_err = io::Error::new(ErrorKind::Other, "io error");
        let err: Error = Error::from(io_err);
        assert!(matches!(err, Error::Io { .. }));

        // `Into` implementation
        let io_err2 = io::Error::new(ErrorKind::Other, "io error2");
        let err2: Error = io_err2.into();
        assert!(matches!(err2, Error::Io { .. }));
    }

    /// Asserts that our custom `Error` type is `Send + Sync`, allowing it
    /// to be safely transferred across thread boundaries in concurrent contexts.
    #[test]
    fn test_error_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Error>();
    }

    /// Checks that error messages remain consistent across various
    /// variants with basic string data.
    #[test]
    fn test_error_message_consistency() {
        let msg = "test message";
        let errors = [
            (
                Error::ContentProcessing {
                    message: msg.into(),
                    source: None,
                },
                "Content Processing Error: test message",
            ),
            (
                Error::Template(msg.into()),
                "Template Error: test message",
            ),
            (
                Error::Config(msg.into()),
                "Configuration Error: test message",
            ),
        ];

        for (error, expected) in errors {
            assert_eq!(error.to_string(), expected);
        }
    }

    /// Demonstrates usage of `map` and `map_err` on our `Result` type.
    #[test]
    fn test_result_combinators() {
        let mut ok_result: Result<_, Error> = Ok(42);
        let err_result = Error::Config("test".into());

        assert_eq!(*ok_result.as_mut().unwrap_or(&mut 0), 42);
        assert_eq!(err_result.to_string(), "Configuration Error: test");

        let mapped = ok_result.map(|n| n * 2);
        assert_eq!(mapped.unwrap(), 84);
    }

    /// Tests downcasting capabilities, ensuring the `Error` type implements `StdError`.
    #[test]
    fn test_error_downcasting() {
        let io_err = io::Error::new(ErrorKind::Other, "test");
        let err = Error::Io {
            source: io_err,
            context: "test context".to_string(),
        };

        let std_error: &dyn StdError = &err;
        assert!(std_error.downcast_ref::<Error>().is_some());
    }

    /// Confirms that a constructed I/O error retains its original message
    /// upon inspection or logging.
    #[test]
    fn test_io_error_cloning() {
        let io_err = io::Error::new(ErrorKind::Other, "original");
        let err = Error::Io {
            source: io_err,
            context: "some context".to_string(),
        };

        let err_string = err.to_string();
        assert!(err_string.contains("original"));
    }

    /// Demonstrates pattern matching over multiple `Error` variants.
    #[test]
    fn test_error_pattern_matching() {
        let errors = vec![
            Error::Config("config".into()),
            Error::Template("template".into()),
            Error::ContentProcessing {
                message: "content".into(),
                source: None,
            },
            Error::Io {
                source: io::Error::new(ErrorKind::Other, "io"),
                context: "some context".to_string(),
            },
        ];

        let mut counts = (0, 0, 0, 0);
        for err in errors {
            match err {
                Error::Config(_) => counts.0 += 1,
                Error::Template(_) => counts.1 += 1,
                Error::ContentProcessing { .. } => counts.2 += 1,
                Error::Io { .. } => counts.3 += 1,
                Error::Other(msg) => {
                    // If you do not need the message, do nothing
                    println!("Other error: {}", msg);
                }
            }
        }

        assert_eq!(counts, (1, 1, 1, 1));
    }

    /// Ensures that any context string in an I/O error is visible within the final
    /// display, aiding debugging or logging.
    #[test]
    fn test_error_context_preservation() {
        let context = "important context";
        let io_err = io::Error::new(ErrorKind::Other, context);
        let err = Error::Io {
            source: io_err,
            context: context.to_string(),
        };

        assert!(err.to_string().contains(context));
    }

    /// Checks conversion to `Error::ContentProcessing` when given a string or &str input.
    #[test]
    fn test_custom_error_conversion() {
        let err = Error::ContentProcessing {
            message: "test".to_string(),
            source: None,
        };
        assert_eq!(err.to_string(), "Content Processing Error: test");

        let err = Error::ContentProcessing {
            message: "test".into(),
            source: None,
        };
        assert_eq!(err.to_string(), "Content Processing Error: test");
    }

    /// Verifies behaviour when error messages are empty strings, ensuring no panics occur.
    #[test]
    fn test_error_empty_messages() {
        let err = Error::Config(String::new());
        assert_eq!(err.to_string(), "Configuration Error: ");

        let err = Error::Template(String::new());
        assert_eq!(err.to_string(), "Template Error: ");
    }

    /// Illustrates how errors can be composed and layered, wrapping one variant
    /// inside another to show a chain of failures.
    #[test]
    fn test_error_composition() {
        fn fallible_operation() -> Result<(), Error> {
            Err(Error::Config("inner error".into()))
        }

        let result = fallible_operation().map_err(|e| {
            Error::ContentProcessing {
                message: format!("outer: {}", e),
                source: None,
            }
        });

        assert!(matches!(result, Err(Error::ContentProcessing { .. })));
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("inner error"));
    }

    /// Checks that the `ContentProcessingErrorBuilder` correctly uses a default
    /// error message when none is provided.
    #[test]
    fn test_content_processing_error_builder_minimal() {
        let builder = ContentProcessingErrorBuilder::new();
        let error = builder.build();

        match error {
            Error::ContentProcessing { message, source } => {
                assert_eq!(message, "Unknown error");
                assert!(source.is_none(), "Expected no source error");
            }
            _ => panic!("Expected a ContentProcessing error"),
        }
    }

    /// Tests that providing only a message (without source, severity, or context)
    /// results in an appropriate `ContentProcessing` error.
    #[test]
    fn test_content_processing_error_builder_message_only() {
        let error = ContentProcessingErrorBuilder::new()
            .message("Just a message")
            .build();

        match error {
            Error::ContentProcessing { message, source } => {
                assert_eq!(message, "Just a message");
                assert!(source.is_none(), "No source should be set");
            }
            _ => panic!("Expected a ContentProcessing error"),
        }
    }

    /// Verifies that multiple context lines are concatenated correctly in the final message.
    #[test]
    fn test_content_processing_error_builder_multiple_context() {
        let error = ContentProcessingErrorBuilder::new()
            .message("Message with multiple contexts")
            .context("Context A")
            .context("Context B")
            .build();

        match error {
            Error::ContentProcessing { message, source } => {
                assert!(
                    message.contains("Context A")
                        && message.contains("Context B"),
                    "Expected both context parts in the error message"
                );
                assert!(source.is_none());
            }
            _ => panic!("Expected a ContentProcessing error"),
        }
    }

    /// Ensures that setting an `ErrorSeverity` properly prefixes the message in the final error.
    #[test]
    fn test_content_processing_error_builder_with_severity() {
        let error = ContentProcessingErrorBuilder::new()
            .message("A critical issue")
            .severity(ErrorSeverity::Critical)
            .build();

        match error {
            Error::ContentProcessing { message, source } => {
                assert!(
                    message.starts_with("[Critical] "),
                    "Expected severity prefix in the error message"
                );
                assert!(source.is_none());
            }
            _ => panic!("Expected a ContentProcessing error"),
        }
    }

    /// Checks behaviour when a source error is set in the `ContentProcessingErrorBuilder`.
    #[test]
    fn test_content_processing_error_builder_with_source() {
        let source_err: Box<dyn StdError + Send + Sync> = Box::new(
            Error::Config("Underlying config issue".to_string()),
        );
        let error = ContentProcessingErrorBuilder::new()
            .message("Top-level content error")
            .source(source_err)
            .build();

        match error {
            Error::ContentProcessing { message, source } => {
                assert_eq!(message, "Top-level content error");
                assert!(
                    source.is_some(),
                    "Expected an underlying source error"
                );
            }
            _ => panic!("Expected a ContentProcessing error"),
        }
    }

    /// Ensures that the `IoErrorBuilder` defaults to an "Unknown IO error" when
    /// no source is provided.
    #[test]
    fn test_io_error_builder_minimal() {
        let builder = IoErrorBuilder::new();
        let error = builder.build();

        match error {
            Error::Io { source, context } => {
                assert_eq!(source.kind(), ErrorKind::Other);
                assert_eq!(source.to_string(), "Unknown IO error");
                assert_eq!(
                    context, "No additional context",
                    "Expected a placeholder context"
                );
            }
            _ => panic!("Expected an Io error variant"),
        }
    }

    /// Tests providing only an operation and path, leaving out source or extra context.
    #[test]
    fn test_io_error_builder_partial() {
        let builder = IoErrorBuilder::new()
            .operation("Reading")
            .path("/tmp/config.json");
        let error = builder.build();

        match error {
            Error::Io { source, context } => {
                assert_eq!(source.kind(), ErrorKind::Other);
                assert_eq!(source.to_string(), "Unknown IO error");
                assert!(
                    context.contains("Operation: Reading"),
                    "Context should note the operation"
                );
                assert!(
                    context.contains("Path: /tmp/config.json"),
                    "Context should note the path"
                );
            }
            _ => panic!("Expected an Io error variant"),
        }
    }

    /// Demonstrates a fully specified IoErrorBuilder usage:
    /// operation, path, context, and a custom source.
    #[test]
    fn test_io_error_builder_full() {
        let io_err = io::Error::new(
            ErrorKind::PermissionDenied,
            "Permission denied",
        );
        let builder = IoErrorBuilder::new()
            .source(io_err)
            .operation("Writing")
            .path("/var/log/app.log")
            .context("Failed due to insufficient permissions");

        let error = builder.build();

        match error {
            Error::Io { source, context } => {
                assert_eq!(source.kind(), ErrorKind::PermissionDenied);
                assert!(context.contains("Operation: Writing"));
                assert!(context.contains("Path: /var/log/app.log"));
                assert!(context.contains(
                    "Failed due to insufficient permissions"
                ));
            }
            _ => panic!("Expected an Io error variant"),
        }
    }

    /// Validates the `content_processing` function with a source.
    #[test]
    fn test_error_content_processing_with_source() {
        let src_err: Box<dyn StdError + Send + Sync> =
            Box::new(io::Error::new(ErrorKind::Other, "Root cause"));
        let err = Error::content_processing(
            "Top-level content error",
            Some(src_err),
        );

        match err {
            Error::ContentProcessing { message, source } => {
                assert_eq!(message, "Top-level content error");
                assert!(source.is_some());
            }
            _ => panic!("Expected ContentProcessing error"),
        }
    }

    /// Validates the `io` function for constructing an `Error::Io` from a source and context.
    #[test]
    fn test_error_io_function() {
        let io_err = io::Error::new(
            ErrorKind::AlreadyExists,
            "File already exists",
        );
        let err = Error::io(io_err, "Could not create the file");

        match err {
            Error::Io { source, context } => {
                assert_eq!(source.kind(), ErrorKind::AlreadyExists);
                assert_eq!(context, "Could not create the file");
            }
            _ => panic!("Expected Io error variant"),
        }
    }

    /// Covers the scenario where multiple context strings are joined,
    /// triggering `"{} (Context: {})"` code in `ContentProcessingErrorBuilder::build`.
    #[test]
    fn test_content_processing_error_context_merge() {
        let error = ContentProcessingErrorBuilder::new()
            .message("Merge test")
            .context("ctx1")
            .context("ctx2")
            .build();

        match error {
            Error::ContentProcessing { message, source } => {
                // Checking that both context parts are present in the final message.
                assert!(
                    message.contains("ctx1") && message.contains("ctx2"),
                    "Merged context strings should appear in final error message"
                );
                assert!(source.is_none());
            }
            _ => panic!("Expected an Error::ContentProcessing variant"),
        }
    }

    /// Exercises the `IoErrorBuilder::with_operation_and_path` method to ensure
    /// both fields are set correctly.
    #[test]
    fn test_io_error_builder_with_operation_and_path() {
        let io_err = io::Error::new(ErrorKind::Other, "some io error");
        let error = IoErrorBuilder::new()
            .source(io_err)
            .with_operation_and_path("Reading file", "/some/path")
            .build();

        match error {
            Error::Io { source, context } => {
                assert_eq!(source.kind(), ErrorKind::Other);
                assert!(context.contains("Operation: Reading file"));
                assert!(context.contains("Path: /some/path"));
            }
            _ => panic!("Expected an Error::Io variant"),
        }
    }

    /// Verifies that `Error::content_processing_builder()` produces a default
    /// `ContentProcessingErrorBuilder`.
    #[test]
    fn test_error_content_processing_builder() {
        // This calls the static method to ensure coverage.
        let builder = Error::content_processing_builder();
        let error = builder.build();
        match error {
            Error::ContentProcessing { .. } => {
                // We expect a default "Unknown error" message due to no `message` set.
            }
            _ => panic!("Expected an Error::ContentProcessing"),
        }
    }

    /// Checks that `Error::io_builder()` returns a fresh `IoErrorBuilder`.
    #[test]
    fn test_error_io_builder() {
        // This confirms coverage for the `io_builder` function.
        let builder = Error::io_builder();
        let error = builder.build();
        match error {
            Error::Io { context, source } => {
                assert_eq!(
                    source.to_string(),
                    "Unknown IO error",
                    "No source was set, so expect a default"
                );
                assert_eq!(
                    context, "No additional context",
                    "No context was provided, so expect a placeholder"
                );
            }
            _ => panic!("Expected an Error::Io variant"),
        }
    }

    /// Ensures coverage for `impl From<&str> for Error`.
    #[test]
    fn test_error_from_str() {
        let err: Error = "some str-based error".into();
        match err {
            Error::Other(msg) => {
                assert_eq!(msg, "some str-based error");
            }
            _ => panic!("Expected an Error::Other variant"),
        }
    }

    /// Ensures coverage for `impl From<String> for Error`.
    #[test]
    fn test_error_from_string() {
        let err: Error = "some String-based error".to_string().into();
        match err {
            Error::Other(msg) => {
                assert_eq!(msg, "some String-based error");
            }
            _ => panic!("Expected an Error::Other variant"),
        }
    }
}
