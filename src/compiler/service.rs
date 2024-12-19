// Copyright © 2025 Static Data Gen.
// All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Compilation service for static site generation
//!
//! This module provides the core functionality for compiling source files
//! into static website content, including HTML generation, RSS feeds,
//! sitemaps, and various metadata files.

use anyhow::{Context, Result};
use html_generator::{generate_html, HtmlConfig};
use rlg::log_format::LogFormat;
use rlg::log_level::LogLevel;
use rss_gen::{
    data::{RssData, RssItem},
    generate_rss, macro_set_rss_data_fields,
};
use sitemap_gen::create_site_map_data;
use std::time::Duration;

use crate::generators::cname::{CnameConfig, CnameGenerator};
use crate::generators::humans::{HumansConfig, HumansGenerator};
use crate::generators::manifest::{ManifestConfig, ManifestGenerator};
use crate::{
    macro_cleanup_directories, macro_create_directories,
    macro_log_info, macro_metadata_option,
    models::data::{FileData, PageData},
    modules::{
        json::{news_sitemap, security, sitemap, txt},
        navigation::NavigationGenerator,
        news_sitemap::create_news_site_map_data,
        robots::create_txt_data,
        security::create_security_data,
        tags::*,
    },
    utilities::{file::add, write::write_files_to_build_directory},
};
use metadata_gen::extract_and_prepare_metadata;
use staticweaver::{Context as TemplateContext, Engine, PageOptions};
use std::{collections::HashMap, fs, path::Path};

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

    let html_content = generate_html(&file.content, &config)
        .context("Failed to generate HTML content")?;

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

    // let json = create_manifest_data(&metadata);

    let manifest_content = ManifestConfig::from_metadata(&metadata)
        .and_then(|config| ManifestGenerator::new(config).generate())
        .unwrap_or_else(|e| {
            eprintln!("Error generating manifest: {}", e);
            String::new()
        });

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
    let news_sitemap_options = create_news_site_map_data(&metadata);
    let tags_data = generate_tags(file, &metadata);

    update_global_tags_data(global_tags_data, &tags_data);

    let txt_options = create_txt_data(&metadata);

    let txt_data = txt(&txt_options);
    // let human_data = human(&human_options);
    let security_data = security(&security_options);
    let sitemap_data = sitemap(sitemap_options?, site_path);
    let news_sitemap_data = news_sitemap(news_sitemap_options);
    // let json_data = serde_json::to_string(&manifest).unwrap_or_else(|e| {
    //     eprintln!("Error serializing JSON: {}", e);
    //     String::new()
    // });

    Ok(FileData {
        cname: cname_content,
        content,
        keyword: keywords.join(", "),
        human: humans_content,
        manifest: manifest_content,
        name: file.name.clone(),
        rss,
        security: security_data,
        sitemap: sitemap_data,
        sitemap_news: news_sitemap_data,
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
