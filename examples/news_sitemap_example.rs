// Copyright © 2025 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # StaticDataGen News Sitemap Examples
//!
//! This program demonstrates the news sitemap generation capabilities
//! of the StaticDataGen library, showcasing various configurations for
//! Google News sitemaps. Each example highlights a specific use case,
//! including handling multiple articles, different genres, multilingual
//! news, image handling, and keyword management.

use staticdatagen::generators::news_sitemap::{
    NewsSiteMapConfig, NewsSiteMapGenerator,
};
use std::collections::HashMap;

/// Ensures required metadata keys are present in the provided metadata.
/// Adds default values for any missing required fields.
///
/// # Arguments
/// * `metadata` - A `HashMap` containing news metadata.
///
/// # Returns
/// * `HashMap<String, String>` - The sanitized metadata with required fields ensured.
fn ensure_required_metadata(
    mut metadata: HashMap<String, String>,
) -> HashMap<String, String> {
    let _ = metadata
        .entry("news_title".to_string())
        .or_insert_with(|| "Untitled Article".to_string());
    let _ = metadata
        .entry("news_publication_date".to_string())
        .or_insert_with(|| "2024-01-01T00:00:00+00:00".to_string());
    metadata
}

/// Entry point for the StaticDataGen News Sitemap Examples program.
///
/// Demonstrates various news sitemap generation scenarios, including:
/// - Basic news entries
/// - Handling multiple articles
/// - News genres
/// - Multilingual news
/// - Image management
/// - Keyword handling
/// - Validation of metadata
///
/// # Errors
/// Returns a `Result` containing a `Box<dyn std::error::Error>` if any error
/// occurs during the execution of the examples.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎉🎉 StaticDataGen News Sitemap Examples\n");

    basic_news_sitemap_example()?;
    multiple_articles_example()?;
    news_genres_example()?;
    multilingual_news_example()?;
    image_handling_example()?;
    keyword_handling_example()?;
    validation_example()?;

    println!("\n✅ All news sitemap examples completed successfully!");

    Ok(())
}

/// Demonstrates basic news sitemap generation.
fn basic_news_sitemap_example() -> Result<(), Box<dyn std::error::Error>>
{
    println!("🦀 Basic News Sitemap Example");
    println!("---------------------------------------------");

    let metadata = ensure_required_metadata(HashMap::from([
        ("news_title".to_string(), "Breaking News Story".to_string()),
        (
            "news_publication_name".to_string(),
            "Example News".to_string(),
        ),
        ("news_language".to_string(), "en".to_string()),
        (
            "news_publication_date".to_string(),
            "2024-02-20T12:00:00+00:00".to_string(),
        ),
    ]));

    let config = NewsSiteMapConfig::new(metadata);
    let generator = NewsSiteMapGenerator::new(config);

    let news_data = generator.generate_xml();
    let sitemap =
        serde_json::to_string_pretty(&news_data).map_err(|e| {
            eprintln!("Critical error serializing news sitemap: {}", e);
            e
        })?;

    println!("    ✅ Generated news sitemap:");
    println!("{}", sitemap);

    Ok(())
}

/// Demonstrates handling multiple news articles.
fn multiple_articles_example() -> Result<(), Box<dyn std::error::Error>>
{
    println!("\n🦀 Multiple Articles Example");
    println!("---------------------------------------------");

    let articles = vec![
        ("Breaking News", "2024-02-20T10:00:00+00:00"),
        ("Technology Update", "2024-02-20T11:30:00+00:00"),
        ("Sports Coverage", "2024-02-20T14:15:00+00:00"),
    ];

    println!("    ✅ Generated entries for multiple articles:");
    for (title, date) in articles {
        let metadata = ensure_required_metadata(HashMap::from([
            ("news_title".to_string(), title.to_string()),
            (
                "news_publication_name".to_string(),
                "Example News".to_string(),
            ),
            ("news_language".to_string(), "en".to_string()),
            ("news_publication_date".to_string(), date.to_string()),
        ]));

        let config = NewsSiteMapConfig::new(metadata);
        let generator = NewsSiteMapGenerator::new(config);

        let news_data = generator.generate_xml();
        let sitemap = serde_json::to_string_pretty(&news_data)
            .map_err(|e| {
                eprintln!(
                    "Critical error serializing news sitemap: {}",
                    e
                );
                e
            })?;

        println!("\n    🧪 Article: {}", title);
        println!("{}", sitemap);
    }

    Ok(())
}

/// Demonstrates different news genres handling.
fn news_genres_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 News Genres Example");
    println!("---------------------------------------------");

    let genres = vec![
        "Blog",
        "PressRelease",
        "Satire",
        "OpEd",
        "Opinion",
        "UserGenerated",
    ];

    for genre in genres {
        let metadata = ensure_required_metadata(HashMap::from([
            ("news_genres".to_string(), genre.to_string()),
            ("news_title".to_string(), format!("{} Article", genre)),
            (
                "news_publication_name".to_string(),
                "Example News".to_string(),
            ),
            ("news_language".to_string(), "en".to_string()),
            (
                "news_publication_date".to_string(),
                "2024-02-20T12:00:00+00:00".to_string(),
            ),
        ]));

        let config = NewsSiteMapConfig::new(metadata);
        let generator = NewsSiteMapGenerator::new(config);

        let news_data = generator.generate_xml();
        let sitemap = serde_json::to_string_pretty(&news_data)
            .map_err(|e| {
                eprintln!(
                    "Critical error serializing news sitemap: {}",
                    e
                );
                e
            })?;

        println!("    ✅ Generated entry for genre: {}", genre);
        println!("{}", sitemap);
    }

    Ok(())
}

/// Demonstrates multilingual news handling.
fn multilingual_news_example() -> Result<(), Box<dyn std::error::Error>>
{
    println!("\n🦀 Multilingual News Example");
    println!("---------------------------------------------");

    let languages = vec![
        ("en", "English News", "Breaking Story"),
        ("fr", "Nouvelles en Français", "Actualité"),
        ("de", "Deutsche Nachrichten", "Nachricht"),
        ("es", "Noticias en Español", "Noticia"),
    ];

    for (lang, pub_name, title) in languages {
        let metadata = ensure_required_metadata(HashMap::from([
            ("news_language".to_string(), lang.to_string()),
            ("news_publication_name".to_string(), pub_name.to_string()),
            ("news_title".to_string(), title.to_string()),
            (
                "news_publication_date".to_string(),
                "2024-02-20T12:00:00+00:00".to_string(),
            ),
        ]));

        let config = NewsSiteMapConfig::new(metadata);
        let generator = NewsSiteMapGenerator::new(config);

        let news_data = generator.generate_xml();
        let sitemap = serde_json::to_string_pretty(&news_data)
            .map_err(|e| {
                eprintln!(
                    "Critical error serializing news sitemap: {}",
                    e
                );
                e
            })?;

        println!("    ✅ Generated entry for language: {}", lang);
        println!("{}", sitemap);
    }

    Ok(())
}

/// Demonstrates image handling in news sitemaps.
fn image_handling_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Image Handling Example");
    println!("---------------------------------------------");

    let image_types = vec![
        ("featured", "hero-image.jpg", "Main story image"),
        ("gallery", "gallery-1.jpg", "Photo gallery image"),
        ("thumbnail", "thumb.jpg", "Thumbnail image"),
    ];

    for (type_name, filename, desc) in image_types {
        let metadata = ensure_required_metadata(HashMap::from([
            ("news_genres".to_string(), "Blog".to_string()),
            ("news_keywords".to_string(), "news, photos".to_string()),
            ("news_language".to_string(), "en".to_string()),
            (
                "news_image_loc".to_string(),
                format!(
                    "https://example.com/images/{}/{}",
                    type_name, filename
                ),
            ),
            (
                "news_loc".to_string(),
                format!(
                    "https://example.com/article-with-{}",
                    type_name
                ),
            ),
            (
                "news_publication_date".to_string(),
                "2024-02-20T12:00:00+00:00".to_string(),
            ),
            (
                "news_publication_name".to_string(),
                "Example News".to_string(),
            ),
            (
                "news_title".to_string(),
                format!("Article with {}", desc),
            ),
        ]));

        let config = NewsSiteMapConfig::new(metadata);
        let generator = NewsSiteMapGenerator::new(config);

        let news_data = generator.generate_xml();
        let sitemap = serde_json::to_string_pretty(&news_data)
            .map_err(|e| {
                eprintln!(
                    "Critical error serializing news sitemap: {}",
                    e
                );
                e
            })?;

        println!("    ✅ Generated entry with {}", desc);
        println!("{}", sitemap);
    }

    Ok(())
}

/// Demonstrates keyword handling in news sitemaps.
fn keyword_handling_example() -> Result<(), Box<dyn std::error::Error>>
{
    println!("\n🦀 Keyword Handling Example");
    println!("---------------------------------------------");

    let keyword_sets = vec![
        "technology, innovation",
        "sports, football, soccer",
        "politics, world news, government",
        "entertainment, movies, music",
    ];

    for keywords in keyword_sets {
        let metadata = ensure_required_metadata(HashMap::from([
            ("news_keywords".to_string(), keywords.to_string()),
            (
                "news_title".to_string(),
                format!(
                    "Article about {}",
                    keywords.split(',').next().unwrap()
                ),
            ),
            (
                "news_publication_name".to_string(),
                "Example News".to_string(),
            ),
            ("news_language".to_string(), "en".to_string()),
            (
                "news_publication_date".to_string(),
                "2024-02-20T12:00:00+00:00".to_string(),
            ),
        ]));

        let config = NewsSiteMapConfig::new(metadata);
        let generator = NewsSiteMapGenerator::new(config);

        let news_data = generator.generate_xml();
        let sitemap = serde_json::to_string_pretty(&news_data)
            .map_err(|e| {
                eprintln!(
                    "Critical error serializing news sitemap: {}",
                    e
                );
                e
            })?;

        println!("    ✅ Generated entry with keywords: {}", keywords);
        println!("{}", sitemap);
    }

    Ok(())
}

/// Demonstrates validation scenarios for news sitemaps.
fn validation_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Validation Example");
    println!("---------------------------------------------");

    let test_cases = vec![
        (
            ensure_required_metadata(HashMap::from([
                ("news_genres".to_string(), "Blog".to_string()),
                ("news_title".to_string(), "Valid Article".to_string()),
                (
                    "news_publication_name".to_string(),
                    "Example News".to_string(),
                ),
                ("news_language".to_string(), "en".to_string()),
                (
                    "news_publication_date".to_string(),
                    "2024-02-20T12:00:00+00:00".to_string(),
                ),
            ])),
            true,
            "Valid news data",
        ),
        (
            HashMap::from([(
                "news_genres".to_string(),
                "InvalidGenre".to_string(),
            )]),
            false,
            "Invalid genre",
        ),
        (
            HashMap::from([(
                "news_language".to_string(),
                "invalid".to_string(),
            )]),
            false,
            "Invalid language code",
        ),
    ];

    for (metadata, should_be_valid, case) in test_cases {
        let config = NewsSiteMapConfig::new(metadata);
        let generator = NewsSiteMapGenerator::new(config);
        let news_data = generator.config.to_news_data();

        let sitemap = serde_json::to_string_pretty(&news_data)
            .unwrap_or_else(|e| {
                eprintln!("Error serializing news sitemap: {}", e);
                String::new()
            });

        if news_data.news_title.is_empty()
            || news_data.news_publication_date.is_empty()
        {
            println!("    ❌ {}: Missing required fields", case);
        } else if sitemap.contains("news:news") == should_be_valid {
            println!("    ✅ {}: Validation working as expected", case);
        } else {
            println!("    ❌ {}: Unexpected validation result", case);
        }
    }

    Ok(())
}
