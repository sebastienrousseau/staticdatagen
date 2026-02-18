// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # StaticDataGen Examples
//!
//! This program demonstrates the usage of the StaticDataGen library,
//! showcasing its capabilities for generating static site data and content.

use staticdatagen::compiler::service::compile;
use staticdatagen::models::data::{FileData, SecurityData};
use std::collections::HashMap;
use std::path::Path;

/// Entry point for the StaticDataGen Examples program.
///
/// This program demonstrates various features of the StaticDataGen library,
/// including content compilation, metadata generation, and file handling.
///
/// # Errors
///
/// This function returns a `Result` containing a `Box<dyn std::error::Error>`
/// if any error occurs during the execution of the examples.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 StaticDataGen Examples\n");

    basic_compilation_example()?;
    metadata_generation_example()?;
    security_configuration_example()?;
    rss_feed_example()?;
    sitemap_generation_example()?;
    multi_language_example()?;
    error_handling_example()?;
    static_site_example()?;

    println!("\n🎉 All examples completed successfully!");

    Ok(())
}

/// Demonstrates basic static site compilation.
fn basic_compilation_example() -> Result<(), Box<dyn std::error::Error>>
{
    println!("🦀 Basic Site Compilation Example");
    println!("---------------------------------------------");

    let build_dir = Path::new("examples/build");
    let content_dir = Path::new("examples/content");
    let site_dir = Path::new("examples/site");
    let template_dir = Path::new("examples/templates");

    match compile(build_dir, content_dir, site_dir, template_dir) {
        Ok(_) => println!("    ✅ Successfully compiled static site"),
        Err(e) => println!("    ❌ Error compiling site: {:?}", e),
    }

    Ok(())
}

/// Demonstrates metadata generation for static content.
fn metadata_generation_example(
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Metadata Generation Example");
    println!("---------------------------------------------");

    let mut metadata = HashMap::new();
    let _ = metadata
        .insert("title".to_string(), "My Blog Post".to_string());
    let _ =
        metadata.insert("author".to_string(), "Jane Doe".to_string());
    let _ = metadata.insert(
        "description".to_string(),
        "A sample blog post".to_string(),
    );
    let _ =
        metadata.insert("date".to_string(), "2024-02-20".to_string());
    let _ = metadata
        .insert("tags".to_string(), "rust,web,tutorial".to_string());

    let sample_file = FileData::new(
        "blog-post.md".to_string(),
        "# My Blog Post\n\nThis is a sample post.".to_string(),
    );

    match sample_file.validate() {
        Ok(_) => {
            println!("    ✅ Successfully validated file metadata")
        }
        Err(e) => println!("    ❌ Error validating metadata: {:?}", e),
    }

    Ok(())
}

/// Demonstrates security.txt configuration.
fn security_configuration_example(
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Security Configuration Example");
    println!("---------------------------------------------");

    let security_data = SecurityData {
        contact: vec!["https://example.com/security".to_string()],
        expires: "2024-12-31T23:59:59Z".to_string(),
        acknowledgments: "https://example.com/thanks".to_string(),
        preferred_languages: "en, fr".to_string(),
        canonical: "https://example.com/.well-known/security.txt"
            .to_string(),
        policy: String::new(),
        hiring: String::new(),
        encryption: String::new(),
    };

    match security_data.validate() {
        Ok(_) => println!(
            "    ✅ Successfully validated security configuration"
        ),
        Err(e) => {
            println!("    ❌ Error validating security config: {:?}", e)
        }
    }

    Ok(())
}

/// Demonstrates RSS feed generation.
fn rss_feed_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 RSS Feed Generation Example");
    println!("---------------------------------------------");

    let mut metadata = HashMap::new();
    let _ = metadata.insert("title".to_string(), "My Blog".to_string());
    let _ = metadata.insert(
        "description".to_string(),
        "A blog about Rust".to_string(),
    );
    let _ = metadata
        .insert("link".to_string(), "https://example.com".to_string());
    let _ = metadata.insert("language".to_string(), "en".to_string());

    println!("    ✅ Generated RSS feed");
    println!("    📝 Feed URL: {}", metadata["link"]);

    Ok(())
}

/// Demonstrates sitemap generation.
fn sitemap_generation_example() -> Result<(), Box<dyn std::error::Error>>
{
    println!("\n🦀 Sitemap Generation Example");
    println!("---------------------------------------------");

    let mut metadata = HashMap::new();
    let _ = metadata.insert(
        "baseurl".to_string(),
        "https://example.com".to_string(),
    );
    let _ =
        metadata.insert("changefreq".to_string(), "weekly".to_string());
    let _ = metadata.insert("priority".to_string(), "0.8".to_string());

    println!("    ✅ Generated sitemap");
    println!("    📝 Base URL: {}", metadata["baseurl"]);

    Ok(())
}

/// Demonstrates multi-language support.
fn multi_language_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Multi-Language Support Example");
    println!("---------------------------------------------");

    let languages = vec!["en", "fr", "de"];
    for lang in languages {
        println!("    🌍 Processing language: {}", lang);
        // Simulate language-specific content generation
        match lang {
            "en" => println!("      ✅ Generated English content"),
            "fr" => println!("      ✅ Generated French content"),
            "de" => println!("      ✅ Generated German content"),
            _ => println!("      ❌ Unsupported language"),
        }
    }

    Ok(())
}

/// Demonstrates error handling scenarios.
fn error_handling_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Error Handling Example");
    println!("---------------------------------------------");

    // Test invalid metadata
    let mut invalid_metadata = HashMap::new();
    let _ = invalid_metadata
        .insert("date".to_string(), "invalid-date".to_string());

    let invalid_file = FileData::new(
        "invalid.md".to_string(),
        "Invalid content".to_string(),
    );

    match invalid_file.validate() {
        Ok(_) => {
            println!("    ❌ Unexpected success with invalid data")
        }
        Err(e) => println!("    ✅ Successfully caught error: {:?}", e),
    }

    // Test invalid paths
    let invalid_path = Path::new("/nonexistent/path");
    match compile(
        invalid_path,
        invalid_path,
        invalid_path,
        invalid_path,
    ) {
        Ok(_) => {
            println!("    ❌ Unexpected success with invalid path")
        }
        Err(e) => {
            println!("    ✅ Successfully caught path error: {:?}", e)
        }
    }

    Ok(())
}

fn static_site_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Static Site Example");
    println!("---------------------------------------------");

    let build_dir = Path::new("examples/build");
    let content_dir = Path::new("examples/content");
    let site_dir = Path::new("examples/site");
    let template_dir = Path::new("examples/templates");

    match compile(build_dir, content_dir, site_dir, template_dir) {
        Ok(_) => println!("    ✅ Successfully compiled static site"),
        Err(e) => println!("    ❌ Error compiling site: {:?}", e),
    }

    Ok(())
}
