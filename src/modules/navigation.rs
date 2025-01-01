// Copyright © 2025 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Navigation Menu Generation Module
//!
//! This module handles the generation of navigation menus for a static site or similar web-based output. It processes file metadata (`FileData`) to create accessible, semantic HTML navigation structures.
//!
//! ## Features
//!
//! - Automatic navigation menu generation from content files
//! - Accessibility support with ARIA attributes
//! - Semantic HTML structure
//! - Support for multiple file types
//! - URL path normalization
//! - Title case conversion for display
//! - **Extended Sanitization and Security** (handles special characters, invalid paths, etc.)
//! - **Alphabetical Sorting** of navigation entries
//! - **Performance Considerations** for large file sets
//!
//! ## Example
//!
//! ```rust
//! use staticdatagen::models::data::FileData;
//! use staticdatagen::modules::navigation::NavigationGenerator;
//!
//! let files = vec![
//!     FileData {
//!         name: "about.md".to_string(),
//!         content: "About page content".to_string(),
//!         ..Default::default()
//!     }
//! ];
//!
//! let nav = NavigationGenerator::generate_navigation(&files);
//! assert!(nav.contains("About"));
//! ```

use rayon::prelude::*;
use std::path::{Component, Path};

use crate::models::data::FileData;
use crate::utilities::directory::to_title_case;

/// A set of supported file extensions for navigation.
const SUPPORTED_EXTENSIONS: [&str; 3] = ["md", "toml", "json"];

/// File name stems (without extension) to exclude from navigation.
const EXCLUDED_FILES: [&str; 5] =
    ["index", "404", "privacy", "terms", "offline"];

/// HTML prefix for the navigation list.
const HTML_PREFIX: &str =
    r#"<ul class="navbar-nav ms-auto mb-2 mb-lg-0">"#;

/// HTML suffix for the navigation list.
const HTML_SUFFIX: &str = "</ul>";

/// HTML prefix for each list item and link.
const LI_PREFIX: &str = r#"<li class="nav-item"><a aria-label=""#;

/// Fragment for adding a `href` attribute to a link.
const HREF_PREFIX: &str = r#"" href=""#;

/// Fragment for adding a `title` attribute to a link.
const TITLE_PREFIX: &str = r#"" title="Navigation link for the "#;

/// Classes applied to the link element.
const CLASS_SUFFIX: &str = r#"" class="text-uppercase p-2">"#;

/// Fragment for closing the link and list item.
const HTML_CLOSE: &str = "</a></li>";

/// An estimated size for each navigation item (used for `String` capacity pre-allocation).
const ESTIMATED_NAV_ITEM_SIZE: usize = 200;

/// Maximum length (in characters) for display text before truncation.
const MAX_DISPLAY_LEN: usize = 64;

/// Navigation menu generator.
///
/// This struct provides methods to generate an HTML-based navigation menu
/// from a collection of [`FileData`].
#[derive(Debug, Clone, Copy)]
pub struct NavigationGenerator;

impl NavigationGenerator {
    /// Generates a navigation menu as an unordered list of links.
    ///
    /// # Arguments
    ///
    /// * `files` - A slice of [`FileData`] structures representing the content files.
    ///
    /// # Returns
    ///
    /// A `String` containing the generated HTML navigation menu. Returns an empty string
    /// if no valid navigation items are found.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use staticdatagen::models::data::FileData;
    /// use staticdatagen::modules::navigation::NavigationGenerator;
    ///
    /// let files = vec![FileData {
    ///     name: "about.md".to_string(),
    ///     content: "About page".to_string(),
    ///     ..Default::default()
    /// }];
    ///
    /// let nav = NavigationGenerator::generate_navigation(&files);
    /// assert!(nav.contains("About"));
    /// assert!(nav.contains("about/index.html"));
    /// ```
    pub fn generate_navigation(files: &[FileData]) -> String {
        if files.is_empty() {
            return String::new();
        }

        // Collect and process valid items in parallel
        let mut nav_items: Vec<_> =
            files.par_iter().filter_map(Self::process_file).collect();

        // Sort navigation items alphabetically by display name
        nav_items.par_sort_by(|a, b| a.0.cmp(&b.0));

        // Pre-calculate capacity
        let estimated_total = nav_items.len() * ESTIMATED_NAV_ITEM_SIZE;
        let mut nav_links = String::with_capacity(estimated_total);

        nav_links.push_str(HTML_PREFIX);

        // Build final HTML in alphabetical order (already sorted)
        let item_html_list: Vec<String> = nav_items
            .into_par_iter()
            .map(|(name, url)| Self::build_item_html(&name, &url))
            .collect();

        for item_html in item_html_list {
            nav_links.push_str(&item_html);
        }

        nav_links.push_str(HTML_SUFFIX);
        nav_links
    }

    /// Builds the HTML for a single navigation item.
    fn build_item_html(name: &str, url: &str) -> String {
        let safe_name = html_escape(name);
        let safe_url = html_escape(url);

        let mut item_html = String::with_capacity(
            safe_name.len() + safe_url.len() + 100,
        );
        item_html.push_str(LI_PREFIX);
        item_html.push_str(&safe_name); // aria-label="<name>"
        item_html.push_str(HREF_PREFIX);
        item_html.push_str(&safe_url);
        item_html.push_str(TITLE_PREFIX);
        item_html.push_str(&safe_name);
        item_html.push_str(" page");
        item_html.push_str(CLASS_SUFFIX);
        item_html.push_str(&safe_name);
        item_html.push_str(HTML_CLOSE);
        item_html
    }

    /// Processes a single file, determining whether it qualifies for the navigation,
    /// sanitizing its name, and extracting its display name (title-cased) plus the URL.
    ///
    /// # Arguments
    ///
    /// * `file` - A reference to a [`FileData`] structure.
    ///
    /// # Returns
    ///
    /// An [`Option`] containing `(display_name, url)` if the file is valid
    /// for navigation, or [`None`] otherwise.
    fn process_file(file: &FileData) -> Option<(String, String)> {
        // First, sanitize the entire file name to remove null bytes or other control characters.
        let sanitized_name = remove_control_chars(&file.name);
        if sanitized_name.is_empty() {
            return None;
        }

        // Quick check for suspicious directory references
        if is_malicious_path(&sanitized_name) {
            return None;
        }

        // Now parse it as a path
        let path = Path::new(&sanitized_name);

        // Extension check
        let extension = path.extension()?.to_str()?;
        if !SUPPORTED_EXTENSIONS.contains(&extension) {
            return None;
        }

        // Stem check
        let file_stem = path.file_stem()?.to_str()?;
        if EXCLUDED_FILES.contains(&file_stem) {
            return None;
        }

        // Build final URL: strip extension + add /index.html
        let url = format!(
            "/{}/index.html",
            path.with_extension("").display()
        );

        // Generate a sanitized, title-cased display name
        let display_name = sanitize_and_titlecase(file_stem);
        if display_name.is_empty() {
            return None;
        }

        Some((display_name, url))
    }
}

/// Checks if a path is potentially malicious by scanning for
/// suspicious directory references (e.g., `..`, `.`, absolute paths, etc.).
fn is_malicious_path(filename: &str) -> bool {
    let path = Path::new(filename);

    // If a path is absolute, skip it
    if path.is_absolute() {
        return true;
    }

    for comp in path.components() {
        match comp {
            // Skip if it has parent directory references (..)
            Component::ParentDir => return true,
            // Treat current directory (.) as suspicious for these tests
            Component::CurDir => return true,
            // Root directory
            Component::RootDir => return true,
            _ => {}
        }
    }
    false
}

/// Filters out null bytes (`\0`), zero-width (`\u{FEFF}`), and *all other* control characters
/// from a string (except possibly `\n` or `\t` if needed).
/// Returns a new `String` with those characters removed.
fn remove_control_chars(input: &str) -> String {
    input
        .chars()
        .filter(|c| {
            // Keep only non-control or whitespace if specifically desired
            !c.is_control() || *c == '\n' || *c == '\t'
        })
        .filter(|c| *c != '\u{FEFF}' && *c != '\0')
        .collect()
}

/// Sanitizes and title-cases a file stem, removing `<` or `>` to avoid HTML injection.
/// Also splits on multiple delimiters (hyphen, underscore, dot, whitespace), applies
/// [`to_title_case`], and truncates to a max length (`MAX_DISPLAY_LEN`).
fn sanitize_and_titlecase(file_stem: &str) -> String {
    // Remove `<` or `>` to prevent injection
    let filtered = file_stem.replace('<', "").replace(['<', '>'], "");

    // Split on multiple delimiters
    let parts: Vec<&str> = filtered
        .split(|c: char| {
            c == '-' || c == '_' || c == '.' || c.is_whitespace()
        })
        .filter(|s| !s.is_empty())
        .collect();

    let mut display_name = String::new();
    for (i, part) in parts.iter().enumerate() {
        if i > 0 {
            display_name.push(' ');
        }
        display_name.push_str(&to_title_case(part));
    }

    // Truncate if needed
    if display_name.len() > MAX_DISPLAY_LEN {
        display_name.truncate(MAX_DISPLAY_LEN);
        display_name.push('…');
    }

    display_name
}

/// Escapes `<`, `>`, and `&` in a string to avoid HTML injection.
fn html_escape(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '&' => escaped.push_str("&amp;"),
            _ => escaped.push(c),
        }
    }
    escaped
}

#[cfg(test)]
#[allow(clippy::pedantic, clippy::nursery)]
mod tests {
    use super::*;
    use crate::models::data::FileData;

    /// Helper function to create test files.
    fn create_test_file(name: &str, content: &str) -> FileData {
        FileData {
            name: name.to_string(),
            content: content.to_string(),
            ..Default::default()
        }
    }

    // ---------------------------------------------------------------------
    // Basic functionality tests
    // ---------------------------------------------------------------------
    #[test]
    fn empty_navigation() {
        let nav = NavigationGenerator::generate_navigation(&[]);
        assert!(
            nav.is_empty(),
            "Empty input should produce empty output"
        );
    }

    #[test]
    fn single_file_navigation() {
        let files = vec![create_test_file("about.md", "About page")];
        let nav = NavigationGenerator::generate_navigation(&files);

        assert!(
            nav.contains("About"),
            "Navigation should contain the page title"
        );
        assert!(
            nav.contains("about/index.html"),
            "Navigation should contain correct URL"
        );
        assert!(
            nav.contains(r#"class="nav-item"#),
            "Should include proper CSS classes"
        );
        assert!(
            nav.contains(r#"aria-label="About"#),
            "Should include ARIA labels"
        );
    }

    #[test]
    fn multiple_files_navigation() {
        let files = vec![
            create_test_file("about.md", "About"),
            create_test_file("blog.md", "Blog"),
            create_test_file("contact.md", "Contact"),
        ];

        let nav = NavigationGenerator::generate_navigation(&files);
        for title in ["About", "Blog", "Contact"] {
            assert!(
                nav.contains(title),
                "Navigation should contain '{}'",
                title
            );
            let expected_url =
                format!("{}/index.html", title.to_lowercase());
            assert!(
                nav.contains(&expected_url),
                "Navigation should contain URL for '{}'",
                title
            );
        }
    }

    // ---------------------------------------------------------------------
    // Filtering tests
    // ---------------------------------------------------------------------
    #[test]
    fn excluded_files() {
        let files = vec![
            create_test_file("index.md", "Home"),
            create_test_file("404.md", "Not Found"),
            create_test_file("about.md", "About"),
            create_test_file("privacy.md", "Privacy"),
            create_test_file("terms.md", "Terms"),
            create_test_file("offline.md", "Offline"),
        ];

        let nav = NavigationGenerator::generate_navigation(&files);

        // Excluded
        for excluded in EXCLUDED_FILES {
            let url_snippet = format!("{}/", excluded);
            assert!(
                !nav.contains(&url_snippet),
                "Navigation should not contain excluded file '{}'",
                excluded
            );
        }

        // Included
        assert!(
            nav.contains("about/"),
            "Navigation should contain non-excluded files"
        );
    }

    #[test]
    fn unsupported_extensions() {
        let files = vec![
            create_test_file("document.txt", "Text file"),
            create_test_file("image.png", "Image file"),
            create_test_file("valid.md", "Markdown file"),
        ];

        let nav = NavigationGenerator::generate_navigation(&files);
        assert!(
            !nav.contains("document"),
            "Navigation should not contain .txt"
        );
        assert!(
            !nav.contains("image"),
            "Navigation should not contain .png"
        );
        assert!(nav.contains("valid"), "Navigation should contain .md");
    }

    // ---------------------------------------------------------------------
    // Formatting tests
    // ---------------------------------------------------------------------
    #[test]
    fn title_case_conversion() {
        let files = vec![
            create_test_file("about-us.md", "About Us"),
            create_test_file("contact-me.md", "Contact Me"),
            create_test_file("my-long-page-title.md", "Long Title"),
        ];

        let nav = NavigationGenerator::generate_navigation(&files);

        assert!(
            nav.contains(">About Us<"),
            "Should properly convert 'about-us' to 'About Us'"
        );
        assert!(
            nav.contains(">Contact Me<"),
            "Should properly convert 'contact-me' to 'Contact Me'"
        );
        assert!(
            nav.contains(">My Long Page Title<"),
            "Should properly convert multiple hyphens"
        );
        assert!(
            nav.contains("href=\"/about-us/index.html\""),
            "URLs should preserve original hyphenation"
        );
        assert!(
            nav.contains("href=\"/my-long-page-title/index.html\""),
            "URLs should preserve all hyphens"
        );
    }

    #[test]
    fn special_characters() {
        let files = vec![
            create_test_file(
                "page-with-numbers-123.md",
                "Numbered Page",
            ),
            create_test_file(
                "page_with_underscore.md",
                "Underscore Page",
            ),
            create_test_file("page.with.dots.md", "Dotted Page"),
        ];

        let nav = NavigationGenerator::generate_navigation(&files);

        assert!(
            nav.contains(">Page With Numbers 123<"),
            "Navigation should handle numbers in titles"
        );
        assert!(
            nav.contains(">Page With Underscore<"),
            "Navigation should handle underscores"
        );
        assert!(
            nav.contains(">Page With Dots<"),
            "Navigation should handle dots"
        );
    }

    // ---------------------------------------------------------------------
    // HTML Structure tests
    // ---------------------------------------------------------------------
    #[test]
    fn nav_structure() {
        let files = vec![create_test_file("about.md", "About")];
        let nav = NavigationGenerator::generate_navigation(&files);

        assert!(
            nav.starts_with(
                r#"<ul class="navbar-nav ms-auto mb-2 mb-lg-0">"#
            ),
            "Should start with proper Bootstrap classes"
        );
        assert!(
            nav.contains(r#"<li class="nav-item"><a"#),
            "Should have correct list item structure"
        );
        assert!(
            nav.contains(r#"class="text-uppercase p-2""#),
            "Should have correct styling classes"
        );
        assert!(
            nav.ends_with("</ul>"),
            "Should end with closing ul tag"
        );
    }

    #[test]
    fn accessibility_attributes() {
        let files = vec![create_test_file("about.md", "About")];
        let nav = NavigationGenerator::generate_navigation(&files);

        assert!(
            nav.contains(r#"aria-label="About""#),
            "Should include aria-label"
        );
        assert!(
            nav.contains(
                r#"title="Navigation link for the About page""#
            ),
            "Should include descriptive titles"
        );
    }

    // ---------------------------------------------------------------------
    // Ordering tests
    // ---------------------------------------------------------------------
    #[test]
    fn navigation_order() {
        let files = vec![
            create_test_file("zebra.md", "Zebra"),
            create_test_file("alpha.md", "Alpha"),
            create_test_file("beta.md", "Beta"),
        ];

        let nav = NavigationGenerator::generate_navigation(&files);

        let nav_lower = nav.to_lowercase();
        let idx_alpha =
            nav_lower.find("alpha").expect("Should find 'alpha'");
        let idx_beta =
            nav_lower.find("beta").expect("Should find 'beta'");
        let idx_zebra =
            nav_lower.find("zebra").expect("Should find 'zebra'");

        assert!(
            idx_alpha < idx_beta,
            "Navigation items should be alphabetically ordered: Alpha before Beta"
        );
        assert!(
            idx_beta < idx_zebra,
            "Navigation items should be alphabetically ordered: Beta before Zebra"
        );
    }

    // ---------------------------------------------------------------------
    // Internationalization tests
    // ---------------------------------------------------------------------
    #[test]
    fn unicode_filenames() {
        let files = vec![
            create_test_file("café.md", "Café"),
            create_test_file("über.md", "Über"),
            create_test_file("日本語.md", "日本語"),
            create_test_file("русский.md", "Русский"),
        ];

        let nav = NavigationGenerator::generate_navigation(&files);

        // Test display text
        assert!(
            nav.contains(">Café<"),
            "Should handle accented characters"
        );
        assert!(nav.contains(">Über<"), "Should handle umlauts");
        assert!(nav.contains(">日本語<"), "Should handle CJK");
        assert!(nav.contains(">Русский<"), "Should handle Cyrillic");

        // Test URLs
        assert!(
            nav.contains("href=\"/café/index.html\""),
            "Should handle accented in URLs"
        );
    }

    #[test]
    fn rtl_support() {
        let files = vec![
            create_test_file("مرحبا.md", "مرحبا"),
            create_test_file("שלום.md", "שלום"),
        ];

        let nav = NavigationGenerator::generate_navigation(&files);
        assert!(nav.contains(">مرحبا<"), "Should handle RTL text");
        assert!(nav.contains(">שלום<"), "Should handle RTL text");
    }

    #[test]
    fn mixed_scripts() {
        let files = vec![
            create_test_file("hello-世界.md", "Hello 世界"),
            create_test_file("こんにちは-world.md", "こんにちは World"),
        ];

        let nav = NavigationGenerator::generate_navigation(&files);
        assert!(
            nav.contains(">Hello 世界<"),
            "Should handle mixed Latin-CJK"
        );
        assert!(
            nav.contains(">こんにちは World<"),
            "Should handle mixed CJK-Latin"
        );
    }

    // ---------------------------------------------------------------------
    // Error handling tests
    // ---------------------------------------------------------------------
    #[test]
    fn malformed_paths() {
        let files = vec![
            create_test_file("../attempt-parent.md", "Invalid Path"),
            create_test_file("./attempt-current.md", "Invalid Path"),
            create_test_file("//attempt-root.md", "Invalid Path"),
        ];

        let nav = NavigationGenerator::generate_navigation(&files);

        // The display_name should not appear if the path is invalid
        assert!(
            !nav.contains("attempt-parent"),
            "Should sanitize or skip parent directory attempts"
        );
        assert!(
            !nav.contains("attempt-current"),
            "Should sanitize or skip current directory attempts"
        );
        assert!(
            !nav.contains("attempt-root"),
            "Should sanitize or skip root directory attempts"
        );
    }

    #[test]
    fn invalid_characters() {
        let files = vec![
            create_test_file("page<script>.md", "XSS Attempt"),
            create_test_file("page\0null.md", "Null Byte"),
            create_test_file("page\u{FEFF}zero-width.md", "Zero Width"),
        ];

        let nav = NavigationGenerator::generate_navigation(&files);

        // Should remove or escape script tags, null bytes, zero-width, etc.
        assert!(!nav.contains("<script>"), "Should sanitize HTML");
        assert!(!nav.contains('\0'), "Should remove null bytes");
        assert!(
            !nav.contains('\u{FEFF}'),
            "Should handle zero-width chars"
        );
    }

    #[test]
    fn extremely_long_names() {
        let long_name = "a".repeat(1000);
        let files = vec![create_test_file(
            &format!("{}.md", long_name),
            "Long",
        )];

        let nav = NavigationGenerator::generate_navigation(&files);

        // The rendered navigation shouldn't explode in size;
        // we truncate very long display names.
        assert!(
            nav.len() < (long_name.len() * 2),
            "Should handle long filenames efficiently"
        );
    }
}
