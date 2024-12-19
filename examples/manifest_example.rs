// Copyright © 2024 StaticDataGen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # StaticDataGen Web App Manifest Examples
//!
//! This program demonstrates the generation of web app manifests
//! using the StaticDataGen library, showing various configurations
//! and use cases for Progressive Web Apps (PWAs).

use staticdatagen::generators::manifest::{
    sanitize_color, sanitize_text,
};
use staticdatagen::generators::manifest::{
    IconConfig, ManifestConfig, ManifestGenerator,
};
use std::collections::HashMap;

/// Entry point for the StaticDataGen Manifest Examples program.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 StaticDataGen Web App Manifest Examples\n");

    basic_manifest_example()?;
    builder_defaults_example()?;
    custom_theme_example()?;
    display_modes_example()?;
    full_pwa_example()?;
    icon_configuration_example()?;
    invalid_inputs_example()?;
    metadata_based_example()?;
    orientation_example()?;
    sanitization_example()?;
    validation_example()?;

    println!("\n🎉 All manifest examples completed successfully!");

    Ok(())
}

/// Demonstrates basic manifest generation.
fn basic_manifest_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Basic Manifest Example");
    println!("---------------------------------------------");

    let config = ManifestConfig::builder()
        .name("My Web App")
        .short_name("MyApp")
        .start_url("/")
        .display("standalone")
        .build()?;

    let generator = ManifestGenerator::new(config);
    let json = generator.generate()?;

    println!("    ✅ Generated basic manifest:");
    println!("{}", json);

    Ok(())
}

/// Demonstrates custom theme configuration.
fn custom_theme_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Custom Theme Example");
    println!("---------------------------------------------");

    let config = ManifestConfig::builder()
        .name("Themed App")
        .background_color("#f0f0f0")
        .theme_color("#2196f3")
        .build()?;

    let generator = ManifestGenerator::new(config);
    let json = generator.generate()?;

    println!("    ✅ Generated themed manifest:");
    println!("{}", json);

    Ok(())
}

/// Demonstrates a complete PWA configuration.
fn full_pwa_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Full PWA Example");
    println!("---------------------------------------------");

    let config = ManifestConfig::builder()
        .name("Complete PWA")
        .short_name("PWA")
        .description("A complete Progressive Web App example")
        .start_url("/")
        .display("standalone")
        .background_color("#ffffff")
        .theme_color("#000000")
        .orientation("portrait")
        .scope("/")
        .add_icon(IconConfig::new("/icons/icon-512x512.png", "512x512"))
        .build()?;

    let generator = ManifestGenerator::new(config);
    let json = generator.generate()?;

    println!("    ✅ Generated full PWA manifest:");
    println!("{}", json);

    Ok(())
}

/// Demonstrates icon configuration.
fn icon_configuration_example() -> Result<(), Box<dyn std::error::Error>>
{
    println!("\n🦀 Icon Configuration Example");
    println!("---------------------------------------------");

    let config = ManifestConfig::builder()
        .name("Icon Demo")
        .add_icon(
            IconConfig::new("/icons/icon-192.png", "192x192")
                .purpose("maskable"),
        )
        .add_icon(
            IconConfig::new("/icons/icon-512.png", "512x512")
                .purpose("any"),
        )
        .add_icon(IconConfig::new("/icons/icon-256.png", "256x256"))
        .build()?;

    let generator = ManifestGenerator::new(config);
    let json = generator.generate()?;

    println!("    ✅ Generated manifest with icons:");
    println!("{}", json);

    Ok(())
}

/// Demonstrates different display modes.
fn display_modes_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Display Modes Example");
    println!("---------------------------------------------");

    let display_modes = vec![
        ("fullscreen", "Full screen mode"),
        ("standalone", "Standalone app mode"),
        ("minimal-ui", "Minimal UI mode"),
        ("browser", "Browser mode"),
    ];

    for (mode, description) in display_modes {
        let config = ManifestConfig::builder()
            .name(format!("{} Demo", mode))
            .display(mode)
            .build();

        match config {
            Ok(config) => {
                let generator = ManifestGenerator::new(config);
                println!("    ✅ {}: {} - Valid", mode, description);
                println!("    {}", generator.generate()?);
            }
            Err(e) => println!(
                "    ❌ {}: {} - Error: {}",
                mode, description, e
            ),
        }
    }

    Ok(())
}

/// Demonstrates orientation configurations.
fn orientation_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Orientation Example");
    println!("---------------------------------------------");

    let orientations = vec![
        "portrait",
        "landscape",
        "portrait-primary",
        "landscape-primary",
        "portrait-secondary",
        "landscape-secondary",
    ];

    for orientation in orientations {
        let config = ManifestConfig::builder()
            .name(format!("{} Orientation Demo", orientation))
            .orientation(orientation)
            .build()?;

        let generator = ManifestGenerator::new(config);
        println!(
            "    ✅ Generated manifest for {} orientation:",
            orientation
        );
        println!("    {}", generator.generate()?);
    }

    Ok(())
}

/// Demonstrates manifest generation from metadata.
fn metadata_based_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Metadata-based Example");
    println!("---------------------------------------------");

    let mut metadata = HashMap::new();
    _ = metadata.insert("name".to_string(), "Metadata App".to_string());
    _ = metadata
        .insert("short_name".to_string(), "MetaApp".to_string());
    _ = metadata.insert(
        "description".to_string(),
        "App from metadata".to_string(),
    );
    _ = metadata.insert("theme-color".to_string(), "blue".to_string());
    _ = metadata
        .insert("background-color".to_string(), "white".to_string());

    match ManifestGenerator::from_metadata(&metadata) {
        Ok(json) => {
            println!("    ✅ Generated manifest from metadata:");
            println!("    {}", json);
        }
        Err(e) => println!("    ❌ Error generating manifest: {}", e),
    }

    Ok(())
}

/// Demonstrates manifest validation.
fn validation_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Validation Example");
    println!("---------------------------------------------");

    let test_cases = vec![
        (
            "Valid Basic",
            ManifestConfig::builder()
                .name("Valid App")
                .short_name("App")
                .build(),
            true,
        ),
        (
            "Invalid Display",
            ManifestConfig::builder()
                .name("Invalid Display")
                .display("invalid")
                .build(),
            false,
        ),
        (
            "Long Short Name",
            ManifestConfig::builder()
                .name("Valid Name")
                .short_name("ThisShortNameIsTooLong")
                .build(),
            false,
        ),
    ];

    for (case, result, should_be_valid) in test_cases {
        match result {
            Ok(config) => {
                if should_be_valid {
                    println!("    ✅ {}: Valid as expected", case);
                    let generator = ManifestGenerator::new(config);
                    println!("    {}", generator.generate()?);
                } else {
                    println!("    ❌ {}: Unexpectedly valid", case);
                }
            }
            Err(e) => {
                if !should_be_valid {
                    println!(
                        "    ✅ {}: Invalid as expected: {}",
                        case, e
                    );
                } else {
                    println!(
                        "    ❌ {}: Unexpected error: {}",
                        case, e
                    );
                }
            }
        }
    }

    Ok(())
}

/// Demonstrates text and color sanitization.
fn sanitization_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Sanitization Example");
    println!("---------------------------------------------");

    let long_text = "This is a very long name that should be truncated appropriately";
    let sanitized_text = sanitize_text(long_text, 20);
    println!("    ✅ Sanitized text: {}", sanitized_text);

    let invalid_color = "invalid-color";
    let sanitized_color = sanitize_color(invalid_color.to_string());
    println!("    ✅ Sanitized color: {}", sanitized_color);

    Ok(())
}

/// Demonstrates manifest builder default values.
fn builder_defaults_example() -> Result<(), Box<dyn std::error::Error>>
{
    println!("\n🦀 Builder Defaults Example");
    println!("---------------------------------------------");

    let config =
        ManifestConfig::builder().name("Default App").build()?;

    println!("    ✅ Manifest with defaults: {:?}", config);

    let generator = ManifestGenerator::new(config);
    let json = generator.generate()?;
    println!("{}", json);

    Ok(())
}

/// Demonstrates manifest validation with invalid inputs.
fn invalid_inputs_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Invalid Inputs Example");
    println!("---------------------------------------------");

    let invalid_color_config = ManifestConfig::builder()
        .name("Invalid Color App")
        .background_color("not-a-color")
        .build();

    match invalid_color_config {
        Ok(_) => println!("    ❌ Unexpectedly valid configuration."),
        Err(e) => {
            println!("    ✅ Detected invalid configuration: {}", e)
        }
    }

    let long_name_config = ManifestConfig::builder()
        .name("ThisNameIsFarTooLongForTheManifestAndShouldFail")
        .build();

    match long_name_config {
        Ok(_) => println!(
            "    ❌ Unexpectedly valid long name configuration."
        ),
        Err(e) => println!("    ✅ Detected invalid long name: {}", e),
    }

    Ok(())
}
