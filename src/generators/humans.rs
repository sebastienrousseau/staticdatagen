// Copyright © 2024 StaticDataGen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Humans.txt Generation Module
//!
//! This module provides functionality for generating `humans.txt` files, which
//! document the people and technologies behind a website.
//!
//! ## Features
//! - **Metadata Conversion**: Create `humans.txt` data from structured metadata.
//! - **Customizable Output**: Supports flexible formatting for team, site, and thanks sections.
//! - **Validation & Sanitization**: Ensures input data adheres to length and format constraints.
//!
//! ## Example Usage
//! ```rust
//! use std::collections::HashMap;
//! use staticdatagen::generators::humans::{HumansConfig, HumansGenerator};
//!
//! let mut metadata = HashMap::new();
//! metadata.insert("author".to_string(), "John Doe".to_string());
//! metadata.insert("author_website".to_string(), "https://example.com".to_string());
//!
//! let config = HumansConfig::from_metadata(&metadata).unwrap(); // Added unwrap
//! let generator = HumansGenerator::new(config);
//! let humans_content = generator.generate();
//!
//! assert!(humans_content.contains("John Doe"));
//! ```

use dtt::dtt_parse;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

/// Maximum length for text fields
const MAX_TEXT_LENGTH: usize = 100;

/// ## Errors in Humans.txt Generation
///
/// Represents various errors that may occur during humans.txt generation and validation.
#[derive(Debug, thiserror::Error)]
pub enum HumansError {
    /// Invalid input for a specific field.
    #[error("Invalid input for field '{field}': {message}")]
    InvalidInput {
        /// Field name where the error occurred.
        field: String,
        /// Message explaining the error.
        message: String,
    },
    /// Missing required metadata.
    #[error("Missing required metadata field: {0}")]
    MissingMetadata(String),
    #[error("Invalid date format: {0}")]
    /// Invalid date format
    InvalidDate(String),
    /// Invalid URL format
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
}

/// ## Humans Configuration
///
/// Configuration for generating `humans.txt`, extracted from metadata.
///
/// Provides fields for team members, site details, and acknowledgments.
#[derive(
    Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default,
)]
pub struct HumansConfig {
    /// Author name.
    pub author: String,
    /// Author's website.
    pub author_website: String,
    /// Author's Twitter handle.
    pub author_twitter: String,
    /// Author's location.
    pub author_location: String,
    /// Site components or technologies used.
    pub site_components: String,
    /// Last update date for the site.
    pub site_last_updated: String,
    /// Web standards followed.
    pub site_standards: String,
    /// Software or platform used.
    pub site_software: String,
    /// Acknowledgments or credits.
    pub thanks: String,
}

/// ## Humans Configuration Builder
#[derive(Default, Debug)]
pub struct HumansConfigBuilder {
    config: HumansConfig,
}

impl HumansConfigBuilder {
    /// Creates a new builder instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the author name
    pub fn author<S: Into<String>>(mut self, author: S) -> Self {
        self.config.author = sanitize_text(&author.into());
        self
    }

    /// Sets the author website
    pub fn author_website<S: Into<String>>(
        mut self,
        website: S,
    ) -> Result<Self, HumansError> {
        self.config.author_website = sanitize_url(&website.into())?;
        Ok(self)
    }

    /// Sets the author Twitter handle
    pub fn author_twitter<S: Into<String>>(
        mut self,
        twitter: S,
    ) -> Self {
        self.config.author_twitter =
            sanitize_twitter_handle(&twitter.into());
        self
    }

    /// Sets the author location
    pub fn author_location<S: Into<String>>(
        mut self,
        location: S,
    ) -> Self {
        self.config.author_location = sanitize_text(&location.into());
        self
    }

    /// Sets the site components
    pub fn site_components<S: Into<String>>(
        mut self,
        components: S,
    ) -> Self {
        self.config.site_components = sanitize_text(&components.into());
        self
    }

    /// Sets the site's last update date
    pub fn site_last_updated<S: Into<String>>(
        mut self,
        date: S,
    ) -> Result<Self, HumansError> {
        self.config.site_last_updated = sanitize_date(&date.into())?;
        Ok(self)
    }

    /// Sets the site standards
    pub fn site_standards<S: Into<String>>(
        mut self,
        standards: S,
    ) -> Self {
        self.config.site_standards = sanitize_text(&standards.into());
        self
    }

    /// Sets the site software
    pub fn site_software<S: Into<String>>(
        mut self,
        software: S,
    ) -> Self {
        self.config.site_software = sanitize_text(&software.into());
        self
    }

    /// Sets the thanks section
    pub fn thanks<S: Into<String>>(mut self, thanks: S) -> Self {
        self.config.thanks = sanitize_text(&thanks.into());
        self
    }

    /// Builds the configuration
    pub fn build(self) -> Result<HumansConfig, HumansError> {
        if self.config.author.trim().is_empty() {
            return Err(HumansError::MissingMetadata(
                "author".to_string(),
            ));
        }
        Ok(self.config)
    }
}

impl HumansConfig {
    /// Creates a new builder instance
    pub fn builder() -> HumansConfigBuilder {
        HumansConfigBuilder::new()
    }

    /// Creates a new `HumansConfig` from structured metadata
    pub fn from_metadata(
        metadata: &HashMap<String, String>,
    ) -> Result<Self, HumansError> {
        let mut builder = Self::builder();

        if let Some(author) = metadata.get("author") {
            builder = builder.author(author);
        }
        if let Some(website) = metadata.get("author_website") {
            builder = builder.author_website(website)?;
        }
        if let Some(twitter) = metadata.get("author_twitter") {
            builder = builder.author_twitter(twitter);
        }
        if let Some(location) = metadata.get("author_location") {
            builder = builder.author_location(location);
        }
        if let Some(components) = metadata.get("site_components") {
            builder = builder.site_components(components);
        }
        if let Some(date) = metadata.get("site_last_updated") {
            builder = builder.site_last_updated(date)?;
        }
        if let Some(standards) = metadata.get("site_standards") {
            builder = builder.site_standards(standards);
        }
        if let Some(software) = metadata.get("site_software") {
            builder = builder.site_software(software);
        }
        if let Some(thanks) = metadata.get("thanks") {
            builder = builder.thanks(thanks);
        }

        builder.build()
    }
}

/// ## Humans Generator
///
/// Generates the content of a `humans.txt` file based on the provided configuration.
#[derive(Debug)]
pub struct HumansGenerator {
    /// Configuration for generating `humans.txt`.
    pub config: HumansConfig,
}

impl HumansGenerator {
    /// Creates a new generator with the provided configuration.
    ///
    /// # Arguments
    /// - `config`: The `HumansConfig` instance containing metadata.
    ///
    /// # Returns
    /// A new `HumansGenerator` instance.
    pub fn new(config: HumansConfig) -> Self {
        Self { config }
    }

    /// Generates the content of a `humans.txt` file.
    ///
    /// # Returns
    /// A string containing the formatted `humans.txt` content.
    pub fn generate(&self) -> String {
        let mut content = String::new();

        // TEAM Section
        content.push_str("/* TEAM */\n");
        if !self.config.author.is_empty() {
            content.push_str(&format!(
                "    Name: {}\n",
                self.config.author
            ));
        }
        if !self.config.author_website.is_empty() {
            content.push_str(&format!(
                "    Website: {}\n",
                self.config.author_website
            ));
        }
        if !self.config.author_twitter.is_empty() {
            content.push_str(&format!(
                "    Twitter: {}\n",
                self.config.author_twitter
            ));
        }
        if !self.config.author_location.is_empty() {
            content.push_str(&format!(
                "    Location: {}\n",
                self.config.author_location
            ));
        }

        // THANKS Section
        content.push_str("\n/* THANKS */\n");
        if !self.config.thanks.is_empty() {
            content.push_str(&format!(
                "    Thanks: {}\n",
                self.config.thanks
            ));
        }

        // SITE Section
        content.push_str("\n/* SITE */\n");
        if !self.config.site_last_updated.is_empty() {
            content.push_str(&format!(
                "    Last update: {}\n",
                self.config.site_last_updated
            ));
        }
        if !self.config.site_standards.is_empty() {
            content.push_str(&format!(
                "    Standards: {}\n",
                self.config.site_standards
            ));
        }
        if !self.config.site_components.is_empty() {
            content.push_str(&format!(
                "    Components: {}\n",
                self.config.site_components
            ));
        }
        if !self.config.site_software.is_empty() {
            content.push_str(&format!(
                "    Software: {}\n",
                self.config.site_software
            ));
        }

        content
    }

    /// Exports the generated `humans.txt` content to a file.
    ///
    /// # Arguments
    /// - `path`: The file path where the `humans.txt` will be written.
    ///
    /// # Returns
    /// A `std::io::Result<()>` indicating success or failure.
    pub fn export_to_file(&self, path: &str) -> std::io::Result<()> {
        std::fs::write(path, self.generate())
    }
}

/// Sanitizes general text content
fn sanitize_text(text: &str) -> String {
    text.trim()
        .chars()
        .filter(|c| !c.is_control())
        .take(MAX_TEXT_LENGTH)
        .collect()
}

/// Sanitizes and validates a URL
fn sanitize_url(url: &str) -> Result<String, HumansError> {
    let url = url.trim();
    if url.is_empty() {
        return Ok(String::new());
    }

    match Url::parse(url) {
        Ok(parsed_url)
            if parsed_url.scheme() == "http"
                || parsed_url.scheme() == "https" =>
        {
            Ok(url.to_string())
        }
        _ => Err(HumansError::InvalidUrl(url.to_string())),
    }
}

/// Sanitizes and validates a Twitter handle
fn sanitize_twitter_handle(handle: &str) -> String {
    let handle = handle.trim();
    if handle.starts_with('@')
        && handle[1..]
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        handle.to_string()
    } else {
        String::new()
    }
}

/// Sanitizes and validates a date string (YYYY-MM-DD format)
fn sanitize_date(date: &str) -> Result<String, HumansError> {
    let date = date.trim();
    if date.is_empty() {
        return Ok(String::new());
    }

    match dtt_parse!(date) {
        Ok(_) => Ok(date.to_string()),
        Err(_) => Err(HumansError::InvalidDate(date.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_humans_content() {
        let config = HumansConfig {
            author: "John Doe".to_string(),
            author_website: "https://example.com".to_string(),
            author_twitter: "@johndoe".to_string(),
            author_location: "New York".to_string(),
            site_components: "Rust, SSG".to_string(),
            site_last_updated: "2024-01-01".to_string(),
            site_standards: "HTML5, CSS3".to_string(),
            site_software: "StaticDataGen".to_string(),
            thanks: "Contributors".to_string(),
        };

        let generator = HumansGenerator::new(config);
        let content = generator.generate();

        assert!(content.contains("John Doe"));
        assert!(content.contains("https://example.com"));
        assert!(content.contains("@johndoe"));
        assert!(content.contains("Contributors"));
    }

    #[test]
    fn test_empty_metadata() {
        let metadata: HashMap<String, String> = HashMap::new();
        assert!(HumansConfig::from_metadata(&metadata).is_err());
    }

    #[test]
    fn test_export_to_file() {
        let config = HumansConfig {
            author: "John Doe".to_string(),
            author_website: "https://example.com".to_string(),
            ..Default::default()
        };

        let generator = HumansGenerator::new(config);
        let file_path = "test_humans.txt";

        generator.export_to_file(file_path).unwrap();

        let content = std::fs::read_to_string(file_path).unwrap();
        assert!(content.contains("John Doe"));
        assert!(content.contains("https://example.com"));

        std::fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_sanitize_text() {
        assert_eq!(sanitize_text("Normal"), "Normal");
        assert_eq!(sanitize_text("Invalid\nChars"), "InvalidChars");
    }

    #[test]
    fn test_sanitize_url() {
        assert_eq!(
            sanitize_url("https://example.com").unwrap(),
            "https://example.com"
        );
        assert!(matches!(
            sanitize_url("ftp://example.com"),
            Err(HumansError::InvalidUrl(_))
        ));
    }

    #[test]
    fn test_sanitize_twitter_handle() {
        assert_eq!(
            sanitize_twitter_handle("@valid_handle"),
            "@valid_handle"
        );
        assert!(sanitize_twitter_handle("invalid").is_empty());
    }

    #[test]
    fn test_builder_new() {
        let builder = HumansConfigBuilder::new();
        assert!(builder.config.author.is_empty());
    }

    #[test]
    fn test_builder_methods() {
        let config = HumansConfig::builder()
            .author("John Doe")
            .author_website("https://example.com")
            .unwrap()
            .author_twitter("@johndoe")
            .author_location("New York")
            .site_components("Rust")
            .site_last_updated("2024-01-01T00:00:00Z")
            .unwrap()
            .site_standards("HTML5")
            .site_software("StaticDataGen")
            .thanks("Contributors")
            .build()
            .unwrap();

        assert_eq!(config.author, "John Doe");
        assert_eq!(config.author_website, "https://example.com");
        assert_eq!(config.author_twitter, "@johndoe");
        assert_eq!(config.author_location, "New York");
        assert_eq!(config.site_components, "Rust");
        assert_eq!(config.site_last_updated, "2024-01-01T00:00:00Z");
        assert_eq!(config.site_standards, "HTML5");
        assert_eq!(config.site_software, "StaticDataGen");
        assert_eq!(config.thanks, "Contributors");
    }

    #[test]
    fn test_builder_invalid_author() {
        let result = HumansConfig::builder().build();
        assert!(matches!(result, Err(HumansError::MissingMetadata(_))));
    }

    #[test]
    fn test_builder_invalid_website() {
        let result = HumansConfig::builder()
            .author("John Doe")
            .author_website("invalid-url")
            .and_then(|builder| builder.build());
        assert!(matches!(result, Err(HumansError::InvalidUrl(_))));
    }

    #[test]
    fn test_builder_invalid_date() {
        let result = HumansConfig::builder()
            .author("John Doe")
            .site_last_updated("invalid-date")
            .and_then(|builder| builder.build());
        assert!(matches!(result, Err(HumansError::InvalidDate(_))));
    }

    #[test]
    fn test_from_metadata_complete() {
        let mut metadata = HashMap::new();
        _ = metadata
            .insert("author".to_string(), "John Doe".to_string());
        _ = metadata.insert(
            "author_website".to_string(),
            "https://example.com".to_string(),
        );
        _ = metadata.insert(
            "author_twitter".to_string(),
            "@johndoe".to_string(),
        );
        _ = metadata.insert(
            "author_location".to_string(),
            "New York".to_string(),
        );
        _ = metadata
            .insert("site_components".to_string(), "Rust".to_string());
        _ = metadata.insert(
            "site_last_updated".to_string(),
            "2024-01-01T00:00:00Z".to_string(),
        );
        _ = metadata
            .insert("site_standards".to_string(), "HTML5".to_string());
        _ = metadata.insert(
            "site_software".to_string(),
            "StaticDataGen".to_string(),
        );
        _ = metadata
            .insert("thanks".to_string(), "Contributors".to_string());

        let config = HumansConfig::from_metadata(&metadata).unwrap();
        assert_eq!(config.author, "John Doe");
        assert_eq!(config.site_software, "StaticDataGen");
    }

    #[test]
    fn test_from_metadata_invalid_website() {
        let mut metadata = HashMap::new();
        _ = metadata
            .insert("author".to_string(), "John Doe".to_string());
        _ = metadata.insert(
            "author_website".to_string(),
            "invalid-url".to_string(),
        );

        let result = HumansConfig::from_metadata(&metadata);
        assert!(matches!(result, Err(HumansError::InvalidUrl(_))));
    }

    #[test]
    fn test_from_metadata_invalid_date() {
        let mut metadata = HashMap::new();
        _ = metadata
            .insert("author".to_string(), "John Doe".to_string());
        _ = metadata.insert(
            "site_last_updated".to_string(),
            "invalid-date".to_string(),
        );

        let result = HumansConfig::from_metadata(&metadata);
        assert!(matches!(result, Err(HumansError::InvalidDate(_))));
    }

    #[test]
    fn test_generate_empty_sections() {
        let config =
            HumansConfig::builder().author("John Doe").build().unwrap();

        let generator = HumansGenerator::new(config);
        let content = generator.generate();

        assert!(content.contains("John Doe"));
        assert!(!content.contains("Website:"));
        assert!(!content.contains("Twitter:"));
        assert!(!content.contains("Thanks:"));
    }

    #[test]
    fn test_sanitize_text_whitespace() {
        assert_eq!(sanitize_text("  Test  "), "Test");
        assert_eq!(sanitize_text("\t\nTest\r\n"), "Test");
    }

    #[test]
    fn test_sanitize_text_length_limit() {
        let long_text = "a".repeat(MAX_TEXT_LENGTH + 10);
        assert_eq!(sanitize_text(&long_text).len(), MAX_TEXT_LENGTH);
    }

    #[test]
    fn test_sanitize_url_empty() {
        assert_eq!(sanitize_url("").unwrap(), "");
        assert_eq!(sanitize_url("   ").unwrap(), "");
    }

    #[test]
    fn test_sanitize_url_valid_schemes() {
        assert!(sanitize_url("http://example.com").is_ok());
        assert!(sanitize_url("https://example.com").is_ok());
    }

    #[test]
    fn test_sanitize_twitter_handle_empty() {
        assert_eq!(sanitize_twitter_handle(""), "");
        assert_eq!(sanitize_twitter_handle("   "), "");
    }

    #[test]
    fn test_sanitize_twitter_handle_invalid_chars() {
        assert_eq!(sanitize_twitter_handle("@handle!"), "");
        assert_eq!(sanitize_twitter_handle("@handle space"), "");
    }

    #[test]
    fn test_sanitize_date_empty() {
        assert_eq!(sanitize_date("").unwrap(), "");
        assert_eq!(sanitize_date("   ").unwrap(), "");
    }

    #[test]
    fn test_error_display() {
        let err = HumansError::InvalidInput {
            field: "test".to_string(),
            message: "invalid".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "Invalid input for field 'test': invalid"
        );
    }

    #[test]
    fn test_config_default() {
        let config = HumansConfig::default();
        assert!(config.author.is_empty());
        assert!(config.author_website.is_empty());
        assert!(config.author_twitter.is_empty());
        assert!(config.author_location.is_empty());
        assert!(config.site_components.is_empty());
        assert!(config.site_last_updated.is_empty());
        assert!(config.site_standards.is_empty());
        assert!(config.site_software.is_empty());
        assert!(config.thanks.is_empty());
    }

    #[test]
    fn test_generator_debug() {
        let config = HumansConfig::default();
        let generator = HumansGenerator::new(config);
        assert!(!format!("{:?}", generator).is_empty());
    }
}
