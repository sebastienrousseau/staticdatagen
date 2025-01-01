#![allow(missing_docs)]
// Copyright © 2025 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Enhanced Benchmarks for the Static Data Gen Library
//!
//! This module contains benchmarks for various operations provided by the library,
//! including content generation, HTML processing, file operations, and metadata handling.
//! Each benchmark function measures performance of a specific feature, preventing
//! compiler optimizations from skewing the timings by using [`criterion::black_box`].

use criterion::{
    black_box, criterion_group, criterion_main, Criterion,
};
use std::collections::HashMap;
use std::path::Path;

use staticdatagen::{
    generators::{
        cname::{CnameConfig, CnameGenerator},
        humans::{HumansConfig, HumansGenerator},
        manifest::{ManifestConfig, ManifestGenerator},
    },
    models::data::{
        CnameData, FileData, HumansData, NewsData, SecurityData,
    },
    modules::{
        json::{cname, human, security},
        navigation::NavigationGenerator,
    },
};

/// Benchmarks the generation of a `CNAME` record using [`CnameGenerator`].
///
/// # Arguments
///
/// * `c` - A mutable reference to the [`Criterion`] benchmarking context.
fn bench_cname_generation(c: &mut Criterion) {
    let config =
        CnameConfig::new("example.com", Some(3600), None).unwrap();
    let generator = CnameGenerator::new(config);

    // Store the return value of `bench_function` to avoid unused-result lint.
    let _c = c.bench_function("generate CNAME", |b| {
        b.iter(|| {
            // Capture and discard the resulting `String` to avoid "unused result" warnings.
            let _ = black_box(generator.generate());
        });
    });
}

/// Benchmarks the generation of `humans.txt` using [`HumansGenerator`].
///
/// # Arguments
///
/// * `c` - A mutable reference to the [`Criterion`] benchmarking context.
fn bench_humans_txt_generation(c: &mut Criterion) {
    let config = HumansConfig {
        author: "John Doe".to_string(),
        author_website: "https://example.com".to_string(),
        author_twitter: "@johndoe".to_string(),
        author_location: "New York".to_string(),
        site_components: "Rust, SSG".to_string(),
        site_last_updated: "2024-01-01".to_string(),
        site_standards: "HTML5, CSS3".to_string(),
        site_software: "StaticDataGen".to_string(),
        thanks: "Contributors".to_string(),
    };
    let generator = HumansGenerator::new(config);

    let _c = c.bench_function("generate humans.txt", |b| {
        b.iter(|| {
            let _ = black_box(generator.generate());
        });
    });
}

/// Benchmarks the generation of `manifest.json` using [`ManifestGenerator`].
///
/// # Arguments
///
/// * `c` - A mutable reference to the [`Criterion`] benchmarking context.
fn bench_manifest_generation(c: &mut Criterion) {
    let config = ManifestConfig::builder()
        .name("Test App")
        .short_name("App")
        .description("Test application")
        .start_url("/")
        .display("standalone")
        .build()
        .unwrap();
    let generator = ManifestGenerator::new(config);

    let _c = c.bench_function("generate manifest", |b| {
        b.iter(|| {
            // `generate()` returns a `Result<String, _>`, so unwrap it.
            let _ = black_box(generator.generate().unwrap());
        });
    });
}

/// Benchmarks the generation of navigation data using [`NavigationGenerator`].
///
/// # Arguments
///
/// * `c` - A mutable reference to the [`Criterion`] benchmarking context.
fn bench_navigation_generation(c: &mut Criterion) {
    let files = vec![
        FileData {
            name: "about.md".to_string(),
            content: "About page".to_string(),
            // No need for struct update syntax since all fields are already explicitly specified
            ..Default::default()
        },
        FileData {
            name: "contact.md".to_string(),
            content: "Contact page".to_string(),
            ..Default::default()
        },
    ];

    let _c = c.bench_function("generate navigation", |b| {
        b.iter(|| {
            let _ = black_box(
                NavigationGenerator::generate_navigation(&files),
            );
        });
    });
}

/// Benchmarks the generation of a `security.txt` string.
///
/// # Arguments
///
/// * `c` - A mutable reference to the [`Criterion`] benchmarking context.
fn bench_security_txt_generation(c: &mut Criterion) {
    let security_data = SecurityData {
        contact: vec!["https://example.com/security".to_string()],
        expires: "2024-12-31T23:59:59Z".to_string(),
        acknowledgments: "https://example.com/thanks".to_string(),
        preferred_languages: "en".to_string(),
        canonical: "https://example.com/.well-known/security.txt"
            .to_string(),
        policy: String::new(),
        hiring: String::new(),
        encryption: String::new(),
    };

    let _c = c.bench_function("generate security.txt", |b| {
        b.iter(|| {
            let _ = black_box(security(&security_data));
        });
    });
}

/// Benchmarks the creation of a new [`NewsData`] instance to simulate processing.
///
/// # Arguments
///
/// * `c` - A mutable reference to the [`Criterion`] benchmarking context.
fn bench_news_data_processing(c: &mut Criterion) {
    let news_data = NewsData {
        news_genres: "Blog, Opinion".to_string(),
        news_keywords: "tech, rust".to_string(),
        news_language: "en".to_string(),
        news_image_loc: "https://example.com/image.jpg".to_string(),
        news_loc: "https://example.com/article".to_string(),
        news_publication_date: "2024-01-01T00:00:00Z".to_string(),
        news_publication_name: "Example News".to_string(),
        news_title: "Test Article".to_string(),
    };

    let _c = c.bench_function("process news data", |b| {
        b.iter(|| {
            let _ = black_box(NewsData::new(news_data.clone()));
        });
    });
}

/// Benchmarks the validation of [`FileData`].
///
/// # Arguments
///
/// * `c` - A mutable reference to the [`Criterion`] benchmarking context.
fn bench_file_data_processing(c: &mut Criterion) {
    let content = "# Test Content\n\nThis is a test markdown file.";
    let file_data =
        FileData::new("test.md".to_string(), content.to_string());

    let _c = c.bench_function("process file data", |b| {
        b.iter(|| {
            // `validate()` returns `Result<(), _>`, so there's no meaningful value to `black_box`.
            // We'll just call `unwrap()` to ensure it doesn't error out.
            file_data.validate().unwrap();
        });
    });
}

/// Benchmarks the creation of a minimal `humans.txt` JSON using [`HumansConfig`].
///
/// # Arguments
///
/// * `c` - A mutable reference to the [`Criterion`] benchmarking context.
fn bench_human_txt_processing(c: &mut Criterion) {
    let mut metadata = HashMap::new();
    // `insert` returns an Option with the old value, so ignore it to avoid "unused result" lint.
    let _ =
        metadata.insert("author".to_string(), "John Doe".to_string());
    let _ = metadata.insert(
        "author_website".to_string(),
        "https://example.com".to_string(),
    );
    let _ = metadata
        .insert("author_twitter".to_string(), "@johndoe".to_string());

    let _c = c.bench_function("process humans.txt", |b| {
        b.iter(|| {
            let humans_config =
                HumansConfig::from_metadata(&metadata).unwrap();
            let humans_data = HumansData::new(
                humans_config.author,
                humans_config.thanks,
            );

            let _ = black_box(human(&humans_data));
        });
    });
}

/// Benchmarks processing of `CNAME` data from metadata using [`CnameConfig`].
///
/// # Arguments
///
/// * `c` - A mutable reference to the [`Criterion`] benchmarking context.
fn bench_cname_processing(c: &mut Criterion) {
    let mut metadata = HashMap::new();
    let _ =
        metadata.insert("cname".to_string(), "example.com".to_string());

    let _c = c.bench_function("process CNAME", |b| {
        b.iter(|| {
            let config =
                CnameConfig::new("example.com", None, None).unwrap();
            let cname_data = CnameData {
                cname: config.domain,
                // Remove needless struct update syntax when no additional fields need filling
            };

            let _ = black_box(cname(&cname_data));
        });
    });
}

/// Benchmarks path sanitization using [`staticdatagen::utilities::security::sanitize_path`].
///
/// # Arguments
///
/// * `c` - A mutable reference to the [`Criterion`] benchmarking context.
fn bench_path_sanitization(c: &mut Criterion) {
    let path = Path::new("content/../sensitive.txt");

    let _c = c.bench_function("sanitize path", |b| {
        b.iter(|| {
            // `sanitize_path` returns a `Result<PathBuf, _>`, so unwrap it.
            let _ = black_box(
                staticdatagen::utilities::security::sanitize_path(path)
                    .unwrap(),
            );
        });
    });
}

// Group all benchmarks
criterion_group!(
    benches,
    bench_cname_generation,
    bench_humans_txt_generation,
    bench_manifest_generation,
    bench_navigation_generation,
    bench_security_txt_generation,
    bench_news_data_processing,
    bench_file_data_processing,
    bench_human_txt_processing,
    bench_cname_processing,
    bench_path_sanitization,
);

// Declare the main benchmark entry point
criterion_main!(benches);
