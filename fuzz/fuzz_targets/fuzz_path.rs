// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Path sanitisation over arbitrary input.
//!
//! This is a security boundary: it decides which paths a build may
//! touch. Two properties beyond not panicking — a path it accepts must
//! never contain a `..` component, and accepting is idempotent, so
//! sanitising twice cannot smuggle anything past the first pass.
#![no_main]

use libfuzzer_sys::fuzz_target;
use staticdatagen::utilities::security::sanitize_path;
use std::path::{Component, Path};

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    if let Ok(clean) = sanitize_path(Path::new(text)) {
        assert!(
            !clean.components().any(|c| c == Component::ParentDir),
            "sanitize_path accepted a traversal: {text:?} -> {clean:?}"
        );
        if let Ok(again) = sanitize_path(&clean) {
            assert_eq!(
                clean, again,
                "sanitising an accepted path changed it"
            );
        }
    }
});
