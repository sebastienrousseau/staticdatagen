// Copyright © 2025 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Tag Generation Module
//!
//! This module handles the creation, management, and generation of tags
//! for static site content. It allows for reading tags from metadata,
//! sanitizing them, and then mapping them to relevant pages or content.
//! It also supports creating HTML views of these tags and writing them
//! into existing files via placeholders. To meet accessibility standards
//! (e.g., WCAG, ARIA), additional ARIA attributes are applied in the
//! generated HTML to ensure tags are announced properly by assistive
//! technologies.
//!
//! ## Features
//! - **Extraction of tags from file metadata.**
//! - **Flexible creation of `TagsData` structures from metadata.**
//! - **Generating accessible HTML content that lists tags with their associated pages.**
//! - **Writing the resulting HTML into an existing `index.html` file.**
//!
//! ## Example Usage
//! ```rust
//! use std::collections::HashMap;
//! use std::path::Path;
//! use staticdatagen::models::data::{FileData, PageData};
//! use staticdatagen::generators::tags::{
//!     generate_tags, create_tags_data, generate_tags_html, write_tags_html_to_file
//! };
//!
//! // Prepare some metadata containing tags
//! let mut metadata = HashMap::new();
//! metadata.insert("tags".to_string(), "tag1,tag2".to_string());
//!
//! // Create a FileData instance with placeholder content
//! let file = FileData {
//!     content: "This is a test with tag1 in the text".to_string(),
//!     ..Default::default()
//! };
//!
//! // Generate a mapping of tag -> page-like data
//! let tags_map = generate_tags(&file, &metadata);
//!
//! // Create `TagsData` from metadata (for more detailed usage elsewhere)
//! let tags_data = create_tags_data(&metadata);
//! println!("Created TagsData struct: {:?}", tags_data);
//!
//! // Suppose you have a global mapping from tags to lists of `PageData`
//! let mut global_tags_data = HashMap::new();
//! global_tags_data.insert(
//!     "tag1".to_string(),
//!     vec![
//!         PageData {
//!             date: "2024-03-10".to_string(),
//!             description: "Description 1".to_string(),
//!             permalink: "/page1".to_string(),
//!             title: "Page 1".to_string(),
//!         }
//!     ]
//! );
//!
//! // Generate HTML for these tags
//! let html_content = generate_tags_html(&global_tags_data);
//!
//! // Write it into `tags/index.html` in the specified output directory
//! let output_path = Path::new("/path/to/output");
//! let result = write_tags_html_to_file(&html_content, &output_path);
//! if let Err(e) = result {
//!     eprintln!("Failed to write tags HTML: {}", e);
//! }
//! ```

use crate::models::data::{FileData, PageData, TagsData};
use crate::utilities::directory::to_title_case;
use std::{
    collections::HashMap,
    fs,
    io::{self, Read, Write},
    path::Path,
};

/// ## Tag Sanitization
///
/// Removes all non-alphanumeric characters from a given tag string.
/// This ensures tags can be matched consistently in text.
///
/// ### Arguments
/// - `tag`: A reference to the original (unsanitized) tag string.
///
/// ### Returns
/// A sanitized `String` containing only alphanumeric characters.
pub fn sanitize_tag(tag: &str) -> String {
    tag.chars().filter(|c| c.is_alphanumeric()).collect()
}

/// ## Generate Tags
///
/// Creates a mapping of sanitized tags to associated metadata, based on the
/// contents of a [`FileData`] and a metadata map.
pub fn generate_tags(
    file: &FileData,
    metadata: &HashMap<String, String>,
) -> HashMap<String, Vec<HashMap<String, String>>> {
    let mut keywords_data_map = HashMap::new();

    let file_content = &file.content;
    let default_tags = String::new();
    let target_tags: Vec<&str> = metadata
        .get("tags")
        .unwrap_or(&default_tags)
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if target_tags.is_empty() {
        println!("No tags found in metadata.");
        return keywords_data_map;
    }

    for tag in target_tags {
        let sanitized_tag = sanitize_tag(tag);

        // Skip tags we do not want to include
        if sanitized_tag.eq_ignore_ascii_case("404")
            || sanitized_tag.eq_ignore_ascii_case("offline")
            || sanitized_tag.eq_ignore_ascii_case("thanks")
            || sanitized_tag.eq_ignore_ascii_case("archive")
            || sanitized_tag.eq_ignore_ascii_case("tag")
            || sanitized_tag.eq_ignore_ascii_case("author")
            || sanitized_tag.eq_ignore_ascii_case("category")
            || sanitized_tag.eq_ignore_ascii_case("search")
            || sanitized_tag.eq_ignore_ascii_case("login")
            || sanitized_tag.eq_ignore_ascii_case("account")
            || sanitized_tag.eq_ignore_ascii_case("profile")
            || sanitized_tag.eq_ignore_ascii_case("unpublished")
            || sanitized_tag.eq_ignore_ascii_case("private")
            || sanitized_tag.eq_ignore_ascii_case("test")
            || sanitized_tag.eq_ignore_ascii_case("navigation")
            || sanitized_tag.eq_ignore_ascii_case("sidebar")
            || sanitized_tag.eq_ignore_ascii_case("footer")
            || sanitized_tag.eq_ignore_ascii_case("cart")
            || sanitized_tag.eq_ignore_ascii_case("checkout")
            || sanitized_tag.eq_ignore_ascii_case("order")
        {
            continue;
        }

        if file_content.contains(&sanitized_tag) {
            let mut tags_data = HashMap::new();
            for key in &[
                "title",
                "date",
                "description",
                "permalink",
                "keywords",
            ] {
                if let Some(value) = metadata.get(*key) {
                    _ = tags_data
                        .insert((*key).to_string(), value.clone());
                }
            }
            keywords_data_map
                .entry(sanitized_tag)
                .or_default()
                .push(tags_data);
        }
    }
    keywords_data_map
}

/// ## Create TagsData
///
/// Builds a [`TagsData`] instance from metadata, populating optional fields.
pub fn create_tags_data(
    metadata: &HashMap<String, String>,
) -> TagsData {
    TagsData {
        dates: metadata.get("date").cloned().unwrap_or_default(),
        titles: metadata.get("title").cloned().unwrap_or_default(),
        descriptions: metadata
            .get("description")
            .cloned()
            .unwrap_or_default(),
        permalinks: metadata
            .get("permalink")
            .cloned()
            .unwrap_or_default(),
        keywords: metadata.get("keywords").cloned().unwrap_or_default(),
    }
}

/// ## Generate Tags HTML
///
/// Creates an HTML snippet showing each tag (with a post count) and the list
/// of pages under that tag. Uses `<section>` elements to group each tag, with
/// `<h3>` headings for clarity. Links have unique `aria-label`s.
pub fn generate_tags_html(
    global_tags_data: &HashMap<String, Vec<PageData>>,
) -> String {
    let mut html_content = String::new();

    // Wrap everything in a container with role="group"
    html_content.push_str(
        "<div role=\"group\" aria-label=\"Tag group\" class=\"tags-wrapper\">\n",
    );

    let mut keys: Vec<&String> = global_tags_data.keys().collect();
    keys.sort();

    let total_posts: usize =
        global_tags_data.values().map(|pages| pages.len()).sum();

    // Main heading for featured tags
    html_content.push_str(&format!(
        "<h2 class=\"featured-tags\" id=\"h2-featured-tags\" tabindex=\"0\" aria-label=\"Featured Tags, total {0}\">Featured Tags ({0})</h2>\n",
        total_posts
    ));

    // For each tag, create a <section> with a heading and a <ul>
    for (tag_index, key) in keys.iter().enumerate() {
        let pages = &global_tags_data[*key];
        let count = pages.len();
        let heading_label =
            format!("Tag: {}, {} Posts", to_title_case(key), count);

        html_content.push_str("<section class=\"tag-group\">\n");

        // <h3> heading for the tag
        html_content.push_str(&format!(
            "<h3 class=\"{}\" id=\"h3-{}\" tabindex=\"0\" role=\"heading\" aria-level=\"3\" aria-label=\"{}\">{} ({} Posts)</h3>\n",
            key.replace(' ', "-"),
            key.replace(' ', "-"),
            html_escape(&heading_label),
            to_title_case(key),
            count
        ));

        // <ul> with role="list"
        html_content.push_str("<ul role=\"list\">\n");

        // Each page is an <li> with role="listitem"
        for (i, page) in pages.iter().enumerate() {
            // Use single quotes around aria-label to allow double quotes inside
            let link_label =
                format!("Visit the \"{}\" page", page.title);
            let item_id = format!(
                "li-{}-{}-{}",
                key.replace(' ', "-"),
                tag_index,
                i
            );

            // Example: adjusting descriptive text in <strong>
            // to provide more unique or useful info:
            let strong_text = if page.title.contains("Home") {
                "This is our homepage.".to_string()
            } else {
                "Learn more on this page.".to_string()
            };

            html_content.push_str(&format!(
                "<li id=\"{item_id}\" role=\"listitem\" class=\"tagged-page-item\">
                   <span class=\"tag-date\">{date}</span>:
                   <a href=\"{link}\" aria-label='{label}'>{title}</a>
                   - <strong>{desc}</strong>
                 </li>\n",
                item_id = item_id,
                date = html_escape(&page.date),
                link = html_escape(&page.permalink),
                label = html_escape(&link_label),
                title = html_escape(&page.title),
                desc = if !page.description.is_empty() {
                    html_escape(&page.description)
                } else {
                    strong_text
                }
            ));
        }

        html_content.push_str("</ul>\n"); // End <ul>
        html_content.push_str("</section>\n"); // End <section>
    }

    html_content.push_str("</div>\n"); // End .tags-wrapper
    html_content
}

/// Minimal escaping for <, >, and & to avoid HTML injection issues.
fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// ## Write Tags HTML to File
///
/// Replaces `[[content]]` in `tags/index.html` with the generated snippet.
pub fn write_tags_html_to_file(
    html_content: &str,
    output_path: &Path,
) -> io::Result<()> {
    let file_path = output_path.join("tags/index.html");

    let mut file = fs::File::open(&file_path)?;
    let mut base_html = String::new();
    // Use `_ = ...` to ignore number-of-bytes result
    _ = file.read_to_string(&mut base_html)?;

    let updated_html = base_html.replace("[[content]]", html_content);

    let mut file = fs::File::create(&file_path)?;
    file.write_all(updated_html.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::data::{FileData, PageData};
    use std::{fs, io::Write, path::Path};

    #[test]
    fn test_sanitize_tag() {
        let original = "Hel!lo-Wor#ld@2024";
        let sanitized = sanitize_tag(original);
        assert_eq!(sanitized, "HelloWorld2024");
    }

    #[test]
    fn test_generate_tags() {
        let file = FileData {
            content: "Testing #tag in the content".to_string(),
            ..Default::default()
        };
        let mut metadata = HashMap::new();
        metadata.insert("tags".to_string(), "tag, another".to_string());
        metadata.insert("title".to_string(), "Test Title".to_string());

        let result = generate_tags(&file, &metadata);
        assert!(result.contains_key("tag"));
        assert!(!result.contains_key("another"));
    }

    #[test]
    fn test_create_tags_data() {
        let mut metadata = HashMap::new();
        metadata.insert("date".to_string(), "2024-03-10".to_string());
        metadata.insert(
            "description".to_string(),
            "A sample description".to_string(),
        );
        metadata
            .insert("keywords".to_string(), "rust, test".to_string());
        metadata.insert("permalink".to_string(), "/sample".to_string());
        metadata
            .insert("title".to_string(), "Sample Title".to_string());

        let tags_data = create_tags_data(&metadata);
        assert_eq!(tags_data.dates, "2024-03-10");
        assert_eq!(tags_data.titles, "Sample Title");
        assert_eq!(tags_data.descriptions, "A sample description");
        assert_eq!(tags_data.permalinks, "/sample");
        assert_eq!(tags_data.keywords, "rust, test");
    }

    #[test]
    fn test_generate_tags_html() {
        let mut global_tags_data = HashMap::new();
        global_tags_data.insert(
            "example".to_string(),
            vec![PageData {
                date: "2024-03-10".to_string(),
                description: "Example Description".to_string(),
                permalink: "/example".to_string(),
                title: "Example Page".to_string(),
            }],
        );

        let html = generate_tags_html(&global_tags_data);
        assert!(html.contains("<section class=\"tag-group\">"));
        assert!(html.contains("Example Page"));
        assert!(html.contains("Example Description"));
        assert!(html.contains("/example"));
        assert!(html.contains("role=\"listitem\""));
    }

    #[test]
    fn test_write_tags_html_to_file() {
        let temp_dir = Path::new("test_output");
        if temp_dir.exists() {
            fs::remove_dir_all(temp_dir).unwrap();
        }
        fs::create_dir_all(temp_dir.join("tags")).unwrap();

        let index_path = temp_dir.join("tags/index.html");
        {
            let mut file = fs::File::create(&index_path).unwrap();
            writeln!(file, "<html><head><title>Test</title></head><body>[[content]]</body></html>").unwrap();
        }

        let html_content = "<section>Content</section>";
        let result = write_tags_html_to_file(html_content, temp_dir);
        assert!(result.is_ok());

        let new_content = fs::read_to_string(&index_path).unwrap();
        assert!(new_content.contains(html_content));
        assert!(!new_content.contains("[[content]]"));

        fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("&"), "&amp;");
        assert_eq!(html_escape("<tag>"), "&lt;tag&gt;");
        assert_eq!(html_escape("5 > 3"), "5 &gt; 3");
    }
}
