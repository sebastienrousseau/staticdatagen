// Copyright © 2024 StaticDataGen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # StaticDataGen News Sitemap Examples
//!
//! This program demonstrates the news sitemap generation capabilities
//! of the StaticDataGen library, showing various configurations for
//! Google News sitemaps.

use staticdatagen::models::data::{NewsData, NewsVisitOptions};
use staticdatagen::modules::json::generate_news_sitemap_entry;
use staticdatagen::modules::json::news_sitemap;
use staticdatagen::modules::news_sitemap::create_news_site_map_data;
use std::collections::HashMap;

/// Entry point for the StaticDataGen News Sitemap Examples program.
///
/// Demonstrates various news sitemap generation scenarios including
/// basic news entries, multiple languages, and different genres.
///
/// # Errors
///
/// Returns a `Result` containing a `Box<dyn std::error::Error>` if any error
/// occurs during the execution of the examples.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 StaticDataGen News Sitemap Examples\n");

    basic_news_sitemap_example()?;
    multiple_articles_example()?;
    news_genres_example()?;
    multilingual_news_example()?;
    image_handling_example()?;
    publication_dates_example()?;
    keyword_handling_example()?;
    validation_example()?;

    println!("\n🎉 All news sitemap examples completed successfully!");

    Ok(())
}

/// Demonstrates basic news sitemap generation.
fn basic_news_sitemap_example() -> Result<(), Box<dyn std::error::Error>>
{
    println!("🦀 Basic News Sitemap Example");
    println!("---------------------------------------------");

    let mut metadata = HashMap::new();
    let _ = metadata.insert(
        "news_title".to_string(),
        "Breaking News Story".to_string(),
    );
    let _ = metadata.insert(
        "news_publication_name".to_string(),
        "Example News".to_string(),
    );
    let _ =
        metadata.insert("news_language".to_string(), "en".to_string());
    let _ = metadata.insert(
        "news_publication_date".to_string(),
        "2024-02-20T12:00:00+00:00".to_string(),
    );

    let news_data = create_news_site_map_data(&metadata);
    let sitemap = news_sitemap(news_data);
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
        let options = NewsVisitOptions::new(
            "https://example.com",
            "Blog",
            "news, updates",
            "en",
            date,
            "Example News",
            title,
        );

        let entry = generate_news_sitemap_entry(&options);
        println!("\n    📰 Article: {}", title);
        println!("{}", entry);
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
        let news_data = NewsData {
            news_genres: genre.to_string(),
            news_keywords: "example, test".to_string(),
            news_language: "en".to_string(),
            news_image_loc: "https://example.com/image.jpg".to_string(),
            news_loc: "https://example.com/article".to_string(),
            news_publication_date: "2024-02-20T12:00:00+00:00"
                .to_string(),
            news_publication_name: "Example News".to_string(),
            news_title: format!("{} Article", genre),
        };

        println!("    ✅ Generated entry for genre: {}", genre);
        let sitemap = news_sitemap(news_data);
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
        let news_data = NewsData {
            news_genres: "Blog".to_string(),
            news_keywords: "news, international".to_string(),
            news_language: lang.to_string(),
            news_image_loc: "https://example.com/image.jpg".to_string(),
            news_loc: format!("https://example.com/{}/article", lang),
            news_publication_date: "2024-02-20T12:00:00+00:00"
                .to_string(),
            news_publication_name: pub_name.to_string(),
            news_title: title.to_string(),
        };

        println!("    ✅ Generated entry for language: {}", lang);
        let sitemap = news_sitemap(news_data);
        println!("{}", sitemap);
    }

    Ok(())
}

/// Demonstrates news image handling.
fn image_handling_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Image Handling Example");
    println!("---------------------------------------------");

    let image_types = vec![
        ("featured", "hero-image.jpg", "Main story image"),
        ("gallery", "gallery-1.jpg", "Photo gallery image"),
        ("thumbnail", "thumb.jpg", "Thumbnail image"),
    ];

    for (type_name, filename, desc) in image_types {
        let news_data = NewsData {
            news_genres: "Blog".to_string(),
            news_keywords: "news, photos".to_string(),
            news_language: "en".to_string(),
            news_image_loc: format!(
                "https://example.com/images/{}/{}",
                type_name, filename
            ),
            news_loc: format!(
                "https://example.com/article-with-{}",
                type_name
            ),
            news_publication_date: "2024-02-20T12:00:00+00:00"
                .to_string(),
            news_publication_name: "Example News".to_string(),
            news_title: format!("Article with {}", desc),
        };

        println!("    ✅ Generated entry with {}", desc);
        let sitemap = news_sitemap(news_data);
        println!("{}", sitemap);
    }

    Ok(())
}

/// Demonstrates publication date handling.
fn publication_dates_example() -> Result<(), Box<dyn std::error::Error>>
{
    println!("\n🦀 Publication Dates Example");
    println!("---------------------------------------------");

    let dates = [
        "2024-02-20T09:00:00+00:00",
        "2024-02-20T12:30:00+00:00",
        "2024-02-20T15:45:00+00:00",
        "2024-02-20T18:15:00+00:00",
    ];

    for (i, date) in dates.iter().enumerate() {
        let news_data = NewsData {
            news_genres: "Blog".to_string(),
            news_keywords: "news, timeline".to_string(),
            news_language: "en".to_string(),
            news_image_loc: "https://example.com/image.jpg".to_string(),
            news_loc: format!("https://example.com/article-{}", i + 1),
            news_publication_date: date.to_string(),
            news_publication_name: "Example News".to_string(),
            news_title: format!("Article {}", i + 1),
        };

        println!("    ✅ Generated entry for date: {}", date);
        let sitemap = news_sitemap(news_data);
        println!("{}", sitemap);
    }

    Ok(())
}

/// Demonstrates keyword handling.
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
        let news_data = NewsData {
            news_genres: "Blog".to_string(),
            news_keywords: keywords.to_string(),
            news_language: "en".to_string(),
            news_image_loc: "https://example.com/image.jpg".to_string(),
            news_loc: "https://example.com/article".to_string(),
            news_publication_date: "2024-02-20T12:00:00+00:00"
                .to_string(),
            news_publication_name: "Example News".to_string(),
            news_title: format!(
                "Article about {}",
                keywords.split(',').next().unwrap()
            ),
        };

        println!("    ✅ Generated entry with keywords: {}", keywords);
        let sitemap = news_sitemap(news_data);
        println!("{}", sitemap);
    }

    Ok(())
}

/// Demonstrates validation scenarios.
fn validation_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Validation Example");
    println!("---------------------------------------------");

    let test_cases = vec![
        (
            NewsData {
                news_genres: "Blog".to_string(),
                news_keywords: "valid, test".to_string(),
                news_language: "en".to_string(),
                news_image_loc: "https://example.com/image.jpg"
                    .to_string(),
                news_loc: "https://example.com/article".to_string(),
                news_publication_date: "2024-02-20T12:00:00+00:00"
                    .to_string(),
                news_publication_name: "Example News".to_string(),
                news_title: "Valid Article".to_string(),
            },
            true,
            "Valid news data",
        ),
        (
            NewsData {
                news_genres: "InvalidGenre".to_string(),
                ..Default::default()
            },
            false,
            "Invalid genre",
        ),
        (
            NewsData {
                news_language: "invalid".to_string(),
                ..Default::default()
            },
            false,
            "Invalid language code",
        ),
    ];

    for (data, should_be_valid, case) in test_cases {
        let sitemap = news_sitemap(data);
        if sitemap.contains("news:news") == should_be_valid {
            println!("    ✅ {}: Validation working as expected", case);
        } else {
            println!("    ❌ {}: Unexpected validation result", case);
        }
    }

    Ok(())
}
