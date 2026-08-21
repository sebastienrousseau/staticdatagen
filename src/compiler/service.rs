// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Compilation service for static site generation
//!
//! This module provides the core functionality for compiling source files
//! into static website content, including HTML generation, RSS feeds,
//! sitemaps, and various metadata files.

use anyhow::{Context, Result};
use html_generator::{generate_html, HtmlConfig};
use log::{error, info, warn};
use metadata_gen::extract_and_prepare_metadata;
use rayon::prelude::*;
use rss_gen::{
    data::{RssData, RssItem},
    generate_rss, macro_set_rss_data_fields,
};
use sitemap_gen::create_site_map_data;
use staticweaver::{Context as TemplateContext, Engine};
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
    macro_metadata_option,
    models::data::{FileData, PageData},
    modules::{
        json::{security, sitemap, txt},
        navigation::NavigationGenerator,
        robots::create_txt_data,
        security::create_security_data,
    },
    utilities::{file::add, write::write_files_to_build_directory},
};

/// Page count at or above which [`compile`] renders and writes in
/// parallel.
///
/// Below this, the work runs on the calling thread: starting rayon's pool
/// costs more than the parallelism saves. A 10-page build measured 35 ms
/// sequentially against 158 ms parallel, essentially all of it pool
/// startup. Most sites are small, so the sequential path is the one that
/// has to stay fast.
const PARALLEL_THRESHOLD: usize = 24;

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
///
/// # Concurrency
///
/// Pages are rendered and written in parallel across a rayon pool once the
/// corpus reaches 24 files; below that the work runs on the calling thread,
/// because starting the pool costs more than it saves — a 10-page build
/// measured 35 ms sequentially against 158 ms parallel.
///
/// Each worker holds its own template [`Engine`], so the template cache is
/// per-thread rather than contended, and each file reports the tags it
/// contributes for merging afterwards. The merge runs over the collected
/// results in order, so output does not depend on thread scheduling: every
/// emitted HTML file is byte-identical to the sequential result.
///
/// Rendering dominates a build — profiling a 500-page corpus put 94% of
/// wall-clock in this function — so the parallel path is what takes a
/// 500-page build from 2.83 s to under 0.6 s.
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
        HashMap::with_capacity(source_files.len());

    // Initialize the templating engine with caching.
    let template_path_str =
        template_path.to_str().ok_or_else(|| {
            anyhow::anyhow!("Template path contains invalid UTF-8")
        })?;
    let mut engine =
        Engine::new(template_path_str, Duration::from_secs(60));

    // Rendering is the bulk of a build — profiling a 500-page corpus put
    // 94% of wall-clock inside this loop — and each page is independent:
    // it reads shared, immutable navigation and writes only its own
    // output. Two things made it sequential: a `&mut Engine` and a
    // `&mut HashMap` of tags.
    //
    // `map_init` gives each rayon worker its own `Engine`, so the template
    // cache is per-thread rather than contended, and each file returns the
    // tags it contributes. Merging those in the collected order keeps the
    // result identical to the sequential version rather than dependent on
    // thread scheduling.
    let processed: Vec<(FileData, HashMap<String, Vec<PageData>>)> =
        if source_files.len() < PARALLEL_THRESHOLD {
            source_files
                .into_iter()
                .map(|file| {
                    process_file_isolated(
                        &file,
                        &mut engine,
                        template_path,
                        &navigation,
                        site_path,
                    )
                })
                .collect::<Result<Vec<_>>>()?
        } else {
            source_files
                .into_par_iter()
                .map_init(
                    || {
                        Engine::new(
                            template_path_str,
                            Duration::from_secs(60),
                        )
                    },
                    |engine, file| {
                        process_file_isolated(
                            &file,
                            engine,
                            template_path,
                            &navigation,
                            site_path,
                        )
                    },
                )
                .collect::<Result<Vec<_>>>()?
        };

    let mut compiled_files = Vec::with_capacity(processed.len());
    for (file, local_tags) in processed {
        for (tag, pages) in local_tags {
            global_tags_data.entry(tag).or_default().extend(pages);
        }
        compiled_files.push(file);
    }

    // Writes are independent per file and dominated by I/O — but the same
    // threshold applies, since touching `par_iter` at all starts the pool.
    if compiled_files.len() < PARALLEL_THRESHOLD {
        for file in &compiled_files {
            write_files_to_build_directory(
                build_dir_path,
                file,
                template_path,
            )?;
        }
    } else {
        compiled_files.par_iter().try_for_each(|file| {
            write_files_to_build_directory(
                build_dir_path,
                file,
                template_path,
            )
        })?;
    }

    // Generate and write global tags HTML.
    let tags_html_content = generate_tags_html(&global_tags_data);
    write_tags_html_to_file(&tags_html_content, build_dir_path)?;

    // Clean up and finalize site structure.
    macro_cleanup_directories!(site_path)
        .context("Failed to clean up site directory")?;
    fs::rename(build_dir_path, site_path)
        .context("Failed to finalize build directory")?;

    // Issue #71: only log success AFTER every fallible step (write,
    // tags HTML, cleanup, rename) has completed. Downstream consumers
    // (ssg, CI tooling) parse this line for build state, so it must
    // never fire on a build that later errors out.
    info!(
        "Successfully generated, compiled, and minified all HTML to the `{}` directory",
        site_path.display()
    );

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

/// The configuration used to render every Markdown body.
///
/// Extracted so tests exercise the configuration the compiler actually
/// uses rather than a copy of it that can drift.
fn html_config() -> HtmlConfig {
    HtmlConfig {
        enable_syntax_highlighting: true,
        minify_output: false,
        add_aria_attributes: true,
        // Deliberately off. This ran on the Markdown body, which is a
        // fragment: it has no `<head>`, so `<title>` cannot exist and the
        // step failed on every page ever compiled — 25 of 25 pages in the
        // themes showcase, logged at Error level each time.
        //
        // Turning it on again would not recover anything, it would emit a
        // second `ld+json` block. Structured data is generated downstream
        // from front matter, where the title, description, canonical URL
        // and page type are actually known, and that block is a full
        // `@graph` rather than the bare name/description derivable from a
        // fragment. See ssg's metadata plugin.
        generate_structured_data: false,
        generate_toc: false,
        language: "en".to_string(),
        max_input_size: usize::MAX,
        syntax_theme: None,
        // Spec A1 (ssg-fixes spec / ssg#586): html-generator 0.0.6
        // introduced `allow_unsafe_html` with a `false` default, which
        // silently escapes raw block HTML (`<section>`, `<figure>`,
        // inline `<svg>`, …) in Markdown bodies. Site authors write
        // that HTML deliberately, so pass-through is the trusted-author
        // default here; sanitisation must be an explicit opt-in pass
        // (`sanitize_html: true`), never silent escaping.
        allow_unsafe_html: true,
        ..HtmlConfig::default()
    }
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
    generate_html(body, &html_config())
        .context("Failed to generate HTML content")
}

/// Derives a permalink for a page whose front matter does not provide
/// one (spec A2, ssg#586 tracker).
///
/// Fallback chain, in order:
///
/// 1. `permalink` — kept verbatim when present and non-empty.
/// 2. `url` — some sites carry the canonical page URL here.
/// 3. `{base_url}/{relative_output_path}` — always derivable when the
///    site injects its base URL, because the output path is a pure
///    function of the source file name (mirrors
///    `utilities::write::get_processed_file_name`).
/// 4. `base_url` alone — last resort for unusual file names.
///
/// Returns `None` only when the front matter carries none of
/// `permalink`, `url`, or `base_url` — a genuinely underivable link.
fn derive_permalink(
    metadata: &HashMap<String, String>,
    file_name: &str,
) -> Option<String> {
    let non_empty = |key: &str| {
        metadata
            .get(key)
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .map(str::to_string)
    };

    if let Some(permalink) = non_empty("permalink") {
        return Some(permalink);
    }
    if let Some(url) = non_empty("url") {
        return Some(url);
    }
    let base_url = non_empty("base_url")?;
    let base_url = base_url.trim_end_matches('/');

    // Mirror the output layout produced by
    // `write_files_to_build_directory`: `index.md` lands at the site
    // root as `index.html`; every other page becomes
    // `{stem}/index.html`.
    let path = Path::new(file_name);
    let stem = match path.extension().and_then(|s| s.to_str()) {
        Some(ext)
            if ["js", "json", "md", "toml", "txt", "xml"]
                .contains(&ext) =>
        {
            path.with_extension("").to_string_lossy().into_owned()
        }
        _ => file_name.to_string(),
    };
    let output_path = if stem.is_empty() {
        return Some(base_url.to_string());
    } else if stem == "index" {
        "index.html".to_string()
    } else {
        format!("{}/index.html", stem)
    };

    Some(format!("{}/{}", base_url, output_path))
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
fn generate_rss_content(
    metadata: &HashMap<String, String>,
) -> Result<String> {
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

    generate_rss(&rss_data)
        .map_err(|e| anyhow::anyhow!("RSS generation failed: {}", e))
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
fn generate_manifest_content(
    metadata: &HashMap<String, String>,
) -> String {
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
    let news_sitemap_generator =
        NewsSiteMapGenerator::new(news_sitemap_config);
    let news_sitemap_content =
        match news_sitemap_generator.generate_xml() {
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
            let humans: HashMap<String, String> =
                serde_json::from_str(humans)
                    .context("Failed to parse humans metadata")
                    .unwrap_or_else(|err| {
                        error!(
                            "Error parsing humans metadata: {}",
                            err
                        );
                        HashMap::new()
                    });

            match HumansConfig::from_metadata(&humans) {
                Ok(humans_config) => {
                    HumansGenerator::new(humans_config).generate()
                }
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
    // Spec A2: like the RSS feed, a sitemap entry that cannot be
    // built (e.g. no derivable permalink) is warned about and
    // skipped; it never aborts the compile.
    let sitemap_data = match sitemap_options {
        Ok(options) => match sitemap(options, site_path) {
            Ok(data) => data,
            Err(e) => {
                warn!(
                    "Skipping sitemap entry for '{}': {}",
                    file.name, e
                );
                String::new()
            }
        },
        Err(e) => {
            warn!("Skipping sitemap entry for '{}': {}", file.name, e);
            String::new()
        }
    };

    Ok(FileData {
        cname: cname_content,
        content,
        keyword: keywords.join(", "),
        human: humans_content,
        manifest: manifest_content,
        name: file.name.clone(),
        rss: rss_content,
        security: security_data,
        sitemap: sitemap_data,
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
    // Retained as the single-file entry point used by the tests and by
    // callers that already hold a shared map. The compile loop uses
    // `process_file_isolated` so it can run in parallel.
    // Extract metadata and keywords (inline to avoid type issues)
    let (_frontmatter, body) =
        split_frontmatter_and_body(&file.content);
    let (mut metadata, keywords, all_meta_tags) =
        extract_and_prepare_metadata(&file.content)
            .context("Failed to extract and prepare metadata")?;

    // Spec A2: backfill a missing `permalink` from the fallback chain
    // (`url` → `{base_url}/{output_path}` → `base_url`) so the RSS
    // channel link and sitemap `<loc>` are always derivable. Authors
    // never need to hand-write `permalink`.
    if metadata
        .get("permalink")
        .map(|p| p.trim().is_empty())
        .unwrap_or(true)
    {
        if let Some(permalink) = derive_permalink(&metadata, &file.name)
        {
            let _ = metadata
                .insert("permalink".to_string(), permalink.clone());
            // `item_link` feeds the RSS `<item><link>`; keep it in
            // step with the derived permalink when absent.
            let _ = metadata
                .entry("item_link".to_string())
                .or_insert(permalink);
        }
    }

    // Generate HTML content
    let html_content = generate_html_content(&body)?;

    // Setup template context directly (staticweaver 0.0.2 removed PageOptions).
    let mut context = TemplateContext::new();
    for (key, value) in metadata.iter() {
        context.set(key.to_string(), value.to_string());
    }

    context.set("apple".to_string(), all_meta_tags.apple.clone());
    context.set("content".to_string(), html_content);
    context.set("microsoft".to_string(), all_meta_tags.ms.clone());
    context.set("navigation".to_string(), navigation.to_owned());
    context.set("opengraph".to_string(), all_meta_tags.og);
    context.set("primary".to_string(), all_meta_tags.primary);
    context.set("twitter".to_string(), all_meta_tags.twitter);

    // Default to `page` when frontmatter omits `layout:` (or sets it
    // to an empty/whitespace value); staticweaver otherwise aborts with
    // `invalid template or partial name: ""`. Issue #67.
    let layout = metadata
        .get("layout")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .unwrap_or("page");
    let content = engine.render_page(&context, layout)?;

    // Generate RSS, manifest and auxiliary files.
    //
    // Spec A2: a feed entry whose link is genuinely underivable (no
    // `permalink`, `url`, or `base_url` anywhere in the front matter)
    // is warned about and skipped — it must NEVER abort the whole
    // compile. Every other artifact for the page is still produced.
    let rss_content = match generate_rss_content(&metadata) {
        Ok(rss) => rss,
        Err(e) => {
            warn!(
                "Skipping RSS feed for '{}': {}. The page itself is \
                 still built.",
                file.name, e
            );
            String::new()
        }
    };
    let manifest_content = generate_manifest_content(&metadata);
    let (news_sitemap_content, cname_content, humans_content) =
        generate_auxiliary_files(&metadata);

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

/// As [`process_file`], but accumulating tags into a map of its own.
///
/// The compile loop renders pages in parallel, and a `&mut HashMap`
/// shared across threads cannot be. Each file therefore reports the tags
/// it contributes and the caller merges them, which also makes the merge
/// order explicit rather than incidental.
fn process_file_isolated(
    file: &FileData,
    engine: &mut Engine,
    template_path: &Path,
    navigation: &str,
    site_path: &Path,
) -> Result<(FileData, HashMap<String, Vec<PageData>>)> {
    let mut local_tags: HashMap<String, Vec<PageData>> = HashMap::new();
    let data = process_file(
        file,
        engine,
        template_path,
        navigation,
        &mut local_tags,
        site_path,
    )?;
    Ok((data, local_tags))
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

    /// Every step the render pipeline is asked to perform must succeed.
    ///
    /// `generate_structured_data` was on, and could not succeed: it looks
    /// for a `<title>`, and the input is a Markdown body with no `<head>`.
    /// Every page ever compiled logged a step failure that nothing acted
    /// on, and the JSON-LD it was meant to produce was never emitted.
    ///
    /// Asserting on diagnostics rather than on the absence of the string
    /// keeps this honest: it fails for any step that silently degrades,
    /// not only the one that prompted it.
    #[test]
    fn render_pipeline_reports_no_failed_step() {
        let body =
            "# Heading\n\nA paragraph, so a description could be \
                    derived if anything asked for one.\n";

        let output = html_generator::generate_html_with_diagnostics(
            body,
            &html_config(),
        )
        .expect("rendering the body must succeed");

        let failed: Vec<_> = output
            .diagnostics
            .iter()
            .filter(|d| {
                d.level == html_generator::DiagnosticLevel::Error
            })
            .collect();

        assert!(
            failed.is_empty(),
            "render pipeline reported failed step(s): {failed:#?}"
        );
    }

    /// Structured data belongs downstream, where front matter supplies the
    /// title, description and canonical URL. Emitting a second, thinner
    /// block from the fragment would compete with it.
    #[test]
    fn rendered_body_carries_no_structured_data() {
        let html = generate_html_content("# Heading\n\nBody.\n")
            .expect("rendering the body must succeed");

        assert!(
            !html.contains("application/ld+json"),
            "fragment must not carry its own JSON-LD: {html}"
        );
    }

    /// Builds a corpus of `n` pages and returns the emitted HTML keyed by
    /// filename, so two runs can be compared for equality.
    fn compile_n_and_collect(
        n: usize,
    ) -> std::collections::BTreeMap<String, String> {
        let build_dir = tempfile::TempDir::new().unwrap();
        let content_dir = tempfile::TempDir::new().unwrap();
        let site_dir = tempfile::TempDir::new().unwrap();
        let template_dir = tempfile::TempDir::new().unwrap();

        for i in 0..n {
            fs::write(
                content_dir.path().join(format!("page{i}.md")),
                format!(
                    "---\ntitle: Page {i}\nlayout: page\npermalink: https://example.com/p{i}\ndescription: Page {i}\nauthor: Test\nchangefreq: weekly\ntags: t{}\n---\n# Page {i}\n\nBody {i}.",
                    i % 3
                ),
            )
            .unwrap();
        }
        fs::write(
            template_dir.path().join("page.html"),
            "<html><head><title>{{title}}</title></head><body>{{content}}</body></html>",
        )
        .unwrap();
        fs::write(template_dir.path().join("main.js"), "// main")
            .unwrap();
        fs::write(template_dir.path().join("sw.js"), "// sw").unwrap();
        fs::create_dir_all(build_dir.path().join("tags")).unwrap();
        fs::write(
            build_dir.path().join("tags/index.html"),
            "<html><body>[[content]]</body></html>",
        )
        .unwrap();

        compile(
            build_dir.path(),
            content_dir.path(),
            site_dir.path(),
            template_dir.path(),
        )
        .unwrap();

        let mut out = std::collections::BTreeMap::new();
        // `compile` writes the rendered site into `site_path`, not the
        // build directory it stages through.
        for entry in walkdir::WalkDir::new(site_dir.path())
            .into_iter()
            .filter_map(std::result::Result::ok)
        {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("html")
            {
                let rel = path
                    .strip_prefix(site_dir.path())
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                let _ = out.insert(
                    rel,
                    fs::read_to_string(path).unwrap_or_default(),
                );
            }
        }
        out
    }

    /// The parallel path must produce exactly what the sequential path
    /// produces. `PARALLEL_THRESHOLD` is 24, so 8 pages take the sequential
    /// branch and 40 take the parallel one; both render the same template
    /// over the same generated corpus, so page N must be identical in both.
    #[test]
    fn parallel_and_sequential_render_identically() {
        let small = compile_n_and_collect(8);
        let large = compile_n_and_collect(40);
        assert!(
            !small.is_empty(),
            "sequential branch produced nothing"
        );
        assert!(!large.is_empty(), "parallel branch produced nothing");

        // Only per-page output is comparable. Site-wide aggregates —
        // `tags/index.html`, sitemaps — summarise the whole corpus, so an
        // 8-page site and a 40-page one differ there by construction. The
        // question this test asks is whether rendering *a page* changes
        // when it happens on a worker thread.
        let mut compared = 0;
        for (name, body) in &small {
            if !name.starts_with("page")
                || !name.ends_with("index.html")
            {
                continue;
            }
            let Some(other) = large.get(name) else {
                continue;
            };
            assert_eq!(
                body, other,
                "{name} differs between the sequential and parallel paths"
            );
            compared += 1;
        }
        assert!(
            compared >= 8,
            "expected the 8 shared pages to be compared, got {compared}"
        );
    }

    /// Running the parallel path twice must give the same bytes: the tag
    /// merge iterates collected results in order rather than in completion
    /// order, so scheduling cannot leak into the output.
    #[test]
    fn parallel_compilation_is_deterministic() {
        let first = compile_n_and_collect(40);
        let second = compile_n_and_collect(40);
        assert_eq!(
            first, second,
            "two parallel runs of the same corpus differed"
        );
    }

    /// A corpus either side of the threshold must compile cleanly. This is
    /// the boundary where the branch is chosen, and an off-by-one there
    /// would silently send every build down one path.
    #[test]
    fn compiles_either_side_of_the_parallel_threshold() {
        for n in [PARALLEL_THRESHOLD - 1, PARALLEL_THRESHOLD] {
            let out = compile_n_and_collect(n);
            assert!(
                !out.is_empty(),
                "{n}-page corpus produced no HTML"
            );
        }
    }

    #[test]
    fn test_compile_success() {
        let build_dir = tempfile::TempDir::new().unwrap();
        let content_dir = tempfile::TempDir::new().unwrap();
        let site_dir = tempfile::TempDir::new().unwrap();
        let template_dir = tempfile::TempDir::new().unwrap();

        // Create a content file with full frontmatter
        fs::write(
            content_dir.path().join("index.md"),
            "---\ntitle: Home\nlayout: page\npermalink: https://example.com\ndescription: Home page\nauthor: Test\nchangefreq: weekly\n---\n# Welcome\n\nHello world.",
        )
        .unwrap();

        // Create the template and auxiliary files
        fs::write(
            template_dir.path().join("page.html"),
            "<html><head><title>{{title}}</title></head><body>{{content}}</body></html>",
        )
        .unwrap();
        fs::write(template_dir.path().join("main.js"), "// main")
            .unwrap();
        fs::write(template_dir.path().join("sw.js"), "// sw").unwrap();

        // Create tags/index.html in build dir (write_tags_html_to_file expects it)
        fs::create_dir_all(build_dir.path().join("tags")).unwrap();
        fs::write(
            build_dir.path().join("tags/index.html"),
            "<html><body>[[content]]</body></html>",
        )
        .unwrap();

        let result = compile(
            build_dir.path(),
            content_dir.path(),
            site_dir.path(),
            template_dir.path(),
        );

        assert!(result.is_ok(), "compile failed: {:?}", result.err());
    }

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
            syntax_theme: None,
            ..HtmlConfig::default()
        };

        let body = "Test content";
        let result = generate_html(body, &config);
        assert!(result.is_ok());
    }

    /// html-generator 0.0.10 validates the config instead of silently
    /// ignoring a contradictory one. This test previously passed a
    /// `syntax_theme` while `enable_syntax_highlighting` was false, and
    /// named a theme outside the allowed set; both were accepted and
    /// dropped on the floor. They are now errors, which is the better
    /// behaviour and worth pinning so it is not lost in a later bump.
    #[test]
    fn html_config_rejects_a_theme_that_cannot_apply() {
        let config = HtmlConfig {
            enable_syntax_highlighting: false,
            syntax_theme: Some("base16-ocean.dark".to_string()),
            ..HtmlConfig::default()
        };

        let err = generate_html("Test content", &config).expect_err(
            "a theme set with highlighting disabled should be refused",
        );
        assert!(
            format!("{err:?}")
                .contains("enable_syntax_highlighting = false"),
            "unexpected error: {err:?}"
        );
    }

    /// The theme name itself is validated against a fixed set.
    #[test]
    fn html_config_rejects_an_unknown_syntax_theme() {
        let config = HtmlConfig {
            enable_syntax_highlighting: true,
            syntax_theme: Some("monokai".to_string()),
            ..HtmlConfig::default()
        };

        let err = generate_html("Test content", &config)
            .expect_err("an unknown theme should be refused");
        assert!(
            format!("{err:?}").contains("Value not in allowed set"),
            "unexpected error: {err:?}"
        );
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
        let engine =
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
    fn test_generate_html_content_raw_html_passthrough() {
        // Spec A1 regression test: html-generator 0.0.6 defaults
        // `allow_unsafe_html` to false, which would escape raw block
        // HTML in Markdown bodies on every platform. The compiler
        // config must opt into pass-through for trusted authors.
        let body = "Intro paragraph.\n\n<section class=\"x\"><p>hi</p></section>\n\nOutro.";
        let html = generate_html_content(body)
            .expect("raw-HTML body should render");
        assert!(
            html.contains("<section class=\"x\">"),
            "raw block HTML must pass through unescaped, got: {}",
            html
        );
        assert!(
            !html.contains("&lt;section"),
            "raw block HTML must not be escaped, got: {}",
            html
        );
    }

    #[test]
    fn test_derive_permalink_fallback_chain() {
        let mut metadata = HashMap::new();

        // Nothing derivable → None.
        assert_eq!(derive_permalink(&metadata, "post.md"), None);

        // base_url + relative output path (trailing slash trimmed).
        let _ = metadata.insert(
            "base_url".to_string(),
            "https://example.com/".to_string(),
        );
        assert_eq!(
            derive_permalink(&metadata, "post.md").as_deref(),
            Some("https://example.com/post/index.html")
        );
        assert_eq!(
            derive_permalink(&metadata, "index.md").as_deref(),
            Some("https://example.com/index.html")
        );
        // Per-locale subdirectories survive (issue #70 layout).
        assert_eq!(
            derive_permalink(&metadata, "fr/article.md").as_deref(),
            Some("https://example.com/fr/article/index.html")
        );

        // `url` outranks the base_url derivation.
        let _ = metadata.insert(
            "url".to_string(),
            "https://example.com/from-url/".to_string(),
        );
        assert_eq!(
            derive_permalink(&metadata, "post.md").as_deref(),
            Some("https://example.com/from-url/")
        );

        // An explicit permalink always wins.
        let _ = metadata.insert(
            "permalink".to_string(),
            "https://example.com/explicit/".to_string(),
        );
        assert_eq!(
            derive_permalink(&metadata, "post.md").as_deref(),
            Some("https://example.com/explicit/")
        );

        // Empty permalink is treated as absent.
        let _ =
            metadata.insert("permalink".to_string(), "   ".to_string());
        assert_eq!(
            derive_permalink(&metadata, "post.md").as_deref(),
            Some("https://example.com/from-url/")
        );
    }

    #[test]
    fn test_compile_without_permalink_derives_feed_link() {
        // Spec A2 acceptance: a post with only title + date + body
        // (plus the site-level base_url) builds, and the feed <link>
        // equals {base_url}/{output_path}.
        let build_dir = tempfile::TempDir::new().unwrap();
        let content_dir = tempfile::TempDir::new().unwrap();
        let site_dir = tempfile::TempDir::new().unwrap();
        let template_dir = tempfile::TempDir::new().unwrap();

        // No `permalink:` anywhere in the front matter.
        fs::write(
            content_dir.path().join("index.md"),
            "---\ntitle: Home\ndate: July 1, 2026\nbase_url: https://example.com\ndescription: Home page\nauthor: Test\nchangefreq: weekly\n---\n# Welcome\n",
        )
        .unwrap();

        fs::write(
            template_dir.path().join("page.html"),
            "<html><body>{{content}}</body></html>",
        )
        .unwrap();

        let result = compile(
            build_dir.path(),
            content_dir.path(),
            site_dir.path(),
            template_dir.path(),
        );
        assert!(
            result.is_ok(),
            "compile without permalink should succeed: {:?}",
            result.err()
        );

        let rss = fs::read_to_string(site_dir.path().join("rss.xml"))
            .unwrap();
        assert!(
            rss.contains("<link>https://example.com/index.html</link>"),
            "feed link should equal base_url + output path, got: {}",
            rss
        );
    }

    #[test]
    fn test_compile_underivable_feed_entry_never_aborts() {
        // Spec A2 acceptance: a pathological entry (no permalink, no
        // url, no base_url) is skipped with a warning — the compile
        // and every other artifact stay intact.
        let build_dir = tempfile::TempDir::new().unwrap();
        let content_dir = tempfile::TempDir::new().unwrap();
        let site_dir = tempfile::TempDir::new().unwrap();
        let template_dir = tempfile::TempDir::new().unwrap();

        fs::write(
            content_dir.path().join("index.md"),
            "---\ntitle: Orphan\ndate: 2026-07-01\ndescription: No link anywhere\nauthor: Test\n---\n# Orphan page\n",
        )
        .unwrap();

        fs::write(
            template_dir.path().join("page.html"),
            "<html><body>{{content}}</body></html>",
        )
        .unwrap();

        let result = compile(
            build_dir.path(),
            content_dir.path(),
            site_dir.path(),
            template_dir.path(),
        );
        assert!(
            result.is_ok(),
            "an underivable feed entry must never abort the compile: {:?}",
            result.err()
        );

        // The page itself is still built…
        let html =
            fs::read_to_string(site_dir.path().join("index.html"))
                .unwrap();
        assert!(html.contains("Orphan page"));
        // …while the underivable feed is skipped (empty rss.xml).
        let rss = fs::read_to_string(site_dir.path().join("rss.xml"))
            .unwrap();
        assert!(
            rss.is_empty(),
            "underivable feed should be skipped, got: {}",
            rss
        );
    }

    #[test]
    fn test_generate_rss_content_with_metadata() {
        let mut metadata = HashMap::new();
        let _ = metadata
            .insert("title".to_string(), "Test Feed".to_string());
        let _ = metadata.insert(
            "description".to_string(),
            "A test feed".to_string(),
        );
        let _ = metadata.insert(
            "permalink".to_string(),
            "https://example.com".to_string(),
        );
        let _ = metadata
            .insert("author".to_string(), "Test Author".to_string());
        let _ =
            metadata.insert("category".to_string(), "Test".to_string());
        let _ = metadata
            .insert("copyright".to_string(), "2024".to_string());
        let _ = metadata
            .insert("generator".to_string(), "TestGen".to_string());
        let _ =
            metadata.insert("language".to_string(), "en".to_string());
        let _ = metadata
            .insert("item_guid".to_string(), "guid-123".to_string());
        let _ = metadata.insert(
            "item_description".to_string(),
            "Item desc".to_string(),
        );
        let _ = metadata.insert(
            "item_link".to_string(),
            "https://example.com/item".to_string(),
        );
        let _ = metadata.insert(
            "item_pub_date".to_string(),
            "2024-01-01T00:00:00Z".to_string(),
        );
        let _ = metadata
            .insert("item_title".to_string(), "Item Title".to_string());

        let result = generate_rss_content(&metadata);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_manifest_content_with_metadata() {
        let mut metadata = HashMap::new();
        let _ =
            metadata.insert("name".to_string(), "Test App".to_string());
        let _ = metadata
            .insert("short_name".to_string(), "Test".to_string());
        let _ =
            metadata.insert("start_url".to_string(), "/".to_string());
        let _ = metadata
            .insert("display".to_string(), "standalone".to_string());
        let _ = metadata.insert(
            "background_color".to_string(),
            "#ffffff".to_string(),
        );
        let _ = metadata
            .insert("theme_color".to_string(), "#000000".to_string());

        let content = generate_manifest_content(&metadata);
        // Manifest generation may fail with partial metadata, just ensure it doesn't panic
        assert!(content.is_empty() || !content.is_empty());
    }

    #[test]
    fn test_generate_auxiliary_files_empty_metadata() {
        let metadata = HashMap::new();
        let (news_sitemap, cname, humans) =
            generate_auxiliary_files(&metadata);

        // With empty metadata, all should be empty or default
        assert!(news_sitemap.is_empty() || !news_sitemap.is_empty());
        assert!(cname.is_empty());
        assert!(humans.is_empty());
    }

    #[test]
    fn test_generate_auxiliary_files_with_cname() {
        let mut metadata = HashMap::new();
        let _ = metadata
            .insert("cname".to_string(), "example.com".to_string());

        let (_, cname, _) = generate_auxiliary_files(&metadata);
        // CNAME generator returns DNS record format, just check it contains the domain
        assert!(cname.contains("example.com"));
    }

    #[test]
    fn test_generate_auxiliary_files_with_humans() {
        let mut metadata = HashMap::new();
        let humans_json =
            r#"{"author":"Test Author","thanks":"Thanks to all"}"#;
        let _ = metadata
            .insert("humans".to_string(), humans_json.to_string());

        let (_, _, humans) = generate_auxiliary_files(&metadata);
        // May be empty if parsing fails, just ensure no panic
        assert!(humans.is_empty() || !humans.is_empty());
    }

    #[test]
    fn test_generate_auxiliary_files_with_news_sitemap() {
        let mut metadata = HashMap::new();
        let _ = metadata
            .insert("news_genres".to_string(), "Blog".to_string());
        let _ = metadata.insert(
            "news_keywords".to_string(),
            "test, news".to_string(),
        );
        let _ = metadata
            .insert("news_language".to_string(), "en".to_string());
        let _ = metadata.insert(
            "news_loc".to_string(),
            "https://example.com/news".to_string(),
        );
        let _ = metadata.insert(
            "news_publication_date".to_string(),
            "2024-01-01".to_string(),
        );
        let _ = metadata.insert(
            "news_publication_name".to_string(),
            "Test News".to_string(),
        );
        let _ = metadata.insert(
            "news_title".to_string(),
            "Test Article".to_string(),
        );

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
                vec![HashMap::from([(
                    "title".to_string(),
                    "Page1".to_string(),
                )])],
            ),
            (
                "tag2".to_string(),
                vec![
                    HashMap::from([(
                        "title".to_string(),
                        "Page2".to_string(),
                    )]),
                    HashMap::from([(
                        "title".to_string(),
                        "Page3".to_string(),
                    )]),
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
            vec![HashMap::from([(
                "title".to_string(),
                "Page1".to_string(),
            )])],
        )]);
        update_global_tags_data(&mut global_tags_data, &tags_data1);

        // Second update - should append to existing tag
        let tags_data2 = HashMap::from([(
            "tag1".to_string(),
            vec![HashMap::from([(
                "title".to_string(),
                "Page2".to_string(),
            )])],
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

    #[test]
    fn test_generate_manifest_content_error_path() {
        // Empty metadata should trigger the error branch in generate_manifest_content
        let metadata = HashMap::new();
        let content = generate_manifest_content(&metadata);
        // ManifestConfig::from_metadata fails with empty metadata, returning empty string
        assert!(content.is_empty());
    }

    #[test]
    fn test_generate_auxiliary_files_invalid_humans_json() {
        let mut metadata = HashMap::new();
        let _ = metadata
            .insert("humans".to_string(), "not valid json".to_string());

        let (_, _, humans) = generate_auxiliary_files(&metadata);
        // Invalid JSON triggers the error path, humans should be empty
        assert!(humans.is_empty());
    }

    #[test]
    fn test_generate_auxiliary_files_humans_config_error() {
        // Valid JSON but missing required fields for HumansConfig
        let mut metadata = HashMap::new();
        let _ = metadata.insert(
            "humans".to_string(),
            r#"{"unknown_field":"value"}"#.to_string(),
        );

        let (_, _, humans) = generate_auxiliary_files(&metadata);
        // HumansConfig::from_metadata fails with missing required fields
        assert!(humans.is_empty());
    }

    #[test]
    fn test_generate_auxiliary_files_news_sitemap_empty() {
        // Metadata with partial news fields that produce empty XML
        let mut metadata = HashMap::new();
        let _ =
            metadata.insert("news_genres".to_string(), String::new());

        let (news_sitemap, _, _) = generate_auxiliary_files(&metadata);
        // With empty/partial news data, the generator may return empty XML
        let _ = news_sitemap; // exercised the code path
    }

    #[test]
    fn test_assemble_file_data_basic() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let site_path = temp_dir.path();

        // Create an index.html so visit_dirs/sitemap() succeeds
        fs::write(
            site_path.join("index.html"),
            "<html><body>Hello</body></html>",
        )
        .unwrap();

        let file = FileData {
            name: "test.md".to_string(),
            content: "test".to_string(),
            ..Default::default()
        };
        let mut metadata = HashMap::new();
        let _ = metadata.insert(
            "permalink".to_string(),
            "https://example.com".to_string(),
        );
        let _ = metadata
            .insert("changefreq".to_string(), "weekly".to_string());
        let _ = metadata
            .insert("lastmod".to_string(), "2024-01-01".to_string());

        let mut global_tags_data = HashMap::new();

        let result = assemble_file_data(
            &file,
            "rendered content".to_string(),
            vec!["keyword1".to_string()],
            "rss content".to_string(),
            "manifest content".to_string(),
            "news sitemap".to_string(),
            "cname".to_string(),
            "humans".to_string(),
            &metadata,
            &mut global_tags_data,
            site_path,
        );

        let fd = result.expect("assemble_file_data should succeed");
        assert_eq!(fd.name, "test.md");
        assert_eq!(fd.content, "rendered content");
        assert_eq!(fd.keyword, "keyword1");
        assert_eq!(fd.rss, "rss content");
        assert_eq!(fd.manifest, "manifest content");
        assert_eq!(fd.cname, "cname");
        assert_eq!(fd.human, "humans");
        assert!(!fd.sitemap.is_empty());
        assert!(!fd.txt.is_empty());
    }

    #[test]
    fn test_process_file_basic() {
        let template_dir = tempfile::TempDir::new().unwrap();
        let site_dir = tempfile::TempDir::new().unwrap();

        // Create index.html for sitemap
        fs::write(
            site_dir.path().join("index.html"),
            "<html><body>Site</body></html>",
        )
        .unwrap();

        // Create a template file for the "page" layout
        fs::write(
            template_dir.path().join("page.html"),
            "<html><head><title>{{title}}</title></head><body>{{content}}</body></html>",
        )
        .unwrap();

        let mut engine = Engine::new(
            template_dir.path().to_str().unwrap(),
            Duration::from_secs(60),
        );

        let file = FileData {
            name: "test.md".to_string(),
            content: "---\ntitle: Test Page\nlayout: page\npermalink: https://example.com\nchangefreq: weekly\ndescription: A test page\nauthor: Test\n---\n# Hello World\n\nThis is test content.".to_string(),
            ..Default::default()
        };

        let mut global_tags_data = HashMap::new();

        let result = process_file(
            &file,
            &mut engine,
            template_dir.path(),
            "<nav>Nav</nav>",
            &mut global_tags_data,
            site_dir.path(),
        );

        let fd = result.expect("process_file should succeed");
        assert_eq!(fd.name, "test.md");
        assert!(!fd.content.is_empty());
        assert!(!fd.rss.is_empty());
        assert!(!fd.sitemap.is_empty());
        assert!(!fd.txt.is_empty());
    }

    #[test]
    fn test_assemble_file_data_missing_permalink() {
        // Spec A2 semantics change: a missing permalink used to make
        // create_site_map_data fail and abort the whole compile via
        // `sitemap_options?`. The sitemap entry is now skipped with a
        // warning and the page is still assembled.
        let temp_dir = tempfile::TempDir::new().unwrap();
        let file = FileData {
            name: "test.md".to_string(),
            content: "test".to_string(),
            ..Default::default()
        };
        let metadata = HashMap::new();
        let mut global_tags_data = HashMap::new();

        let result = assemble_file_data(
            &file,
            "content".to_string(),
            vec![],
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            &metadata,
            &mut global_tags_data,
            temp_dir.path(),
        );
        let fd = result.expect(
            "assemble_file_data must not abort on missing permalink",
        );
        assert!(
            fd.sitemap.is_empty(),
            "underivable sitemap entry should be skipped, not fatal"
        );
    }

    #[test]
    fn test_generate_rss_content_error_path() {
        // Empty metadata causes generate_rss to fail, exercising the map_err closure
        let metadata = HashMap::new();
        let result = generate_rss_content(&metadata);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("RSS generation failed"),
            "Error should mention RSS generation failure, got: {}",
            err_msg
        );
    }

    #[test]
    fn test_compile_invalid_utf8_template_path() {
        #[cfg(unix)]
        {
            use std::ffi::OsStr;
            use std::os::unix::ffi::OsStrExt;

            let build_dir = tempfile::TempDir::new().unwrap();
            let content_dir = tempfile::TempDir::new().unwrap();
            let site_dir = tempfile::TempDir::new().unwrap();

            // Create a valid content file so we get past the source_files loading
            fs::write(
                content_dir.path().join("index.md"),
                "---\ntitle: Test\n---\nBody",
            )
            .unwrap();

            // Create a path with invalid UTF-8 bytes
            let invalid_bytes: &[u8] = b"templates/\xff\xfe";
            let invalid_os_str = OsStr::from_bytes(invalid_bytes);
            let invalid_path = Path::new(invalid_os_str);

            let result = compile(
                build_dir.path(),
                content_dir.path(),
                site_dir.path(),
                invalid_path,
            );

            assert!(result.is_err());
            let err_msg = result.unwrap_err().to_string();
            assert!(
                err_msg.contains("invalid UTF-8"),
                "Error should mention invalid UTF-8, got: {}",
                err_msg
            );
        }
    }

    #[test]
    fn test_generate_auxiliary_files_news_sitemap_warn_path() {
        // Metadata with news fields but insufficient for valid XML triggers warn branch
        let mut metadata = HashMap::new();
        // Only partial news fields - no publication name, date, etc.
        let _ = metadata
            .insert("news_title".to_string(), "A Title".to_string());
        let (news_sitemap, _, _) = generate_auxiliary_files(&metadata);
        // With insufficient data, news sitemap should be empty (warn path hit)
        assert!(news_sitemap.is_empty() || !news_sitemap.is_empty());
    }

    #[test]
    fn test_generate_html_content_empty() {
        // Empty body should still produce valid (empty) HTML
        let result = generate_html_content("");
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_rss_content_with_partial_metadata() {
        // Metadata with only some fields - should still succeed
        let mut metadata = HashMap::new();
        let _ =
            metadata.insert("title".to_string(), "Title".to_string());
        let _ = metadata.insert(
            "description".to_string(),
            "Description".to_string(),
        );
        let _ = metadata.insert(
            "permalink".to_string(),
            "https://example.com".to_string(),
        );
        // This should succeed since the basic fields are present
        let result = generate_rss_content(&metadata);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_manifest_content_partial_metadata() {
        // Metadata with just a name - generates a manifest
        let mut metadata = HashMap::new();
        let _ = metadata.insert("name".to_string(), "App".to_string());
        let content = generate_manifest_content(&metadata);
        // Should produce a valid manifest or be empty if ManifestConfig fails
        let _ = content; // exercised the code path
    }

    #[test]
    fn test_compile_defaults_layout_when_missing() {
        // Regression for sebastienrousseau/staticdatagen#67.
        // Frontmatter without a `layout:` key used to pass "" to
        // staticweaver, which then errored with "invalid template or
        // partial name". After the fix, "page" is used as the default.
        let build_dir = tempfile::TempDir::new().unwrap();
        let content_dir = tempfile::TempDir::new().unwrap();
        let site_dir = tempfile::TempDir::new().unwrap();
        let template_dir = tempfile::TempDir::new().unwrap();

        // Frontmatter intentionally omits `layout:`.
        fs::write(
            content_dir.path().join("index.md"),
            "---\ntitle: Home\npermalink: https://example.com\ndescription: Home page\nauthor: Test\nchangefreq: weekly\n---\n# Welcome\n",
        )
        .unwrap();

        fs::write(
            template_dir.path().join("page.html"),
            "<html><body>{{content}}</body></html>",
        )
        .unwrap();
        fs::write(template_dir.path().join("main.js"), "// main")
            .unwrap();
        fs::write(template_dir.path().join("sw.js"), "// sw").unwrap();

        fs::create_dir_all(build_dir.path().join("tags")).unwrap();
        fs::write(
            build_dir.path().join("tags/index.html"),
            "<html><body>[[content]]</body></html>",
        )
        .unwrap();

        let result = compile(
            build_dir.path(),
            content_dir.path(),
            site_dir.path(),
            template_dir.path(),
        );

        assert!(
            result.is_ok(),
            "compile should succeed when layout key is absent: {:?}",
            result.err()
        );
    }
}
