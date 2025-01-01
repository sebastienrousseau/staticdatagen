#![allow(missing_docs)]
// Copyright © 2025 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Enhanced benchmarks for the Static Data Gen library.
//!
//! This module contains benchmarks for various operations provided by the library,
//! including content generation, HTML processing, file operations, and metadata handling.

use criterion::{
    black_box, criterion_group, criterion_main, Criterion,
};
use staticdatagen::models::data::CnameData;
use staticdatagen::models::data::HumansData;
use staticdatagen::{
    generators::{
        cname::{CnameConfig, CnameGenerator},
        humans::{HumansConfig, HumansGenerator},
        manifest::{ManifestConfig, ManifestGenerator},
    },
    models::data::{FileData, NewsData, SecurityData},
    modules::{
        json::{cname, human, security},
        navigation::NavigationGenerator,
    },
};
use std::collections::HashMap;
use std::path::Path;

/// Benchmark CNAME generation
fn bench_cname_generation(c: &mut Criterion) {
    let config =
        CnameConfig::new("example.com", Some(3600), None).unwrap();
    let generator = CnameGenerator::new(config);

    c.bench_function("generate CNAME", |b| {
        b.iter(|| {
            black_box(generator.generate());
        });
    });
}

/// Benchmark humans.txt generation
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

    c.bench_function("generate humans.txt", |b| {
        b.iter(|| {
            black_box(generator.generate());
        });
    });
}

/// Benchmark manifest.json generation
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

    c.bench_function("generate manifest", |b| {
        b.iter(|| {
            black_box(generator.generate().unwrap());
        });
    });
}

/// Benchmark navigation generation
fn bench_navigation_generation(c: &mut Criterion) {
    let files = vec![
        FileData {
            name: "about.md".to_string(),
            content: "About page".to_string(),
            ..Default::default()
        },
        FileData {
            name: "contact.md".to_string(),
            content: "Contact page".to_string(),
            ..Default::default()
        },
    ];

    c.bench_function("generate navigation", |b| {
        b.iter(|| {
            black_box(NavigationGenerator::generate_navigation(&files));
        });
    });
}

/// Benchmark security.txt generation
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

    c.bench_function("generate security.txt", |b| {
        b.iter(|| {
            black_box(security(&security_data));
        });
    });
}

/// Benchmark news data processing
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

    c.bench_function("process news data", |b| {
        b.iter(|| {
            black_box(NewsData::new(news_data.clone()));
        });
    });
}

/// Benchmark file data processing
fn bench_file_data_processing(c: &mut Criterion) {
    let content = "# Test Content\n\nThis is a test markdown file.";
    let file_data =
        FileData::new("test.md".to_string(), content.to_string());

    c.bench_function("process file data", |b| {
        b.iter(|| {
            black_box(file_data.validate().unwrap());
        });
    });
}

/// Benchmark human.txt data processing
fn bench_human_txt_processing(c: &mut Criterion) {
    let mut metadata = HashMap::new();
    metadata.insert("author".to_string(), "John Doe".to_string());
    metadata.insert(
        "author_website".to_string(),
        "https://example.com".to_string(),
    );
    metadata
        .insert("author_twitter".to_string(), "@johndoe".to_string());

    c.bench_function("process humans.txt", |b| {
        b.iter(|| {
            let humans_config =
                HumansConfig::from_metadata(&metadata).unwrap();
            let humans_data = HumansData::new(
                humans_config.author,
                humans_config.thanks,
            );
            black_box(human(&humans_data));
        });
    });
}

/// Benchmark CNAME data processing
fn bench_cname_processing(c: &mut Criterion) {
    let mut metadata = HashMap::new();
    metadata.insert("cname".to_string(), "example.com".to_string());

    c.bench_function("process CNAME", |b| {
        b.iter(|| {
            let config =
                CnameConfig::new("example.com", None, None).unwrap();
            let cname_data = CnameData {
                cname: config.domain,
                ..Default::default()
            };
            black_box(cname(&cname_data));
        });
    });
}

/// Benchmark path sanitization
fn bench_path_sanitization(c: &mut Criterion) {
    let path = Path::new("content/../sensitive.txt");

    c.bench_function("sanitize path", |b| {
        b.iter(|| {
            black_box(
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

criterion_main!(benches);
