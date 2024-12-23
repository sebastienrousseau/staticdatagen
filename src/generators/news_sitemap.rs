// Copyright © 2025 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! News Sitemap Generation Module
//!
//! This module handles the creation and generation of Google News sitemaps,
//! which help search engines discover and index news content. It follows the
//! Google News Sitemap protocol specification.
//!
//! # Features
//! - Creation of news sitemap data structures from metadata
//! - Validation of news publication dates
//! - Proper XML formatting for news sitemaps
//! - Support for news-specific metadata (genres, keywords, etc.)
//!
//! # Example
//! ```
//! use std::collections::HashMap;
//! use staticdatagen::generators::news_sitemap::{NewsSiteMapConfig, NewsSiteMapGenerator};
//!
//! let mut metadata = HashMap::new();
//! metadata.insert("news_title".to_string(), "Breaking News".to_string());
//! metadata.insert(
//!     "news_publication_date".to_string(),
//!     "Tue, 20 Feb 2024 15:15:15 GMT".to_string(),
//! );
//!
//! let config = NewsSiteMapConfig::new(metadata);
//! let generator = NewsSiteMapGenerator::new(config);
//! let news_sitemap = generator.generate_xml();
//! ```

use crate::models::data::NewsData;
use std::collections::HashMap;
use xml::writer::EmitterConfig;
use xml::writer::events::XmlEvent;
use time::{format_description, OffsetDateTime};

/// Configuration for generating a news sitemap.
#[derive(Debug, Clone)]
pub struct NewsSiteMapConfig {
    metadata: HashMap<String, String>,
}

impl NewsSiteMapConfig {
    /// Creates a new `NewsSiteMapConfig` with the provided metadata.
    pub fn new(metadata: HashMap<String, String>) -> Self {
        Self { metadata }
    }

    /// Retrieves a sanitized value from the metadata or a default.
    fn get_sanitized(&self, key: &str, default: &str) -> String {
        sanitize_text(
            self.metadata.get(key).unwrap_or(&default.to_string()),
        )
    }

    /// Formats and retrieves the publication date from the metadata.
    fn get_formatted_date(&self) -> String {
        format_publication_date(
            self.metadata
                .get("news_publication_date")
                .unwrap_or(&String::new()),
        )
    }

    /// Builds a `NewsData` object based on the metadata.
    pub fn to_news_data(&self) -> NewsData {
        NewsData {
            news_genres: validate_genres(
                self.metadata
                    .get("news_genres")
                    .unwrap_or(&String::new()),
            ),
            news_image_loc: validate_url(
                self.metadata
                    .get("news_image_loc")
                    .unwrap_or(&String::new()),
            ),
            news_keywords: validate_keywords(
                self.metadata
                    .get("news_keywords")
                    .unwrap_or(&String::new()),
            ),
            news_language: validate_language(
                self.metadata
                    .get("news_language")
                    .unwrap_or(&String::new()),
            ),
            news_loc: validate_url(
                self.metadata.get("news_loc").unwrap_or(&String::new()),
            ),
            news_publication_date: self.get_formatted_date(),
            news_publication_name: self
                .get_sanitized("news_publication_name", ""),
            news_title: self.get_sanitized("news_title", ""),
        }
    }
}

/// Generator for creating a news sitemap.
#[derive(Debug, Clone)]
pub struct NewsSiteMapGenerator {
    config: NewsSiteMapConfig,
}

impl NewsSiteMapGenerator {
    /// Creates a new `NewsSiteMapGenerator` with the provided configuration.
    pub fn new(config: NewsSiteMapConfig) -> Self {
        Self { config }
    }

    /// Generates the news sitemap XML.
    pub fn generate_xml(&self) -> String {
    let news_data = self.config.to_news_data();
    let mut output = Vec::new();
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(&mut output);

    writer
        .write(XmlEvent::start_element("urlset")
            .attr("xmlns", "http://www.sitemaps.org/schemas/sitemap/0.9")
            .attr("xmlns:news", "http://www.google.com/schemas/sitemap-news/0.9"))
        .unwrap();

    writer.write(XmlEvent::start_element("url")).unwrap();
    writer.write(XmlEvent::start_element("loc")).unwrap();
    writer.write(XmlEvent::characters(&news_data.news_loc)).unwrap();
    writer.write(XmlEvent::end_element()).unwrap(); // End <loc>

    writer.write(XmlEvent::start_element("news:news")).unwrap();
    writer.write(XmlEvent::start_element("news:publication")).unwrap();
    writer.write(XmlEvent::start_element("news:name")).unwrap();
    writer.write(XmlEvent::characters(&news_data.news_publication_name)).unwrap();
    writer.write(XmlEvent::end_element()).unwrap(); // End <news:name>
    writer.write(XmlEvent::start_element("news:language")).unwrap();
    writer.write(XmlEvent::characters(&news_data.news_language)).unwrap();
    writer.write(XmlEvent::end_element()).unwrap(); // End <news:language>
    writer.write(XmlEvent::end_element()).unwrap(); // End <news:publication>

    writer.write(XmlEvent::start_element("news:publication_date")).unwrap();
    writer.write(XmlEvent::characters(&format_publication_date(&news_data.news_publication_date))).unwrap();
    writer.write(XmlEvent::end_element()).unwrap(); // End <news:publication_date>

    writer.write(XmlEvent::start_element("news:title")).unwrap();
    writer.write(XmlEvent::characters(&news_data.news_title)).unwrap();
    writer.write(XmlEvent::end_element()).unwrap(); // End <news:title>

    writer.write(XmlEvent::end_element()).unwrap(); // End <news:news>
    writer.write(XmlEvent::end_element()).unwrap(); // End <url>
    writer.write(XmlEvent::end_element()).unwrap(); // End <urlset>

    String::from_utf8(output).unwrap_or_default()
}




}

/// Formats publication dates from "Tue, 20 Feb 2024 15:15:15 GMT" to ISO 8601.
fn format_publication_date(input: &str) -> String {
    if let Ok(parsed) = OffsetDateTime::parse(input, &format_description::parse("%a, %d %b %Y %H:%M:%S GMT").unwrap()) {
        parsed.format(&format_description::well_known::Rfc3339).unwrap_or_default()
    } else {
        let now = OffsetDateTime::now_utc();
        now.format(&format_description::well_known::Rfc3339).unwrap_or_default()
    }
}

/// Validates and filters news genres based on Google News specifications.
fn validate_genres(genres: &str) -> String {
    let valid_genres = [
        "PressRelease",
        "Satire",
        "Blog",
        "OpEd",
        "Opinion",
        "UserGenerated",
    ];

    genres
        .split(',')
        .filter_map(|g| {
            let cleaned = g.trim();
            if valid_genres.contains(&cleaned) {
                Some(cleaned.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .join(", ")
}

/// Validates and sanitizes news keywords.
fn validate_keywords(keywords: &str) -> String {
    keywords
        .split(',')
        .take(10) // Google News limit
        .map(|k| k.trim())
        .filter(|k| !k.is_empty())
        .collect::<Vec<&str>>()
        .join(", ")
}

/// Validates language codes to ensure compliance with ISO 639-1.
fn validate_language(lang: &str) -> String {
    if lang.len() == 2 && lang.chars().all(|c| c.is_ascii_lowercase()) {
        lang.to_string()
    } else {
        "en".to_string() // Default to English
    }
}

/// Validates URLs to ensure they are well-formed and safe.
fn validate_url(url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        if url.contains('<') || url.contains('>') || url.contains('"') {
            String::new()
        } else {
            url.to_string()
        }
    } else {
        String::new()
    }
}

/// Sanitizes text by removing control characters and limiting length.
fn sanitize_text(text: &str) -> String {
    text.chars()
        .filter(|c| !c.is_control())
        .take(1000) // Reasonable limit for titles and names
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_news_sitemap_generation() {
        let mut metadata = HashMap::new();
        let _ = metadata
            .insert("news_title".to_string(), "Test News".to_string());
        let _ = metadata.insert(
            "news_publication_date".to_string(),
            "Tue, 20 Feb 2024 15:15:15 GMT".to_string(),
        );

        let config = NewsSiteMapConfig::new(metadata);
        let generator = NewsSiteMapGenerator::new(config);
        let news_data = generator.config.to_news_data();

        assert_eq!(news_data.news_title, "Test News");
        assert_eq!(
            news_data.news_publication_date,
            "2024-02-20T15:15:15+00:00"
        );
    }

    #[test]
fn test_format_publication_date() {
    let input = "Tue, 20 Feb 2024 15:15:15 GMT";
    assert_eq!(
        format_publication_date(input),
        "2024-02-20T15:15:15+00:00"
    );

    // Invalid formats
    assert_eq!(
        format_publication_date("Invalid Date"),
        "2025-01-01T00:00:00+00:00"
    ); // Verify fallback is applied

    assert_eq!(
        format_publication_date(""),
        "2025-01-01T00:00:00+00:00"
    ); // Verify fallback is applied
}


    #[test]
    fn test_validate_genres() {
        assert_eq!(
            validate_genres("Blog, OpEd, Invalid"),
            "Blog, OpEd"
        );
        assert_eq!(
            validate_genres("PressRelease,Satire"),
            "PressRelease, Satire"
        );
        assert!(validate_genres("Invalid").is_empty());
    }

    #[test]
    fn test_validate_keywords() {
        assert_eq!(
            validate_keywords("news, breaking, update"),
            "news, breaking, update"
        );

        // Test limit
        let many_keywords = (0..20)
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(",");
        assert_eq!(
            validate_keywords(&many_keywords).split(", ").count(),
            10
        );
    }

    #[test]
    fn test_validate_language() {
        assert_eq!(validate_language("en"), "en");
        assert_eq!(validate_language("fr"), "fr");
        assert_eq!(validate_language("invalid"), "en");
        assert_eq!(validate_language(""), "en");
    }

    #[test]
    fn test_validate_url() {
        assert_eq!(
            validate_url("https://example.com"),
            "https://example.com"
        );
        assert!(validate_url("invalid-url").is_empty());
        assert!(validate_url("https://example.com<script>").is_empty());
    }

    #[test]
    fn test_sanitize_text() {
        assert_eq!(sanitize_text("Normal text"), "Normal text");
        assert_eq!(
            sanitize_text("Text\nwith\tcontrol\rchars"),
            "Textwithcontrolchars"
        );

        // Test length limit
        let long_text = "a".repeat(2000);
        assert_eq!(sanitize_text(&long_text).len(), 1000);
    }
}
