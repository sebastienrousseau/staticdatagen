// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! What a compiled page carries from its front matter.
//!
//! The `{{primary}}`, `{{opengraph}}` and `{{twitter}}` layout variables
//! are the meta tags `metadata-gen` renders, and the README's output
//! stability guarantee covers them. These tests pin the shape that
//! `metadata-gen` 0.0.8 produces, so a dependency move that changes it
//! fails here rather than on a downstream site.

use std::{fs, path::Path};

/// Compiles one Markdown page through [`staticdatagen::compile`] with a
/// layout that prints the meta tag groups, and returns the page.
fn compile_page(markdown: &str) -> String {
    let build = tempfile::TempDir::new().unwrap();
    let content = tempfile::TempDir::new().unwrap();
    let site = tempfile::TempDir::new().unwrap();
    let templates = tempfile::TempDir::new().unwrap();

    fs::write(content.path().join("page.md"), markdown).unwrap();
    fs::write(
        templates.path().join("page.html"),
        "<html><head>{{primary}}\n{{opengraph}}\n{{twitter}}</head>\
         <body>{{content}}</body></html>",
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

const FRONT_MATTER: &str = "---\n\
title: Fish and Chips\n\
layout: page\n\
permalink: https://example.com/page\n\
description: Fish & Chips <b>\n\
author: Test\n\
og:title: Fish & Chips\n\
twitter:card: summary\n\
---\n\
Body.\n";

/// Open Graph tags use `property=`, as the protocol requires; Twitter
/// tags keep `name=`.
#[test]
fn open_graph_tags_use_property() {
    let html = compile_page(FRONT_MATTER);

    assert!(
        html.contains(
            "<meta property=\"og:title\" content=\"Fish &amp; Chips\">"
        ),
        "og:title must render with property=, got: {html}"
    );
    assert!(
        !html.contains("name=\"og:"),
        "no Open Graph tag may use name=, got: {html}"
    );
    assert!(
        html.contains(
            "<meta name=\"twitter:card\" content=\"summary\">"
        ),
        "twitter:card must keep name=, got: {html}"
    );
}

/// Meta tag attribute values escape `&`, `<` and `>` as well as `"`,
/// so front matter text cannot break out of the attribute or the tag.
#[test]
fn meta_tag_values_are_escaped() {
    let html = compile_page(FRONT_MATTER);

    assert!(
        html.contains(
            "<meta name=\"description\" content=\"Fish &amp; Chips &lt;b&gt;\">"
        ),
        "description must be attribute-escaped, got: {html}"
    );
    assert!(
        !html.contains("content=\"Fish & Chips <b>\""),
        "an unescaped value reached a meta tag, got: {html}"
    );
}

/// A page whose front matter is TOML, or YAML after a UTF-8 BOM, renders
/// its body only. The front matter used to reach the page as text: a TOML
/// block always, and a BOM page once `metadata-gen` 0.0.8 began to find
/// its front matter.
#[test]
fn front_matter_never_reaches_the_body() {
    let pages = [
        "+++\ntitle = \"T\"\nlayout = \"page\"\n\
         permalink = \"https://example.com/page\"\n\
         description = \"D\"\n+++\nBody text.\n",
        "\u{feff}---\ntitle: T\nlayout: page\n\
         permalink: https://example.com/page\ndescription: D\n---\n\
         Body text.\n",
    ];
    for markdown in pages {
        let html = compile_page(markdown);
        let body = html.split("<body>").nth(1).unwrap_or_default();

        assert!(body.contains("Body text."), "body lost, got: {html}");
        assert!(
            !body.contains("permalink"),
            "front matter leaked into the body, got: {html}"
        );
    }
}
