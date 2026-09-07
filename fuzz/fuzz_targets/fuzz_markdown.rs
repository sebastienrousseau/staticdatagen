// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Content preprocessing and plain-text extraction over arbitrary input.
//!
//! Both run over author-supplied Markdown before anything else sees it,
//! and both walk the string by hand rather than through a parser, which
//! is where byte-boundary panics live.
#![no_main]

use libfuzzer_sys::fuzz_target;
use regex::Regex;
use staticdatagen::modules::{
    plaintext::generate_plain_text, preprocessor::preprocess_content,
};

fuzz_target!(|data: &[u8]| {
    let Ok(content) = std::str::from_utf8(data) else {
        return;
    };

    let class_regex = Regex::new(r#"\.class=&quot;([^&]+)&quot;"#)
        .expect("static pattern");
    let img_regex = Regex::new(r"(<img[^>]*)(/?>)").expect("static pattern");
    let _ = preprocess_content(content, &class_regex, &img_regex);

    let _ = generate_plain_text(content, "t", "d", "a", "c", "k");
});
