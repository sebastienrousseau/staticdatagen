// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(clippy::unwrap_used, clippy::expect_used)]

//! Demonstrates directory utilities: title case, frontmatter
//! extraction, path truncation, and comrak options.

use staticdatagen::utilities::directory::{
    create_comrak_options, extract_front_matter, to_title_case,
    truncate,
};
use std::path::Path;
use std::time::Instant;

fn main() {
    println!("=== Directory Utilities Example ===\n");

    // --- Title case conversion ---
    let inputs = ["hello-world", "my_first_post", "API"];
    let start = Instant::now();
    for input in &inputs {
        let result = to_title_case(input);
        println!("  title_case({:?}) = {:?}", input, result);
        assert!(!result.is_empty());
    }
    println!(
        "  Converted {} cases in {:?}",
        inputs.len(),
        start.elapsed()
    );

    // --- Frontmatter extraction ---
    let content = "---\ntitle: Test\n---\n# Body";
    let front_matter = extract_front_matter(content);
    println!("\n  Frontmatter: {:?}", front_matter.trim());

    // --- Path truncation ---
    let path = Path::new("/usr/local/share/doc/index.html");
    let truncated = truncate(path, 2);
    println!("  Truncated path: {:?}", truncated);

    // --- Comrak options ---
    let opts = create_comrak_options();
    println!("\n  Comrak autolink: {}", opts.extension.autolink);
    println!("  Comrak footnotes: {}", opts.extension.footnotes);

    println!("\n  All directory examples completed successfully!");
}
