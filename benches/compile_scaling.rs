// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(missing_docs, clippy::unwrap_used, clippy::expect_used)]

//! # Compile scaling benchmark
//!
//! `compile` is where a build spends its time — profiling a 500-page corpus
//! put 94% of wall-clock inside it, which is what motivated the parallel
//! rewrite in 0.0.12. The README quotes a figure for it, and until now
//! nothing in the repository reproduced that figure: the two existing
//! benchmark files measure navigation, front matter, generators and path
//! sanitisation, none of which call `compile`.
//!
//! This measures the claim directly, at the sizes that matter:
//!
//! - **12 pages** — below `PARALLEL_THRESHOLD`, so this stays on the calling
//!   thread. Guards against the threshold regressing into pool-spawn overhead
//!   for small sites.
//! - **50 / 200 / 500 pages** — above it, so this is the parallel path. 500 is
//!   the size the README quotes.
//!
//! Run with `cargo bench --bench compile_scaling`.

use criterion::{
    criterion_group, criterion_main, BenchmarkId, Criterion, Throughput,
};
use std::fs;
use std::hint::black_box;
use std::path::Path;
use tempfile::TempDir;

/// Builds a corpus of `n` Markdown pages with front matter representative of
/// a real site: every field the compiler reads is present, so the benchmark
/// exercises the parsing path rather than a degenerate empty one.
fn corpus(dir: &Path, n: usize) {
    fs::create_dir_all(dir).unwrap();
    for i in 0..n {
        let body = format!(
            "---\n\
             title: Page {i}\n\
             description: A benchmark page, number {i} of the corpus.\n\
             permalink: https://example.com/page-{i}\n\
             layout: page\n\
             author: Benchmark\n\
             changefreq: weekly\n\
             tags: bench,page-{tag}\n\
             ---\n\n\
             # Page {i}\n\n\
             Body text for page {i}. It is deliberately more than one line so\n\
             the Markdown pass has something to do beyond a heading.\n\n\
             - a list item\n\
             - another item\n\n\
             A closing paragraph.\n",
            i = i,
            tag = i % 7,
        );
        fs::write(dir.join(format!("page-{i}.md")), body).unwrap();
    }
}

/// The minimal template `compile` needs.
///
/// The engine resolves a template per page from the `layout:` front-matter
/// key, defaulting to `page`. The corpus above sets `layout: page`, so this
/// must be written as `page.html` — a `default.html` is never looked up and
/// leaves `compile` returning
/// `Err("I/O error: No such file or directory")` after writing nothing,
/// which would turn this into a benchmark of the error path.
const TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head><meta charset="UTF-8"><title>{{title}}</title></head>
<body>{{content}}</body>
</html>"#;

/// Name matters: it must match the corpus's `layout:` value.
const TEMPLATE_NAME: &str = "page.html";

/// Prepares a corpus and returns the directories, excluded from timing.
fn prepare(pages: usize) -> (TempDir, [std::path::PathBuf; 4]) {
    let tmp = TempDir::new().unwrap();
    let content = tmp.path().join("content");
    let build = tmp.path().join("build");
    let site = tmp.path().join("public");
    let templates = tmp.path().join("templates");

    corpus(&content, pages);
    for d in [&build, &site, &templates] {
        fs::create_dir_all(d).unwrap();
    }
    fs::write(templates.join(TEMPLATE_NAME), TEMPLATE).unwrap();

    (tmp, [build, content, site, templates])
}

/// One `compile` run over an already-prepared corpus.
///
/// Corpus creation is deliberately outside this function: writing 500
/// Markdown files is filesystem work that has nothing to do with the thing
/// being measured, and at 500 pages it dominated the first version of this
/// benchmark.
///
/// Every path is inside a `TempDir`, so a run cannot touch the repository —
/// the failure mode `service_example` shipped with before 0.0.13.
fn compile_once(dirs: &[std::path::PathBuf; 4]) {
    let result = staticdatagen::compiler::service::compile(
        black_box(&dirs[0]),
        black_box(&dirs[1]),
        black_box(&dirs[2]),
        black_box(&dirs[3]),
    );
    // A failed compile does almost no work, so a silent Err would turn this
    // into a benchmark of the error path — which is exactly what the first
    // version of this file measured.
    assert!(
        result.is_ok(),
        "compile must succeed or the measurement is meaningless: {result:?}"
    );
}

fn compile_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("compile_scaling");

    // Building the corpus dominates setup, so keep the sample count low —
    // this benchmark is about relative scaling, not micro-precision.
    let _ = group.sample_size(10);

    for &pages in &[12usize, 50, 200, 500] {
        let _ = group.throughput(Throughput::Elements(pages as u64));
        let _ = group.bench_with_input(
            BenchmarkId::from_parameter(pages),
            &pages,
            |b, &pages| {
                // Re-prepared per batch rather than per iteration: `compile`
                // writes into `site`, so a fresh tree each batch keeps runs
                // comparable without paying corpus setup on every iteration.
                b.iter_batched(
                    || prepare(pages),
                    |(tmp, dirs)| {
                        compile_once(&dirs);
                        drop(tmp);
                    },
                    criterion::BatchSize::PerIteration,
                );
            },
        );
    }

    group.finish();
}

criterion_group!(benches, compile_scaling);
criterion_main!(benches);
