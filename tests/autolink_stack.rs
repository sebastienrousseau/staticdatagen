// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Regression tests for GHSA-xg9p-p4jc-c46g.
//!
//! comrak 0.54 and earlier recursed once per bare email autolink in a
//! text node, so a paragraph of a few thousand addresses overflowed the
//! stack. A stack overflow aborts the process instead of panicking,
//! which is why these tests have a binary of their own: on a regression
//! the abort takes down only this file's test run, and `cargo test`
//! reports it as a SIGABRT.
//!
//! Two paths reach comrak: `create_comrak_options` with this crate's
//! own `comrak`, and [`staticdatagen::compile`], which renders through
//! `html-generator` and `mdx-gen`.

use std::{fs, path::Path};

use staticdatagen::utilities::directory::create_comrak_options;

/// Bare email addresses in one paragraph. In a debug build, comrak 0.54
/// renders 400 of them on a 512 KiB stack and aborts at 500.
const EMAILS: usize = 3_000;

/// Small enough that recursion per autolink cannot fit, large enough
/// for comrak's ordinary work and for compiling a one-page site.
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

#[test]
fn bare_email_autolinks_compile_on_a_small_stack() {
    let html = on_small_stack(|| compile_page(&emails()));
    assert_eq!(html.matches("mailto:user").count(), EMAILS);
}

/// Compiles one page whose body is `body` through
/// [`staticdatagen::compile`] and returns the page.
fn compile_page(body: &str) -> String {
    let build = tempfile::TempDir::new().unwrap();
    let content = tempfile::TempDir::new().unwrap();
    let site = tempfile::TempDir::new().unwrap();
    let templates = tempfile::TempDir::new().unwrap();

    fs::write(
        content.path().join("page.md"),
        format!(
            "---\ntitle: Addresses\nlayout: page\n\
             description: Bare addresses\n\
             permalink: https://example.com/page\n---\n{body}\n"
        ),
    )
    .unwrap();
    fs::write(
        templates.path().join("page.html"),
        "<html><body>{{content}}</body></html>",
    )
    .unwrap();
    fs::write(templates.path().join("main.js"), "// main").unwrap();
    fs::write(templates.path().join("sw.js"), "// sw").unwrap();
    fs::create_dir_all(build.path().join("tags")).unwrap();
    fs::write(
        build.path().join("tags/index.html"),
        "<html><body>[[content]]</body></html>",
    )
    .unwrap();

    staticdatagen::compile(
        build.path(),
        content.path(),
        site.path(),
        templates.path(),
    )
    .unwrap();

    read_page(site.path())
}

/// The one `index.html` the page compiled to.
fn read_page(site: &Path) -> String {
    let page = walkdir::WalkDir::new(site)
        .into_iter()
        .filter_map(Result::ok)
        .map(walkdir::DirEntry::into_path)
        .find(|p| {
            p.file_name().is_some_and(|n| n == "index.html")
                && p.to_string_lossy().contains("page")
        })
        .expect("the page is written");
    fs::read_to_string(page).unwrap()
}
