// Copyright © 2025 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # StaticDataGen Navigation Examples
//!
//! This program demonstrates the navigation generation capabilities
//! of the StaticDataGen library, showing various ways to create
//! and structure site navigation.

use staticdatagen::models::data::FileData;
use staticdatagen::modules::navigation::NavigationGenerator;

/// Entry point for the StaticDataGen Navigation Examples program.
///
/// Demonstrates various navigation generation scenarios including
/// basic navigation, hierarchical structures, and custom configurations.
///
/// # Errors
///
/// Returns a `Result` containing a `Box<dyn std::error::Error>` if any error
/// occurs during the execution of the examples.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 StaticDataGen Navigation Examples\n");

    basic_navigation_example()?;
    hierarchical_navigation_example()?;
    blog_navigation_example()?;
    docs_navigation_example()?;
    i18n_navigation_example()?;
    custom_sorting_example()?;
    excluded_files_example()?;
    validation_example()?;

    println!("\n🎉 All navigation examples completed successfully!");

    Ok(())
}

/// Demonstrates basic navigation generation.
fn basic_navigation_example() -> Result<(), Box<dyn std::error::Error>>
{
    println!("🦀 Basic Navigation Example");
    println!("---------------------------------------------");

    let files = vec![
        FileData::new(
            "about.md".to_string(),
            "# About\nAbout page content".to_string(),
        ),
        FileData::new(
            "contact.md".to_string(),
            "# Contact\nContact page content".to_string(),
        ),
        FileData::new(
            "services.md".to_string(),
            "# Services\nServices page content".to_string(),
        ),
    ];

    let nav = NavigationGenerator::generate_navigation(&files);
    println!("    ✅ Generated navigation:");
    println!("{}", nav);

    Ok(())
}

/// Demonstrates hierarchical navigation structure.
fn hierarchical_navigation_example(
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Hierarchical Navigation Example");
    println!("---------------------------------------------");

    let files = vec![
        FileData::new(
            "about/team.md".to_string(),
            "# Team\nTeam page content".to_string(),
        ),
        FileData::new(
            "about/history.md".to_string(),
            "# History\nHistory page content".to_string(),
        ),
        FileData::new(
            "services/consulting.md".to_string(),
            "# Consulting\nConsulting services".to_string(),
        ),
        FileData::new(
            "services/training.md".to_string(),
            "# Training\nTraining services".to_string(),
        ),
    ];

    let nav = NavigationGenerator::generate_navigation(&files);
    println!("    ✅ Generated hierarchical navigation:");
    println!("{}", nav);

    Ok(())
}

/// Demonstrates blog navigation structure.
fn blog_navigation_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Blog Navigation Example");
    println!("---------------------------------------------");

    let files = vec![
        FileData::new(
            "blog/2024/01/first-post.md".to_string(),
            "# First Post\nFirst blog post content".to_string(),
        ),
        FileData::new(
            "blog/2024/02/second-post.md".to_string(),
            "# Second Post\nSecond blog post content".to_string(),
        ),
        FileData::new(
            "blog/categories/rust.md".to_string(),
            "# Rust Category\nRust related posts".to_string(),
        ),
        FileData::new(
            "blog/tags/tutorial.md".to_string(),
            "# Tutorial Tag\nTutorial posts".to_string(),
        ),
    ];

    let nav = NavigationGenerator::generate_navigation(&files);
    println!("    ✅ Generated blog navigation:");
    println!("{}", nav);

    Ok(())
}

/// Demonstrates documentation navigation structure.
fn docs_navigation_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Documentation Navigation Example");
    println!("---------------------------------------------");

    let files = vec![
        FileData::new(
            "docs/getting-started.md".to_string(),
            "# Getting Started\nGetting started guide".to_string(),
        ),
        FileData::new(
            "docs/api/reference.md".to_string(),
            "# API Reference\nAPI documentation".to_string(),
        ),
        FileData::new(
            "docs/tutorials/basic.md".to_string(),
            "# Basic Tutorial\nBasic tutorial content".to_string(),
        ),
        FileData::new(
            "docs/tutorials/advanced.md".to_string(),
            "# Advanced Tutorial\nAdvanced tutorial content"
                .to_string(),
        ),
    ];

    let nav = NavigationGenerator::generate_navigation(&files);
    println!("    ✅ Generated documentation navigation:");
    println!("{}", nav);

    Ok(())
}

/// Demonstrates multi-language navigation.
fn i18n_navigation_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Internationalized Navigation Example");
    println!("---------------------------------------------");

    let files = vec![
        // English
        FileData::new(
            "en/about.md".to_string(),
            "# About\nAbout us".to_string(),
        ),
        FileData::new(
            "en/contact.md".to_string(),
            "# Contact\nContact us".to_string(),
        ),
        // French
        FileData::new(
            "fr/about.md".to_string(),
            "# À propos\nÀ propos de nous".to_string(),
        ),
        FileData::new(
            "fr/contact.md".to_string(),
            "# Contact\nContactez-nous".to_string(),
        ),
        // German
        FileData::new(
            "de/about.md".to_string(),
            "# Über uns\nÜber uns".to_string(),
        ),
        FileData::new(
            "de/contact.md".to_string(),
            "# Kontakt\nKontaktieren Sie uns".to_string(),
        ),
    ];

    for lang in ["en", "fr", "de"] {
        let lang_files: Vec<FileData> = files
            .iter()
            .filter(|f| f.name.starts_with(&format!("{}/", lang)))
            .cloned()
            .collect();

        let nav = NavigationGenerator::generate_navigation(&lang_files);
        println!(
            "    ✅ Generated {} navigation:",
            lang.to_uppercase()
        );
        println!("{}", nav);
    }

    Ok(())
}

/// Demonstrates custom navigation sorting.
fn custom_sorting_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Custom Sorting Example");
    println!("---------------------------------------------");

    let mut files = vec![
        FileData::new(
            "03-advanced.md".to_string(),
            "# Advanced\nAdvanced content".to_string(),
        ),
        FileData::new(
            "01-intro.md".to_string(),
            "# Introduction\nIntro content".to_string(),
        ),
        FileData::new(
            "02-basics.md".to_string(),
            "# Basics\nBasic content".to_string(),
        ),
    ];

    // Sort files by name before generating navigation
    files.sort_by(|a, b| a.name.cmp(&b.name));

    let nav = NavigationGenerator::generate_navigation(&files);
    println!("    ✅ Generated sorted navigation:");
    println!("{}", nav);

    Ok(())
}

/// Demonstrates handling of excluded files.
fn excluded_files_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Excluded Files Example");
    println!("---------------------------------------------");

    let files = vec![
        FileData::new(
            "index.md".to_string(),
            "# Home\nHome page content".to_string(),
        ),
        FileData::new(
            "404.md".to_string(),
            "# Not Found\n404 page content".to_string(),
        ),
        FileData::new(
            "about.md".to_string(),
            "# About\nAbout page content".to_string(),
        ),
        FileData::new(
            "_draft.md".to_string(),
            "# Draft\nDraft content".to_string(),
        ),
    ];

    let nav = NavigationGenerator::generate_navigation(&files);
    println!("    ✅ Generated navigation (excluding special files):");
    println!("{}", nav);

    Ok(())
}

/// Demonstrates navigation validation.
fn validation_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Validation Example");
    println!("---------------------------------------------");

    // Test different file types
    let test_files = vec![
        ("valid.md", true, "Markdown file"),
        ("valid.html", false, "HTML file"),
        ("invalid.txt", false, "Text file"),
        ("invalid", false, "No extension"),
        (".hidden.md", false, "Hidden file"),
    ];

    for (filename, should_be_included, description) in test_files {
        let files = vec![FileData::new(
            filename.to_string(),
            "Test content".to_string(),
        )];

        let nav = NavigationGenerator::generate_navigation(&files);
        let is_included = !nav.is_empty();

        if is_included == should_be_included {
            println!(
                "    ✅ {}: Correctly {}included",
                description,
                if should_be_included { "" } else { "not " }
            );
        } else {
            println!(
                "    ❌ {}: Incorrectly {}included",
                description,
                if is_included { "" } else { "not " }
            );
        }
    }

    Ok(())
}
