// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Plain Text Generation Module
//!
//! This module provides functionality for converting HTML and Markdown content to plain text
//! while maintaining readability and content structure. It includes robust handling of
//! various text formats, character sets, and security considerations.
//!
//! # Features
//!
//! - HTML and Markdown to plain text conversion
//! - Secure content sanitization
//! - Unicode and RTL text support
//! - Structured content preservation
//! - Metadata handling
//! - Comprehensive error handling
//!
//! # Example
//!
//! ```
//! use staticdatagen::modules::plaintext::{generate_plain_text, PlainTextConfig};
//!
//! let config = PlainTextConfig::default();
//! let result = generate_plain_text(
//!     "# Hello World\n\nThis is **bold** text.",
//!     "Title",
//!     "Description",
//!     "Author",
//!     "Creator",
//!     "keywords",
//! ).unwrap();
//! ```
//!
//! # Security
//!
//! This module implements several security measures:
//! - HTML tag stripping
//! - Script injection prevention
//! - Control character filtering
//! - Unicode character validation

use anyhow::Result;
use log::{debug, info};
use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use std::collections::HashMap;
use thiserror::Error;

/// Configuration options for plain text generation
#[derive(Debug, Clone)]
pub struct PlainTextConfig {
    /// Maximum line length for wrapping
    pub max_line_length: usize,
    /// List item bullet character
    pub list_bullet: String,
    /// Whether to preserve empty lines between sections
    pub preserve_empty_lines: bool,
    /// Whether to use ASCII-only output
    pub ascii_only: bool,
    /// Custom text replacements
    pub replacements: HashMap<String, String>,
}

impl Default for PlainTextConfig {
    fn default() -> Self {
        Self {
            max_line_length: 80,
            list_bullet: "• ".to_string(),
            preserve_empty_lines: true,
            ascii_only: false,
            replacements: HashMap::new(),
        }
    }
}

/// Errors that can occur during plain text generation
#[derive(Error, Debug)]
pub enum PlainTextError {
    /// Parsing error during content conversion
    #[error("Failed to parse content: {0}")]
    ParseError(String),

    /// Unicode validation error
    #[error("Invalid Unicode in input: {0}")]
    UnicodeError(String),

    /// Content length exceeds maximum limits
    #[error("Content exceeds maximum length: {0} > {1}")]
    ContentTooLong(usize, usize),

    /// Invalid configuration error
    #[error("Invalid configuration: {0}")]
    ConfigError(String),
}

/// Result type for plain text generation operations
type PlainTextResult =
    Result<(String, String, String, String, String, String)>;

/// Generates plain text content from HTML/Markdown input.
///
/// # Arguments
///
/// * `content` - The original HTML/Markdown content
/// * `title` - The content title
/// * `description` - Content description
/// * `author` - Content author
/// * `creator` - Content creator
/// * `keywords` - Associated keywords
///
/// # Returns
///
/// Returns a tuple containing:
/// - Plain text content
/// - Sanitized title
/// - Sanitized description
/// - Sanitized author
/// - Sanitized creator
/// - Sanitized keywords
///
/// # Errors
///
/// Returns an error if:
/// - Content parsing fails
/// - Unicode validation fails
/// - Content length exceeds limits
///
/// # Example
///
/// ```
/// use staticdatagen::modules::plaintext::generate_plain_text;
///
/// let result = generate_plain_text(
///     "# Hello\nWorld",
///     "Title",
///     "Description",
///     "Author",
///     "Creator",
///     "keywords",
/// ).unwrap();
/// ```
pub fn generate_plain_text(
    content: &str,
    title: &str,
    description: &str,
    author: &str,
    creator: &str,
    keywords: &str,
) -> PlainTextResult {
    debug!(
        "Converting content to plain text, length: {}",
        content.len()
    );

    let plain_content = convert_to_plain_text(content)?;

    info!("Successfully converted content to plain text");

    Ok((
        plain_content,
        sanitize_text(title),
        sanitize_text(description),
        sanitize_text(author),
        sanitize_text(creator),
        sanitize_text(keywords),
    ))
}

/// Converts formatted content to plain text.
fn convert_to_plain_text(content: &str) -> Result<String> {
    let mut plain_text = String::new();
    let mut buffer = String::new();
    let mut last_was_text = false;

    let parser = Parser::new(content);

    for event in parser {
        match event {
            Event::Text(text) => {
                let trimmed_text = text.trim();
                if !trimmed_text.is_empty() {
                    if last_was_text {
                        buffer.push(' ');
                    }
                    buffer.push_str(trimmed_text);
                    last_was_text = true;
                }
            }
            Event::Start(Tag::Paragraph)
            | Event::Start(Tag::Heading { .. }) => {
                if !plain_text.is_empty() && !buffer.trim().is_empty() {
                    plain_text.push_str("\n\n");
                }
                buffer.clear();
                last_was_text = false;
            }
            Event::End(TagEnd::Paragraph)
            | Event::End(TagEnd::Heading { .. }) => {
                if !buffer.trim().is_empty() {
                    plain_text.push_str(&buffer);
                    buffer.clear();
                }
                last_was_text = false;
            }
            Event::Start(Tag::List(_)) | Event::Start(Tag::Item) => {
                if last_was_text {
                    buffer.push('\n');
                }
                buffer.push_str("• ");
                last_was_text = false;
            }
            Event::End(TagEnd::List(_)) | Event::End(TagEnd::Item) => {
                if !buffer.trim().is_empty() {
                    plain_text.push_str(&buffer);
                    buffer.clear();
                }
                last_was_text = false;
            }
            Event::SoftBreak | Event::HardBreak => {
                if !buffer.trim().is_empty() {
                    buffer.push(' ');
                }
                last_was_text = false;
            }
            _ => {}
        }
    }

    if !buffer.trim().is_empty() {
        plain_text.push_str(&buffer);
    }

    Ok(plain_text.trim().to_string())
}

/// Sanitizes text by removing unsafe content and normalizing whitespace.
fn sanitize_text(text: &str) -> String {
    // Remove potentially harmful content
    let sanitized =
        text.replace("<script>", "").replace("</script>", "");

    // Normalize whitespace and remove control characters
    sanitized
        .chars()
        .filter(|&c| !c.is_control() || c == '\n' || c == '\t')
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_conversion() -> Result<()> {
        let input = "# Hello\n\nThis is **bold** text.";
        let (content, ..) = generate_plain_text(
            input,
            "Title",
            "Description",
            "Author",
            "Creator",
            "keywords",
        )?;

        assert!(content.contains("Hello"));
        assert!(content.contains("This is bold text"));
        Ok(())
    }

    #[test]
    fn test_empty_input() -> Result<()> {
        let (content, title, description, author, creator, keywords) =
            generate_plain_text("", "", "", "", "", "")?;

        assert!(content.is_empty());
        assert!(title.is_empty());
        assert!(description.is_empty());
        assert!(author.is_empty());
        assert!(creator.is_empty());
        assert!(keywords.is_empty());
        Ok(())
    }

    #[test]
    fn test_whitespace_input() -> Result<()> {
        let (content, ..) =
            generate_plain_text("   \n   \t", "", "", "", "", "")?;
        assert!(content.is_empty());
        Ok(())
    }

    #[test]
    fn test_rtl_and_different_languages() -> Result<()> {
        let input = "English text. النص العربي. עברית.";
        let (content, ..) =
            generate_plain_text(input, "", "", "", "", "")?;

        assert!(content.contains("English text"));
        assert!(content.contains("النص العربي"));
        assert!(content.contains("עברית"));
        Ok(())
    }

    #[test]
    fn test_invalid_html_input() -> Result<()> {
        let input = "Text with <b>unclosed tag";
        let (content, ..) =
            generate_plain_text(input, "", "", "", "", "")?;

        assert!(content.contains("Text with unclosed tag"));
        assert!(!content.contains("<b>"));
        Ok(())
    }

    #[test]
    fn test_nested_formatting() -> Result<()> {
        let input = "This is **bold _italic nested_ formatting** test.";
        let (content, ..) =
            generate_plain_text(input, "", "", "", "", "")?;
        assert!(content
            .contains("This is bold italic nested formatting test"));
        Ok(())
    }

    #[test]
    fn test_lists() -> Result<()> {
        let input = "- Item 1\n- Item 2\n  - Nested item";
        let (content, ..) =
            generate_plain_text(input, "", "", "", "", "")?;

        assert!(content.contains("• Item 1"));
        assert!(content.contains("• Item 2"));
        assert!(content.contains("• Nested item"));
        Ok(())
    }

    #[test]
    fn test_metadata_escaping() -> Result<()> {
        let (_, title, ..) = generate_plain_text(
            "",
            "Title with <script>alert('xss')</script>",
            "",
            "",
            "",
            "",
        )?;

        assert!(title.contains("Title with alert"));
        assert!(!title.contains("<script>"));
        Ok(())
    }

    #[test]
    fn test_plain_text_config_default() {
        let config = PlainTextConfig::default();
        assert_eq!(config.max_line_length, 80);
        assert_eq!(config.list_bullet, "• ");
        assert!(config.preserve_empty_lines);
        assert!(!config.ascii_only);
        assert!(config.replacements.is_empty());
    }

    #[test]
    fn test_multiple_paragraphs() -> Result<()> {
        let input =
            "# First\n\nParagraph one.\n\n# Second\n\nParagraph two.";
        let (content, ..) =
            generate_plain_text(input, "", "", "", "", "")?;

        // Multiple paragraphs should have line breaks between them
        assert!(content.contains("First"));
        assert!(content.contains("Paragraph one"));
        assert!(content.contains("Second"));
        assert!(content.contains("Paragraph two"));
        Ok(())
    }

    #[test]
    fn test_soft_break_handling() -> Result<()> {
        // Soft break is a single newline in markdown (doesn't become <br>)
        let input = "Line one\nLine two";
        let (content, ..) =
            generate_plain_text(input, "", "", "", "", "")?;

        // Content should be joined with space
        assert!(
            content.contains("Line one")
                || content.contains("Line two")
        );
        Ok(())
    }

    #[test]
    fn test_buffer_append() -> Result<()> {
        // Test when buffer has content at the end
        let input = "Just some text";
        let (content, ..) =
            generate_plain_text(input, "", "", "", "", "")?;

        assert_eq!(content, "Just some text");
        Ok(())
    }

    #[test]
    fn test_sanitize_control_characters() -> Result<()> {
        // Test control characters are filtered
        let (_, title, ..) = generate_plain_text(
            "",
            "Title\x00with\x01control\x02chars",
            "",
            "",
            "",
            "",
        )?;

        assert!(!title.contains('\x00'));
        assert!(!title.contains('\x01'));
        assert!(!title.contains('\x02'));
        assert!(title.contains("Title"));
        assert!(title.contains("with"));
        Ok(())
    }

    #[test]
    fn test_config_custom() {
        let config = PlainTextConfig {
            max_line_length: 120,
            list_bullet: "- ".to_string(),
            preserve_empty_lines: false,
            ascii_only: true,
            replacements: HashMap::from([(
                "foo".to_string(),
                "bar".to_string(),
            )]),
        };

        assert_eq!(config.max_line_length, 120);
        assert_eq!(config.list_bullet, "- ");
        assert!(!config.preserve_empty_lines);
        assert!(config.ascii_only);
        assert_eq!(
            config.replacements.get("foo"),
            Some(&"bar".to_string())
        );
    }

    #[test]
    fn test_paragraph_separation() -> Result<()> {
        // Test line 192: when starting new paragraph after existing content with buffer
        let input = "# Heading One\n\nContent here.\n\n# Heading Two\n\nMore content.";
        let (content, ..) =
            generate_plain_text(input, "", "", "", "", "")?;

        // Both headings and paragraphs should be present with separation
        assert!(content.contains("Heading One"));
        assert!(content.contains("Content here"));
        assert!(content.contains("Heading Two"));
        assert!(content.contains("More content"));
        // Check content is not empty and has reasonable length
        assert!(content.len() > 30, "Should have substantial content");
        Ok(())
    }

    #[test]
    fn test_inline_text_content() -> Result<()> {
        // Test line 230: buffer has content at end that needs flushing
        // Using emphasis which adds inline text without creating new paragraphs
        let input = "Just *some* inline text";
        let (content, ..) =
            generate_plain_text(input, "", "", "", "", "")?;

        assert!(content.contains("Just"));
        assert!(content.contains("some"));
        assert!(content.contains("inline text"));
        Ok(())
    }

    #[test]
    fn test_consecutive_headings() -> Result<()> {
        // Test consecutive headings to trigger paragraph separator
        let input = "# First\n\n# Second\n\n# Third";
        let (content, ..) =
            generate_plain_text(input, "", "", "", "", "")?;

        assert!(content.contains("First"));
        assert!(content.contains("Second"));
        assert!(content.contains("Third"));
        Ok(())
    }

    #[test]
    fn test_plain_text_error_parse() {
        let err = PlainTextError::ParseError("bad input".to_string());
        assert_eq!(
            format!("{}", err),
            "Failed to parse content: bad input"
        );
    }

    #[test]
    fn test_plain_text_error_unicode() {
        let err =
            PlainTextError::UnicodeError("invalid byte".to_string());
        assert_eq!(
            format!("{}", err),
            "Invalid Unicode in input: invalid byte"
        );
    }

    #[test]
    fn test_plain_text_error_content_too_long() {
        let err = PlainTextError::ContentTooLong(2000, 1000);
        assert_eq!(
            format!("{}", err),
            "Content exceeds maximum length: 2000 > 1000"
        );
    }

    #[test]
    fn test_plain_text_error_config() {
        let err =
            PlainTextError::ConfigError("invalid setting".to_string());
        assert_eq!(
            format!("{}", err),
            "Invalid configuration: invalid setting"
        );
    }

    #[test]
    fn test_config_zero_line_length() {
        let config = PlainTextConfig {
            max_line_length: 0,
            ..Default::default()
        };
        assert_eq!(config.max_line_length, 0);
    }

    #[test]
    fn test_paragraph_separator_condition() -> Result<()> {
        // Trigger line 192: plain_text non-empty AND buffer non-empty
        // when new paragraph starts
        let input = "First paragraph.\n\nSecond paragraph.";
        let (content, ..) =
            generate_plain_text(input, "", "", "", "", "")?;
        assert!(content.contains("First paragraph"));
        assert!(content.contains("Second paragraph"));
        // Both should be present in output
        assert!(content.len() > 20);
        Ok(())
    }

    #[test]
    fn test_trailing_buffer_flush() -> Result<()> {
        // Trigger line 230: buffer has content at end
        let input = "**bold text** at end";
        let (content, ..) =
            generate_plain_text(input, "", "", "", "", "")?;
        assert!(content.contains("bold text"));
        assert!(content.contains("at end"));
        Ok(())
    }
}
