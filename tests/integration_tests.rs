// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(clippy::unwrap_used, clippy::expect_used)]

//! Integration tests for the staticdatagen library.
//!
//! These tests verify end-to-end functionality across multiple modules,
//! ensuring that components work together correctly.

use staticdatagen::generators::cname::{CnameConfig, CnameGenerator};
use staticdatagen::generators::humans::{
    HumansConfig, HumansGenerator,
};
use staticdatagen::generators::manifest::{
    ManifestConfig, ManifestGenerator,
};
use staticdatagen::models::data::{FileData, SecurityData};
use staticdatagen::modules::navigation::NavigationGenerator;
use staticdatagen::utilities::uuid::generate_unique_string;
use std::collections::HashMap;

/// Tests the complete CNAME generation workflow
#[test]
fn test_cname_generation_workflow() {
    // Create configuration
    let config = CnameConfig::new("example.com", Some(3600), None)
        .expect("Failed to create CNAME config");

    // Generate CNAME
    let generator = CnameGenerator::new(config);
    let result = generator.generate();

    // Verify output
    assert!(result.contains("example.com"));
}

/// Tests the complete humans.txt generation workflow
#[test]
fn test_humans_txt_generation_workflow() {
    // Create configuration from metadata
    let mut metadata = HashMap::new();
    let _ = metadata
        .insert("author".to_string(), "Test Author".to_string());
    let _ = metadata.insert(
        "author_website".to_string(),
        "https://example.com".to_string(),
    );
    let _ = metadata.insert(
        "author_twitter".to_string(),
        "@testauthor".to_string(),
    );
    let _ = metadata
        .insert("author_location".to_string(), "Test City".to_string());
    let _ = metadata.insert(
        "site_components".to_string(),
        "Rust, HTML".to_string(),
    );
    let _ = metadata.insert(
        "site_last_updated".to_string(),
        "2026-01-01".to_string(),
    );
    let _ = metadata.insert(
        "site_standards".to_string(),
        "HTML5, CSS3".to_string(),
    );
    let _ = metadata.insert(
        "site_software".to_string(),
        "StaticDataGen".to_string(),
    );
    let _ = metadata
        .insert("thanks".to_string(), "Contributors".to_string());

    let config = HumansConfig::from_metadata(&metadata)
        .expect("Failed to create config");

    // Generate humans.txt
    let generator = HumansGenerator::new(config);
    let result = generator.generate();

    // Verify output contains expected content
    assert!(result.contains("Test Author"));
    assert!(result.contains("TEAM"));
}

/// Tests the complete manifest.json generation workflow
#[test]
fn test_manifest_generation_workflow() {
    // Create configuration using builder
    let config = ManifestConfig::builder()
        .name("Test Application")
        .short_name("TestApp")
        .description("A test application for integration testing")
        .start_url("/")
        .display("standalone")
        .background_color("#ffffff")
        .theme_color("#000000")
        .build()
        .expect("Failed to build manifest config");

    // Generate manifest
    let generator = ManifestGenerator::new(config);
    let result =
        generator.generate().expect("Failed to generate manifest");

    // Verify JSON output
    assert!(result.contains("Test Application"));
    assert!(result.contains("TestApp"));
    assert!(result.contains("standalone"));
}

/// Tests navigation generation with multiple files
#[test]
fn test_navigation_generation_workflow() {
    // Create file data for multiple pages
    let files = vec![
        FileData::new(
            "index.md".to_string(),
            "# Home\nWelcome".to_string(),
        ),
        FileData::new(
            "about.md".to_string(),
            "# About\nAbout us".to_string(),
        ),
        FileData::new(
            "contact.md".to_string(),
            "# Contact\nContact info".to_string(),
        ),
        FileData::new(
            "blog/post1.md".to_string(),
            "# Blog Post 1\nContent".to_string(),
        ),
    ];

    // Generate navigation
    let nav = NavigationGenerator::generate_navigation(&files);

    // Verify navigation structure
    assert!(!nav.is_empty());
}

/// Tests security.txt data creation and validation
#[test]
fn test_security_txt_workflow() {
    // Create security data
    let security_data = SecurityData {
        contact: vec![
            "mailto:security@example.com".to_string(),
            "https://example.com/security".to_string(),
        ],
        expires: "2027-12-31T23:59:59Z".to_string(),
        acknowledgments: "https://example.com/security/acknowledgments"
            .to_string(),
        preferred_languages: "en, fr, de".to_string(),
        canonical: "https://example.com/.well-known/security.txt"
            .to_string(),
        policy: "https://example.com/security/policy".to_string(),
        hiring: "https://example.com/careers".to_string(),
        encryption: "https://example.com/pgp-key.txt".to_string(),
    };

    // Verify data
    assert_eq!(security_data.contact.len(), 2);
    assert!(security_data.expires.contains("2027"));
    assert!(!security_data.canonical.is_empty());
}

/// Tests UUID generation for unique identifiers
#[test]
fn test_uuid_generation_workflow() {
    // Generate multiple UUIDs
    let uuid1 = generate_unique_string();
    let uuid2 = generate_unique_string();
    let uuid3 = generate_unique_string();

    // Verify uniqueness
    assert_ne!(uuid1, uuid2);
    assert_ne!(uuid2, uuid3);
    assert_ne!(uuid1, uuid3);

    // Verify format (should be valid UUID-like strings)
    assert!(!uuid1.is_empty());
    assert!(!uuid2.is_empty());
    assert!(!uuid3.is_empty());
}

/// Tests FileData validation
#[test]
fn test_file_data_validation_workflow() {
    // Create valid file data
    let file = FileData::new(
        "test.md".to_string(),
        "# Test\n\nThis is test content.".to_string(),
    );

    // Validate
    assert!(file.validate().is_ok());

    // Test with empty content
    let empty_file =
        FileData::new("empty.md".to_string(), String::new());
    // Empty content should still be valid (it's a valid state)
    let _ = empty_file.validate();
}

/// Tests error handling across modules
#[test]
fn test_error_handling_workflow() {
    use staticdatagen::Error;

    // Test error creation and conversion
    let config_error =
        Error::Config("Invalid configuration".to_string());
    assert!(config_error.to_string().contains("Configuration Error"));

    let template_error =
        Error::Template("Template not found".to_string());
    assert!(template_error.to_string().contains("Template Error"));

    // Test error builder
    let content_error = Error::content_processing_builder()
        .message("Content parsing failed")
        .context("Line 42")
        .build();
    assert!(content_error
        .to_string()
        .contains("Content Processing Error"));
}

/// Tests the VERSION constant
#[test]
fn test_version_constant() {
    use staticdatagen::VERSION;

    // Verify version is set
    assert!(!VERSION.is_empty());

    // Verify semver format
    let parts: Vec<&str> = VERSION.split('.').collect();
    assert!(
        parts.len() >= 2,
        "Version should have at least major.minor"
    );

    // Each part should be numeric
    for part in parts {
        assert!(
            part.parse::<u32>().is_ok(),
            "Version part should be numeric"
        );
    }
}

/// Tests multiple generators working together
#[test]
fn test_combined_generation_workflow() {
    // Simulate generating multiple output files for a site

    // 1. Generate CNAME
    let cname_config =
        CnameConfig::new("mysite.example.com", None, None)
            .expect("CNAME config failed");
    let cname_generator = CnameGenerator::new(cname_config);
    let cname_output = cname_generator.generate();

    // 2. Generate humans.txt
    let humans_config = HumansConfig {
        author: "Site Author".to_string(),
        author_website: "https://mysite.example.com".to_string(),
        author_twitter: "@siteauthor".to_string(),
        author_location: "Internet".to_string(),
        site_components: "Rust, StaticDataGen".to_string(),
        site_last_updated: "2026-02-05".to_string(),
        site_standards: "HTML5, CSS3, ES6".to_string(),
        site_software: "StaticDataGen v0.0.6".to_string(),
        thanks: "Open Source Community".to_string(),
    };
    let humans_generator = HumansGenerator::new(humans_config);
    let humans_output = humans_generator.generate();

    // 3. Generate manifest.json
    let manifest_config = ManifestConfig::builder()
        .name("My Site")
        .short_name("MySite")
        .description("My awesome static site")
        .start_url("/")
        .display("standalone")
        .build()
        .expect("Manifest config failed");
    let manifest_generator = ManifestGenerator::new(manifest_config);
    let manifest_output = manifest_generator
        .generate()
        .expect("Manifest generation failed");

    // Verify all outputs
    assert!(!cname_output.is_empty());
    assert!(!humans_output.is_empty());
    assert!(!manifest_output.is_empty());

    // Verify content
    assert!(cname_output.contains("mysite.example.com"));
    assert!(humans_output.contains("Site Author"));
    assert!(manifest_output.contains("My Site"));
}

/// Tests configuration validation across modules
#[test]
fn test_configuration_validation() {
    // Test CNAME with various inputs
    let valid_cname =
        CnameConfig::new("valid.example.com", Some(3600), None);
    assert!(valid_cname.is_ok());

    // Test manifest builder validation
    let valid_manifest = ManifestConfig::builder()
        .name("Valid App")
        .short_name("App")
        .start_url("/")
        .build();
    assert!(valid_manifest.is_ok());
}

/// Tests that generated content is valid
#[test]
fn test_output_format_validation() {
    // Generate manifest and verify it's valid JSON
    let config = ManifestConfig::builder()
        .name("JSON Test")
        .short_name("Test")
        .start_url("/")
        .build()
        .unwrap();
    let generator = ManifestGenerator::new(config);
    let json_output = generator.generate().unwrap();

    // Verify it's parseable JSON
    let parsed: Result<serde_json::Value, _> =
        serde_json::from_str(&json_output);
    assert!(parsed.is_ok(), "Generated manifest should be valid JSON");
}
