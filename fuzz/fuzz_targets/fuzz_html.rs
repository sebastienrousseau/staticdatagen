// SPDX-License-Identifier: Apache-2.0 OR MIT
//! HTML post-processing over arbitrary input.
//!
//! `post_process_html` rewrites generated markup with regex
//! substitutions and manual string building. It must be total: a panic
//! here aborts a site build partway through writing output.
#![no_main]

use libfuzzer_sys::fuzz_target;
use regex::Regex;
use staticdatagen::modules::postprocessor::post_process_html;

fuzz_target!(|data: &[u8]| {
    let Ok(html) = std::str::from_utf8(data) else {
        return;
    };
    let class_regex = Regex::new(r#"\.class=&quot;([^&]+)&quot;"#)
        .expect("static pattern");
    let img_regex = Regex::new(r"(<img[^>]*)(/?>)").expect("static pattern");
    let _ = post_process_html(html, &class_regex, &img_regex);
});
