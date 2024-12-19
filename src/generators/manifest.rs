// Copyright © 2024 StaticDataGen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Web App Manifest Generator
//!
//! This module provides functionality for generating `manifest.json` files for
//! Progressive Web Apps (PWA). It handles the generation and validation of
//! manifest files according to the Web App Manifest specification.
//!
//! # Features
//!
//! - Type-safe manifest generation using the builder pattern
//! - Validation of all manifest fields
//! - Support for PWA icons with customizable properties
//! - Automatic sanitization of user input
//! - Comprehensive error handling
//!
//! # Example
//!
//! ```rust
//! use staticdatagen::generators::manifest::{ManifestGenerator, ManifestConfig, IconConfig};
//!
//! let config = ManifestConfig::builder()
//!     .name("My App")
//!     .short_name("App")
//!     .description("A progressive web app")
//!     .theme_color("#ffffff")
//!     .add_icon(IconConfig::new("/icon.svg", "512x512"))
//!     .build()?;
//!
//! let generator = ManifestGenerator::new(config);
//! let manifest_json = generator.generate()?;
//! # Ok::<(), staticdatagen::generators::manifest::ManifestError>(())
//! ```

use std::collections::HashMap;
use thiserror::Error;

/// Constants defining default values for manifest fields.
pub mod defaults {
    /// Default start URL for the web app.
    pub const START_URL: &str = ".";
    /// Default display mode for the web app.
    pub const DISPLAY: &str = "standalone";
    /// Default background color for the web app.
    pub const BACKGROUND: &str = "#ffffff";
    /// Default orientation for the web app.
    pub const ORIENTATION: &str = "portrait-primary";
    /// Default scope for the web app.
    pub const SCOPE: &str = "/";
    /// Default icon size for the web app.
    pub const ICON_SIZE: &str = "512x512";
    /// Default icon MIME type for the web app.
    pub const ICON_TYPE: &str = "image/svg+xml";
    /// Default icon purpose for the web app.
    pub const ICON_PURPOSE: &str = "any maskable";
}

/// Errors that can occur during manifest generation and validation.
#[derive(Debug, Error)]
pub enum ManifestError {
    /// The manifest name is invalid or missing.
    #[error("Invalid manifest name: {0}")]
    InvalidName(String),

    /// The color value is invalid.
    #[error("Invalid color value: {0}")]
    InvalidColor(String),

    /// The icon URL is invalid.
    #[error("Invalid icon URL: {0}")]
    InvalidIconUrl(String),

    /// The display mode is invalid.
    #[error("Invalid display mode: {0}")]
    InvalidDisplayMode(String),

    /// JSON serialization failed.
    #[error("Failed to serialize manifest: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// Configuration for manifest generation.
#[derive(Debug, Clone)]
pub struct ManifestConfig {
    name: String,
    short_name: Option<String>,
    description: Option<String>,
    start_url: String,
    display: String,
    background_color: String,
    theme_color: Option<String>,
    icons: Vec<IconConfig>,
    orientation: String,
    scope: String,
}

/// Configuration for PWA icons.
#[derive(Debug, Clone)]
pub struct IconConfig {
    src: String,
    sizes: String,
    icon_type: Option<String>,
    purpose: Option<String>,
}

impl IconConfig {
    /// Creates a new icon configuration.
    ///
    /// # Arguments
    ///
    /// * `src` - The URL of the icon
    /// * `sizes` - The icon sizes (e.g., "512x512")
    pub fn new(
        src: impl Into<String>,
        sizes: impl Into<String>,
    ) -> Self {
        Self {
            src: src.into(),
            sizes: sizes.into(),
            icon_type: Some(defaults::ICON_TYPE.to_string()),
            purpose: Some(defaults::ICON_PURPOSE.to_string()),
        }
    }

    /// Sets the icon MIME type.
    pub fn icon_type(mut self, icon_type: impl Into<String>) -> Self {
        self.icon_type = Some(icon_type.into());
        self
    }

    /// Sets the icon purpose.
    pub fn purpose(mut self, purpose: impl Into<String>) -> Self {
        self.purpose = Some(purpose.into());
        self
    }
}

impl ManifestConfig {
    /// Creates a new manifest configuration builder.
    pub fn builder() -> ManifestConfigBuilder {
        ManifestConfigBuilder::default()
    }

    /// Creates a manifest configuration from metadata.
    pub fn from_metadata(
        metadata: &HashMap<String, String>,
    ) -> Result<Self, ManifestError> {
        let mut builder = ManifestConfigBuilder::default();

        if let Some(name) = metadata.get("name") {
            builder = builder.name(name);
        }
        if let Some(short_name) = metadata.get("short_name") {
            builder = builder.short_name(short_name);
        }
        if let Some(description) = metadata.get("description") {
            builder = builder.description(description);
        }
        if let Some(theme_color) = metadata.get("theme-color") {
            builder = builder.theme_color(theme_color);
        }
        if let Some(background_color) = metadata.get("background-color")
        {
            builder = builder.background_color(background_color);
        }
        if let Some(icon) = metadata.get("icon") {
            builder = builder
                .add_icon(IconConfig::new(icon, defaults::ICON_SIZE));
        }

        builder.build()
    }
}

/// Builder for manifest configuration.
#[derive(Debug, Default)]
pub struct ManifestConfigBuilder {
    name: Option<String>,
    short_name: Option<String>,
    description: Option<String>,
    start_url: Option<String>,
    display: Option<String>,
    background_color: Option<String>,
    theme_color: Option<String>,
    icons: Vec<IconConfig>,
    orientation: Option<String>,
    scope: Option<String>,
}

impl ManifestConfigBuilder {
    /// Sets the app name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets the app short name.
    pub fn short_name(mut self, name: impl Into<String>) -> Self {
        self.short_name = Some(name.into());
        self
    }

    /// Sets the app description.
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Sets the start URL.
    pub fn start_url(mut self, url: impl Into<String>) -> Self {
        self.start_url = Some(url.into());
        self
    }

    /// Sets the display mode.
    pub fn display(mut self, display: impl Into<String>) -> Self {
        self.display = Some(display.into());
        self
    }

    /// Sets the background color.
    pub fn background_color(
        mut self,
        color: impl Into<String>,
    ) -> Self {
        self.background_color = Some(color.into());
        self
    }

    /// Sets the theme color.
    pub fn theme_color(mut self, color: impl Into<String>) -> Self {
        self.theme_color = Some(color.into());
        self
    }

    /// Adds an icon configuration.
    pub fn add_icon(mut self, icon: IconConfig) -> Self {
        self.icons.push(icon);
        self
    }

    /// Sets the orientation.
    pub fn orientation(
        mut self,
        orientation: impl Into<String>,
    ) -> Self {
        self.orientation = Some(orientation.into());
        self
    }

    /// Sets the scope.
    pub fn scope(mut self, scope: impl Into<String>) -> Self {
        self.scope = Some(scope.into());
        self
    }

    /// Builds the manifest configuration.
    pub fn build(self) -> Result<ManifestConfig, ManifestError> {
        let name = self.name.unwrap_or_default();
        if name.is_empty() {
            return Err(ManifestError::InvalidName(
                "Name cannot be empty".to_string(),
            ));
        }

        Ok(ManifestConfig {
            name: sanitize_text(&name, 45),
            short_name: self.short_name.map(|n| sanitize_text(&n, 12)),
            description: self
                .description
                .map(|d| sanitize_text(&d, 120)),
            start_url: self
                .start_url
                .unwrap_or_else(|| defaults::START_URL.to_string()),
            display: self
                .display
                .unwrap_or_else(|| defaults::DISPLAY.to_string()),
            background_color: self
                .background_color
                .map(sanitize_color)
                .unwrap_or_else(|| defaults::BACKGROUND.to_string()),
            theme_color: self.theme_color.map(sanitize_color),
            icons: self.icons,
            orientation: self
                .orientation
                .unwrap_or_else(|| defaults::ORIENTATION.to_string()),
            scope: self
                .scope
                .unwrap_or_else(|| defaults::SCOPE.to_string()),
        })
    }
}

/// Generator for web app manifests.
#[derive(Debug)]
pub struct ManifestGenerator {
    config: ManifestConfig,
}

impl ManifestGenerator {
    /// Creates a new manifest generator with the specified configuration.
    pub fn new(config: ManifestConfig) -> Self {
        Self { config }
    }

    /// Creates a manifest generator from metadata.
    pub fn from_metadata(
        metadata: &HashMap<String, String>,
    ) -> Result<String, ManifestError> {
        let config = ManifestConfig::from_metadata(metadata)?;
        let generator = Self::new(config);
        generator.generate()
    }

    /// Generates the manifest JSON.
    pub fn generate(&self) -> Result<String, ManifestError> {
        let manifest = serde_json::json!({
            "name": self.config.name,
            "short_name": self.config.short_name,
            "description": self.config.description,
            "start_url": self.config.start_url,
            "display": self.config.display,
            "background_color": self.config.background_color,
            "theme_color": self.config.theme_color,
            "icons": self.config.icons.iter().map(|icon| {
                let mut map = serde_json::Map::new();
                _ = map.insert("src".to_string(), serde_json::Value::String(icon.src.clone()));
                _ = map.insert("sizes".to_string(), serde_json::Value::String(icon.sizes.clone()));
                if let Some(ref t) = icon.icon_type {
                    _ = map.insert("type".to_string(), serde_json::Value::String(t.clone()));
                }
                if let Some(ref p) = icon.purpose {
                    _ = map.insert("purpose".to_string(), serde_json::Value::String(p.clone()));
                }
                map
            }).collect::<Vec<_>>(),
            "orientation": self.config.orientation,
            "scope": self.config.scope,
        });

        serde_json::to_string_pretty(&manifest)
            .map_err(ManifestError::SerializationError)
    }
}

// Helper functions

/// Sanitizes a text string by removing control characters and limiting its length.
///
/// # Parameters
///
/// * `text`: A reference to the input text string.
/// * `max_length`: The maximum length to which the text should be limited.
///
/// # Returns
///
/// A sanitized string with control characters removed and limited to the specified maximum length.
///
/// # Examples
///
/// ```rust
/// use staticdatagen::generators::manifest::{sanitize_text, sanitize_color};
///
/// let text = "Hello\nWorld";
/// let sanitized = sanitize_text(text, 10);
/// assert_eq!(sanitized, "HelloWorld");
///
/// let color = sanitize_color("#fff".to_string());
/// assert_eq!(color, "#fff");
/// ```
pub fn sanitize_text(text: &str, max_length: usize) -> String {
    text.chars()
        .filter(|c| !c.is_control())
        .take(max_length)
        .collect()
}

/// Sanitizes a color string by validating its format and returning the original color if valid,
/// or the default background color if invalid.
///
/// # Parameters
///
/// * `color`: A string representing the color to be sanitized.
///
/// # Returns
///
/// A string representing the sanitized color.
///
/// # Examples
///
/// ```rust
/// use staticdatagen::generators::manifest::sanitize_color;
///
/// assert_eq!(sanitize_color("#fff".to_string()), "#fff");
/// assert_eq!(sanitize_color("#ffffff".to_string()), "#ffffff");
/// assert_eq!(sanitize_color("rgb(255,255,255)".to_string()), "rgb(255,255,255)");
/// assert_eq!(sanitize_color("invalid".to_string()), "#ffffff");
/// ```
pub fn sanitize_color(color: String) -> String {
    if (color.starts_with('#')
        && (color.len() == 4 || color.len() == 7)
        && color[1..].chars().all(|c| c.is_ascii_hexdigit()))
        || (color.starts_with("rgb(") && color.ends_with(')'))
    {
        color
    } else {
        defaults::BACKGROUND.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_manifest() {
        let config =
            ManifestConfig::builder().name("Test App").build().unwrap();

        let generator = ManifestGenerator::new(config);
        let json = generator.generate().unwrap();

        assert!(json.contains("Test App"));
    }

    #[test]
    fn test_complete_manifest() {
        let config = ManifestConfig::builder()
            .name("Test App")
            .short_name("App")
            .description("A test application")
            .theme_color("#ffffff")
            .background_color("#000000")
            .add_icon(IconConfig::new("/icon.svg", "512x512"))
            .build()
            .unwrap();

        let generator = ManifestGenerator::new(config);
        let json = generator.generate().unwrap();

        assert!(json.contains("Test App"));
        assert!(json.contains("/icon.svg"));
        assert!(json.contains("#ffffff"));
        assert!(json.contains("#000000"));
    }

    #[test]
    fn test_invalid_manifest() {
        let result = ManifestConfig::builder().name("").build();

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ManifestError::InvalidName(_)
        ));
    }

    #[test]
    fn test_metadata_conversion() {
        let mut metadata = HashMap::new();
        _ = metadata.insert("name".to_string(), "Test App".to_string());
        _ = metadata
            .insert("theme-color".to_string(), "#ffffff".to_string());

        let result = ManifestGenerator::from_metadata(&metadata);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Test App"));
    }

    #[test]
    fn test_icon_builder() {
        let icon = IconConfig::new("/icon.svg", "512x512")
            .icon_type("image/svg+xml")
            .purpose("any maskable");

        assert_eq!(icon.src, "/icon.svg");
        assert_eq!(icon.sizes, "512x512");
        assert_eq!(icon.icon_type.unwrap(), "image/svg+xml");
        assert_eq!(icon.purpose.unwrap(), "any maskable");
    }

    #[test]
    fn test_manifest_builder_all_fields() {
        let config = ManifestConfig::builder()
            .name("Test App")
            .short_name("App")
            .description("Test Description")
            .start_url("/start")
            .display("standalone")
            .background_color("#ffffff")
            .theme_color("#000000")
            .orientation("portrait")
            .scope("/scope")
            .build()
            .unwrap();

        assert_eq!(config.name, "Test App");
        assert_eq!(config.short_name.unwrap(), "App");
        assert_eq!(config.description.unwrap(), "Test Description");
        assert_eq!(config.start_url, "/start");
        assert_eq!(config.display, "standalone");
        assert_eq!(config.background_color, "#ffffff");
        assert_eq!(config.theme_color.unwrap(), "#000000");
        assert_eq!(config.orientation, "portrait");
        assert_eq!(config.scope, "/scope");
    }

    #[test]
    fn test_manifest_builder_defaults() {
        let config =
            ManifestConfig::builder().name("Test App").build().unwrap();

        assert_eq!(config.start_url, defaults::START_URL);
        assert_eq!(config.display, defaults::DISPLAY);
        assert_eq!(config.background_color, defaults::BACKGROUND);
        assert_eq!(config.orientation, defaults::ORIENTATION);
        assert_eq!(config.scope, defaults::SCOPE);
    }

    #[test]
    fn test_sanitize_text_length() {
        assert_eq!(sanitize_text("Hello", 3), "Hel");
        assert_eq!(sanitize_text("Hello\nWorld", 10), "HelloWorld");
        assert_eq!(sanitize_text("", 5), "");
    }

    #[test]
    fn test_sanitize_color_validation() {
        assert_eq!(sanitize_color("#fff".to_string()), "#fff");
        assert_eq!(sanitize_color("#ffffff".to_string()), "#ffffff");
        assert_eq!(
            sanitize_color("rgb(255,255,255)".to_string()),
            "rgb(255,255,255)"
        );
        assert_eq!(
            sanitize_color("invalid".to_string()),
            defaults::BACKGROUND
        );
        assert_eq!(
            sanitize_color("#ffff".to_string()),
            defaults::BACKGROUND
        );
        assert_eq!(
            sanitize_color("#fffffff".to_string()),
            defaults::BACKGROUND
        );
        assert_eq!(
            sanitize_color("#xyz".to_string()),
            defaults::BACKGROUND
        );
    }

    #[test]
    fn test_icon_config_methods() {
        let icon = IconConfig::new("/icon.svg", "512x512");
        assert_eq!(icon.src, "/icon.svg");
        assert_eq!(icon.sizes, "512x512");
        assert_eq!(
            icon.icon_type.as_ref().unwrap(),
            defaults::ICON_TYPE
        );
        assert_eq!(
            icon.purpose.as_ref().unwrap(),
            defaults::ICON_PURPOSE
        );
        let modified_icon =
            icon.clone().icon_type("image/png").purpose(
                icon.purpose
                    .clone()
                    .unwrap_or_else(|| "any maskable".to_string()),
            );
        assert_eq!(modified_icon.icon_type.unwrap(), "image/png");
        assert_eq!(modified_icon.purpose.unwrap(), "any maskable");
    }

    #[test]
    fn test_manifest_from_metadata_empty() {
        let empty_metadata = HashMap::new();
        let result = ManifestConfig::from_metadata(&empty_metadata);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ManifestError::InvalidName(_)
        ));
    }

    #[test]
    fn test_manifest_from_metadata_complete() {
        let mut metadata = HashMap::new();
        _ = metadata.insert("name".to_string(), "Test App".to_string());
        _ = metadata
            .insert("short_name".to_string(), "App".to_string());
        _ = metadata.insert(
            "description".to_string(),
            "Test Description".to_string(),
        );
        _ = metadata
            .insert("theme-color".to_string(), "#000000".to_string());
        _ = metadata.insert(
            "background-color".to_string(),
            "#ffffff".to_string(),
        );
        _ = metadata
            .insert("icon".to_string(), "/icon.svg".to_string());

        let config = ManifestConfig::from_metadata(&metadata).unwrap();
        assert_eq!(config.name, "Test App");
        assert_eq!(config.short_name.unwrap(), "App");
        assert_eq!(config.description.unwrap(), "Test Description");
        assert_eq!(config.theme_color.unwrap(), "#000000");
        assert_eq!(config.background_color, "#ffffff");
        assert!(!config.icons.is_empty());
        assert_eq!(config.icons[0].src, "/icon.svg");
    }

    #[test]
    fn test_manifest_generator_json_structure() {
        let config = ManifestConfig::builder()
            .name("Test App")
            .add_icon(IconConfig::new("/icon.svg", "512x512"))
            .build()
            .unwrap();

        let generator = ManifestGenerator::new(config);
        let json = generator.generate().unwrap();

        // Parse the JSON to verify its structure
        let manifest: serde_json::Value =
            serde_json::from_str(&json).unwrap();

        assert!(manifest.is_object());
        assert!(manifest.get("name").is_some());
        assert!(manifest.get("icons").unwrap().is_array());

        let icons = manifest.get("icons").unwrap().as_array().unwrap();
        assert_eq!(icons.len(), 1);

        let icon = &icons[0];
        assert_eq!(
            icon.get("src").unwrap().as_str().unwrap(),
            "/icon.svg"
        );
        assert_eq!(
            icon.get("sizes").unwrap().as_str().unwrap(),
            "512x512"
        );
    }

    #[test]
    fn test_manifest_json_formatting() {
        let config =
            ManifestConfig::builder().name("Test App").build().unwrap();

        let generator = ManifestGenerator::new(config);
        let json = generator.generate().unwrap();

        // Verify it's properly formatted JSON
        assert!(json.contains("{\n"));
        assert!(json.contains("  ")); // Check for indentation
        assert!(json.ends_with("\n}"));
    }

    #[test]
    fn test_long_text_sanitization() {
        let long_name = "a".repeat(100);
        let long_description = "b".repeat(200);

        let config = ManifestConfig::builder()
            .name(long_name)
            .description(long_description)
            .build()
            .unwrap();

        assert_eq!(config.name.len(), 45);
        assert_eq!(config.description.unwrap().len(), 120);
    }

    #[test]
    fn test_control_characters_sanitization() {
        let config = ManifestConfig::builder()
            .name("Test\0App\n\r\t")
            .build()
            .unwrap();

        assert_eq!(config.name, "TestApp");
    }
}
