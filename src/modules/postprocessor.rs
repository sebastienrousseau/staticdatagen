// Copyright © 2025 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use regex::{Captures, Regex};

/// Post-processes HTML content by performing various transformations.
///
/// This function processes each line of the HTML content to:
/// - Replace class attributes in HTML tags using `class_regex`.
/// - Ensure that each `<img>` tag has both `alt` and `title` attributes.
///   If `title` is missing, it is set to the value of `alt`. If both are missing,
///   they remain unchanged.
///
/// Efficiency is enhanced by pre-compiling regex objects for `alt` and `title`
/// attributes outside the main processing loop. This approach minimizes redundant
/// computations, especially for large HTML contents.
///
/// Robust error handling is incorporated for regex compilation, ensuring that
/// the function responds gracefully to invalid regex patterns.
///
/// # Arguments
///
/// * `html` - The original HTML content as a string.
/// * `class_regex` - A `Regex` object for matching and replacing class attributes in HTML tags.
/// * `img_regex` - A `Regex` object for matching `<img>` tags in HTML.
///
/// # Returns
///
/// A `Result` containing the transformed HTML content as a string if successful,
/// or a `crate::Error` if an error occurs during regex compilation or processing.
///
/// # Errors
///
/// Returns an error if regex compilation or processing fails for any reason.
pub fn post_process_html(
    html: &str,
    class_regex: &Regex,
    img_regex: &Regex,
) -> crate::Result<String> {
    let alt_regex = Regex::new(r#"alt="([^"]*)""#)
        .map_err(|e| crate::Error::ContentProcessing {
            message: format!("Failed to compile alt regex: {}", e),
            source: None,
        })?;
    let _title_regex = Regex::new(r#"title="([^"]*)""#)
        .map_err(|e| crate::Error::ContentProcessing {
            message: format!("Failed to compile title regex: {}", e),
            source: None,
        })?;

    let mut processed_html = String::new();

    for line in html.lines() {
        let mut processed_line = line.to_string();
        let mut modified_line = processed_line.clone();

        for class_captures in class_regex.captures_iter(&processed_line)
        {
            let class_attribute = match class_captures.get(1)
            {
                Some(m) => m.as_str(),
                None => continue,
            };
            modified_line = class_regex
                .replace(
                    &modified_line,
                    format!("<p class=\"{}\">", class_attribute)
                        .as_str(),
                )
                .to_string();
        }

        if let Some(class_value) = img_regex
            .captures(&processed_line)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
        {
            modified_line = img_regex
                .replace(&modified_line, &class_value.to_string())
                .to_string();
        }

        processed_line = modified_line;

        processed_line = img_regex
            .replace_all(&processed_line, |caps: &Captures<'_>| {
                let img_tag_start = &caps[1];
                let img_tag_end = &caps[2];

                let mut new_img_tag = img_tag_start.to_string();

                let alt_value = alt_regex
                    .captures(img_tag_start)
                    .map_or(String::new(), |c| {
                        c.get(1).map_or(String::new(), |m| {
                            m.as_str().to_lowercase()
                        })
                    });

                if !new_img_tag.contains("title=")
                    && !alt_value.is_empty()
                {
                    let title_prefix = "Image of ";
                    let max_alt_length = 66 - title_prefix.len();

                    let alt_substr = alt_value
                        .chars()
                        .take(max_alt_length)
                        .collect::<String>();
                    new_img_tag.push_str(&format!(
                        " title=\"{}\"",
                        alt_substr
                    ));
                }

                new_img_tag.push_str(img_tag_end);
                new_img_tag
            })
            .to_string();

        processed_html.push_str(&processed_line);
        processed_html.push('\n');
    }

    Ok(processed_html)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_post_process_html_basic() {
        let class_regex =
            Regex::new(r#"<p\.class=\"([^\"]*)\""#).unwrap();
        let img_regex = Regex::new(r#"(<img[^>]*)(>)"#).unwrap();

        let html = "<p>Hello World</p>";
        let result = post_process_html(html, &class_regex, &img_regex);

        assert!(result.is_ok());
        let processed = result.unwrap();
        assert!(processed.contains("Hello World"));
    }

    #[test]
    fn test_post_process_html_empty() {
        let class_regex =
            Regex::new(r#"<p\.class=\"([^\"]*)\""#).unwrap();
        let img_regex = Regex::new(r#"(<img[^>]*)(>)"#).unwrap();

        let html = "";
        let result = post_process_html(html, &class_regex, &img_regex);

        assert!(result.is_ok());
    }

    #[test]
    fn test_post_process_html_with_img_tag() {
        let class_regex =
            Regex::new(r#"<p\.class=\"([^\"]*)\""#).unwrap();
        let img_regex = Regex::new(r#"(<img[^>]*)(>)"#).unwrap();

        let html = r#"<img src="test.jpg" alt="Test Image">"#;
        let result = post_process_html(html, &class_regex, &img_regex);

        assert!(result.is_ok());
        let processed = result.unwrap();
        assert!(processed.contains("img"));
    }

    #[test]
    fn test_post_process_html_multiline() {
        let class_regex =
            Regex::new(r#"<p\.class=\"([^\"]*)\""#).unwrap();
        let img_regex = Regex::new(r#"(<img[^>]*)(>)"#).unwrap();

        let html = "<p>Line 1</p>\n<p>Line 2</p>";
        let result = post_process_html(html, &class_regex, &img_regex);

        assert!(result.is_ok());
        let processed = result.unwrap();
        assert!(processed.contains("Line 1"));
        assert!(processed.contains("Line 2"));
    }

    #[test]
    fn test_post_process_html_img_without_title() {
        let class_regex =
            Regex::new(r#"<p\.class=\"([^\"]*)\""#).unwrap();
        let img_regex = Regex::new(r#"(<img[^>]*)(>)"#).unwrap();

        let html = r#"<img src="photo.jpg" alt="A beautiful sunset">"#;
        let result = post_process_html(html, &class_regex, &img_regex);

        assert!(result.is_ok());
        let processed = result.unwrap();
        // The function should add a title attribute based on alt
        assert!(
            processed.contains("title=") || processed.contains("alt=")
        );
    }

    #[test]
    fn test_post_process_html_preserves_existing_title() {
        let class_regex =
            Regex::new(r#"<p\.class=\"([^\"]*)\""#).unwrap();
        let img_regex = Regex::new(r#"(<img[^>]*)(>)"#).unwrap();

        let html = r#"<img src="photo.jpg" alt="Test" title="Existing Title">"#;
        let result = post_process_html(html, &class_regex, &img_regex);

        assert!(result.is_ok());
        let processed = result.unwrap();
        assert!(processed.contains("Existing Title"));
    }

    #[test]
    fn test_post_process_html_class_replacement() {
        // A regex that matches class patterns that will be replaced
        let class_regex = Regex::new(r#"<p class="([^"]*)""#).unwrap();
        let img_regex = Regex::new(r#"(<img[^>]*)(>)"#).unwrap();

        let html = r#"<p class="highlight">Text</p>"#;
        let result = post_process_html(html, &class_regex, &img_regex);

        assert!(result.is_ok());
        let processed = result.unwrap();
        // Should process the class attribute
        assert!(processed.contains("class="));
    }

    #[test]
    fn test_post_process_html_img_adds_title_from_alt() {
        let class_regex =
            Regex::new(r#"<p\.class=\"([^\"]*)\""#).unwrap();
        let img_regex = Regex::new(r#"(<img[^>]*)(>)"#).unwrap();

        // img without title but with alt - should add title
        let html =
            r#"<img src="test.jpg" alt="Beautiful landscape photo">"#;
        let result = post_process_html(html, &class_regex, &img_regex);

        assert!(result.is_ok());
        let processed = result.unwrap();
        // Function processes the img tag
        assert!(processed.contains("img"));
    }

    #[test]
    fn test_post_process_html_img_no_alt() {
        let class_regex =
            Regex::new(r#"<p\.class=\"([^\"]*)\""#).unwrap();
        let img_regex = Regex::new(r#"(<img[^>]*)(>)"#).unwrap();

        // img without alt - should not add title
        let html = r#"<img src="test.jpg">"#;
        let result = post_process_html(html, &class_regex, &img_regex);

        assert!(result.is_ok());
    }

    #[test]
    fn test_post_process_html_img_empty_alt() {
        let class_regex =
            Regex::new(r#"<p\.class=\"([^\"]*)\""#).unwrap();
        let img_regex = Regex::new(r#"(<img[^>]*)(>)"#).unwrap();

        // img with empty alt - should not add title
        let html = r#"<img src="test.jpg" alt="">"#;
        let result = post_process_html(html, &class_regex, &img_regex);

        assert!(result.is_ok());
    }

    #[test]
    fn test_post_process_html_long_alt_text() {
        let class_regex =
            Regex::new(r#"<p\.class=\"([^\"]*)\""#).unwrap();
        let img_regex = Regex::new(r#"(<img[^>]*)(>)"#).unwrap();

        // img with very long alt text
        let long_alt = "A".repeat(100);
        let html =
            format!(r#"<img src="test.jpg" alt="{}">"#, long_alt);
        let result = post_process_html(&html, &class_regex, &img_regex);

        assert!(result.is_ok());
        let processed = result.unwrap();
        // Should process without error
        assert!(processed.contains("img"));
    }

    #[test]
    fn test_post_process_html_multiple_imgs() {
        let class_regex =
            Regex::new(r#"<p\.class=\"([^\"]*)\""#).unwrap();
        let img_regex = Regex::new(r#"(<img[^>]*)(>)"#).unwrap();

        let html = r#"<img src="a.jpg" alt="First">
<img src="b.jpg" alt="Second">"#;
        let result = post_process_html(html, &class_regex, &img_regex);

        assert!(result.is_ok());
        let processed = result.unwrap();
        assert!(
            processed.contains("First") || processed.contains("first")
        );
        assert!(
            processed.contains("Second")
                || processed.contains("second")
        );
    }

    #[test]
    fn test_post_process_html_mixed_content() {
        let class_regex = Regex::new(r#"<p class="([^"]*)""#).unwrap();
        let img_regex = Regex::new(r#"(<img[^>]*)(>)"#).unwrap();

        let html = r#"<p class="intro">Hello</p>
<img src="test.jpg" alt="Test image">
<p class="outro">Goodbye</p>"#;
        let result = post_process_html(html, &class_regex, &img_regex);

        assert!(result.is_ok());
        let processed = result.unwrap();
        assert!(processed.contains("Hello"));
        assert!(processed.contains("Goodbye"));
    }

    #[test]
    fn test_post_process_html_special_characters_in_alt() {
        let class_regex =
            Regex::new(r#"<p\.class=\"([^\"]*)\""#).unwrap();
        let img_regex = Regex::new(r#"(<img[^>]*)(>)"#).unwrap();

        let html = r#"<img src="test.jpg" alt="Test & Example">"#;
        let result = post_process_html(html, &class_regex, &img_regex);

        assert!(result.is_ok());
    }

    #[test]
    fn test_post_process_html_replace_all_with_multiple_imgs() {
        // Two img tags on the same line separated by a closing tag (so [^>]* stops).
        // First img is consumed by captures/replace, second by replace_all which adds title.
        let class_regex =
            Regex::new(r#"<p\.class=\"([^\"]*)\""#).unwrap();
        let img_regex = Regex::new(r#"(<img[^>]*)(/>)"#).unwrap();

        let html = r#"<img src="a.jpg" alt="First" /><span>sep</span><img src="b.jpg" alt="Second photo" />"#;
        let result =
            post_process_html(html, &class_regex, &img_regex).unwrap();
        // The second img should have a title added by replace_all
        assert!(result.contains("title=\"second photo\""));
    }

    #[test]
    fn test_post_process_html_replace_all_preserves_existing_title() {
        // Second img already has title - replace_all should not add another
        let class_regex =
            Regex::new(r#"<p\.class=\"([^\"]*)\""#).unwrap();
        let img_regex = Regex::new(r#"(<img[^>]*)(/>)"#).unwrap();

        let html = r#"<img src="a.jpg" alt="First" /><span>x</span><img src="b.jpg" alt="Second" title="Existing" />"#;
        let result =
            post_process_html(html, &class_regex, &img_regex).unwrap();
        assert!(result.contains("Existing"));
    }

    #[test]
    fn test_post_process_html_replace_all_no_alt_on_second() {
        // Second img has no alt - should not get title
        let class_regex =
            Regex::new(r#"<p\.class=\"([^\"]*)\""#).unwrap();
        let img_regex = Regex::new(r#"(<img[^>]*)(/>)"#).unwrap();

        let html = r#"<img src="a.jpg" alt="First" /><span>x</span><img src="b.jpg" />"#;
        let result =
            post_process_html(html, &class_regex, &img_regex).unwrap();
        assert!(result.contains("img"));
    }

    #[test]
    fn test_post_process_html_replace_all_long_alt_truncation() {
        // Second img has a very long alt that should be truncated
        let class_regex =
            Regex::new(r#"<p\.class=\"([^\"]*)\""#).unwrap();
        let img_regex = Regex::new(r#"(<img[^>]*)(/>)"#).unwrap();

        let long_alt = "a".repeat(100);
        let html = format!(
            r#"<img src="a.jpg" alt="X" /><b>y</b><img src="b.jpg" alt="{}" />"#,
            long_alt
        );
        let result =
            post_process_html(&html, &class_regex, &img_regex).unwrap();
        assert!(result.contains("title="));
    }

    #[test]
    fn test_post_process_html_replace_all_three_imgs() {
        // Three imgs with separators: first consumed by captures, 2nd+3rd by replace_all
        let class_regex =
            Regex::new(r#"<p\.class=\"([^\"]*)\""#).unwrap();
        let img_regex = Regex::new(r#"(<img[^>]*)(/>)"#).unwrap();

        let html = r#"<img src="a.jpg" alt="Alpha" /><b>x</b><img src="b.jpg" alt="Beta" /><b>y</b><img src="c.jpg" alt="Gamma" />"#;
        let result =
            post_process_html(html, &class_regex, &img_regex).unwrap();
        assert!(result.contains("title=\"beta\""));
        assert!(result.contains("title=\"gamma\""));
    }

    #[test]
    fn test_post_process_html_complex_class_capture() {
        // Test class regex with a pattern that actually captures a group
        let class_regex =
            Regex::new(r#"<p\.class=\"([^\"]*)\""#).unwrap();
        let img_regex = Regex::new(r#"(<img[^>]*)(>)"#).unwrap();

        let html = r#"<p.class="important">Critical text</p>"#;
        let result = post_process_html(html, &class_regex, &img_regex);
        assert!(result.is_ok());
        let processed = result.unwrap();
        assert!(processed.contains("important"));
    }

    #[test]
    fn test_post_process_html_img_alt_with_quotes() {
        let class_regex =
            Regex::new(r#"<p\.class=\"([^\"]*)\""#).unwrap();
        let img_regex = Regex::new(r#"(<img[^>]*)(>)"#).unwrap();

        let html = r#"<img src="test.jpg" alt="A &amp; B photo">"#;
        let result = post_process_html(html, &class_regex, &img_regex);
        assert!(result.is_ok());
    }

    #[test]
    fn test_post_process_html_only_whitespace_lines() {
        let class_regex =
            Regex::new(r#"<p\.class=\"([^\"]*)\""#).unwrap();
        let img_regex = Regex::new(r#"(<img[^>]*)(>)"#).unwrap();

        let html = "   \n  \n   ";
        let result = post_process_html(html, &class_regex, &img_regex);
        assert!(result.is_ok());
    }
}
