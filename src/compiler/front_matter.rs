// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Where a page's front matter ends and its body begins.
//!
//! Re-exported as [`crate::compiler::service::split_frontmatter_and_body`].

use metadata_gen::{detect_front_matter, FrontMatterFormat};

/// Splits a Markdown content string into frontmatter and body parts.
///
/// The front matter is the block `metadata-gen` finds, so the body and the
/// metadata always agree on where the front matter ends: YAML (`---`),
/// TOML (`+++`) and JSON (`{ }`) blocks, after an optional UTF-8 BOM or
/// blank lines. Lines are rejoined with `\n` and both parts are trimmed.
///
/// When `metadata-gen` finds no front matter, the content is split on the
/// first two `---` lines, as before.
///
/// # Parameters
///
/// * `content` - A reference to a string containing the Markdown content.
///
/// # Returns
///
/// A tuple containing two strings:
/// - The first string represents the frontmatter part of the content.
/// - The second string represents the body part of the content.
///
/// With no front matter the first string is empty and the second is the
/// whole content, trimmed.
pub fn split_frontmatter_and_body(content: &str) -> (String, String) {
    match detect_front_matter(content) {
        Some((format, raw, body_offset)) if is_block(format, raw) => (
            normalise_lines(raw),
            normalise_lines(&content[body_offset..]),
        ),
        _ => split_on_dashes(content),
    }
}

/// Whether `metadata-gen`'s block is front matter. A leading `{` that is
/// not a JSON object (a `{{< shortcode >}}`, say) is reported as a JSON
/// block running to the end of the document; that is body, not front
/// matter.
fn is_block(format: FrontMatterFormat, raw: &str) -> bool {
    format != FrontMatterFormat::Json
        || serde_json::from_str::<serde_json::Value>(raw).is_ok()
}

/// Rejoins `text` line by line with `\n` and trims it, which is how the
/// split has always shaped both parts (CRLF input included).
fn normalise_lines(text: &str) -> String {
    text.lines()
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

/// The split for content with no front matter `metadata-gen` recognises:
/// everything between the first two `---` lines is front matter.
fn split_on_dashes(content: &str) -> (String, String) {
    let mut lines = content.lines();
    let mut frontmatter = String::new();
    let mut body = String::new();
    let mut in_frontmatter = false;

    for line in &mut lines {
        if line.trim() == "---" {
            if in_frontmatter {
                // Ending the frontmatter
                break;
            } else {
                // Starting the frontmatter
                in_frontmatter = true;
                continue;
            }
        }

        if in_frontmatter {
            frontmatter.push_str(line);
            frontmatter.push('\n');
        } else {
            body.push_str(line);
            body.push('\n');
        }
    }

    // Append the rest of the lines to the body
    for line in lines {
        body.push_str(line);
        body.push('\n');
    }

    (frontmatter.trim().to_string(), body.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::split_frontmatter_and_body;

    #[test]
    fn bom_before_the_fence_is_not_body() {
        let (fm, body) = split_frontmatter_and_body(
            "\u{feff}---\ntitle: T\n---\nBody.\n",
        );
        assert_eq!(fm, "title: T");
        assert_eq!(body, "Body.");
    }

    #[test]
    fn toml_front_matter_is_not_body() {
        let (fm, body) = split_frontmatter_and_body(
            "+++\ntitle = \"T\"\n+++\nBody.\n",
        );
        assert_eq!(fm, "title = \"T\"");
        assert_eq!(body, "Body.");
    }

    #[test]
    fn json_front_matter_is_not_body() {
        let (fm, body) = split_frontmatter_and_body(
            "{\n\"title\": \"T\"\n}\nBody.\n",
        );
        assert!(fm.contains("\"title\": \"T\""), "front matter: {fm}");
        assert_eq!(body, "Body.");
    }

    #[test]
    fn a_leading_shortcode_is_body() {
        let content = "{{% note %}}Body{{% /note %}}\n";
        let (fm, body) = split_frontmatter_and_body(content);
        assert!(fm.is_empty(), "front matter: {fm}");
        assert_eq!(body, "{{% note %}}Body{{% /note %}}");
    }

    #[test]
    fn crlf_body_is_rejoined_with_lf() {
        let (fm, body) = split_frontmatter_and_body(
            "---\r\ntitle: T\r\n---\r\nLine 1\r\nLine 2\r\n",
        );
        assert_eq!(fm, "title: T");
        assert_eq!(body, "Line 1\nLine 2");
    }

    #[test]
    fn a_thematic_break_in_the_body_stays_in_the_body() {
        let (fm, body) = split_frontmatter_and_body(
            "+++\ntitle = \"T\"\n+++\nAbove\n\n---\n\nBelow\n",
        );
        assert_eq!(fm, "title = \"T\"");
        assert_eq!(body, "Above\n\n---\n\nBelow");
    }
}
