// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(clippy::unwrap_used, clippy::expect_used)]

//! Demonstrates file scanning: reading content directories into
//! FileData structs.

use staticdatagen::utilities::file::add;
use std::fs;
use std::time::Instant;

fn main() {
    println!("=== File Scanner Example ===\n");

    let dir = tempfile::tempdir().unwrap();

    // --- Create test content files ---
    fs::write(
        dir.path().join("index.md"),
        "---\ntitle: Home\n---\n# Welcome",
    )
    .unwrap();
    fs::write(
        dir.path().join("about.md"),
        "---\ntitle: About\n---\n# About Us",
    )
    .unwrap();
    fs::write(dir.path().join("style.css"), "body { color: black; }")
        .unwrap();

    // --- Scan directory ---
    let start = Instant::now();
    let files = add(dir.path()).unwrap();
    let elapsed = start.elapsed();

    println!("  Directory: {}", dir.path().display());
    println!("  Files found: {}", files.len());
    for f in &files {
        println!("    - {} ({} bytes)", f.name, f.content.len());
    }
    println!("  Scanned in: {:?}", elapsed);

    // --- Empty directory ---
    let empty_dir = tempfile::tempdir().unwrap();
    let empty_files = add(empty_dir.path()).unwrap();
    assert!(empty_files.is_empty());
    println!("  Empty dir: 0 files (OK)");

    println!("\n  All file scanner examples completed successfully!");
}
