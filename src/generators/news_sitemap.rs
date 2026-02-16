// Copyright © 2025-2026 Static Data Gen. All rights reserved.
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
use log::warn;
use std::collections::HashMap;
use time::{format_description, OffsetDateTime};
use xml::writer::events::XmlEvent;
use xml::writer::EmitterConfig;

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
            news_publication_name: self.get_sanitized(
                "news_publication_name",
                "Unnamed Publication",
            ),
            news_title: self
                .get_sanitized("news_title", "Untitled Article"),
        }
    }
}

/// Generator for creating a news sitemap.
#[derive(Debug, Clone)]
pub struct NewsSiteMapGenerator {
    /// Configuration for the news sitemap generator.
    pub config: NewsSiteMapConfig,
}

impl NewsSiteMapGenerator {
    /// Creates a new `NewsSiteMapGenerator` with the provided configuration.
    pub fn new(config: NewsSiteMapConfig) -> Self {
        Self { config }
    }

    /// Generates the news sitemap XML.
    pub fn generate_xml(&self) -> String {
        match self.try_generate_xml() {
            Ok(xml) => xml,
            Err(e) => {
                warn!(
                    "Failed to generate news sitemap XML: {}",
                    e
                );
                String::new()
            }
        }
    }

    /// Internal XML generation with proper error propagation.
    fn try_generate_xml(
        &self,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let news_data = self.config.to_news_data();
        let mut output = Vec::new();
        let mut writer = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(&mut output);

        writer.write(
            XmlEvent::start_element("urlset")
                .attr(
                    "xmlns",
                    "http://www.sitemaps.org/schemas/sitemap/0.9",
                )
                .attr(
                    "xmlns:news",
                    "http://www.google.com/schemas/sitemap-news/0.9",
                ),
        )?;

        writer.write(
            XmlEvent::start_element("url"),
        )?;
        writer.write(
            XmlEvent::start_element("loc"),
        )?;
        writer.write(XmlEvent::characters(
            &news_data.news_loc,
        ))?;
        writer.write(XmlEvent::end_element())?;

        writer.write(
            XmlEvent::start_element("news:news"),
        )?;
        writer.write(
            XmlEvent::start_element("news:publication"),
        )?;
        writer.write(
            XmlEvent::start_element("news:name"),
        )?;
        writer.write(XmlEvent::characters(
            &news_data.news_publication_name,
        ))?;
        writer.write(XmlEvent::end_element())?;
        writer.write(
            XmlEvent::start_element("news:language"),
        )?;
        writer.write(XmlEvent::characters(
            &news_data.news_language,
        ))?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;

        writer.write(
            XmlEvent::start_element(
                "news:publication_date",
            ),
        )?;
        writer.write(XmlEvent::characters(
            &news_data.news_publication_date,
        ))?;
        writer.write(XmlEvent::end_element())?;

        writer.write(
            XmlEvent::start_element("news:title"),
        )?;
        writer.write(XmlEvent::characters(
            &news_data.news_title,
        ))?;
        writer.write(XmlEvent::end_element())?;

        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;

        Ok(String::from_utf8(output)?)
    }
}

/// Formats publication dates from "Tue, 20 Feb 2024 15:15:15 GMT" to ISO 8601.
fn format_publication_date(input: &str) -> String {
    match OffsetDateTime::parse(
        input,
        &format_description::well_known::Rfc2822,
    ) {
        Ok(parsed) => parsed
            .format(&format_description::well_known::Rfc3339)
            .unwrap_or_default(),
        Err(e) => {
            warn!("Parsing failed: {}. Using fallback.", e);
            OffsetDateTime::now_utc()
                .format(&format_description::well_known::Rfc3339)
                .unwrap_or_default()
        }
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
    // Reject non-HTTP(S) schemes
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return String::new();
    }

    // Reject URLs with dangerous or invalid characters
    let has_dangerous_chars = url.chars().any(|c| {
        c == '<'
            || c == '>'
            || c == '"'
            || c == '\''
            || c == '\0'
            || c.is_control()
    });
    if has_dangerous_chars {
        return String::new();
    }

    // Reject URLs containing whitespace
    if url.chars().any(|c| c.is_whitespace()) {
        return String::new();
    }

    // Reject excessively long URLs
    if url.len() > 2048 {
        return String::new();
    }

    url.to_string()
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
        let _ = metadata.insert(
            "news_loc".to_string(),
            "https://example.com".to_string(),
        );

        let config = NewsSiteMapConfig::new(metadata);
        let generator = NewsSiteMapGenerator::new(config);
        let news_data = generator.config.to_news_data();

        assert_eq!(news_data.news_title, "Test News");

        // Assert that the result is either "2024-02-20T15:15:15Z" or "2024-02-20T15:15:15+00:00"
        assert!(
            news_data.news_publication_date == "2024-02-20T15:15:15Z"
                || news_data.news_publication_date
                    == "2024-02-20T15:15:15+00:00"
        );

        assert_eq!(news_data.news_loc, "https://example.com");
    }

    #[test]
    fn test_date_parsing_debug() {
        let input = "Tue, 20 Feb 2024 15:15:15 GMT";

        match OffsetDateTime::parse(
            input,
            &format_description::well_known::Rfc2822,
        ) {
            Ok(parsed) => println!("Parsed date: {}", parsed),
            Err(e) => panic!("Failed to parse date: {}", e),
        }
    }

    #[test]
    fn test_format_publication_date() {
        let input = "Tue, 20 Feb 2024 15:15:15 GMT";

        let result = format_publication_date(input);

        // Assert that the result is either "2024-02-20T15:15:15Z" or "2024-02-20T15:15:15+00:00"
        assert!(
            result == "2024-02-20T15:15:15Z"
                || result == "2024-02-20T15:15:15+00:00"
        );

        // Invalid formats should fall back
        let fallback = format_publication_date("Invalid Date");
        let fallback_now = OffsetDateTime::now_utc()
            .format(&format_description::well_known::Rfc3339)
            .unwrap();
        assert!(fallback.starts_with(&fallback_now[..10])); // Compare only the date part
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
        assert!(validate_genres("").is_empty());
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
            validate_keywords(&many_keywords).split(',').count(),
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
        assert_eq!(
            validate_url("http://example.com"),
            "http://example.com" // This should NOT return empty
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

    #[test]
    fn test_get_sanitized() {
        let mut metadata = HashMap::new();
        let _ =
            metadata.insert("key1".to_string(), "value1".to_string());

        let config = NewsSiteMapConfig::new(metadata);

        // Existing key
        assert_eq!(config.get_sanitized("key1", "default"), "value1");

        // Missing key with default
        assert_eq!(config.get_sanitized("key2", "default"), "default");
    }

    #[test]
    fn test_get_formatted_date() {
        let mut metadata = HashMap::new();
        let _ = metadata.insert(
            "news_publication_date".to_string(),
            "Tue, 20 Feb 2024 15:15:15 GMT".to_string(),
        );

        let config = NewsSiteMapConfig::new(metadata);

        // Valid date
        assert_eq!(config.get_formatted_date(), "2024-02-20T15:15:15Z");

        // Missing date
        let empty_config = NewsSiteMapConfig::new(HashMap::new());
        assert!(empty_config.get_formatted_date().starts_with(
            &OffsetDateTime::now_utc()
                .format(&format_description::well_known::Rfc3339)
                .unwrap()[..10]
        ));
    }

    #[test]
    fn test_generate_xml() {
        let mut metadata = HashMap::new();
        let _ = metadata
            .insert("news_title".to_string(), "Test News".to_string());
        let _ = metadata.insert(
            "news_publication_date".to_string(),
            "Tue, 20 Feb 2024 15:15:15 GMT".to_string(),
        );
        let _ = metadata.insert(
            "news_loc".to_string(),
            "https://example.com".to_string(),
        );
        let _ = metadata
            .insert("news_language".to_string(), "en".to_string());
        let _ = metadata.insert(
            "news_publication_name".to_string(),
            "Test Publication".to_string(),
        );

        let config = NewsSiteMapConfig::new(metadata);
        let generator = NewsSiteMapGenerator::new(config);

        let xml = generator.generate_xml();
        // eprintln!("Generated XML: {}", xml);

        // Ensure required elements exist in the XML
        assert!(xml.contains("<urlset"));
        assert!(xml.contains("<url>"));
        assert!(xml.contains("<loc>https://example.com</loc>"));
        assert!(xml.contains("<news:name>Test Publication</news:name>"));
        assert!(xml.contains("<news:title>Test News</news:title>"));
        assert!(xml.contains("<news:language>en</news:language>"));
        assert!(
        xml.contains("<news:publication_date>2024-02-20T15:15:15Z</news:publication_date>")
            || xml.contains("<news:publication_date>2024-02-20T15:15:15+00:00</news:publication_date>")
    );
    }

    #[test]
    fn test_validate_genres_edge_cases() {
        // All valid genres
        assert_eq!(
            validate_genres("PressRelease, Blog, Opinion"),
            "PressRelease, Blog, Opinion"
        );

        // Mix of valid and invalid genres
        assert_eq!(
            validate_genres("PressRelease, InvalidGenre, Blog"),
            "PressRelease, Blog"
        );

        // Only invalid genres
        assert!(validate_genres("InvalidGenre").is_empty());

        // Empty input
        assert!(validate_genres("").is_empty());
    }

    #[test]
    fn test_validate_keywords_edge_cases() {
        // Valid keywords
        assert_eq!(
            validate_keywords("keyword1, keyword2, keyword3"),
            "keyword1, keyword2, keyword3"
        );

        // Keywords exceeding limit
        assert_eq!(
            validate_keywords("1,2,3,4,5,6,7,8,9,10,11"),
            "1, 2, 3, 4, 5, 6, 7, 8, 9, 10"
        );

        // Empty input
        assert!(validate_keywords("").is_empty());
    }

    #[test]
    fn test_validate_url_edge_cases() {
        // Valid URL with https
        assert_eq!(
            validate_url("https://example.com"),
            "https://example.com"
        );

        // Valid URL with http
        assert_eq!(
            validate_url("http://example.com"),
            "http://example.com"
        );

        // Invalid URL
        assert!(validate_url("not-a-valid-url").is_empty());

        // URL with unsafe characters
        assert!(validate_url("https://example.com<script>").is_empty());
    }

    #[test]
    fn test_sanitize_text_edge_cases() {
        // Normal text
        assert_eq!(sanitize_text("Normal text"), "Normal text");

        // Text with control characters
        assert_eq!(
            sanitize_text("Text\nwith\rcontrols"),
            "Textwithcontrols"
        );

        // Text exceeding length limit
        let long_text = "a".repeat(2000);
        assert_eq!(sanitize_text(&long_text).len(), 1000);
    }

    #[test]
    fn test_to_news_data_empty_metadata() {
        let empty_metadata = HashMap::new();
        let config = NewsSiteMapConfig::new(empty_metadata);
        let news_data = config.to_news_data();

        assert_eq!(news_data.news_title, "Untitled Article");
        assert_eq!(
            news_data.news_publication_name,
            "Unnamed Publication"
        );
        assert_eq!(news_data.news_loc, "");
        assert_eq!(news_data.news_language, "en");
        assert!(news_data.news_genres.is_empty());
        assert!(news_data.news_keywords.is_empty());
        assert!(news_data.news_image_loc.is_empty());
        assert!(news_data.news_publication_date.starts_with(
            &OffsetDateTime::now_utc()
                .format(&format_description::well_known::Rfc3339)
                .unwrap()[..10]
        )); // Fallback date
    }

    #[test]
    fn test_to_news_data_missing_keys() {
        let mut metadata = HashMap::new();
        let _ = metadata.insert(
            "news_title".to_string(),
            "Sample News".to_string(),
        );

        let config = NewsSiteMapConfig::new(metadata);
        let news_data = config.to_news_data();

        assert_eq!(news_data.news_title, "Sample News");
        assert_eq!(
            news_data.news_publication_name,
            "Unnamed Publication"
        );
        assert_eq!(news_data.news_loc, "");
        assert_eq!(news_data.news_language, "en");
        assert!(news_data.news_genres.is_empty());
        assert!(news_data.news_keywords.is_empty());
        assert!(news_data.news_image_loc.is_empty());
    }

    #[test]
    fn test_to_news_data_invalid_metadata() {
        let mut metadata = HashMap::new();
        let _ = metadata.insert(
            "news_title".to_string(),
            "Invalid\nTitle".to_string(),
        );
        let _ = metadata
            .insert("news_loc".to_string(), "invalid-url".to_string());
        let _ = metadata.insert(
            "news_language".to_string(),
            "invalid-lang".to_string(),
        );
        let _ = metadata.insert(
            "news_genres".to_string(),
            "InvalidGenre".to_string(),
        );
        let _ = metadata.insert(
        "news_keywords".to_string(),
        "key1, key2, key3, key4, key5, key6, key7, key8, key9, key10, key11".to_string(),
    );

        let config = NewsSiteMapConfig::new(metadata);
        let news_data = config.to_news_data();

        assert_eq!(news_data.news_title, "InvalidTitle"); // Sanitized
        assert_eq!(news_data.news_loc, ""); // Invalid URL ignored
        assert_eq!(news_data.news_language, "en"); // Invalid language fallback
        assert!(news_data.news_genres.is_empty()); // Invalid genre ignored
        assert_eq!(news_data.news_keywords, "key1, key2, key3, key4, key5, key6, key7, key8, key9, key10");
        // Limited to 10
    }

    #[test]
    fn test_max_length_input() {
        let long_title = "a".repeat(1000); // Max length
        let mut metadata = HashMap::new();
        let _ = metadata
            .insert("news_title".to_string(), long_title.clone());

        let config = NewsSiteMapConfig::new(metadata);
        let news_data = config.to_news_data();

        assert_eq!(news_data.news_title, long_title); // No truncation
    }

    #[test]
    fn test_generate_xml_edge_cases() {
        let mut metadata = HashMap::new();
        let _ = metadata.insert(
            "news_title".to_string(),
            "Edge Case News".to_string(),
        );
        let _ = metadata.insert(
            "news_publication_date".to_string(),
            "Tue, 20 Feb 2024 15:15:15 GMT".to_string(),
        );
        let _ = metadata.insert(
            "news_loc".to_string(),
            "https://example.com".to_string(),
        );
        let _ = metadata
            .insert("news_language".to_string(), "fr".to_string());
        let _ = metadata.insert(
            "news_publication_name".to_string(),
            "Edge Publication".to_string(),
        );

        let config = NewsSiteMapConfig::new(metadata);
        let generator = NewsSiteMapGenerator::new(config);

        let xml = generator.generate_xml();

        assert!(xml.contains("<news:title>Edge Case News</news:title>"));
        assert!(xml.contains("<news:language>fr</news:language>"));
        assert!(xml.contains("<news:publication_date>2024-02-20T15:15:15Z</news:publication_date>"));
    }

    #[test]
    fn test_sanitize_text_control_characters() {
        let input = "Text with control\ncharacters\rand\tspaces.";
        assert_eq!(
            sanitize_text(input),
            "Text with controlcharactersandspaces."
        );
    }

    #[test]
    fn test_validate_url_null_byte() {
        assert!(validate_url("https://example.com/\0evil").is_empty());
    }

    #[test]
    fn test_validate_url_control_chars() {
        assert!(validate_url("https://example.com/\npath").is_empty());
    }

    #[test]
    fn test_validate_url_whitespace() {
        assert!(validate_url("https://example.com/path with spaces")
            .is_empty());
    }

    #[test]
    fn test_validate_url_single_quote() {
        assert!(validate_url("https://example.com/page'injection")
            .is_empty());
    }

    #[test]
    fn test_validate_url_too_long() {
        let long_url =
            format!("https://example.com/{}", "a".repeat(2048));
        assert!(validate_url(&long_url).is_empty());
    }

    mod proptests {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn validate_url_never_panics(
                s in ".*"
            ) {
                let _ = validate_url(&s);
            }

            #[test]
            fn validate_url_rejects_non_http(
                scheme in "[a-z]{2,10}",
                rest in "[a-zA-Z0-9/._-]{0,50}"
            ) {
                let url = format!(
                    "{}://{}",
                    scheme,
                    rest
                );
                if scheme != "http"
                    && scheme != "https"
                {
                    prop_assert!(
                        validate_url(&url).is_empty(),
                        "Non-HTTP URL accepted: {}",
                        url
                    );
                }
            }

            #[test]
            fn validate_language_never_panics(
                s in ".*"
            ) {
                let result = validate_language(&s);
                prop_assert!(
                    !result.is_empty(),
                    "Language validation returned \
                     empty for: {}",
                    s
                );
            }

            #[test]
            fn validate_keywords_limits_to_ten(
                kws in prop::collection::vec(
                    "[a-z]{1,10}",
                    0..30
                )
            ) {
                let input = kws.join(",");
                let result = validate_keywords(&input);
                let count = if result.is_empty() {
                    0
                } else {
                    result.split(", ").count()
                };
                prop_assert!(
                    count <= 10,
                    "Keywords exceeded limit: {}",
                    count
                );
            }

            #[test]
            fn sanitize_text_never_panics(
                s in ".*"
            ) {
                let result = sanitize_text(&s);
                prop_assert!(
                    result.len() <= 4000,
                    "Sanitized text too long"
                );
            }
        }
    }
}
