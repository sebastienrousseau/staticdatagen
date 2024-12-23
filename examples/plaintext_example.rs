// Copyright © 2025 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # StaticDataGen Plaintext Generation Examples
//!
//! This program demonstrates the plaintext generation capabilities
//! of the StaticDataGen library, showing various text conversion
//! and formatting scenarios.

use staticdatagen::modules::plaintext::{
    generate_plain_text, PlainTextConfig,
};

/// Entry point for the StaticDataGen Plaintext Examples program.
///
/// Demonstrates various plaintext generation scenarios including
/// markdown conversion, metadata handling, and text formatting.
///
/// # Errors
///
/// Returns a `Result` containing a `Box<dyn std::error::Error>` if any error
/// occurs during the execution of the examples.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 StaticDataGen Plaintext Generation Examples\n");

    basic_conversion_example()?;
    markdown_formatting_example()?;
    metadata_handling_example()?;
    unicode_handling_example()?;
    list_handling_example()?;
    table_conversion_example()?;
    special_characters_example()?;
    line_wrapping_example()?;

    println!("\n🎉 All plaintext examples completed successfully!");

    Ok(())
}

/// Demonstrates basic plaintext conversion.
fn basic_conversion_example() -> Result<(), Box<dyn std::error::Error>>
{
    println!("🦀 Basic Conversion Example");
    println!("---------------------------------------------");

    let html_content = r#"
        <h1>Welcome to Our Site</h1>
        <p>This is a simple paragraph with some <strong>bold</strong> and <em>italic</em> text.</p>
        <p>Another paragraph with a <a href="https://example.com">link</a>.</p>
    "#;

    match generate_plain_text(
        html_content,
        "Welcome Page",
        "A simple welcome page",
        "John Doe",
        "StaticDataGen",
        "welcome, example",
    ) {
        Ok((content, title, desc, author, creator, keywords)) => {
            println!("    ✅ Generated plaintext content:");
            println!("    Title: {}", title);
            println!("    Description: {}", desc);
            println!("    Author: {}", author);
            println!("    Creator: {}", creator);
            println!("    Keywords: {}", keywords);
            println!("\n    Content:\n{}", content);
        }
        Err(e) => println!("    ❌ Conversion error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates markdown formatting conversion.
fn markdown_formatting_example(
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Markdown Formatting Example");
    println!("---------------------------------------------");

    let markdown_content = r#"
# Main Heading

## Section 1
This is **bold** text and *italic* text.

## Section 2
- List item 1
- List item 2
  - Nested item
  - Another nested item

## Section 3
1. First item
2. Second item
3. Third item
    "#;

    match generate_plain_text(
        markdown_content,
        "Formatted Document",
        "A document with various formatting",
        "Jane Smith",
        "StaticDataGen",
        "markdown, formatting",
    ) {
        Ok((content, _, _, _, _, _)) => {
            println!("    ✅ Generated plaintext from markdown:");
            println!("{}", content);
        }
        Err(e) => println!("    ❌ Conversion error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates metadata handling.
fn metadata_handling_example() -> Result<(), Box<dyn std::error::Error>>
{
    println!("\n🦀 Metadata Handling Example");
    println!("---------------------------------------------");

    let content_with_metadata = r#"---
title: Example Document
author: John Smith
description: A document with metadata
keywords: metadata, example, test
---

# Main Content

This is the main content of the document.
    "#;

    match generate_plain_text(
        content_with_metadata,
        "Metadata Example",
        "Testing metadata handling",
        "John Smith",
        "StaticDataGen",
        "metadata, test",
    ) {
        Ok((content, title, desc, author, _, keywords)) => {
            println!("    ✅ Extracted metadata:");
            println!("    Title: {}", title);
            println!("    Description: {}", desc);
            println!("    Author: {}", author);
            println!("    Keywords: {}", keywords);
            println!("\n    Content:\n{}", content);
        }
        Err(e) => println!("    ❌ Metadata extraction error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates Unicode text handling.
fn unicode_handling_example() -> Result<(), Box<dyn std::error::Error>>
{
    println!("\n🦀 Unicode Handling Example");
    println!("---------------------------------------------");

    let unicode_content = r#"
# Multi-Language Content

## English
Hello, World!

## French
Bonjour le monde! 🇫🇷

## German
Hallo Welt! 🇩🇪

## Japanese
こんにちは世界！🇯🇵

## Russian
Привет, мир! 🇷🇺
    "#;

    match generate_plain_text(
        unicode_content,
        "Unicode Example",
        "Multi-language content",
        "Global Team",
        "StaticDataGen",
        "unicode, multilingual",
    ) {
        Ok((content, _, _, _, _, _)) => {
            println!("    ✅ Generated plaintext with Unicode:");
            println!("{}", content);
        }
        Err(e) => println!("    ❌ Unicode handling error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates list handling.
fn list_handling_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 List Handling Example");
    println!("---------------------------------------------");

    let list_content = r#"
# Shopping List

## Groceries
* Apples
* Bananas
* Bread
  * Whole wheat
  * Sourdough
* Milk

## Hardware
1. Screwdriver
2. Nails
   1. 2-inch
   2. 3-inch
3. Hammer
    "#;

    match generate_plain_text(
        list_content,
        "Lists Example",
        "Document with various lists",
        "List Maker",
        "StaticDataGen",
        "lists, bullets, numbers",
    ) {
        Ok((content, _, _, _, _, _)) => {
            println!("    ✅ Generated plaintext from lists:");
            println!("{}", content);
        }
        Err(e) => println!("    ❌ List conversion error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates table conversion.
fn table_conversion_example() -> Result<(), Box<dyn std::error::Error>>
{
    println!("\n🦀 Table Conversion Example");
    println!("---------------------------------------------");

    let table_content = r#"
# Product Catalog

| Product | Price | Stock |
|---------|-------|-------|
| Apple   | $1.00 | 100   |
| Orange  | $0.75 | 150   |
| Banana  | $0.50 | 200   |

## Additional Notes
- All prices in USD
- Stock levels updated daily
    "#;

    match generate_plain_text(
        table_content,
        "Table Example",
        "Document with tables",
        "Catalog Team",
        "StaticDataGen",
        "tables, products",
    ) {
        Ok((content, _, _, _, _, _)) => {
            println!("    ✅ Generated plaintext from table:");
            println!("{}", content);
        }
        Err(e) => println!("    ❌ Table conversion error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates special characters handling.
fn special_characters_example() -> Result<(), Box<dyn std::error::Error>>
{
    println!("\n🦀 Special Characters Example");
    println!("---------------------------------------------");

    let special_content = r#"
# Special Characters Test

## Symbols
Copyright © 2024
Registered ®
Trademark ™

## Math Symbols
Temperature: 20°C
Pi: π
Area: 50m²

## Currency
Price: $99.99
Euro: €50
Pounds: £75
    "#;

    match generate_plain_text(
        special_content,
        "Special Characters",
        "Testing special character handling",
        "Symbol Tester",
        "StaticDataGen",
        "symbols, characters",
    ) {
        Ok((content, _, _, _, _, _)) => {
            println!(
                "    ✅ Generated plaintext with special characters:"
            );
            println!("{}", content);
        }
        Err(e) => println!("    ❌ Special character error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates line wrapping configuration.
fn line_wrapping_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Line Wrapping Example");
    println!("---------------------------------------------");

    let long_content = r#"
# Long Line Example

This is a very long line that should be wrapped according to the configuration settings. It contains enough text to demonstrate how the line wrapping functionality works with different maximum line length settings.

## Another Section
Here's another paragraph with a long line that needs to be wrapped. The wrapping should maintain readability while ensuring no line exceeds the specified maximum length.
    "#;

    let configs = vec![(40, "Narrow"), (60, "Medium"), (80, "Wide")];

    for (width, desc) in configs {
        let _config = PlainTextConfig {
            max_line_length: width,
            ..Default::default()
        };

        println!("\n    {} Format ({})", desc, width);
        println!("    {}", "-".repeat(40));

        match generate_plain_text(
            long_content,
            "Wrapping Test",
            "Testing line wrapping",
            "Format Tester",
            "StaticDataGen",
            "wrapping, format",
        ) {
            Ok((content, _, _, _, _, _)) => {
                println!("{}", content);
            }
            Err(e) => println!("    ❌ Wrapping error: {:?}", e),
        }
    }

    Ok(())
}
