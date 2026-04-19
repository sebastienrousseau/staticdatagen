// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(clippy::unwrap_used, clippy::expect_used)]

//! Demonstrates the file writer: writing FileData to build
//! directories.

use staticdatagen::models::data::FileData;
use staticdatagen::utilities::write::write_files_to_build_directory;
use std::fs;
use std::time::Instant;

fn main() {
    println!("=== File Writer Example ===\n");

    let build_dir = tempfile::tempdir().unwrap();
    let template_dir = tempfile::tempdir().unwrap();

    // Create template files
    fs::write(template_dir.path().join("main.js"), "// main").unwrap();
    fs::write(template_dir.path().join("sw.js"), "// sw").unwrap();

    // --- Write index files ---
    let file = FileData {
        name: "index.md".to_string(),
        content: "<html><body><p>Hello</p></body></html>".to_string(),
        rss: "<rss></rss>".to_string(),
        txt: "User-agent: *\nAllow: /".to_string(),
        cname: "example.com".to_string(),
        human: "# humans.txt".to_string(),
        manifest: r#"{"name":"test"}"#.to_string(),
        sitemap: "<urlset/>".to_string(),
        sitemap_news: "<news/>".to_string(),
        security: "Contact: security@example.com".to_string(),
        ..Default::default()
    };

    let start = Instant::now();
    let result = write_files_to_build_directory(
        build_dir.path(),
        &file,
        template_dir.path(),
    );
    let elapsed = start.elapsed();

    println!("  Build dir: {}", build_dir.path().display());
    println!("  Result:    {:?}", result.is_ok());
    println!("  Written in: {:?}", elapsed);

    // Verify files created
    let index = build_dir.path().join("index.html");
    let cname = build_dir.path().join("CNAME");
    println!("  index.html: {}", index.exists());
    println!("  CNAME:      {}", cname.exists());

    println!("\n  All file writer examples completed successfully!");
}
