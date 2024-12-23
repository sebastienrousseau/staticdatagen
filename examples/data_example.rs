// Copyright © 2025 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # StaticDataGen Data Models Examples
//!
//! This program demonstrates the usage of various data models and structures
//! in the StaticDataGen library, including FileData, PageData, SecurityData,
//! and other data types.

use staticdatagen::models::data::{
    CnameData, FileData, HumansData, IconData, ManifestData, NewsData,
    PageData, RssData, SecurityData, TagsData, TxtData,
};

/// Entry point for the StaticDataGen Data Models Examples program.
///
/// Demonstrates the creation and validation of various data structures
/// used in the static site generation process.
///
/// # Errors
///
/// Returns a `Result` containing a `Box<dyn std::error::Error>` if any error
/// occurs during the execution of the examples.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 StaticDataGen Data Models Examples\n");

    file_data_example()?;
    page_data_example()?;
    security_data_example()?;
    humans_data_example()?;
    news_data_example()?;
    rss_data_example()?;
    manifest_data_example()?;
    tags_data_example()?;
    cname_data_example()?;
    robots_txt_example()?;

    println!("\n🎉 All data model examples completed successfully!");

    Ok(())
}

/// Demonstrates FileData model usage.
fn file_data_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 FileData Example");
    println!("---------------------------------------------");

    let file_data = FileData::new(
        "example.md".to_string(),
        "# Example Content\n\nThis is some markdown content."
            .to_string(),
    );

    match file_data.validate() {
        Ok(_) => {
            println!("    ✅ File name: {}", file_data.name);
            println!("    ✅ Is markdown: {}", file_data.is_markdown());
            println!("    ✅ Extension: {:?}", file_data.extension());
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates PageData model usage.
fn page_data_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 PageData Example");
    println!("---------------------------------------------");

    let page_data = PageData::new(
        "Example Page".to_string(),
        "A comprehensive example page".to_string(),
        "2024-02-20T12:00:00Z".to_string(),
        "/example".to_string(),
    );

    match page_data.validate() {
        Ok(_) => {
            println!("    ✅ Title: {}", page_data.title);
            println!("    ✅ Description: {}", page_data.description);
            println!("    ✅ Date: {}", page_data.date);
            println!("    ✅ Permalink: {}", page_data.permalink);
            println!(
                "    ✅ Sanitized title: {}",
                page_data.sanitized_title()
            );
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates SecurityData model usage.
fn security_data_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 SecurityData Example");
    println!("---------------------------------------------");

    let security_data = SecurityData {
        contact: vec![
            "https://example.com/security".to_string(),
            "mailto:security@example.com".to_string(),
        ],
        expires: "2024-12-31T23:59:59Z".to_string(),
        acknowledgments: "https://example.com/thanks".to_string(),
        preferred_languages: "en, fr, de".to_string(),
        canonical: "https://example.com/.well-known/security.txt"
            .to_string(),
        policy: "https://example.com/security-policy".to_string(),
        hiring: "https://example.com/jobs".to_string(),
        encryption: "https://example.com/pgp-key.txt".to_string(),
    };

    match security_data.validate() {
        Ok(_) => {
            println!(
                "    ✅ Contact methods: {}",
                security_data.contact.len()
            );
            println!("    ✅ Valid until: {}", security_data.expires);
            println!(
                "    ✅ Languages: {}",
                security_data.preferred_languages
            );
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates HumansData model usage.
fn humans_data_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 HumansData Example");
    println!("---------------------------------------------");

    let humans_data = HumansData::new(
        "John Doe".to_string(),
        "Thanks to all contributors".to_string(),
    );

    let mut enhanced_data = humans_data;
    enhanced_data.author_website = "https://example.com".to_string();
    enhanced_data.author_twitter = "@johndoe".to_string();
    enhanced_data.author_location = "San Francisco, CA".to_string();
    enhanced_data.site_last_updated = "2024-02-20".to_string();
    enhanced_data.site_standards = "HTML5, CSS3".to_string();
    enhanced_data.site_components = "Rust, SSG".to_string();

    match enhanced_data.validate() {
        Ok(_) => {
            println!("    ✅ Author: {}", enhanced_data.author);
            println!(
                "    ✅ Location: {}",
                enhanced_data.author_location
            );
            println!(
                "    ✅ Website: {}",
                enhanced_data.author_website
            );
            println!(
                "    ✅ Twitter: {}",
                enhanced_data.author_twitter
            );
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates NewsData model usage.
fn news_data_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 NewsData Example");
    println!("---------------------------------------------");

    let news_data = NewsData {
        news_genres: "Blog, OpEd".to_string(),
        news_keywords: "rust, web, ssg".to_string(),
        news_language: "en".to_string(),
        news_image_loc: "https://example.com/image.jpg".to_string(),
        news_loc: "https://example.com/news/article".to_string(),
        news_publication_date: "2024-02-20T12:00:00Z".to_string(),
        news_publication_name: "Example News".to_string(),
        news_title: "Latest Updates".to_string(),
    };

    match news_data.validate() {
        Ok(_) => {
            println!("    ✅ Title: {}", news_data.news_title);
            println!(
                "    ✅ Publication: {}",
                news_data.news_publication_name
            );
            println!("    ✅ Genres: {}", news_data.news_genres);
            println!("    ✅ Language: {}", news_data.news_language);
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates RssData model usage.
fn rss_data_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 RssData Example");
    println!("---------------------------------------------");

    let mut rss_data = RssData::new();
    rss_data.title = "My Blog".to_string();
    rss_data.link = "https://example.com".to_string();
    rss_data.description =
        "A blog about Rust and web development".to_string();
    rss_data.language = "en".to_string();
    rss_data.pub_date = "2024-02-20T12:00:00Z".to_string();
    rss_data.ttl = "60".to_string();

    match rss_data.validate() {
        Ok(_) => {
            println!("    ✅ Feed title: {}", rss_data.title);
            println!("    ✅ Feed link: {}", rss_data.link);
            println!("    ✅ Language: {}", rss_data.language);
            println!("    ✅ TTL: {}", rss_data.ttl);
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates ManifestData model usage.
fn manifest_data_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 ManifestData Example");
    println!("---------------------------------------------");

    let mut manifest = ManifestData::new();
    manifest.name = "My PWA App".to_string();
    manifest.short_name = "MyApp".to_string();
    manifest.start_url = "/".to_string();
    manifest.display = "standalone".to_string();
    manifest.background_color = "#ffffff".to_string();
    manifest.theme_color = "#000000".to_string();

    // Add icons
    let icon = IconData::new(
        "/icons/icon-512x512.png".to_string(),
        "512x512".to_string(),
    );
    manifest.icons.push(icon);

    match manifest.validate() {
        Ok(_) => {
            println!("    ✅ App name: {}", manifest.name);
            println!("    ✅ Short name: {}", manifest.short_name);
            println!("    ✅ Display mode: {}", manifest.display);
            println!("    ✅ Icons: {}", manifest.icons.len());
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates TagsData model usage.
fn tags_data_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 TagsData Example");
    println!("---------------------------------------------");

    let tags_data = TagsData::new(
        "2024-02-20".to_string(),
        "Example Post".to_string(),
        "A post about examples".to_string(),
        "/example".to_string(),
        "example, test, demo".to_string(),
    );

    match tags_data.validate() {
        Ok(_) => {
            println!("    ✅ Title: {}", tags_data.titles);
            println!("    ✅ Date: {}", tags_data.dates);
            println!("    ✅ Keywords: {}", tags_data.keywords);
            println!(
                "    ✅ Keyword list: {:?}",
                tags_data.keywords_list()
            );
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates CnameData model usage.
fn cname_data_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 CnameData Example");
    println!("---------------------------------------------");

    let cname_data = CnameData::new("example.com".to_string());

    match cname_data.validate() {
        Ok(_) => {
            println!("    ✅ Domain: {}", cname_data.cname);
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates TxtData (robots.txt) model usage.
fn robots_txt_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Robots.txt Example");
    println!("---------------------------------------------");

    let txt_data = TxtData::new("https://example.com".to_string());

    match txt_data.validate() {
        Ok(_) => {
            println!("    ✅ Permalink: {}", txt_data.permalink);
            println!("    ✅ Generated content:");
            println!("{}", txt_data.generate_content());
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}
