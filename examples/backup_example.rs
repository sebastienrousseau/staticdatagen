// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(clippy::unwrap_used, clippy::expect_used)]

//! Demonstrates file backup: creating .src.html backup copies.

use staticdatagen::utilities::backup::backup_file;
use std::fs;
use std::time::Instant;

fn main() {
    println!("=== Backup Utility Example ===\n");

    let dir = tempfile::tempdir().unwrap();

    // --- Create and back up a file ---
    let source = dir.path().join("index.md");
    fs::write(&source, "# Hello\n\nBackup test content.").unwrap();

    let start = Instant::now();
    let backup_path = backup_file(&source).unwrap();
    let elapsed = start.elapsed();

    println!(
        "  Source:  {}",
        source.file_name().unwrap().to_str().unwrap()
    );
    println!(
        "  Backup:  {}",
        backup_path.file_name().unwrap().to_str().unwrap()
    );
    println!("  Exists:  {}", backup_path.exists());
    println!("  Created in: {:?}", elapsed);

    // --- Verify content preserved ---
    let original = fs::read_to_string(&source).unwrap();
    let backed_up = fs::read_to_string(&backup_path).unwrap();
    assert_eq!(original, backed_up);
    println!("  Content match: OK");

    println!("\n  All backup examples completed successfully!");
}
