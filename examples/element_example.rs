// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(clippy::unwrap_used, clippy::expect_used)]

//! Demonstrates XML element writing with proper escaping.

use quick_xml::Writer;
use staticdatagen::utilities::element::write_element;
use std::io::Cursor;
use std::time::Instant;

fn main() {
    println!("=== XML Element Writer Example ===\n");

    // --- Write a simple element ---
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    write_element(&mut writer, "title", "Hello World").unwrap();
    let xml =
        String::from_utf8(writer.into_inner().into_inner()).unwrap();
    println!("  Simple:  {}", xml);
    assert!(xml.contains("<title>Hello World</title>"));

    // --- Special character escaping ---
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    write_element(
        &mut writer,
        "desc",
        "Tom & Jerry <friends> \"best\"",
    )
    .unwrap();
    let xml =
        String::from_utf8(writer.into_inner().into_inner()).unwrap();
    println!("  Escaped: {}", xml);
    assert!(xml.contains("&amp;"));
    assert!(xml.contains("&lt;"));

    // --- Empty value (no element written) ---
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    write_element(&mut writer, "empty", "").unwrap();
    let xml =
        String::from_utf8(writer.into_inner().into_inner()).unwrap();
    assert!(xml.is_empty());
    println!("  Empty:   (no output) OK");

    // --- Batch performance ---
    let start = Instant::now();
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    for i in 0..10_000 {
        write_element(&mut writer, "item", &format!("value_{}", i))
            .unwrap();
    }
    println!("\n  Wrote 10,000 elements in {:?}", start.elapsed());

    println!("\n  All element examples completed successfully!");
}
