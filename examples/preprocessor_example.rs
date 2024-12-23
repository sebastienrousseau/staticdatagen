// Copyright © 2025 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # StaticDataGen Preprocessor Examples
//!
//! This program demonstrates the preprocessing capabilities of the
//! StaticDataGen library, showing various content transformations
//! and preparations before final generation.

use regex::Regex;
use staticdatagen::modules::preprocessor::preprocess_content;
use std::error::Error;

/// Entry point for the StaticDataGen Preprocessor Examples program.
///
/// Demonstrates various preprocessing scenarios including class
/// attribute updates, image handling, and content transformations.
///
/// # Errors
///
/// Returns a `Result` containing a `Box<dyn std::error::Error>` if any error
/// occurs during the execution of the examples.
pub fn main() -> Result<(), Box<dyn Error>> {
    println!("\n🧪 StaticDataGen Preprocessor Examples\n");

    class_attributes_example()?;
    image_processing_example()?;
    code_block_example()?;
    custom_components_example()?;
    frontmatter_example()?;
    shortcode_example()?;
    syntax_highlighting_example()?;
    validation_example()?;

    println!("\n🎉 All preprocessor examples completed successfully!");

    Ok(())
}

/// Demonstrates class attribute processing.
fn class_attributes_example() -> Result<(), Box<dyn Error>> {
    println!("🦀 Class Attributes Example");
    println!("---------------------------------------------");

    let content = r#"
<div class="container">
    <p class="text-large">Large text paragraph</p>
    <p class="text-small">Small text paragraph</p>
</div>

{.custom-class}
# Heading with Custom Class

{.highlight}
This paragraph will have a highlight class.
    "#;

    let class_regex = Regex::new(r#"<p class=["']([^"']+)["']>"#)?;
    let img_regex = Regex::new(r"(<img[^>]+)(/>)")?;

    match preprocess_content(content, &class_regex, &img_regex) {
        Ok(processed) => {
            println!("    ✅ Processed content with class attributes:");
            println!("{}", processed);
        }
        Err(e) => println!("    ❌ Processing error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates image tag processing.
fn image_processing_example() -> Result<(), Box<dyn Error>> {
    println!("\n🦀 Image Processing Example");
    println!("---------------------------------------------");

    let content = r#"
# Image Examples

![Basic Image](image.jpg)

![Responsive Image](photo.jpg){.responsive}

<img src="picture.jpg" alt="HTML Image" class="large"/>

![Image with Title](banner.jpg "Banner Image"){.hero .center}
    "#;

    let class_regex = Regex::new(r#"<p class=["']([^"']+)["']>"#)?;
    let img_regex = Regex::new(r"(<img[^>]+)(/>)")?;

    match preprocess_content(content, &class_regex, &img_regex) {
        Ok(processed) => {
            println!("    ✅ Processed content with images:");
            println!("{}", processed);
        }
        Err(e) => println!("    ❌ Processing error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates code block processing.
fn code_block_example() -> Result<(), Box<dyn Error>> {
    println!("\n🦀 Code Block Example");
    println!("---------------------------------------------");

    let content = r#"
# Code Examples

```rust
fn main() {
    println!("Hello, World!");
}
```

```html
<div class="container">
    <p>Some HTML content</p>
</div>
```

{.code-wrapper}
```css
.container {
    max-width: 1200px;
    margin: 0 auto;
}
```
    "#;

    let class_regex = Regex::new(r#"<p class=["']([^"']+)["']>"#)?;
    let img_regex = Regex::new(r"(<img[^>]+)(/>)")?;

    match preprocess_content(content, &class_regex, &img_regex) {
        Ok(processed) => {
            println!("    ✅ Processed content with code blocks:");
            println!("{}", processed);
        }
        Err(e) => println!("    ❌ Processing error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates custom component processing.
fn custom_components_example() -> Result<(), Box<dyn Error>> {
    println!("\n🦀 Custom Components Example");
    println!("---------------------------------------------");

    let content = r#"
# Custom Components

{{> alert type="info" }}
This is an info alert message
{{/ alert}}

{{> card title="Feature" }}
This is a feature card content
{{/ card}}

{.custom-wrapper}
{{> tabs }}
- Tab 1 content
- Tab 2 content
{{/ tabs}}
    "#;

    let class_regex = Regex::new(r#"<p class=['"]([^'"]+)['"]>"#)?;
    let img_regex = Regex::new(r"(<img[^>]+)(/>)")?;

    match preprocess_content(content, &class_regex, &img_regex) {
        Ok(processed) => {
            println!(
                "    ✅ Processed content with custom components:"
            );
            println!("{}", processed);
        }
        Err(e) => println!("    ❌ Processing error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates frontmatter processing.
fn frontmatter_example() -> Result<(), Box<dyn Error>> {
    println!("\n🦀 Frontmatter Example");
    println!("---------------------------------------------");

    let content = r#"---
title: Example Page
description: A page showing frontmatter processing
date: 2024-02-20
tags: [example, preprocessing]
layout: default
---

# Main Content

This is the main content of the page.

{.highlight}
This paragraph has a custom class.
    "#;

    let class_regex = Regex::new(r#"<p class=['"]([^'"]+)['"]>"#)?;
    let img_regex = Regex::new(r"(<img[^>]+)(/>)")?;

    match preprocess_content(content, &class_regex, &img_regex) {
        Ok(processed) => {
            println!("    ✅ Processed content with frontmatter:");
            println!("{}", processed);
        }
        Err(e) => println!("    ❌ Processing error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates shortcode processing.
fn shortcode_example() -> Result<(), Box<dyn Error>> {
    println!("\n🦀 Shortcode Example");
    println!("---------------------------------------------");

    let content = r#"
# Shortcode Examples

{{% youtube id="12345" %}}

{{% tweet id="67890" %}}

{.shortcode-wrapper}
{{% gallery folder="vacation" %}}
    "#;

    let class_regex = Regex::new(r#"<p class=['"]([^'"]+)['"]>"#)?;
    let img_regex = Regex::new(r"(<img[^>]+)(/>)")?;

    match preprocess_content(content, &class_regex, &img_regex) {
        Ok(processed) => {
            println!("    ✅ Processed content with shortcodes:");
            println!("{}", processed);
        }
        Err(e) => println!("    ❌ Processing error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates syntax highlighting preprocessing.
fn syntax_highlighting_example() -> Result<(), Box<dyn Error>> {
    println!("\n🦀 Syntax Highlighting Example");
    println!("---------------------------------------------");

    let content = r#"
# Code Examples with Syntax Highlighting

```rust,linenos
fn main() {
    // A comment
    let x = 42;
    println!("The answer is: {}", x);
}
```

{.code-block}
```python,emphasize=2-3
def greet(name):
    # Highlighted lines
    message = f"Hello, {name}!"
    return message
```
    "#;

    let class_regex = Regex::new(r#"<p class=['"]([^'"]+)['"]>"#)?;
    let img_regex = Regex::new(r"(<img[^>]+)(/>)")?;

    match preprocess_content(content, &class_regex, &img_regex) {
        Ok(processed) => {
            println!(
                "    ✅ Processed content with syntax highlighting:"
            );
            println!("{}", processed);
        }
        Err(e) => println!("    ❌ Processing error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates content validation during preprocessing.
fn validation_example() -> Result<(), Box<dyn Error>> {
    println!("\n🦀 Validation Example");
    println!("---------------------------------------------");

    let test_cases = vec![
        (
            "Valid content with class\n{.valid}\nTest content",
            true,
            "Valid class syntax",
        ),
        (
            "Invalid class syntax\n{invalid}\nTest content",
            true,
            "Invalid class syntax",
        ),
        (
            "Valid image\n![Alt](image.jpg){.img}\n",
            true,
            "Valid image syntax",
        ),
        (
            "Invalid image\n![Alt](image.jpg{.img}\n",
            false,
            "Invalid image syntax",
        ),
    ];

    let class_regex = Regex::new(r#"<p class=['"]([^'"]+)['"]>"#)?;
    let img_regex = Regex::new(r"(<img[^>]+)(/>)")?;

    for (content, should_succeed, case) in test_cases {
        match preprocess_content(content, &class_regex, &img_regex) {
            Ok(_) => {
                if should_succeed {
                    println!("    ✅ {}: Processed successfully", case);
                } else {
                    println!("    ❌ {}: Unexpected success", case);
                }
            }
            Err(e) => {
                if !should_succeed {
                    println!("    ✅ {}: Failed as expected", case);
                } else {
                    println!(
                        "    ❌ {}: Unexpected error: {:?}",
                        case, e
                    );
                }
            }
        }
    }

    Ok(())
}
