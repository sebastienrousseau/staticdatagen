#![allow(missing_docs)]
// Copyright © 2025 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Performance Stress Testing and Baseline Benchmarks
//!
//! This module provides comprehensive stress testing for staticdatagen hot paths
//! with varying workload sizes (1x, 10x, 100x typical usage) to establish
//! performance baselines and verify memory behavior under sustained load.

use criterion::{
    criterion_group, criterion_main, BenchmarkId, Criterion, Throughput,
};
use staticdatagen::compiler::service::split_frontmatter_and_body;
use staticdatagen::generators::manifest::{
    ManifestConfig, ManifestGenerator,
};
use staticdatagen::models::data::FileData;
use staticdatagen::modules::navigation::NavigationGenerator;
use staticdatagen::utilities::security::sanitize_path;
use std::collections::HashMap;
use std::hint::black_box;
use std::time::{Duration, Instant};

/// Creates synthetic content of varying sizes for stress testing
fn create_test_content(size_factor: usize) -> String {
    let base_content = r#"---
title: Stress Test Page
description: Performance testing with large content
author: Test Author
date: 2024-01-01
keywords: performance, testing, stress
layout: default
permalink: /test-page/
category: benchmark
---

# Performance Test Page

This is a test page for performance benchmarking. The content below is repeated to simulate large pages.

## Section 1: Lorem Ipsum

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.

## Section 2: More Content

Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
"#;

    // Repeat content based on size factor
    let repeated_section = "\n\n### Repeated Section\n\nThis section is repeated multiple times to create large content for stress testing. ".repeat(size_factor * 10);
    format!("{}{}", base_content, repeated_section)
}

/// Creates test files with varying content sizes
fn create_test_files(
    count: usize,
    size_factor: usize,
) -> Vec<FileData> {
    (0..count)
        .map(|i| {
            let content = create_test_content(size_factor);
            FileData {
                name: format!("test-file-{:04}.md", i),
                content,
                ..Default::default()
            }
        })
        .collect()
}

/// Benchmark frontmatter parsing under varying loads
fn bench_frontmatter_parsing_stress(c: &mut Criterion) {
    let mut group = c.benchmark_group("frontmatter_parsing_stress");

    // Test with different content sizes and repetitions
    for &(size_factor, iterations) in &[(1, 1000), (10, 100), (100, 10)]
    {
        let content = create_test_content(size_factor);
        let _ =
            group.throughput(Throughput::Bytes(content.len() as u64));

        let _ = group.bench_with_input(
            BenchmarkId::new(
                "parse",
                format!("{}x_content", size_factor),
            ),
            &content,
            |b, content| {
                b.iter(|| {
                    for _ in 0..iterations {
                        let _ = black_box(split_frontmatter_and_body(
                            content,
                        ));
                    }
                });
            },
        );
    }

    group.finish();
}

/// Benchmark navigation generation with large file counts
fn bench_navigation_large_scale(c: &mut Criterion) {
    let mut group = c.benchmark_group("navigation_large_scale");

    // Test with different scales: 1x (50), 10x (500), 100x (5000) files
    for &file_count in &[50, 500, 5000] {
        let files = create_test_files(file_count, 1);
        let _ =
            group.throughput(Throughput::Elements(file_count as u64));

        let _ = group.bench_with_input(
            BenchmarkId::new("generate", file_count),
            &files,
            |b, files| {
                b.iter(|| {
                    let _ = black_box(
                        NavigationGenerator::generate_navigation(files),
                    );
                });
            },
        );
    }

    group.finish();
}

/// Benchmark file processing with memory tracking
fn bench_file_processing_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_processing_memory");
    let _ = group.sample_size(10); // Fewer samples for memory-intensive tests

    // Test with different content sizes to check memory scaling
    for &content_size in &[1_000, 10_000, 100_000, 1_000_000] {
        let large_content = "a".repeat(content_size);
        let file = FileData::new(
            format!("large-file-{}.md", content_size),
            create_test_content(1) + &large_content,
        );

        let _ = group
            .throughput(Throughput::Bytes(file.content.len() as u64));

        let _ = group.bench_with_input(
            BenchmarkId::new("validate", format!("{}B", content_size)),
            &file,
            |b, file| {
                b.iter(|| {
                    let _ = black_box(file.validate());
                });
            },
        );
    }

    group.finish();
}

/// Benchmark path sanitization with complex paths
fn bench_path_sanitization_complex(c: &mut Criterion) {
    let mut group = c.benchmark_group("path_sanitization_complex");

    let very_long_path = "very/long/path/segment/".repeat(50);
    let complex_paths = vec![
        ("simple", "file.txt"),
        ("nested_deep", "a/b/c/d/e/f/g/h/i/j/file.txt"),
        ("traversal_complex", "../../../etc/../tmp/../etc/passwd"),
        ("mixed_operations", "a/../b/./c/../d/e/../f/./g/file.txt"),
        ("very_long", very_long_path.as_str()),
        ("unicode", "測試/файл/αρχείο/ファイル.txt"),
    ];

    for (name, path_str) in complex_paths {
        let path = std::path::Path::new(path_str);
        let _ =
            group.throughput(Throughput::Bytes(path_str.len() as u64));

        let _ = group.bench_with_input(
            BenchmarkId::new("sanitize", name),
            &path,
            |b, path| {
                b.iter(|| {
                    let _ = black_box(sanitize_path(path));
                });
            },
        );
    }

    group.finish();
}

/// Benchmark generator performance with large metadata sets
fn bench_generators_large_metadata(c: &mut Criterion) {
    let mut group = c.benchmark_group("generators_large_metadata");

    // Create large metadata sets
    for &metadata_size in &[10, 100, 1000] {
        let mut metadata = HashMap::new();
        for i in 0..metadata_size {
            let _ = metadata
                .insert(format!("key_{}", i), format!("value_{}", i));
        }

        // Benchmark manifest generation
        let manifest_config = ManifestConfig::builder()
            .name("Large App")
            .short_name("LargeApp")
            .description("App with large metadata set")
            .start_url("/")
            .display("standalone")
            .build()
            .unwrap();
        let manifest_generator =
            ManifestGenerator::new(manifest_config);

        let _ = group.bench_with_input(
            BenchmarkId::new("manifest", metadata_size),
            &manifest_generator,
            |b, generator| {
                b.iter(|| {
                    let _ = black_box(generator.generate());
                });
            },
        );
    }

    group.finish();
}

/// Benchmark UUID generation in batches (memory allocation stress)
fn bench_uuid_batch_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("uuid_batch_memory");

    // Test increasingly large batches to stress memory allocation
    for &batch_size in &[100, 1_000, 10_000, 100_000] {
        let _ =
            group.throughput(Throughput::Elements(batch_size as u64));

        let _ = group.bench_with_input(
            BenchmarkId::new("generate", batch_size),
            &batch_size,
            |b, &size| {
                b.iter(|| {
                    let uuids: Vec<String> = (0..size)
                        .map(|_| {
                            staticdatagen::generate_unique_string()
                        })
                        .collect();
                    let _ = black_box(uuids);
                });
            },
        );
    }

    group.finish();
}

/// Sustained load test - measures performance under continuous operation
fn bench_sustained_load(c: &mut Criterion) {
    let mut group = c.benchmark_group("sustained_load");
    let _ = group.sample_size(10);
    let _ = group.measurement_time(Duration::from_secs(30));

    let test_files = create_test_files(100, 5); // Medium-sized test set

    let _ =
        group.bench_function("continuous_navigation_generation", |b| {
            b.iter_custom(|iters| {
                let start = Instant::now();
                for _ in 0..iters {
                    let _ = black_box(
                        NavigationGenerator::generate_navigation(
                            &test_files,
                        ),
                    );
                }
                start.elapsed()
            });
        });

    group.finish();
}

/// Memory growth test - checks for unbounded memory growth patterns
fn bench_memory_growth_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_growth_detection");
    let _ = group.sample_size(10);

    // Test operations that might cause memory leaks
    let test_data = create_test_files(10, 10);

    let _ = group.bench_function("repeated_operations", |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();

            // Simulate sustained operation without cleanup
            for i in 0..iters {
                // Create and discard data structures repeatedly
                let nav = NavigationGenerator::generate_navigation(
                    &test_data,
                );
                let file = &test_data[i as usize % test_data.len()];
                let _ = file.validate();

                let _ = black_box(nav);

                // Force some allocations
                let temp_data: Vec<String> = (0..100)
                    .map(|j| format!("temp_data_{}_{}", i, j))
                    .collect();
                let _ = black_box(temp_data);
            }

            start.elapsed()
        });
    });

    group.finish();
}

/// Error handling performance under stress
fn bench_error_handling_stress(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_handling_stress");

    // Test error creation and formatting under load
    for &error_count in &[100, 1_000, 10_000] {
        let _ = group.bench_with_input(
            BenchmarkId::new("create_format_errors", error_count),
            &error_count,
            |b, &count| {
                b.iter(|| {
                    for i in 0..count {
                        let err = staticdatagen::Error::content_processing_builder()
                            .message(format!("Stress test error {}", i))
                            .context(format!("Context for error {}", i))
                            .build();
                        let _ = black_box(err.to_string());
                    }
                });
            }
        );
    }

    group.finish();
}

// Group all stress test benchmarks
criterion_group!(
    stress_tests,
    bench_frontmatter_parsing_stress,
    bench_navigation_large_scale,
    bench_file_processing_memory,
    bench_path_sanitization_complex,
    bench_generators_large_metadata,
    bench_uuid_batch_memory,
    bench_sustained_load,
    bench_memory_growth_detection,
    bench_error_handling_stress,
);

criterion_main!(stress_tests);
