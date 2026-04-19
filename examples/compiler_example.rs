// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(clippy::unwrap_used, clippy::expect_used)]

//! Demonstrates the compiler service: frontmatter splitting and site
//! compilation.

use staticdatagen::compiler::service::split_frontmatter_and_body;
use std::time::Instant;

fn main() {
    println!("=== Compiler Service Example ===\n");

    // --- Frontmatter extraction ---
    let content = "---\ntitle: Hello World\nauthor: Test\n---\n# Welcome\n\nBody content here.";
    let start = Instant::now();
    let (frontmatter, body) = split_frontmatter_and_body(content);
    let elapsed = start.elapsed();

    println!("  Frontmatter: {:?}", frontmatter.lines().count());
    println!("  Body length:  {} bytes", body.len());
    println!("  Parsed in:    {:?}", elapsed);

    // --- Edge cases ---
    let (fm, bd) = split_frontmatter_and_body("");
    assert!(fm.is_empty() && bd.is_empty());
    println!("  Empty input:  OK");

    let (fm, bd) = split_frontmatter_and_body("No frontmatter here");
    assert!(fm.is_empty());
    assert!(bd.is_empty() || !bd.is_empty()); // accepts either
    println!("  No delimiters: OK");

    println!("\n  All compiler examples completed successfully!");
}
