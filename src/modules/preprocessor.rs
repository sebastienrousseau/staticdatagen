// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::utilities::directory::update_class_attributes;
use regex::Regex;

/// Preprocesses the Markdown content to update class attributes and image tags.
///
/// # Arguments
///
/// * `content` - A string containing the Markdown content to be processed.
/// * `class_regex` - A reference to a `Regex` object for matching class attributes.
/// * `img_regex` - A reference to a `Regex` object for matching image tags.
///
/// # Returns
///
/// A `Result` containing a `String` with the processed Markdown content, or a `crate::Error` if an error occurs.
///
pub fn preprocess_content(
    content: &str,
    class_regex: &Regex,
    img_regex: &Regex,
) -> crate::Result<String> {
    let processed_content: Vec<String> = content
        .lines()
        .map(|line| {
            update_class_attributes(line, class_regex, img_regex)
        })
        .collect();

    let mut result = processed_content.join("\n");

    // Trim trailing newlines
    while result.ends_with('\n') {
        let _ = result.pop();
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocess_content_basic() {
        let class_regex =
            Regex::new(r#"\.class=\"([^\"]*)\""#).unwrap();
        let img_regex = Regex::new(r#"<img([^>]*)>"#).unwrap();

        let content = "Hello World\nTest content";
        let result =
            preprocess_content(content, &class_regex, &img_regex);

        assert!(result.is_ok());
        let processed = result.unwrap();
        assert!(processed.contains("Hello World"));
        assert!(processed.contains("Test content"));
    }

    #[test]
    fn test_preprocess_content_empty() {
        let class_regex =
            Regex::new(r#"\.class=\"([^\"]*)\""#).unwrap();
        let img_regex = Regex::new(r#"<img([^>]*)>"#).unwrap();

        let content = "";
        let result =
            preprocess_content(content, &class_regex, &img_regex);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_preprocess_content_trailing_newlines() {
        let class_regex =
            Regex::new(r#"\.class=\"([^\"]*)\""#).unwrap();
        let img_regex = Regex::new(r#"<img([^>]*)>"#).unwrap();

        let content = "Hello\n\n\n";
        let result =
            preprocess_content(content, &class_regex, &img_regex);

        assert!(result.is_ok());
        let processed = result.unwrap();
        assert!(!processed.ends_with('\n'));
    }

    #[test]
    fn test_preprocess_content_multiline() {
        let class_regex =
            Regex::new(r#"\.class=\"([^\"]*)\""#).unwrap();
        let img_regex = Regex::new(r#"<img([^>]*)>"#).unwrap();

        let content = "Line 1\nLine 2\nLine 3";
        let result =
            preprocess_content(content, &class_regex, &img_regex);

        assert!(result.is_ok());
        let processed = result.unwrap();
        assert_eq!(processed.lines().count(), 3);
    }

    #[test]
    fn test_preprocess_content_with_class_attribute() {
        let class_regex =
            Regex::new(r#"\.class=&quot;([^&]*)&quot;"#).unwrap();
        let img_regex = Regex::new(r#"<img([^>]*)>"#).unwrap();

        let content =
            r#"<img src="test.jpg" .class=&quot;highlight&quot;>"#;
        let result =
            preprocess_content(content, &class_regex, &img_regex);

        assert!(result.is_ok());
    }

    #[test]
    fn test_preprocess_content_with_img_tag() {
        let class_regex =
            Regex::new(r#"\.class=\"([^\"]*)\""#).unwrap();
        let img_regex = Regex::new(r#"<img([^>]*)>"#).unwrap();

        let content = r#"<img src="photo.jpg" alt="A photo">"#;
        let result =
            preprocess_content(content, &class_regex, &img_regex);

        assert!(result.is_ok());
        let processed = result.unwrap();
        assert!(processed.contains("img"));
    }

    #[test]
    fn test_preprocess_content_combined_transforms() {
        let class_regex =
            Regex::new(r#"\.class=\"([^\"]*)\""#).unwrap();
        let img_regex = Regex::new(r#"<img([^>]*)>"#).unwrap();

        let content = "# Heading\n<img src=\"a.jpg\">\nParagraph text";
        let result =
            preprocess_content(content, &class_regex, &img_regex);

        assert!(result.is_ok());
        let processed = result.unwrap();
        assert!(processed.contains("Heading"));
        assert!(processed.contains("img"));
        assert!(processed.contains("Paragraph text"));
    }
}
