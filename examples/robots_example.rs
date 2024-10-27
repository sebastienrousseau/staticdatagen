// Copyright © 2024 StaticDataGen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # StaticDataGen Robots.txt Examples
//!
//! This program demonstrates the usage of robots.txt file generation
//! in the StaticDataGen library, showing various ways to create and
//! manage website crawling directives and sitemap references.

use staticdatagen::models::data::TxtData;
use staticdatagen::modules::robots::{
    create_txt_data, generate_txt_content,
};
use std::collections::HashMap;

/// Entry point for the StaticDataGen Robots.txt Examples program.
///
/// Demonstrates various robots.txt file generation scenarios and shows
/// different ways to control search engine crawling behavior.
///
/// # Errors
///
/// Returns a `Result` containing a `Box<dyn std::error::Error>` if any error
/// occurs during the execution of the examples.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 StaticDataGen Robots.txt Examples\n");

    basic_robots_example()?;
    custom_directives_example()?;
    sitemap_reference_example()?;
    subdirectory_rules_example()?;
    multi_sitemap_example()?;
    crawl_delay_example()?;
    user_agent_example()?;
    validation_example()?;

    println!("\n🎉 All robots.txt examples completed successfully!");

    Ok(())
}

/// Demonstrates basic robots.txt file creation.
fn basic_robots_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Basic Robots.txt Example");
    println!("---------------------------------------------");

    let txt_data = TxtData::new("https://example.com".to_string());

    match txt_data.validate() {
        Ok(_) => {
            let content = generate_txt_content(&txt_data);
            println!("    ✅ Generated robots.txt content:");
            println!("{}", content);
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates custom crawling directives.
fn custom_directives_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Custom Directives Example");
    println!("---------------------------------------------");

    let mut metadata = HashMap::new();
    metadata.insert("permalink".to_string(), "https://example.com".to_string());
    metadata.insert("allow".to_string(), "/blog/".to_string());
    metadata.insert("disallow".to_string(), "/admin/".to_string());

    let txt_data = create_txt_data(&metadata);
    match txt_data.validate() {
        Ok(_) => {
            println!("    ✅ Custom directives added:");
            println!("    🌐 Site: {}", txt_data.permalink);
            let content = generate_txt_content(&txt_data);
            println!("    📝 Content:");
            println!("{}", content);
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates sitemap reference addition.
fn sitemap_reference_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Sitemap Reference Example");
    println!("---------------------------------------------");

    let mut metadata = HashMap::new();
    metadata.insert("permalink".to_string(), "https://example.com".to_string());
    metadata.insert("sitemap".to_string(), "https://example.com/sitemap.xml".to_string());

    let txt_data = create_txt_data(&metadata);
    match txt_data.validate() {
        Ok(_) => {
            let content = generate_txt_content(&txt_data);
            println!("    ✅ Generated content with sitemap:");
            println!("{}", content);
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates subdirectory crawling rules.
fn subdirectory_rules_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Subdirectory Rules Example");
    println!("---------------------------------------------");

    let mut metadata = HashMap::new();
    metadata.insert("permalink".to_string(), "https://example.com".to_string());
    metadata.insert("disallow_paths".to_string(),
        "/private/*,/tmp/*,/admin/*".to_string());

    let txt_data = create_txt_data(&metadata);
    match txt_data.validate() {
        Ok(_) => {
            let content = generate_txt_content(&txt_data);
            println!("    ✅ Generated content with path rules:");
            println!("{}", content);
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates multiple sitemap configurations.
fn multi_sitemap_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Multiple Sitemaps Example");
    println!("---------------------------------------------");

    let mut metadata = HashMap::new();
    metadata.insert("permalink".to_string(), "https://example.com".to_string());
    metadata.insert("sitemaps".to_string(),
        "sitemap.xml,news-sitemap.xml,images-sitemap.xml".to_string());

    let txt_data = create_txt_data(&metadata);
    match txt_data.validate() {
        Ok(_) => {
            let content = generate_txt_content(&txt_data);
            println!("    ✅ Generated content with multiple sitemaps:");
            println!("{}", content);
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates crawl delay settings.
fn crawl_delay_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Crawl Delay Example");
    println!("---------------------------------------------");

    let mut metadata = HashMap::new();
    metadata.insert("permalink".to_string(), "https://example.com".to_string());
    metadata.insert("crawl_delay".to_string(), "10".to_string());

    let txt_data = create_txt_data(&metadata);
    match txt_data.validate() {
        Ok(_) => {
            let content = generate_txt_content(&txt_data);
            println!("    ✅ Generated content with crawl delay:");
            println!("{}", content);
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates user agent specific rules.
fn user_agent_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 User Agent Example");
    println!("---------------------------------------------");

    let agents = vec![
        ("*", "Allow: /"),
        ("Googlebot", "Allow: /blog/\nDisallow: /private/"),
        ("Bingbot", "Allow: /public/\nDisallow: /internal/"),
    ];

    for (agent, rules) in agents {
        let mut metadata = HashMap::new();
        metadata.insert("permalink".to_string(), "https://example.com".to_string());
        metadata.insert("user_agent".to_string(), agent.to_string());
        metadata.insert("rules".to_string(), rules.to_string());

        let txt_data = create_txt_data(&metadata);
        match txt_data.validate() {
            Ok(_) => {
                println!("    ✅ Rules for {}: {}", agent, rules);
            }
            Err(e) => println!("    ❌ Error for {}: {:?}", agent, e),
        }
    }

    Ok(())
}

/// Demonstrates validation of robots.txt data.
fn validation_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Validation Example");
    println!("---------------------------------------------");

    let test_cases = vec![
        (
            TxtData::new("".to_string()),
            false,
            "Empty permalink",
        ),
        (
            TxtData::new("https://example.com".to_string()),
            true,
            "Valid permalink",
        ),
        (
            TxtData::new("invalid-url".to_string()),
            false,
            "Invalid URL format",
        ),
        (
            TxtData::new("http://example.com/invalid/path".to_string()),
            false,
            "URL with path",
        ),
    ];

    for (data, should_be_valid, case) in test_cases {
        match data.validate() {
            Ok(_) => {
                if should_be_valid {
                    println!("    ✅ Valid case: {}", case);
                } else {
                    println!(
                        "    ❌ Unexpected validation success: {}",
                        case
                    );
                }
            }
            Err(e) => {
                if !should_be_valid {
                    println!(
                        "    ✅ Expected validation failure: {}",
                        case
                    );
                } else {
                    println!(
                        "    ❌ Unexpected validation error for {}: {:?}",
                        case, e
                    );
                }
            }
        }
    }

    Ok(())
}
