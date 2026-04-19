// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(clippy::unwrap_used, clippy::expect_used)]

//! Demonstrates HTML post-processing: class attribute injection and
//! img title generation.

use regex::Regex;
use staticdatagen::modules::postprocessor::post_process_html;
use std::time::Instant;

fn main() {
    println!("=== Postprocessor Example ===\n");

    let class_regex = Regex::new(r#"<p\.class="([^"]*)""#).unwrap();
    let img_regex = Regex::new(r#"(<img[^>]*)(>)"#).unwrap();

    // --- Basic HTML processing ---
    let html = r#"<p>Hello world</p>
<img src="photo.jpg" alt="Sunset over the ocean">
<p.class="highlight">Important text</p>"#;

    let start = Instant::now();
    let result =
        post_process_html(html, &class_regex, &img_regex).unwrap();
    let elapsed = start.elapsed();

    println!("  Input:  {} lines", html.lines().count());
    println!("  Output: {} lines", result.lines().count());
    println!("  Title injected: {}", result.contains("title="));
    println!(
        "  Class applied:  {}",
        result.contains("class=\"highlight\"")
    );
    println!("  Processed in:   {:?}", elapsed);

    // --- Empty input ---
    let empty =
        post_process_html("", &class_regex, &img_regex).unwrap();
    assert!(empty.trim().is_empty());
    println!("  Empty input:    OK");

    println!("\n  All postprocessor examples completed successfully!");
}
