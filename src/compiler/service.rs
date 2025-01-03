// Copyright © 2025 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Compilation service for static site generation
//!
//! This module provides the core functionality for compiling source files
//! into static website content, including HTML generation, RSS feeds,
//! sitemaps, and various metadata files.

use anyhow::{Context, Result};
use html_generator::{generate_html, HtmlConfig};
use metadata_gen::extract_and_prepare_metadata;
use rlg::{log_format::LogFormat, log_level::LogLevel};
use rss_gen::{
    data::{RssData, RssItem},
    generate_rss, macro_set_rss_data_fields,
};
use sitemap_gen::create_site_map_data;
use staticweaver::{Context as TemplateContext, Engine, PageOptions};
use std::{collections::HashMap, fs, path::Path, time::Duration};

use crate::{
    generators::{
        cname::{CnameConfig, CnameGenerator},
        humans::{HumansConfig, HumansGenerator},
        manifest::{ManifestConfig, ManifestGenerator},
        news_sitemap::{NewsSiteMapConfig, NewsSiteMapGenerator},
        tags::*,
    },
    macro_cleanup_directories, macro_create_directories,
    macro_log_info, macro_metadata_option,
    models::data::{FileData, PageData},
    modules::{
        json::{security, sitemap, txt},
        navigation::NavigationGenerator,
        robots::create_txt_data,
        security::create_security_data,
    },
    utilities::{file::add, write::write_files_to_build_directory},
};

/// Compiles source files in a specified directory into static site content.
/// Generates HTML pages, RSS feeds, sitemaps, and other essential metadata files.
///
/// # Arguments
///
/// * `build_dir_path` - The path to the temporary build directory.
/// * `content_path` - The path to the content directory with source files.
/// * `site_path` - The path to the output site directory.
/// * `template_path` - The path to the template directory for HTML templates.
///
/// # Returns
///
/// Returns `Ok(())` if compilation succeeds. If an error occurs, a detailed
/// `anyhow::Error` is returned.
pub fn compile(
    build_dir_path: &Path,
    content_path: &Path,
    site_path: &Path,
    template_path: &Path,
) -> Result<()> {
    // Create necessary directories with error context.
    macro_create_directories!(build_dir_path, site_path)
        .context("Failed to create build and site directories")?;

    // Load source files for compilation.
    let source_files = add(content_path).context(
        "Failed to load source files from content directory",
    )?;

    // Generate the navigation structure.
    let navigation =
        NavigationGenerator::generate_navigation(&source_files);

    let mut global_tags_data: HashMap<String, Vec<PageData>> =
        HashMap::new();

    // Initialize the templating engine with caching.
    let mut engine = Engine::new(
        template_path.to_str().unwrap(),
        Duration::from_secs(60),
    );

    // Compile source files into `compiled_files`, collecting results as `FileData`.
    let compiled_files: Result<Vec<FileData>> = source_files
        .into_iter()
        .map(|file| {
            process_file(
                &file,
                &mut engine,
                template_path,
                &navigation,
                &mut global_tags_data,
                site_path,
            )
        })
        .collect();

    // Log compilation completion message.
    let cli_description = format!(
        "<Notice>: Successfully generated, compiled, and minified all HTML to the `{:?}` directory",
        site_path.display()
    );

    macro_log_info!(
        &LogLevel::INFO,
        "compiler.rs",
        &cli_description,
        &LogFormat::CLF
    );

    // Write each compiled file to the output directory.
    for file in &compiled_files? {
        write_files_to_build_directory(
            build_dir_path,
            file,
            template_path,
        )?;
    }

    // Generate and write global tags HTML.
    let tags_html_content = generate_tags_html(&global_tags_data);
    write_tags_html_to_file(&tags_html_content, build_dir_path)?;

    // Clean up and finalize site structure.
    macro_cleanup_directories!(site_path)
        .context("Failed to clean up site directory")?;
    fs::rename(build_dir_path, site_path)
        .context("Failed to finalize build directory")?;

    Ok(())
}

/// Splits a Markdown content string into frontmatter and body parts.
///
/// The function uses the `---` separator to divide the content into two parts:
/// the frontmatter (metadata) and the body (main content).
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
/// If the `---` separator is not found in the content, both strings will be empty.
pub fn split_frontmatter_and_body(content: &str) -> (String, String) {
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

/// Processes a single file, generating necessary content and metadata.
///
/// # Arguments
///
/// * `file` - A reference to `FileData` representing the source file.
/// * `engine` - A mutable reference to the templating `Engine`.
/// * `_template_path` - The path to the template directory (optional).
/// * `navigation` - HTML navigation content.
/// * `global_tags_data` - Mutable reference to global tags data for aggregation.
/// * `site_path` - The path to the output site directory.
///
/// # Returns
///
/// Returns `Result<FileData>` containing the processed file data.
fn process_file(
    file: &FileData,
    engine: &mut Engine,
    _template_path: &Path,
    navigation: &str,
    global_tags_data: &mut HashMap<String, Vec<PageData>>,
    site_path: &Path,
) -> Result<FileData> {
    // Preprocess to separate frontmatter and body
    let (_frontmatter, body) =
        split_frontmatter_and_body(&file.content);

    // println!("Frontmatter: {}", frontmatter);

    let (metadata, keywords, all_meta_tags) =
        extract_and_prepare_metadata(&file.content)
            .context("Failed to extract and prepare metadata")?;

    let _security_options = create_security_data(&metadata);
    let config = HtmlConfig {
        enable_syntax_highlighting: true,
        minify_output: false,
        add_aria_attributes: true,
        generate_structured_data: true,
        generate_toc: false,
        language: "en".to_string(),
        max_input_size: usize::MAX,
        syntax_theme: None,
    };

    let html_content = generate_html(&body, &config)
        .context("Failed to generate HTML content")?;

    // println!("HTML Content: {}", html_content);

    let mut page_options = PageOptions::new();
    for (key, value) in metadata.iter() {
        page_options.set(key.to_string(), value.to_string());
    }

    page_options.set("apple".to_string(), all_meta_tags.apple.clone());
    page_options.set("content".to_string(), html_content);
    page_options.set("microsoft".to_string(), all_meta_tags.ms.clone());
    page_options.set("navigation".to_string(), navigation.to_owned());
    page_options.set("opengraph".to_string(), all_meta_tags.og);
    page_options.set("primary".to_string(), all_meta_tags.primary);
    page_options.set("twitter".to_string(), all_meta_tags.twitter);

    let mut context = TemplateContext::new();
    for (key, value) in page_options.elements.iter() {
        context.set(key.to_string(), value.to_string());
    }

    let content = engine.render_page(
        &context,
        metadata.get("layout").cloned().unwrap_or_default().as_str(),
    )?;

    let mut rss_data = RssData::new(None);

    macro_set_rss_data_fields!(
        rss_data,
        AtomLink = macro_metadata_option!(metadata, "atom_link"),
        Author = macro_metadata_option!(metadata, "author"),
        Category = macro_metadata_option!(metadata, "category"),
        Copyright = macro_metadata_option!(metadata, "copyright"),
        Description = macro_metadata_option!(metadata, "description"),
        Docs = macro_metadata_option!(metadata, "docs"),
        Generator = macro_metadata_option!(metadata, "generator"),
        ImageTitle = macro_metadata_option!(metadata, "image_title"),
        ImageUrl = macro_metadata_option!(metadata, "image_url"),
        Language = macro_metadata_option!(metadata, "language"),
        LastBuildDate =
            macro_metadata_option!(metadata, "last_build_date"),
        Link = macro_metadata_option!(metadata, "permalink"),
        ManagingEditor =
            macro_metadata_option!(metadata, "managing_editor"),
        PubDate = macro_metadata_option!(metadata, "pub_date"),
        Title = macro_metadata_option!(metadata, "title"),
        Ttl = macro_metadata_option!(metadata, "ttl"),
        Webmaster = macro_metadata_option!(metadata, "webmaster")
    );

    let item = RssItem::new()
        .guid(macro_metadata_option!(metadata, "item_guid"))
        .description(macro_metadata_option!(
            metadata,
            "item_description"
        ))
        .link(macro_metadata_option!(metadata, "item_link"))
        .pub_date(macro_metadata_option!(metadata, "item_pub_date"))
        .title(macro_metadata_option!(metadata, "item_title"));
    rss_data.add_item(item);

    let rss = generate_rss(&rss_data)?;

    let manifest_content = ManifestConfig::from_metadata(&metadata)
        .and_then(|config| ManifestGenerator::new(config).generate())
        .unwrap_or_else(|e| {
            eprintln!("Error generating manifest: {}", e);
            String::new()
        });

    let news_sitemap_config = NewsSiteMapConfig::new(metadata.clone());
    let news_sitemap_generator =
        NewsSiteMapGenerator::new(news_sitemap_config);

    let news_sitemap_content =
        match news_sitemap_generator.generate_xml() {
            xml if !xml.is_empty() => xml, // Use the generated XML string
            _ => {
                eprintln!("Error generating news sitemap XML.");
                String::new() // Default to an empty string if XML generation fails
            }
        };

    let cname_content = metadata
        .get("cname")
        .and_then(|domain| CnameConfig::new(domain, None, None).ok())
        .map(|config| CnameGenerator::new(config).generate())
        .unwrap_or_default();

    let humans_content = metadata
        .get("humans")
        .map(|humans| {
            // Try parsing the "humans" string into a HashMap
            let humans: HashMap<String, String> =
                serde_json::from_str(humans)
                    .context("Failed to parse humans metadata")
                    .unwrap_or_else(|err| {
                        eprintln!(
                            "Error parsing humans metadata: {}",
                            err
                        );
                        HashMap::new() // Default to an empty HashMap if parsing fails
                    });

            // Generate humans.txt content
            match HumansConfig::from_metadata(&humans) {
                Ok(humans_config) => {
                    HumansGenerator::new(humans_config).generate()
                }
                Err(err) => {
                    eprintln!("Error creating HumansConfig: {}", err);
                    String::new() // Default to an empty string if creation fails
                }
            }
        })
        .unwrap_or_default();

    // let human_options = create_human_data(&metadata);
    let security_options = create_security_data(&metadata);
    let sitemap_options = create_site_map_data(&metadata);
    // let news_sitemap_options = create_news_site_map_data(&metadata);

    let tags_data = generate_tags(file, &metadata);

    update_global_tags_data(global_tags_data, &tags_data);

    let txt_options = create_txt_data(&metadata);

    let txt_data = txt(&txt_options);
    // let human_data = human(&human_options);
    let security_data = security(&security_options);
    let sitemap_data = sitemap(sitemap_options?, site_path);

    Ok(FileData {
        cname: cname_content,
        content,
        keyword: keywords.join(", "),
        human: humans_content,
        manifest: manifest_content,
        name: file.name.clone(),
        rss,
        security: security_data,
        sitemap: sitemap_data?,
        sitemap_news: news_sitemap_content,
        txt: txt_data,
    })
}

/// Updates the global tags data with new tag information.
///
/// # Arguments
///
/// * `global_tags_data` - Mutable reference to global tags data hashmap.
/// * `tags_data` - Reference to the tags data hashmap to be merged.
fn update_global_tags_data(
    global_tags_data: &mut HashMap<String, Vec<PageData>>,
    tags_data: &HashMap<String, Vec<HashMap<String, String>>>,
) {
    for (tag, pages_data) in tags_data {
        let page_info: Vec<PageData> = pages_data
            .iter()
            .map(|page_data| PageData {
                title: page_data
                    .get("title")
                    .cloned()
                    .unwrap_or_default(),
                description: page_data
                    .get("description")
                    .cloned()
                    .unwrap_or_default(),
                permalink: page_data
                    .get("permalink")
                    .cloned()
                    .unwrap_or_default(),
                date: page_data
                    .get("date")
                    .cloned()
                    .unwrap_or_default(),
            })
            .collect();

        global_tags_data
            .entry(tag.clone())
            .or_default()
            .extend(page_info);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rss_gen::data::RssDataField;

    #[test]
    fn test_compile_missing_directories() {
        let build_dir_path = Path::new("/nonexistent/build");
        let content_path = Path::new("/nonexistent/content");
        let site_path = Path::new("/nonexistent/site");
        let template_path = Path::new("/nonexistent/templates");

        let result = compile(
            build_dir_path,
            content_path,
            site_path,
            template_path,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_split_frontmatter_and_body_with_separator() {
        let content = "---\ntitle: Test\n---\nThis is the body.";
        let (frontmatter, body) = split_frontmatter_and_body(content);

        assert_eq!(frontmatter, "title: Test");
        assert_eq!(body, "This is the body.");
    }

    #[test]
    fn test_split_frontmatter_and_body_no_separator() {
        let content = "This is just the body.";
        let (frontmatter, body) = split_frontmatter_and_body(content);

        assert!(frontmatter.is_empty());
        assert_eq!(body, "This is just the body.");
    }

    #[test]
    fn test_split_frontmatter_and_body_empty_content() {
        let content = "";
        let (frontmatter, body) = split_frontmatter_and_body(content);

        assert!(frontmatter.is_empty());
        assert!(body.is_empty());
    }

    #[test]
    fn test_update_global_tags_data() {
        let mut global_tags_data = HashMap::new();
        let tags_data = HashMap::from([(
            "tag1".to_string(),
            vec![HashMap::from([
                ("title".to_string(), "Page1".to_string()),
                ("description".to_string(), "Description1".to_string()),
                ("permalink".to_string(), "/page1".to_string()),
                ("date".to_string(), "2024-12-23".to_string()),
            ])],
        )]);

        update_global_tags_data(&mut global_tags_data, &tags_data);

        assert!(global_tags_data.contains_key("tag1"));
        assert_eq!(global_tags_data["tag1"].len(), 1);
        assert_eq!(global_tags_data["tag1"][0].title, "Page1");
    }

    #[test]
    fn test_split_frontmatter_and_body_multiple_separators() {
        let content = "---\ntitle: Test\n---\n---\nThis is the body.";
        let (frontmatter, body) = split_frontmatter_and_body(content);

        assert_eq!(frontmatter, "title: Test");
        assert_eq!(body, "---\nThis is the body.");
    }

    #[test]
    fn test_process_file_invalid_metadata() {
        let file = FileData {
            name: "invalid_metadata".to_string(),
            content: "---\ninvalid_yaml: { missing_value\n---\nBody."
                .to_string(),
            ..Default::default()
        };
        let mut engine =
            Engine::new("/templates", Duration::from_secs(60));
        let mut global_tags_data = HashMap::new();
        let navigation = "Navigation HTML";
        let site_path = Path::new("/site");

        let result = process_file(
            &file,
            &mut engine,
            Path::new("/templates"),
            navigation,
            &mut global_tags_data,
            site_path,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_split_frontmatter_and_body_with_empty_frontmatter() {
        let content = "---\n---\nThis is the body.";
        let (frontmatter, body) = split_frontmatter_and_body(content);

        assert!(frontmatter.is_empty());
        assert_eq!(body, "This is the body.");
    }

    #[test]
    fn test_update_global_tags_data_empty_tags() {
        let mut global_tags_data = HashMap::new();
        let tags_data: HashMap<String, Vec<HashMap<String, String>>> =
            HashMap::new();

        update_global_tags_data(&mut global_tags_data, &tags_data);

        assert!(global_tags_data.is_empty());
    }

    #[test]
    fn test_split_frontmatter_and_body_invalid_format() {
        let content = "---\ninvalid_yaml_content\nBody content.";
        let (frontmatter, body) = split_frontmatter_and_body(content);

        assert_eq!(frontmatter, "invalid_yaml_content\nBody content.");
        assert!(body.is_empty());
    }

    #[test]
    fn test_compile_missing_navigation() {
        let file = FileData {
            name: "test".to_string(),
            content: "---\ntitle: Test\n---\nBody.".to_string(),
            ..Default::default()
        };

        let mut engine =
            Engine::new("/templates", Duration::from_secs(60));
        let mut global_tags_data = HashMap::new();
        let navigation = "";
        let site_path = Path::new("/site");

        let result = process_file(
            &file,
            &mut engine,
            Path::new("/templates"),
            navigation,
            &mut global_tags_data,
            site_path,
        );

        assert!(result.is_err());
    }

    // Test handling of edge cases in HTML config
    #[test]
    fn test_html_config_edge_cases() {
        let config = HtmlConfig {
            enable_syntax_highlighting: false,
            minify_output: true,
            add_aria_attributes: false,
            generate_structured_data: false,
            generate_toc: true,
            language: "fr".to_string(),
            max_input_size: 100,
            syntax_theme: Some("monokai".to_string()),
        };

        let body = "Test content";
        let result = generate_html(body, &config);
        assert!(result.is_ok());
    }

    // Test metadata extraction with various fields
    #[test]
    fn test_metadata_extraction() {
        let content = r#"---
title: Test Page
description: A test description
author: John Doe
date: 2025-01-01
keywords: test, example
---
Content here"#;

        let (frontmatter, _) = split_frontmatter_and_body(content);
        assert!(frontmatter.contains("title: Test Page"));
        assert!(frontmatter.contains("author: John Doe"));
    }

    // Test RSS data generation
    #[test]
    fn test_rss_data_generation() {
        let mut metadata = HashMap::new();
        let _ = metadata
            .insert("title".to_string(), "Test Title".to_string());
        let _ = metadata.insert(
            "description".to_string(),
            "Test Description".to_string(),
        );
        let _ = metadata.insert(
            "permalink".to_string(),
            "https://example.com".to_string(),
        );

        let mut rss_data = RssData::new(None);
        macro_set_rss_data_fields!(
            rss_data,
            Title = macro_metadata_option!(metadata, "title"),
            Description =
                macro_metadata_option!(metadata, "description"),
            Link = macro_metadata_option!(metadata, "permalink")
        );

        let result = generate_rss(&rss_data);
        assert!(result.is_ok());
    }

    // Test multiple file compilation
    #[test]
    fn test_multiple_file_compilation() {
        let files = vec![
            FileData {
                name: "test1.md".to_string(),
                content: "# Test 1".to_string(),
                ..Default::default()
            },
            FileData {
                name: "test2.md".to_string(),
                content: "# Test 2".to_string(),
                ..Default::default()
            },
        ];

        let navigation =
            NavigationGenerator::generate_navigation(&files);
        assert!(!navigation.is_empty());
    }

    // Test error handling for invalid templates
    #[test]
    fn test_invalid_template_handling() {
        let mut engine =
            Engine::new("/nonexistent", Duration::from_secs(60));
        let context = TemplateContext::new();
        let result = engine.render_page(&context, "nonexistent");
        assert!(result.is_err());
    }

    // Test metadata handling with missing required fields
    #[test]
    fn test_missing_required_metadata() {
        let content = "---\n---\nBody content";
        let file = FileData {
            name: "test.md".to_string(),
            content: content.to_string(),
            ..Default::default()
        };

        let mut engine =
            Engine::new("/templates", Duration::from_secs(60));
        let navigation = "Navigation";
        let mut global_tags_data = HashMap::new();
        let site_path = Path::new("/site");

        let result = process_file(
            &file,
            &mut engine,
            Path::new("/templates"),
            navigation,
            &mut global_tags_data,
            site_path,
        );

        assert!(result.is_err());
    }

    // Test handling of malformed RSS data
    #[test]
    fn test_malformed_rss_data() {
        let rss_data = RssData::new(None);
        // Set invalid fields
        let _ = rss_data
            .clone()
            .set(RssDataField::Title, "invalid_value".to_string());

        let result = generate_rss(&rss_data);
        assert!(result.is_err());
    }
}
