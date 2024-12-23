// Copyright © 2025 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # StaticDataGen Compiler Service Examples
//!
//! This program demonstrates the usage of the StaticDataGen compiler service,
//! showing various compilation scenarios and configurations.

use staticdatagen::compiler::service::compile;
use std::fs;
use std::path::Path;

/// Entry point for the StaticDataGen Compiler Service Examples program.
///
/// Demonstrates various compilation scenarios using the compiler service,
/// including template processing, content generation, and error handling.
///
/// # Errors
///
/// Returns a `Result` containing a `Box<dyn std::error::Error>` if any error
/// occurs during the execution of the examples.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 StaticDataGen Compiler Service Examples\n");

    setup_example_directories()?;
    basic_compilation_example()?;
    template_processing_example()?;
    content_compilation_example()?;
    metadata_handling_example()?;
    directory_structure_example()?;
    error_handling_example()?;
    cleanup_example_directories()?;

    println!(
        "\n🎉 All compiler service examples completed successfully!"
    );

    Ok(())
}

/// Sets up example directories for testing.
fn setup_example_directories() -> Result<(), Box<dyn std::error::Error>>
{
    println!("🦀 Setting Up Example Directories");
    println!("---------------------------------------------");

    let dirs = [
        "examples/content",
        "examples/templates",
        "examples/build",
        "examples/site",
    ];

    for dir in &dirs {
        fs::create_dir_all(dir)?;
        println!("    ✅ Created directory: {}", dir);
    }

    // Create a sample template file
    let template_content = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>{{title}}</title>
</head>
<body>
    {{content}}
</body>
</html>"#;

    fs::write("examples/templates/default.html", template_content)?;
    println!("    ✅ Created sample template file");

    Ok(())
}

/// Demonstrates basic site compilation.
fn basic_compilation_example() -> Result<(), Box<dyn std::error::Error>>
{
    println!("\n🦀 Basic Site Compilation Example");
    println!("---------------------------------------------");

    let build_dir = Path::new("examples/build");
    let content_dir = Path::new("examples/content");
    let site_dir = Path::new("examples/site");
    let template_dir = Path::new("examples/templates");

    // Create a sample content file
    let content = "# Hello World\n\nThis is a test page.";
    fs::write(content_dir.join("index.md"), content)?;

    match compile(build_dir, content_dir, site_dir, template_dir) {
        Ok(_) => println!("    ✅ Basic compilation successful"),
        Err(e) => println!("    ❌ Compilation failed: {:?}", e),
    }

    Ok(())
}

/// Demonstrates template processing features.
fn template_processing_example(
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Template Processing Example");
    println!("---------------------------------------------");

    // Create a custom template
    let custom_template = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>{{title}}</title>
    <meta name="description" content="{{description}}">
</head>
<body>
    <header>
        <h1>{{title}}</h1>
    </header>
    <main>
        {{content}}
    </main>
    <footer>
        <p>Created with StaticDataGen</p>
    </footer>
</body>
</html>"#;

    fs::write("examples/templates/custom.html", custom_template)?;

    println!("    ✅ Created custom template");
    println!("    ✅ Template processing example completed");

    Ok(())
}

/// Demonstrates content compilation features.
fn content_compilation_example(
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Content Compilation Example");
    println!("---------------------------------------------");

    let content_path = Path::new("examples/content");

    // Create sample content files
    let files = [
        ("page1.md", "# Page 1\n\nThis is page 1."),
        ("page2.md", "# Page 2\n\nThis is page 2."),
        ("blog/post1.md", "# Blog Post 1\n\nThis is a blog post."),
    ];

    for (filename, content) in &files {
        let file_path = content_path.join(filename);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(file_path, content)?;
        println!("    ✅ Created content file: {}", filename);
    }

    Ok(())
}

/// Demonstrates metadata handling during compilation.
fn metadata_handling_example() -> Result<(), Box<dyn std::error::Error>>
{
    println!("\n🦀 Metadata Handling Example");
    println!("---------------------------------------------");

    let content_path = Path::new("examples/content");
    let content_with_metadata = r#"---
title: Sample Page
description: A sample page demonstrating metadata
author: John Doe
date: 2024-02-20
tags: sample, example
---

# Sample Page

This is a sample page with metadata."#;

    fs::write(
        content_path.join("with-metadata.md"),
        content_with_metadata,
    )?;
    println!("    ✅ Created content with metadata");

    Ok(())
}

/// Demonstrates directory structure handling.
fn directory_structure_example(
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Directory Structure Example");
    println!("---------------------------------------------");

    let content_path = Path::new("examples/content");
    let dirs = ["blog", "blog/2024", "pages", "docs", "docs/api"];

    for dir in &dirs {
        fs::create_dir_all(content_path.join(dir))?;
        println!("    ✅ Created directory: {}", dir);

        // Create an index file in each directory
        let index_content =
            format!("# Welcome to {}\n\nThis is the index page.", dir);
        fs::write(
            content_path.join(dir).join("index.md"),
            index_content,
        )?;
        println!("    ✅ Created index file in: {}", dir);
    }

    Ok(())
}

/// Demonstrates error handling scenarios.
fn error_handling_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Error Handling Example");
    println!("---------------------------------------------");

    // Test compilation with invalid path
    let invalid_path = Path::new("/nonexistent/path");
    match compile(
        invalid_path,
        invalid_path,
        invalid_path,
        invalid_path,
    ) {
        Ok(_) => println!("    ❌ Expected error with invalid path"),
        Err(e) => println!("    ✅ Successfully caught error: {:?}", e),
    }

    // Test with invalid content file
    let content_path = Path::new("examples/content");
    fs::write(content_path.join("invalid.txt"), "Invalid file type")?;
    println!("    ✅ Created invalid content file");

    Ok(())
}

/// Cleans up example directories.
fn cleanup_example_directories(
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Cleaning Up Example Directories");
    println!("---------------------------------------------");

    let dirs = [
        "examples/content",
        "examples/templates",
        "examples/build",
        "examples/site",
    ];

    for dir in &dirs {
        fs::remove_dir_all(dir)?;
        println!("    ✅ Removed directory: {}", dir);
    }

    Ok(())
}
