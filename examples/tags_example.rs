// Copyright © 2024 StaticDataGen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # StaticDataGen Tags Examples
//!
//! This program demonstrates the usage of tag generation and handling
//! in the StaticDataGen library, showing various ways to create and
//! manage content tags, tag pages, and tag-based navigation.

use staticdatagen::models::data::{FileData, PageData, TagsData};
use staticdatagen::modules::tags::{
    create_tags_data, generate_tags, generate_tags_html,
};
use std::collections::HashMap;

/// Entry point for the StaticDataGen Tags Examples program.
///
/// Demonstrates various tag generation scenarios and shows different
/// ways to organize and display content tags.
///
/// # Errors
///
/// Returns a `Result` containing a `Box<dyn std::error::Error>` if any error
/// occurs during the execution of the examples.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 StaticDataGen Tags Examples\n");

    basic_tags_example()?;
    multiple_tags_example()?;
    tag_page_example()?;
    hierarchical_tags_example()?;
    tag_cloud_example()?;
    tag_metadata_example()?;
    tag_filtering_example()?;
    validation_example()?;

    println!("\n🎉 All tags examples completed successfully!");

    Ok(())
}

/// Demonstrates basic tag generation.
fn basic_tags_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Basic Tags Example");
    println!("---------------------------------------------");

    let file = FileData::new(
        "post.md".to_string(),
        "A post about Rust and web development".to_string(),
    );

    let mut metadata = HashMap::new();
    metadata.insert("tags".to_string(), "rust, web".to_string());
    metadata.insert("title".to_string(), "Learning Rust".to_string());
    metadata.insert("date".to_string(), "2024-02-20".to_string());

    let tags = generate_tags(&file, &metadata);
    match TagsData::new(
        "2024-02-20".to_string(),
        "Learning Rust".to_string(),
        "A post about Rust".to_string(),
        "/blog/learning-rust".to_string(),
        "rust, web".to_string(),
    )
    .validate()
    {
        Ok(_) => {
            println!("    ✅ Generated tags:");
            for (tag, pages) in &tags {
                println!("    🏷️ {}: {} pages", tag, pages.len());
            }
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates handling multiple tags per content.
fn multiple_tags_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Multiple Tags Example");
    println!("---------------------------------------------");

    let posts = vec![
        (
            "Rust Web Development",
            "rust, web, programming",
            "/blog/rust-web",
        ),
        (
            "Advanced Rust Tips",
            "rust, advanced, tips",
            "/blog/rust-tips",
        ),
        (
            "Web Security",
            "web, security, best-practices",
            "/blog/web-security",
        ),
    ];

    let mut global_tags_data: HashMap<String, Vec<PageData>> =
        HashMap::new();

    for (title, tags, permalink) in posts {
        let mut metadata = HashMap::new();
        metadata.insert("title".to_string(), title.to_string());
        metadata.insert("tags".to_string(), tags.to_string());
        metadata.insert("permalink".to_string(), permalink.to_string());

        let page_data = PageData::new(
            title.to_string(),
            "Description".to_string(),
            "2024-02-20".to_string(),
            permalink.to_string(),
        );

        for tag in tags.split(", ") {
            global_tags_data
                .entry(tag.to_string())
                .or_default()
                .push(page_data.clone());
        }
    }

    println!("    ✅ Generated tag groupings:");
    for (tag, pages) in &global_tags_data {
        println!("    🏷️ {}: {} articles", tag, pages.len());
    }

    Ok(())
}

/// Demonstrates tag page generation.
fn tag_page_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Tag Page Example");
    println!("---------------------------------------------");

    let mut tag_pages: HashMap<String, Vec<PageData>> = HashMap::new();

    // Add some sample pages for a tag
    tag_pages.insert(
        "rust".to_string(),
        vec![
            PageData::new(
                "Rust Basics".to_string(),
                "Introduction to Rust".to_string(),
                "2024-02-20".to_string(),
                "/blog/rust-basics".to_string(),
            ),
            PageData::new(
                "Advanced Rust".to_string(),
                "Advanced Rust concepts".to_string(),
                "2024-02-21".to_string(),
                "/blog/advanced-rust".to_string(),
            ),
        ],
    );

    let html_content = generate_tags_html(&tag_pages);
    println!("    ✅ Generated tag page HTML:");
    println!("{}", html_content);

    Ok(())
}

/// Demonstrates hierarchical tag organization.
fn hierarchical_tags_example() -> Result<(), Box<dyn std::error::Error>>
{
    println!("\n🦀 Hierarchical Tags Example");
    println!("---------------------------------------------");

    let hierarchical_tags = vec![
        "programming/rust/basics",
        "programming/rust/advanced",
        "programming/web/frontend",
        "programming/web/backend",
    ];

    for tag_path in hierarchical_tags {
        let mut metadata = HashMap::new();
        metadata.insert("tags".to_string(), tag_path.to_string());

        let tags_data = create_tags_data(&metadata);
        println!("    ✅ Processed tag path: {}", tag_path);
        println!("       Keywords: {}", tags_data.keywords);
    }

    Ok(())
}

/// Demonstrates tag cloud generation.
fn tag_cloud_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Tag Cloud Example");
    println!("---------------------------------------------");

    let mut tag_counts: HashMap<String, usize> = HashMap::new();

    // Sample tag frequencies
    tag_counts.insert("rust".to_string(), 15);
    tag_counts.insert("web".to_string(), 10);
    tag_counts.insert("programming".to_string(), 8);
    tag_counts.insert("tutorial".to_string(), 5);

    println!("    ✅ Tag cloud weights:");
    for (tag, count) in &tag_counts {
        let weight = match count {
            0..=5 => "small",
            6..=10 => "medium",
            _ => "large",
        };
        println!("    🏷️ {} ({}): {}", tag, count, weight);
    }

    Ok(())
}

/// Demonstrates tag metadata handling.
fn tag_metadata_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Tag Metadata Example");
    println!("---------------------------------------------");

    let tags_data = TagsData::new(
        "2024-02-20".to_string(),
        "Programming Topics".to_string(),
        "Collection of programming articles".to_string(),
        "/tags".to_string(),
        "programming, development, tutorials".to_string(),
    );

    match tags_data.validate() {
        Ok(_) => {
            println!("    ✅ Tag metadata:");
            println!("    📅 Date: {}", tags_data.dates);
            println!("    📝 Title: {}", tags_data.titles);
            println!("    🔗 Permalink: {}", tags_data.permalinks);
            println!("    🏷️ Keywords: {}", tags_data.keywords);
            println!(
                "    Keywords list: {:?}",
                tags_data.keywords_list()
            );
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates tag filtering and sorting.
fn tag_filtering_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Tag Filtering Example");
    println!("---------------------------------------------");

    let mut tag_pages: HashMap<String, Vec<PageData>> = HashMap::new();

    // Add sample pages with dates
    let rust_pages = vec![
        PageData::new(
            "Recent Rust".to_string(),
            "Recent post".to_string(),
            "2024-02-20".to_string(),
            "/blog/recent".to_string(),
        ),
        PageData::new(
            "Old Rust".to_string(),
            "Older post".to_string(),
            "2024-01-01".to_string(),
            "/blog/old".to_string(),
        ),
    ];

    tag_pages.insert("rust".to_string(), rust_pages);

    println!("    ✅ Filtered and sorted pages:");
    for (tag, mut pages) in tag_pages {
        // Sort pages by date
        pages.sort_by(|a, b| b.date.cmp(&a.date));

        println!("    🏷️ {}:", tag);
        for page in pages {
            println!("       📅 {} - {}", page.date, page.title);
        }
    }

    Ok(())
}

/// Demonstrates validation of tag data.
fn validation_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Validation Example");
    println!("---------------------------------------------");

    let test_cases = vec![
        (
            TagsData::new(
                "invalid-date".to_string(),
                "Title".to_string(),
                "Description".to_string(),
                "/permalink".to_string(),
                "tags".to_string(),
            ),
            false,
            "Invalid date format",
        ),
        (
            TagsData::new(
                "2024-02-20".to_string(),
                "".to_string(),
                "Description".to_string(),
                "/permalink".to_string(),
                "tags".to_string(),
            ),
            false,
            "Empty title",
        ),
        (
            TagsData::new(
                "2024-02-20".to_string(),
                "Title".to_string(),
                "Description".to_string(),
                "invalid-permalink".to_string(),
                "tags".to_string(),
            ),
            false,
            "Invalid permalink format",
        ),
        (
            TagsData::new(
                "2024-02-20".to_string(),
                "Valid Title".to_string(),
                "Valid Description".to_string(),
                "/valid/permalink".to_string(),
                "valid, tags".to_string(),
            ),
            true,
            "Valid tag data",
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
