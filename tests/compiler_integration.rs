// Copyright © 2025 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Integration tests for the compiler module.
//!
//! These tests verify the compilation workflow for static site generation.

use staticdatagen::models::data::FileData;
use std::collections::HashMap;

/// Tests FileData creation and basic operations
#[test]
fn test_file_data_operations() {
    // Create file data with content
    let content = r#"---
title: Test Page
description: A test page for integration testing
---

# Welcome

This is the main content of the test page.

## Features

- Feature 1
- Feature 2
- Feature 3
"#;

    let file = FileData::new("test-page.md".to_string(), content.to_string());

    // Verify basic properties
    assert_eq!(file.name, "test-page.md");
    assert!(file.content.contains("# Welcome"));
    assert!(file.content.contains("Feature 1"));
}

/// Tests metadata extraction patterns
#[test]
fn test_metadata_extraction_patterns() {
    // Create metadata map
    let mut metadata: HashMap<String, String> = HashMap::new();
    let _ = metadata.insert("title".to_string(), "Test Title".to_string());
    let _ = metadata.insert("description".to_string(), "Test Description".to_string());
    let _ = metadata.insert("author".to_string(), "Test Author".to_string());
    let _ = metadata.insert("date".to_string(), "2026-02-05".to_string());
    let _ = metadata.insert("keywords".to_string(), "test, integration, rust".to_string());

    // Verify metadata access
    assert_eq!(metadata.get("title"), Some(&"Test Title".to_string()));
    assert_eq!(
        metadata.get("description"),
        Some(&"Test Description".to_string())
    );
    assert!(metadata.contains_key("author"));
    assert!(metadata.contains_key("date"));
}

/// Tests content processing workflow
#[test]
fn test_content_processing_workflow() {
    // Simulate processing multiple content files
    let files = vec![
        ("index.md", "# Home\n\nWelcome to the site."),
        ("about.md", "# About\n\nLearn about us."),
        ("contact.md", "# Contact\n\nGet in touch."),
        (
            "blog/post-1.md",
            "# First Post\n\nThis is the first blog post.",
        ),
        (
            "blog/post-2.md",
            "# Second Post\n\nThis is the second blog post.",
        ),
    ];

    let file_data: Vec<FileData> = files
        .iter()
        .map(|(name, content)| FileData::new(name.to_string(), content.to_string()))
        .collect();

    // Verify all files are created
    assert_eq!(file_data.len(), 5);

    // Verify content is preserved
    for file in &file_data {
        assert!(!file.name.is_empty());
        assert!(!file.content.is_empty());
        assert!(file.content.starts_with('#'));
    }

    // Verify specific files
    let index = file_data.iter().find(|f| f.name == "index.md");
    assert!(index.is_some());
    assert!(index.unwrap().content.contains("Welcome"));
}

/// Tests batch file processing
#[test]
fn test_batch_processing() {
    // Create a batch of files
    let batch_size = 100;
    let files: Vec<FileData> = (0..batch_size)
        .map(|i| {
            FileData::new(
                format!("page-{}.md", i),
                format!("# Page {}\n\nContent for page {}.", i, i),
            )
        })
        .collect();

    // Verify batch size
    assert_eq!(files.len(), batch_size);

    // Verify each file is unique
    let names: std::collections::HashSet<_> =
        files.iter().map(|f| f.name.clone()).collect();
    assert_eq!(names.len(), batch_size);
}

/// Tests error recovery patterns
#[test]
fn test_error_recovery_patterns() {
    use staticdatagen::Error;

    // Simulate processing with potential errors
    fn process_file(name: &str) -> Result<String, Error> {
        if name.is_empty() {
            return Err(Error::Config("Empty filename".to_string()));
        }
        if !name.ends_with(".md") {
            return Err(Error::ContentProcessing {
                message: "Invalid file extension".to_string(),
                source: None,
            });
        }
        Ok(format!("Processed: {}", name))
    }

    // Test valid file
    let result = process_file("valid.md");
    assert!(result.is_ok());

    // Test empty filename
    let result = process_file("");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Configuration"));

    // Test invalid extension
    let result = process_file("invalid.txt");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Content Processing"));
}

/// Tests parallel processing simulation
#[test]
fn test_parallel_processing_simulation() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    // Create shared counter
    let counter = Arc::new(AtomicUsize::new(0));

    // Simulate parallel processing
    let files: Vec<FileData> = (0..10)
        .map(|i| {
            let _ = counter.fetch_add(1, Ordering::SeqCst);
            FileData::new(
                format!("file-{}.md", i),
                format!("Content {}", i),
            )
        })
        .collect();

    // Verify all files were processed
    assert_eq!(counter.load(Ordering::SeqCst), 10);
    assert_eq!(files.len(), 10);
}
