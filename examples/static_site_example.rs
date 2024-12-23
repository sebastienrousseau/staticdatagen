// Copyright © 2024 StaticDataGen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # StaticDataGen Example
//!
//! This example demonstrates how to use the StaticDataGen library to generate
//! static site content, including HTML pages, RSS feeds, sitemaps, and various
//! metadata files.
//!
//! The example shows:
//! - Basic site compilation
//! - Metadata generation
//! - Security configuration
//! - RSS feed generation
//! - Sitemap creation
//! - Multi-language support
//! - Web app manifest generation
//! - Local server setup

use anyhow::Result;
use http_handle::Server;
use staticdatagen::{
    compiler::service::compile,
    generators::{
        cname::{CnameConfig, CnameGenerator},
        humans::{HumansConfig, HumansGenerator},
        manifest::{IconConfig, ManifestConfig, ManifestGenerator},
    },
    models::data::{FileData, SecurityData},
};
use std::{collections::HashMap, path::Path};

/// Main entry point demonstrating StaticDataGen usage
fn main() -> Result<()> {
    println!("🚀 Starting StaticDataGen Example...\n");

    // Define directory paths
    let build_dir = Path::new("examples/build"); // Temporary build directory
    let site_dir = Path::new("examples/site"); // Final output directory
    let content_dir = Path::new("examples/content"); // Source content files
    let template_dir = Path::new("examples/templates"); // HTML templates

    // Create and resolve template context
    println!("🔧 Resolving template tags...");
    let mut context = HashMap::new();
    resolve_template_tags(&mut context);

    // 1. Basic site compilation with context
    println!("📂 Compiling static site...");
    compile_with_context(
        build_dir,
        content_dir,
        site_dir,
        template_dir,
        &context,
    )?;

    // Other steps remain the same
    println!("📱 Generating Web App Manifest...");
    generate_manifest()?;
    println!("🔒 Setting up security.txt...");
    configure_security()?;
    println!("🌐 Creating CNAME record...");
    setup_cname()?;
    println!("👥 Creating humans.txt...");
    generate_humans_txt()?;
    println!("📄 Processing file data...");
    handle_file_data()?;

    println!("🌍 Starting local server...");
    let server =
        Server::new("127.0.0.1:3000", site_dir.to_str().unwrap());
    server.start()?;

    println!("\n✨ StaticDataGen example completed successfully!");
    println!("   Visit http://127.0.0.1:3000 to view your site.");

    Ok(())
}

/// Updated compile function to accept context
fn compile_with_context(
    build_dir: &Path,
    content_dir: &Path,
    site_dir: &Path,
    template_dir: &Path,
    context: &HashMap<String, String>,
) -> Result<()> {
    // Example: Pass `context` to relevant rendering logic
    println!("Using context: {:?}", context);

    // Ensure context is utilized in your rendering pipeline
    compile(build_dir, content_dir, site_dir, template_dir)
}

/// Generates a web app manifest for PWA support
fn generate_manifest() -> Result<()> {
    let manifest = ManifestConfig::builder()
        .name("StaticDataGen Example")
        .short_name("StaticGen")
        .description("A static site generator example")
        .start_url("/")
        .display("standalone")
        .background_color("#ffffff")
        .theme_color("#000000")
        .orientation("portrait")
        .scope("/")
        .add_icon(
            IconConfig::new("/icons/icon-512x512.png", "512x512")
                .purpose("maskable"),
        )
        .build()?;

    let generator = ManifestGenerator::new(manifest);
    let _json = generator.generate()?;
    println!("  ✅ Manifest generated successfully");

    Ok(())
}

/// Sets up security.txt configuration
fn configure_security() -> Result<()> {
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
        hiring: "https://example.com/security-jobs".to_string(),
        encryption: "https://example.com/pgp-key.txt".to_string(),
    };

    security_data.validate()?;
    println!("  ✅ Security configuration validated");

    Ok(())
}

/// Creates CNAME record for custom domain
fn setup_cname() -> Result<()> {
    let config = CnameConfig::new("example.com", Some(3600), None)?;
    let generator = CnameGenerator::new(config);
    let _content = generator.generate();
    println!("  ✅ CNAME record generated");

    Ok(())
}

/// Generates humans.txt file
fn generate_humans_txt() -> Result<()> {
    let mut metadata = HashMap::new();
    _ = metadata
        .insert("author".to_string(), "Development Team".to_string());
    _ = metadata.insert(
        "author_website".to_string(),
        "https://example.com".to_string(),
    );
    _ = metadata
        .insert("author_twitter".to_string(), "@devteam".to_string());
    _ = metadata
        .insert("author_location".to_string(), "Global".to_string());
    _ = metadata.insert(
        "site_components".to_string(),
        "Rust, StaticDataGen".to_string(),
    );

    let config = HumansConfig::from_metadata(&metadata)?;
    let generator = HumansGenerator::new(config);
    let _content = generator.generate();
    println!("  ✅ humans.txt generated");

    Ok(())
}

/// Demonstrates file data handling
fn handle_file_data() -> Result<()> {
    let file = FileData::new(
        "example.md".to_string(),
        "# Example Content\n\nThis is a test page.".to_string(),
    );

    file.validate()?;
    println!("  ✅ File data validated");

    Ok(())
}

/// Fix template placeholders by ensuring all required tags are resolved.
fn resolve_template_tags(context: &mut HashMap<String, String>) {
    // Ensure the "primary" tag exists
    if !context.contains_key("primary") {
        _ = context
            .insert("primary".to_string(), "default_value".to_string());
    }
}
