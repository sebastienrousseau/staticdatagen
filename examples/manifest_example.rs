// Copyright © 2024 StaticDataGen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # StaticDataGen Web App Manifest Examples
//!
//! This program demonstrates the generation of web app manifests
//! using the StaticDataGen library, showing various configurations
//! and use cases for Progressive Web Apps (PWAs).

use staticdatagen::models::data::{IconData, ManifestData};
use staticdatagen::modules::manifest::create_manifest_data;
use std::collections::HashMap;

/// Entry point for the StaticDataGen Manifest Examples program.
///
/// Demonstrates various web app manifest generation scenarios including
/// basic PWA setup, custom themes, and different display modes.
///
/// # Errors
///
/// Returns a `Result` containing a `Box<dyn std::error::Error>` if any error
/// occurs during the execution of the examples.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 StaticDataGen Web App Manifest Examples\n");

    basic_manifest_example()?;
    custom_theme_example()?;
    full_pwa_example()?;
    icon_configuration_example()?;
    display_modes_example()?;
    orientation_example()?;
    metadata_based_example()?;
    validation_example()?;

    println!("\n🎉 All manifest examples completed successfully!");

    Ok(())
}

/// Demonstrates basic manifest generation.
fn basic_manifest_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Basic Manifest Example");
    println!("---------------------------------------------");

    let mut manifest = ManifestData::new();
    manifest.name = "My Web App".to_string();
    manifest.short_name = "MyApp".to_string();
    manifest.start_url = "/".to_string();
    manifest.display = "standalone".to_string();

    match manifest.validate() {
        Ok(_) => {
            println!("    ✅ Basic manifest validated:");
            println!("    📱 Name: {}", manifest.name);
            println!("    📱 Short Name: {}", manifest.short_name);
            println!("    🔗 Start URL: {}", manifest.start_url);
            println!("    🖥️ Display: {}", manifest.display);
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates custom theme configuration.
fn custom_theme_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Custom Theme Example");
    println!("---------------------------------------------");

    let mut manifest = ManifestData::new();
    manifest.name = "Themed App".to_string();
    manifest.background_color = "#f0f0f0".to_string();
    manifest.theme_color = "#2196f3".to_string();

    match manifest.validate() {
        Ok(_) => {
            println!("    ✅ Theme configuration:");
            println!(
                "    🎨 Background: {}",
                manifest.background_color
            );
            println!("    🎨 Theme: {}", manifest.theme_color);
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates a complete PWA configuration.
fn full_pwa_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Full PWA Example");
    println!("---------------------------------------------");

    let mut manifest = ManifestData::new();
    manifest.name = "Complete PWA".to_string();
    manifest.short_name = "PWA".to_string();
    manifest.start_url = "/".to_string();
    manifest.display = "standalone".to_string();
    manifest.background_color = "#ffffff".to_string();
    manifest.theme_color = "#000000".to_string();
    manifest.description =
        "A complete Progressive Web App example".to_string();
    manifest.orientation = "portrait".to_string();
    manifest.scope = "/".to_string();

    let icon = IconData::new(
        "/icons/icon-512x512.png".to_string(),
        "512x512".to_string(),
    );
    manifest.icons.push(icon);

    match manifest.validate() {
        Ok(_) => {
            println!("    ✅ Full PWA manifest validated");
            println!("    📱 App Info:");
            println!("       Name: {}", manifest.name);
            println!("       Description: {}", manifest.description);
            println!("    🎨 Theme:");
            println!(
                "       Background: {}",
                manifest.background_color
            );
            println!("       Theme: {}", manifest.theme_color);
            println!("    📱 Display:");
            println!("       Mode: {}", manifest.display);
            println!("       Orientation: {}", manifest.orientation);
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates icon configuration.
fn icon_configuration_example() -> Result<(), Box<dyn std::error::Error>>
{
    println!("\n🦀 Icon Configuration Example");
    println!("---------------------------------------------");

    let mut manifest = ManifestData::new();
    manifest.name = "Icon Demo".to_string();

    let icon_configs = vec![
        ("192x192", "/icons/icon-192.png", Some("maskable")),
        ("512x512", "/icons/icon-512.png", Some("any")),
        ("256x256", "/icons/icon-256.png", None),
    ];

    for (size, src, purpose) in icon_configs {
        let mut icon = IconData::new(src.to_string(), size.to_string());
        if let Some(p) = purpose {
            icon.purpose = Some(p.to_string());
        }
        manifest.icons.push(icon);
    }

    match manifest.validate() {
        Ok(_) => {
            println!("    ✅ Icon configuration:");
            for (i, icon) in manifest.icons.iter().enumerate() {
                println!("    🖼️ Icon {}:", i + 1);
                println!("       Size: {}", icon.sizes);
                println!("       Source: {}", icon.src);
                if let Some(purpose) = &icon.purpose {
                    println!("       Purpose: {}", purpose);
                }
            }
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

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
        let mut manifest = ManifestData::new();
        manifest.name = format!("{} Demo", mode);
        manifest.display = mode.to_string();

        match manifest.validate() {
            Ok(_) => {
                println!("    ✅ {}: {} - Valid", mode, description)
            }
            Err(e) => println!(
                "    ❌ {}: {} - Error: {:?}",
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
        let mut manifest = ManifestData::new();
        manifest.name = format!("{} Orientation Demo", orientation);
        manifest.orientation = orientation.to_string();

        match manifest.validate() {
            Ok(_) => println!("    ✅ {}: Valid", orientation),
            Err(e) => {
                println!("    ❌ {}: Error: {:?}", orientation, e)
            }
        }
    }

    Ok(())
}

/// Demonstrates manifest generation from metadata.
fn metadata_based_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Metadata-based Example");
    println!("---------------------------------------------");

    let mut metadata = HashMap::new();
    let _ = metadata.insert("name".to_string(), "Metadata App".to_string());
    let _ = metadata.insert("short_name".to_string(), "MetaApp".to_string());
    let _ = metadata.insert(
        "description".to_string(),
        "App from metadata".to_string(),
    );
    let _ = metadata.insert("theme-color".to_string(), "blue".to_string());
    let _ = metadata.insert(
        "background-color".to_string(),
        "white".to_string(),
    );

    let manifest = create_manifest_data(&metadata);

    println!("    ✅ Generated from metadata:");
    println!("    📱 Name: {}", manifest.name);
    println!("    📱 Short Name: {}", manifest.short_name);
    println!("    📝 Description: {}", manifest.description);
    println!("    🎨 Theme Color: {}", manifest.theme_color);
    println!("    🎨 Background: {}", manifest.background_color);

    Ok(())
}

/// Demonstrates manifest validation.
fn validation_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Validation Example");
    println!("---------------------------------------------");

    let test_cases = vec![
        (
            "Valid Basic",
            ManifestData {
                name: "Valid App".to_string(),
                short_name: "App".to_string(),
                start_url: "/".to_string(),
                display: "standalone".to_string(),
                ..Default::default()
            },
            true,
        ),
        (
            "Invalid Display",
            ManifestData {
                name: "Invalid Display".to_string(),
                display: "invalid".to_string(),
                ..Default::default()
            },
            false,
        ),
        (
            "Long Short Name",
            ManifestData {
                name: "Valid Name".to_string(),
                short_name: "ThisShortNameIsTooLong".to_string(),
                ..Default::default()
            },
            false,
        ),
    ];

    for (case, manifest, should_be_valid) in test_cases {
        match manifest.validate() {
            Ok(_) => {
                if should_be_valid {
                    println!("    ✅ {}: Valid as expected", case);
                } else {
                    println!("    ❌ {}: Unexpectedly valid", case);
                }
            }
            Err(e) => {
                if !should_be_valid {
                    println!("    ✅ {}: Invalid as expected", case);
                } else {
                    println!(
                        "    ❌ {}: Unexpected error: {:?}",
                        case, e
                    );
                }
            }
        }
    }

    Ok(())
}
