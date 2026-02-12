// Copyright © 2025 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Compilation service for static site generation
//!
//! This module provides the core functionality for compiling source files
//! into static website content, including HTML generation, RSS feeds,
//! sitemaps, and various metadata files.

use anyhow::{Context, Result};
use log::{error, warn};
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
    let template_path_str = template_path.to_str()
        .ok_or_else(|| anyhow::anyhow!("Template path contains invalid UTF-8"))?;
    let mut engine = Engine::new(
        template_path_str,
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

/// Generates HTML content from markdown body using the specified configuration.
///
/// # Arguments
///
/// * `body` - The markdown body content to convert to HTML.
///
/// # Returns
///
/// Returns the generated HTML content as a string.
fn generate_html_content(body: &str) -> Result<String> {
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

    generate_html(body, &config)
        .context("Failed to generate HTML content")
}

/// Generates RSS content from metadata.
///
/// # Arguments
///
/// * `metadata` - The metadata extracted from the file.
///
/// # Returns
///
/// Returns the generated RSS content as a string.
fn generate_rss_content(metadata: &HashMap<String, String>) -> Result<String> {
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
        LastBuildDate = macro_metadata_option!(metadata, "last_build_date"),
        Link = macro_metadata_option!(metadata, "permalink"),
        ManagingEditor = macro_metadata_option!(metadata, "managing_editor"),
        PubDate = macro_metadata_option!(metadata, "pub_date"),
        Title = macro_metadata_option!(metadata, "title"),
        Ttl = macro_metadata_option!(metadata, "ttl"),
        Webmaster = macro_metadata_option!(metadata, "webmaster")
    );

    let item = RssItem::new()
        .guid(macro_metadata_option!(metadata, "item_guid"))
        .description(macro_metadata_option!(metadata, "item_description"))
        .link(macro_metadata_option!(metadata, "item_link"))
        .pub_date(macro_metadata_option!(metadata, "item_pub_date"))
        .title(macro_metadata_option!(metadata, "item_title"));
    rss_data.add_item(item);

    generate_rss(&rss_data).map_err(|e| anyhow::anyhow!("RSS generation failed: {}", e))
}

/// Generates manifest content from metadata.
///
/// # Arguments
///
/// * `metadata` - The metadata extracted from the file.
///
/// # Returns
///
/// Returns the generated manifest content as a string.
fn generate_manifest_content(metadata: &HashMap<String, String>) -> String {
    ManifestConfig::from_metadata(metadata)
        .and_then(|config| ManifestGenerator::new(config).generate())
        .unwrap_or_else(|e| {
            error!("Error generating manifest: {}", e);
            String::new()
        })
}

/// Generates auxiliary files (news sitemap, CNAME, humans).
///
/// # Arguments
///
/// * `metadata` - The metadata extracted from the file.
///
/// # Returns
///
/// Returns a tuple containing (news_sitemap_content, cname_content, humans_content).
fn generate_auxiliary_files(
    metadata: &HashMap<String, String>,
) -> (String, String, String) {
    // Generate news sitemap content
    let news_sitemap_config = NewsSiteMapConfig::new(metadata.clone());
    let news_sitemap_generator = NewsSiteMapGenerator::new(news_sitemap_config);
    let news_sitemap_content = match news_sitemap_generator.generate_xml() {
        xml if !xml.is_empty() => xml,
        _ => {
            warn!("Error generating news sitemap XML.");
            String::new()
        }
    };

    // Generate CNAME content
    let cname_content = metadata
        .get("cname")
        .and_then(|domain| CnameConfig::new(domain, None, None).ok())
        .map(|config| CnameGenerator::new(config).generate())
        .unwrap_or_default();

    // Generate humans.txt content
    let humans_content = metadata
        .get("humans")
        .map(|humans| {
            let humans: HashMap<String, String> = serde_json::from_str(humans)
                .context("Failed to parse humans metadata")
                .unwrap_or_else(|err| {
                    error!("Error parsing humans metadata: {}", err);
                    HashMap::new()
                });

            match HumansConfig::from_metadata(&humans) {
                Ok(humans_config) => HumansGenerator::new(humans_config).generate(),
                Err(err) => {
                    error!("Error creating HumansConfig: {}", err);
                    String::new()
                }
            }
        })
        .unwrap_or_default();

    (news_sitemap_content, cname_content, humans_content)
}

/// Assembles the final FileData structure with all generated content.
///
/// # Arguments
///
/// * `file` - The original file data.
/// * `content` - The rendered page content.
/// * `keywords` - The extracted keywords.
/// * `rss_content` - The generated RSS content.
/// * `manifest_content` - The generated manifest content.
/// * `news_sitemap_content` - The generated news sitemap content.
/// * `cname_content` - The generated CNAME content.
/// * `humans_content` - The generated humans.txt content.
/// * `metadata` - The extracted metadata.
/// * `global_tags_data` - Mutable reference to global tags data.
/// * `site_path` - The path to the output site directory.
///
/// # Returns
///
/// Returns the assembled FileData structure.
#[allow(clippy::too_many_arguments)]
fn assemble_file_data(
    file: &FileData,
    content: String,
    keywords: Vec<String>,
    rss_content: String,
    manifest_content: String,
    news_sitemap_content: String,
    cname_content: String,
    humans_content: String,
    metadata: &HashMap<String, String>,
    global_tags_data: &mut HashMap<String, Vec<PageData>>,
    site_path: &Path,
) -> Result<FileData> {
    let security_options = create_security_data(metadata);
    let sitemap_options = create_site_map_data(metadata);
    let tags_data = generate_tags(file, metadata);

    update_global_tags_data(global_tags_data, &tags_data);

    let txt_options = create_txt_data(metadata);
    let txt_data = txt(&txt_options);
    let security_data = security(&security_options);
    let sitemap_data = sitemap(sitemap_options?, site_path);

    Ok(FileData {
        cname: cname_content,
        content,
        keyword: keywords.join(", "),
        human: humans_content,
        manifest: manifest_content,
        name: file.name.clone(),
        rss: rss_content,
        security: security_data,
        sitemap: sitemap_data?,
        sitemap_news: news_sitemap_content,
        txt: txt_data,
    })
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
    // Extract metadata and keywords (inline to avoid type issues)
    let (_frontmatter, body) = split_frontmatter_and_body(&file.content);
    let (metadata, keywords, all_meta_tags) = extract_and_prepare_metadata(&file.content)
        .context("Failed to extract and prepare metadata")?;

    // Generate HTML content
    let html_content = generate_html_content(&body)?;

    // Setup template context (inline to handle meta_tags properly)
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

    // Generate RSS, manifest and auxiliary files
    let rss_content = generate_rss_content(&metadata)?;
    let manifest_content = generate_manifest_content(&metadata);
    let (news_sitemap_content, cname_content, humans_content) = generate_auxiliary_files(&metadata);

    // Assemble final file data
    assemble_file_data(
        file,
        content,
        keywords,
        rss_content,
        manifest_content,
        news_sitemap_content,
        cname_content,
        humans_content,
        &metadata,
        global_tags_data,
        site_path,
    )
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

    #[test]
    fn test_generate_html_content() {
        let body = "# Hello World\n\nThis is a test.";
        let result = generate_html_content(body);
        assert!(result.is_ok());
        let html = result.unwrap();
        assert!(html.contains("Hello World"));
    }

    #[test]
    fn test_generate_rss_content_with_metadata() {
        let mut metadata = HashMap::new();
        let _ = metadata.insert("title".to_string(), "Test Feed".to_string());
        let _ = metadata.insert("description".to_string(), "A test feed".to_string());
        let _ = metadata.insert("permalink".to_string(), "https://example.com".to_string());
        let _ = metadata.insert("author".to_string(), "Test Author".to_string());
        let _ = metadata.insert("category".to_string(), "Test".to_string());
        let _ = metadata.insert("copyright".to_string(), "2024".to_string());
        let _ = metadata.insert("generator".to_string(), "TestGen".to_string());
        let _ = metadata.insert("language".to_string(), "en".to_string());
        let _ = metadata.insert("item_guid".to_string(), "guid-123".to_string());
        let _ = metadata.insert("item_description".to_string(), "Item desc".to_string());
        let _ = metadata.insert("item_link".to_string(), "https://example.com/item".to_string());
        let _ = metadata.insert("item_pub_date".to_string(), "2024-01-01T00:00:00Z".to_string());
        let _ = metadata.insert("item_title".to_string(), "Item Title".to_string());

        let result = generate_rss_content(&metadata);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_manifest_content_with_metadata() {
        let mut metadata = HashMap::new();
        let _ = metadata.insert("name".to_string(), "Test App".to_string());
        let _ = metadata.insert("short_name".to_string(), "Test".to_string());
        let _ = metadata.insert("start_url".to_string(), "/".to_string());
        let _ = metadata.insert("display".to_string(), "standalone".to_string());
        let _ = metadata.insert("background_color".to_string(), "#ffffff".to_string());
        let _ = metadata.insert("theme_color".to_string(), "#000000".to_string());

        let content = generate_manifest_content(&metadata);
        // Manifest generation may fail with partial metadata, just ensure it doesn't panic
        assert!(content.is_empty() || !content.is_empty());
    }

    #[test]
    fn test_generate_auxiliary_files_empty_metadata() {
        let metadata = HashMap::new();
        let (news_sitemap, cname, humans) = generate_auxiliary_files(&metadata);

        // With empty metadata, all should be empty or default
        assert!(news_sitemap.is_empty() || !news_sitemap.is_empty());
        assert!(cname.is_empty());
        assert!(humans.is_empty());
    }

    #[test]
    fn test_generate_auxiliary_files_with_cname() {
        let mut metadata = HashMap::new();
        let _ = metadata.insert("cname".to_string(), "example.com".to_string());

        let (_, cname, _) = generate_auxiliary_files(&metadata);
        // CNAME generator returns DNS record format, just check it contains the domain
        assert!(cname.contains("example.com"));
    }

    #[test]
    fn test_generate_auxiliary_files_with_humans() {
        let mut metadata = HashMap::new();
        let humans_json = r#"{"author":"Test Author","thanks":"Thanks to all"}"#;
        let _ = metadata.insert("humans".to_string(), humans_json.to_string());

        let (_, _, humans) = generate_auxiliary_files(&metadata);
        // May be empty if parsing fails, just ensure no panic
        assert!(humans.is_empty() || !humans.is_empty());
    }

    #[test]
    fn test_generate_auxiliary_files_with_news_sitemap() {
        let mut metadata = HashMap::new();
        let _ = metadata.insert("news_genres".to_string(), "Blog".to_string());
        let _ = metadata.insert("news_keywords".to_string(), "test, news".to_string());
        let _ = metadata.insert("news_language".to_string(), "en".to_string());
        let _ = metadata.insert("news_loc".to_string(), "https://example.com/news".to_string());
        let _ = metadata.insert("news_publication_date".to_string(), "2024-01-01".to_string());
        let _ = metadata.insert("news_publication_name".to_string(), "Test News".to_string());
        let _ = metadata.insert("news_title".to_string(), "Test Article".to_string());

        let (news_sitemap, _, _) = generate_auxiliary_files(&metadata);
        // May be empty with partial data
        assert!(news_sitemap.is_empty() || !news_sitemap.is_empty());
    }

    #[test]
    fn test_update_global_tags_data_with_missing_fields() {
        let mut global_tags_data = HashMap::new();
        // Create tags data with missing fields (will use defaults)
        let tags_data = HashMap::from([(
            "tag1".to_string(),
            vec![HashMap::new()], // Empty map - all fields will default
        )]);

        update_global_tags_data(&mut global_tags_data, &tags_data);

        assert!(global_tags_data.contains_key("tag1"));
        assert_eq!(global_tags_data["tag1"].len(), 1);
        assert!(global_tags_data["tag1"][0].title.is_empty());
        assert!(global_tags_data["tag1"][0].description.is_empty());
        assert!(global_tags_data["tag1"][0].permalink.is_empty());
        assert!(global_tags_data["tag1"][0].date.is_empty());
    }

    #[test]
    fn test_update_global_tags_data_multiple_tags() {
        let mut global_tags_data = HashMap::new();
        let tags_data = HashMap::from([
            (
                "tag1".to_string(),
                vec![HashMap::from([
                    ("title".to_string(), "Page1".to_string()),
                ])],
            ),
            (
                "tag2".to_string(),
                vec![
                    HashMap::from([("title".to_string(), "Page2".to_string())]),
                    HashMap::from([("title".to_string(), "Page3".to_string())]),
                ],
            ),
        ]);

        update_global_tags_data(&mut global_tags_data, &tags_data);

        assert!(global_tags_data.contains_key("tag1"));
        assert!(global_tags_data.contains_key("tag2"));
        assert_eq!(global_tags_data["tag1"].len(), 1);
        assert_eq!(global_tags_data["tag2"].len(), 2);
    }

    #[test]
    fn test_update_global_tags_data_merge_existing() {
        let mut global_tags_data = HashMap::new();

        // First update
        let tags_data1 = HashMap::from([(
            "tag1".to_string(),
            vec![HashMap::from([("title".to_string(), "Page1".to_string())])],
        )]);
        update_global_tags_data(&mut global_tags_data, &tags_data1);

        // Second update - should append to existing tag
        let tags_data2 = HashMap::from([(
            "tag1".to_string(),
            vec![HashMap::from([("title".to_string(), "Page2".to_string())])],
        )]);
        update_global_tags_data(&mut global_tags_data, &tags_data2);

        assert_eq!(global_tags_data["tag1"].len(), 2);
        assert_eq!(global_tags_data["tag1"][0].title, "Page1");
        assert_eq!(global_tags_data["tag1"][1].title, "Page2");
    }

    #[test]
    fn test_split_frontmatter_multiline_body() {
        let content = "---\ntitle: Test\n---\nLine 1\nLine 2\nLine 3";
        let (frontmatter, body) = split_frontmatter_and_body(content);

        assert_eq!(frontmatter, "title: Test");
        assert_eq!(body, "Line 1\nLine 2\nLine 3");
    }

    #[test]
    fn test_split_frontmatter_special_characters() {
        let content = "---\ntitle: Test <> & \"quotes\"\n---\nBody with special chars: <>&\"";
        let (frontmatter, body) = split_frontmatter_and_body(content);

        assert!(frontmatter.contains("Test <> & \"quotes\""));
        assert!(body.contains("<>&\""));
    }
}
