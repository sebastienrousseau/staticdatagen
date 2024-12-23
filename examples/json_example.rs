// Copyright © 2025 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # StaticDataGen JSON Generation Examples
//!
//! This program demonstrates the various JSON generation capabilities
//! of the StaticDataGen library, including web manifests, sitemaps,
//! CNAME records, and other data files.

use sitemap_gen::{ChangeFreq, SiteMapData};
use staticdatagen::models::data::{
    CnameData, HumansData, IconData, ManifestData, NewsData,
    SecurityData, TxtData,
};
use staticdatagen::modules::json::{
    cname, human, manifest, news_sitemap, security, sitemap, txt,
};
use std::path::Path;
use std::str::FromStr;
use url::Url;

/// Entry point for the StaticDataGen JSON Examples program.
///
/// # Errors
///
/// Returns a `Result` containing a `Box<dyn std::error::Error>` if any error
/// occurs during the execution of the examples.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 StaticDataGen JSON Generation Examples\n");

    manifest_example()?;
    sitemap_example()?;
    news_sitemap_example()?;
    cname_example()?;
    security_example()?;
    humans_example()?;
    robots_example()?;
    combined_example()?;

    println!(
        "\n🎉 All JSON generation examples completed successfully!"
    );

    Ok(())
}

/// Demonstrates web app manifest generation.
fn manifest_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Web App Manifest Example");
    println!("---------------------------------------------");

    let mut manifest_data = ManifestData::new();
    manifest_data.name = "My Progressive Web App".to_string();
    manifest_data.short_name = "MyPWA".to_string();
    manifest_data.start_url = "/".to_string();
    manifest_data.display = "standalone".to_string();
    manifest_data.background_color = "#ffffff".to_string();
    manifest_data.theme_color = "#000000".to_string();

    // Add app icons
    let icons = vec![
        IconData::new(
            "/icons/icon-192x192.png".to_string(),
            "192x192".to_string(),
        ),
        IconData::new(
            "/icons/icon-512x512.png".to_string(),
            "512x512".to_string(),
        ),
    ];
    manifest_data.icons = icons;

    match manifest(&manifest_data) {
        Ok(json) => {
            println!("    ✅ Generated manifest.json:");
            println!("{}", json);
        }
        Err(e) => println!("    ❌ Error generating manifest: {:?}", e),
    }

    Ok(())
}

/// Demonstrates sitemap generation.
fn sitemap_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Sitemap Example");
    println!("---------------------------------------------");

    let site_map_data = SiteMapData {
        loc: Url::from_str("https://example.com")?,
        lastmod: "2024-02-20".to_string(),
        changefreq: ChangeFreq::Daily,
    };

    let output = sitemap(site_map_data, Path::new("public"));
    println!("    ✅ Generated sitemap:");
    println!("{:?}", output);

    Ok(())
}

/// Demonstrates news sitemap generation.
fn news_sitemap_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 News Sitemap Example");
    println!("---------------------------------------------");

    let news_data = NewsData {
        news_genres: "Blog, OpEd".to_string(),
        news_keywords: "technology, web development".to_string(),
        news_language: "en".to_string(),
        news_image_loc: "https://example.com/image.jpg".to_string(),
        news_loc: "https://example.com/news/article".to_string(),
        news_publication_date: "2024-02-20T12:00:00Z".to_string(),
        news_publication_name: "Example News".to_string(),
        news_title: "Latest Updates".to_string(),
    };

    let output = news_sitemap(news_data);
    println!("    ✅ Generated news sitemap:");
    println!("{}", output);

    Ok(())
}

/// Demonstrates CNAME record generation.
fn cname_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 CNAME Record Example");
    println!("---------------------------------------------");

    let cname_data = CnameData {
        cname: "example.com".to_string(),
    };

    let output = cname(&cname_data);
    println!("    ✅ Generated CNAME record:");
    println!("{}", output);

    Ok(())
}

/// Demonstrates security.txt generation.
fn security_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Security.txt Example");
    println!("---------------------------------------------");

    let security_data = SecurityData {
        contact: vec![
            "https://example.com/security".to_string(),
            "mailto:security@example.com".to_string(),
        ],
        expires: "2024-12-31T23:59:59Z".to_string(),
        acknowledgments: "https://example.com/thanks".to_string(),
        preferred_languages: "en, fr".to_string(),
        canonical: "https://example.com/.well-known/security.txt"
            .to_string(),
        policy: "https://example.com/security-policy".to_string(),
        hiring: "https://example.com/security-jobs".to_string(),
        encryption: "https://example.com/pgp-key.txt".to_string(),
    };

    let output = security(&security_data);
    println!("    ✅ Generated security.txt:");
    println!("{}", output);

    Ok(())
}

/// Demonstrates humans.txt generation.
fn humans_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Humans.txt Example");
    println!("---------------------------------------------");

    let humans_data = HumansData {
        author: "Development Team".to_string(),
        author_website: "https://example.com".to_string(),
        author_twitter: "@devteam".to_string(),
        author_location: "Global".to_string(),
        thanks: "Thanks to all contributors".to_string(),
        site_last_updated: "2024-02-20".to_string(),
        site_standards: "HTML5, CSS3".to_string(),
        site_components: "Rust, StaticDataGen".to_string(),
        site_software: "VS Code, Git".to_string(),
    };

    let output = human(&humans_data);
    println!("    ✅ Generated humans.txt:");
    println!("{}", output);

    Ok(())
}

/// Demonstrates robots.txt generation.
fn robots_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Robots.txt Example");
    println!("---------------------------------------------");

    let txt_data = TxtData {
        permalink: "https://example.com".to_string(),
    };

    let output = txt(&txt_data);
    println!("    ✅ Generated robots.txt:");
    println!("{}", output);

    Ok(())
}

/// Demonstrates combined generation of multiple files.
fn combined_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Combined Generation Example");
    println!("---------------------------------------------");

    // Set up basic site data
    let base_url = "https://example.com";

    // Generate manifest
    let mut manifest_data = ManifestData::new();
    manifest_data.name = "Example Site".to_string();
    manifest_data.short_name = "Example".to_string();
    manifest_data.start_url = "/".to_string();

    let manifest_json = manifest(&manifest_data)?;

    // Generate security.txt
    let security_content = security(&SecurityData {
        contact: vec!["mailto:security@example.com".to_string()],
        expires: "2024-12-31T23:59:59Z".to_string(),
        ..Default::default()
    });

    // Generate CNAME
    let cname_content = cname(&CnameData {
        cname: base_url.replace("https://", "").to_string(),
    });

    // Generate humans.txt
    let humans_content = human(&HumansData::new(
        "Site Team".to_string(),
        "Thanks to everyone".to_string(),
    ));

    println!("    ✅ Generated all site files:");
    println!("    📄 manifest.json:");
    println!("{}", manifest_json);
    println!("    📄 security.txt:");
    println!("{}", security_content);
    println!("    📄 CNAME:");
    println!("{}", cname_content);
    println!("    📄 humans.txt:");
    println!("{}", humans_content);

    Ok(())
}
