// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Regression test for GHSA-xg9p-p4jc-c46g.
//!
//! comrak 0.54 and earlier recursed once per bare email autolink in a
//! text node, so a paragraph of a few thousand addresses overflowed the
//! stack. A stack overflow aborts the process instead of panicking,
//! which is why this test has a binary of its own: on a regression the
//! abort takes down only this file's test run, and `cargo test` reports
//! it as a SIGABRT.

use staticdatagen::utilities::directory::create_comrak_options;

/// Bare email addresses in one paragraph. In a debug build, comrak 0.54
/// renders 400 of them on a 512 KiB stack and aborts at 500.
const EMAILS: usize = 3_000;

/// Small enough that recursion per autolink cannot fit, large enough
/// for comrak's ordinary work.
const STACK_BYTES: usize = 512 * 1024;

/// One paragraph of [`EMAILS`] distinct bare addresses.
fn emails() -> String {
    (0..EMAILS)
        .map(|i| format!("user{i}@example.com "))
        .collect()
}

/// Runs `work` on a thread with a [`STACK_BYTES`] stack.
fn on_small_stack<F>(work: F) -> String
where
    F: FnOnce() -> String + Send + 'static,
{
    std::thread::Builder::new()
        .name("autolink-512k".to_owned())
        .stack_size(STACK_BYTES)
        .spawn(work)
        .expect("spawn the render thread")
        .join()
        .expect("the render thread panicked")
}

#[test]
fn bare_email_autolinks_render_on_a_small_stack() {
    let markdown = emails();
    let html = on_small_stack(move || {
        comrak::markdown_to_html(&markdown, &create_comrak_options())
    });
    assert_eq!(html.matches("<a href=\"mailto:user").count(), EMAILS);
}
