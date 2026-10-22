// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Every character HTML gives meaning to is escaped, wherever it falls.
//!
//! From comrak 0.56 the HTML escaper finds `<`, `>`, `&` and `"` with
//! memchr-n, which scans 16, 32 or 64 bytes at a time with SIMD and then
//! handles the tail separately. A match the vector kernels or the tail
//! path missed would reach the page unescaped. That is a cross-site
//! scripting bug, and an ordinary rendering test would not see it unless
//! the character happened to sit at the missed offset.
//!
//! So each character is placed at every offset of every length up to
//! [`MAX_LEN`], which covers the scalar path, each vector width and
//! every tail length. The text sits in a code span: it goes through the
//! same escaper as paragraph text, but smart punctuation leaves it alone,
//! so `"` must come out as `&quot;` rather than a curly quote.

use staticdatagen::utilities::directory::create_comrak_options;

/// Longer than two 64-byte vectors plus a full tail.
const MAX_LEN: usize = 160;

/// Each character and the entity the escaper must write for it.
const SPECIAL: [(char, &str); 4] = [
    ('<', "&lt;"),
    ('>', "&gt;"),
    ('&', "&amp;"),
    ('"', "&quot;"),
];

#[test]
fn every_special_character_is_escaped_at_every_offset() {
    let options = create_comrak_options();
    for len in 1..=MAX_LEN {
        for (ch, entity) in SPECIAL {
            for at in 0..len {
                let text: String = (0..len)
                    .map(|i| if i == at { ch } else { 'a' })
                    .collect();
                let html = comrak::markdown_to_html(
                    &format!("`{text}`"),
                    &options,
                );
                let expected = format!(
                    "<p><code>{}</code></p>\n",
                    text.replace(ch, entity)
                );
                assert_eq!(
                    html, expected,
                    "{ch:?} at offset {at} of {len} was not escaped"
                );
            }
        }
    }
}
