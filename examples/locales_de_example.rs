// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(clippy::unwrap_used, clippy::expect_used)]

//! Demonstrates German locale translations.

use staticdatagen::locales::de::translate;

fn main() {
    println!("=== German Locale Example ===\n");

    let keys =
        ["Hello", "Goodbye", "main_logger_msg", "lib_banner_log_msg"];

    for key in &keys {
        match translate(key) {
            Ok(val) => {
                println!("  de[{:20}] = {}", key, val)
            }
            Err(e) => {
                println!("  de[{:20}] = ERROR: {}", key, e)
            }
        }
    }

    // --- Missing key ---
    let missing = translate("nonexistent_key");
    println!("\n  Missing key error: {}", missing.is_err());

    println!("\n  All German locale examples completed successfully!");
}
