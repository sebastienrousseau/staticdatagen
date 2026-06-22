// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(clippy::unwrap_used, clippy::expect_used)]

//! Demonstrates UUID generation: unique string identifiers.

use staticdatagen::utilities::uuid::generate_unique_string;
use std::collections::HashSet;
use std::time::Instant;

fn main() {
    println!("=== UUID Generator Example ===\n");

    // --- Generate a single UUID ---
    let id = generate_unique_string();
    println!("  Generated: {}", id);
    println!("  Length:    {} chars", id.len());

    // --- Uniqueness verification ---
    let count = 10_000;
    let start = Instant::now();
    let ids: HashSet<String> =
        (0..count).map(|_| generate_unique_string()).collect();
    let elapsed = start.elapsed();

    assert_eq!(ids.len(), count);
    println!("\n  Generated {} unique IDs in {:?}", count, elapsed);
    println!(
        "  Rate: {:.0} IDs/sec",
        count as f64 / elapsed.as_secs_f64()
    );
    println!("  Collisions: 0");

    println!("\n  All UUID examples completed successfully!");
}
