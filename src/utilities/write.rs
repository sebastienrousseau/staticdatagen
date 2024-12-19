// Copyright © 2024 Shokunin Static Site Generator.
// All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! File writing utilities for the static site generator
//!
//! This module provides functionality for writing files to the build directory,
//! including handling various file types, minification, and content generation.
//!
//! # Overview
//!
//! This module provides a set of functions to write a static site's generated
//! content to a specified build directory. It handles:
//! - Writing index and content files
//! - Minifying HTML files (when applicable)
//! - Copying auxiliary files such as scripts and service workers
//! - Printing directory headers for better visibility of the generated structure
//!
//! Functions use `anyhow::Result` for error propagation and leverage `log`
//! statements for debugging and informational messages.

use anyhow::{Context, Result};
use log::{debug, info};
use std::fs::{self, copy, read_dir};
use std::path::Path;
use std::time::Instant;

use crate::models::data::FileData;
use html_generator::performance::minify_html;

/// Constants for auxiliary files that should be copied to the build directory.
const OTHER_FILES: [&str; 2] = ["main.js", "sw.js"];

/// Constants for index and configuration files that should be placed in the root build directory.
const INDEX_FILES: [&str; 9] = [
    "CNAME",
    "humans.txt",
    "index.html",
    "manifest.json",
    "robots.txt",
    "rss.xml",
    "security.txt",
    "sitemap.xml",
    "news-sitemap.xml",
];

/// Writes the files to the build directory.
///
/// This function orchestrates writing either the index files (if the current file
/// is the "index") or content files (otherwise). It also handles copying auxiliary
/// files and printing section headers.
///
/// # Arguments
///
/// * `build_dir_path` - The path to the build directory
/// * `file` - The `FileData` object containing file name, content, and related metadata
/// * `template_path` - The path to the template directory containing auxiliary files
///
/// # Returns
///
/// Returns `Ok(())` if successful, or an error if any operation fails.
///
/// # Example
///
/// ```no_run
/// use std::path::Path;
/// use staticdatagen::models::data::FileData;
/// use staticdatagen::utilities::write::write_files_to_build_directory;
///
/// let build_dir = Path::new("build");
/// let template_dir = Path::new("templates");
/// let file = FileData::default();
///
/// write_files_to_build_directory(build_dir, &file, template_dir)
///     .expect("Failed to write files");
/// ```
pub fn write_files_to_build_directory(
    build_dir_path: &Path,
    file: &FileData,
    template_path: &Path,
) -> Result<()> {
    info!(
        "Starting file write to build directory: {}",
        build_dir_path.display()
    );

    let start_time = Instant::now();
    let file_name = get_processed_file_name(&file.name);
    let index_html_minified = file_name == "index";
    let dir_name = build_dir_path.join(&file_name);

    debug!("Processed file name: '{}'", file_name);
    debug!("Index HTML minification: {}", index_html_minified);

    if file_name == "index" {
        info!("Writing index files...");
        write_index_files(build_dir_path, file, index_html_minified)
            .context("Failed to write index files")?;

        info!("Copying auxiliary files...");
        copy_auxiliary_files(template_path, build_dir_path)
            .context("Failed to copy auxiliary files")?;
    } else {
        info!("Writing content files to '{}'", dir_name.display());
        write_content_files(&dir_name, file, index_html_minified)
            .context("Failed to write content files")?;

        info!("Printing section headers...");
        print_section_headers(&dir_name, start_time)
            .context("Failed to print section headers")?;
    }

    info!(
        "Successfully wrote files to build directory: {}",
        build_dir_path.display()
    );
    Ok(())
}

/// Gets the processed file name without extension for supported file types.
///
/// This function checks if the file name has one of the known extensions
/// (`js`, `json`, `md`, `toml`, `txt`, `xml`) and strips it, returning just the base name.
/// If the extension is not recognized, the original name is returned as-is.
///
/// # Arguments
///
/// * `original_name` - The original file name as a string slice.
///
/// # Returns
///
/// A `String` containing the processed file name without certain extensions.
fn get_processed_file_name(original_name: &str) -> String {
    debug!("Getting processed file name for '{}'", original_name);
    let path = Path::new(original_name);
    match path.extension().and_then(|s| s.to_str()) {
        Some(ext)
            if ["js", "json", "md", "toml", "txt", "xml"]
                .contains(&ext) =>
        {
            let processed = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or(original_name)
                .to_string();
            debug!("Processed file name: '{}'", processed);
            processed
        }
        _ => {
            debug!("No processing needed for '{}'", original_name);
            original_name.to_string()
        }
    }
}

/// Writes content to a file with optional HTML minification.
///
/// If `minify` is `true` and `file_name` is `"index.html"`, the file will be minified after writing.
///
/// # Arguments
///
/// * `dir_path` - Directory path where the file will be written
/// * `file_name` - Name of the file to write
/// * `content` - Content to write to the file
/// * `minify` - Whether to minify HTML content after writing
///
/// # Returns
///
/// Returns `Ok(())` if successful, or an error if writing fails.
fn write_file(
    dir_path: &Path,
    file_name: &str,
    content: &str,
    minify: bool,
) -> Result<()> {
    let file_path = dir_path.join(file_name);
    debug!("Writing file: '{}'", file_path.display());

    fs::write(&file_path, content).with_context(|| {
        format!("Failed to write file at '{}'", file_path.display())
    })?;

    if minify && file_name == "index.html" {
        debug!("Minifying HTML file: '{}'", file_path.display());
        minify_file(&file_path)
            .context("Failed to minify HTML file")?;
    }

    Ok(())
}

/// Minifies an HTML file's content.
///
/// This function reads the file at `file_path`, minifies it using `minify_html`,
/// and then writes the minified content back to the same file.
///
/// # Arguments
///
/// * `file_path` - Path to the HTML file to minify
///
/// # Returns
///
/// Returns `Ok(())` if successful, or an error if minification fails.
fn minify_file(file_path: &Path) -> Result<()> {
    debug!("Minifying file '{}'", file_path.display());
    let minified_content =
        minify_html(file_path).with_context(|| {
            format!(
                "Failed to minify HTML content at '{}'",
                file_path.display()
            )
        })?;

    fs::write(file_path, minified_content).with_context(|| {
        format!(
            "Failed to write minified HTML content at '{}'",
            file_path.display()
        )
    })?;

    debug!("Minification complete for '{}'", file_path.display());
    Ok(())
}

/// Copies a template file from the template directory to the destination directory.
///
/// # Arguments
///
/// * `template_path` - Source template directory path
/// * `dest_dir` - Destination directory path
/// * `file_name` - Name of the file to copy
///
/// # Returns
///
/// Returns `Ok(())` if successful, or an error if copying fails.
fn copy_template_file(
    template_path: &Path,
    dest_dir: &Path,
    file_name: &str,
) -> Result<()> {
    let src_path = template_path.join(file_name);
    let dest_path = dest_dir.join(file_name);
    debug!(
        "Copying template file from '{}' to '{}'",
        src_path.display(),
        dest_path.display()
    );

    let _ = copy(&src_path, &dest_path).with_context(|| {
        format!(
            "Failed to copy template file from '{}' to '{}'",
            src_path.display(),
            dest_path.display()
        )
    })?;

    debug!("Successfully copied file '{}'", file_name);
    Ok(())
}

/// Returns a vector of tuples (file_name, content) from a `FileData` object.
///
/// The files returned here are the standard content files (e.g., `index.html`, `manifest.json`, etc.)
/// that will be written to directories other than the root (except `index.html` which may be root).
///
/// # Arguments
///
/// * `file` - A reference to the `FileData` object
///
/// # Returns
///
/// A vector of tuples `(file_name, content)`.
fn get_file_paths(file: &FileData) -> Vec<(&'static str, &str)> {
    debug!("Retrieving file paths from FileData");
    vec![
        ("index.html", &file.content),
        ("manifest.json", &file.manifest),
        ("robots.txt", &file.txt),
        ("rss.xml", &file.rss),
        ("sitemap.xml", &file.sitemap),
        ("news-sitemap.xml", &file.sitemap_news),
    ]
}

/// Retrieves content from a `FileData` object based on the provided file name.
///
/// If the file name matches one of the known special files (like "CNAME", "index.html", etc.),
/// the corresponding field from `FileData` is returned. Otherwise, an empty string is returned.
///
/// # Arguments
///
/// * `file` - The `FileData` object
/// * `file_name` - The file name for which content is requested
///
/// # Returns
///
/// A `String` containing the content of the requested file.
fn get_file_content(file: &FileData, file_name: &str) -> String {
    match file_name {
        "CNAME" => file.cname.clone(),
        "humans.txt" => file.human.clone(),
        "index.html" => file.content.clone(),
        "manifest.json" => file.manifest.clone(),
        "robots.txt" => file.txt.clone(),
        "rss.xml" => file.rss.clone(),
        "security.txt" => file.security.clone(),
        "sitemap.xml" => file.sitemap.clone(),
        "news-sitemap.xml" => file.sitemap_news.clone(),
        _ => String::new(),
    }
}

/// Writes index files (like `CNAME`, `index.html`, `robots.txt`, etc.) to the build directory.
///
/// This function writes a fixed set of known index files into the root of the build directory.
/// If `index_html_minified` is true, `index.html` will be minified post-write.
///
/// # Arguments
///
/// * `build_dir_path` - Path to the build directory
/// * `file` - The `FileData` object containing content for these files
/// * `index_html_minified` - Whether to minify `index.html` after writing
///
/// # Returns
///
/// `Ok(())` if all index files are written successfully, or an error if any fail.
fn write_index_files(
    build_dir_path: &Path,
    file: &FileData,
    index_html_minified: bool,
) -> Result<()> {
    debug!("Writing index files to '{}'", build_dir_path.display());
    for file_name in &INDEX_FILES {
        debug!("Writing index file: '{}'", file_name);
        write_file(
            build_dir_path,
            file_name,
            &get_file_content(file, file_name),
            index_html_minified,
        )
        .with_context(|| {
            format!(
                "Failed to write file '{}' in '{}'",
                file_name,
                build_dir_path.display()
            )
        })?;
    }
    Ok(())
}

/// Copies auxiliary files (e.g., JavaScript and service worker files) from the template directory
/// to the build directory.
///
/// # Arguments
///
/// * `template_path` - Source template directory path containing the auxiliary files
/// * `build_dir_path` - Path to the build directory where the files should be copied
///
/// # Returns
///
/// `Ok(())` if the auxiliary files are successfully copied, or an error if any fail.
fn copy_auxiliary_files(
    template_path: &Path,
    build_dir_path: &Path,
) -> Result<()> {
    debug!(
        "Copying auxiliary files from '{}' to '{}'",
        template_path.display(),
        build_dir_path.display()
    );
    for file_name in &OTHER_FILES {
        debug!("Copying auxiliary file: '{}'", file_name);
        copy_template_file(template_path, build_dir_path, file_name)
            .with_context(|| {
                format!("Failed to copy auxiliary file '{}'", file_name)
            })?;
    }
    Ok(())
}

/// Writes content files (e.g., `index.html`, `manifest.json`, `robots.txt`) to the specified directory.
///
/// If the directory does not exist, it is created first. If `index_html_minified` is true and
/// `index.html` is one of the files being written, that file will be minified after writing.
///
/// # Arguments
///
/// * `dir_name` - The directory where the content files should be placed
/// * `file` - The `FileData` object containing the file contents
/// * `index_html_minified` - Whether to minify `index.html` after writing
///
/// # Returns
///
/// `Ok(())` if successful, or an error if any file writing operation fails.
fn write_content_files(
    dir_name: &Path,
    file: &FileData,
    index_html_minified: bool,
) -> Result<()> {
    debug!("Creating directory '{}'", dir_name.display());
    fs::create_dir_all(dir_name).with_context(|| {
        format!(
            "Failed to create content directory '{}'",
            dir_name.display()
        )
    })?;

    for (file_name, content) in &get_file_paths(file) {
        debug!("Writing content file: '{}'", file_name);
        write_file(dir_name, file_name, content, index_html_minified)
            .with_context(|| {
            format!(
                "Failed to write content file '{}' in '{}'",
                file_name,
                dir_name.display()
            )
        })?;
    }
    Ok(())
}

/// Prints section headers for a directory and includes timing information.
///
/// This function reads the directory contents, printing out directories in uppercase and files
/// with a `-` prefix. It also prints how long the operation took.
///
/// # Arguments
///
/// * `dir_path` - The directory path for which section headers are printed
/// * `start_time` - The time at which the file-writing operation started, for elapsed time calculation
///
/// # Returns
///
/// `Ok(())` if successful, or an error if directory reading fails.
fn print_section_headers(
    dir_path: &Path,
    start_time: Instant,
) -> Result<()> {
    debug!(
        "Reading directory to print section headers: '{}'",
        dir_path.display()
    );
    let mut section_headers = Vec::new();

    for entry in read_dir(dir_path)
        .with_context(|| {
            format!("Failed to read directory '{}'", dir_path.display())
        })?
        .flatten()
    {
        let path = entry.path();
        if let Some(file_name) =
            path.file_name().and_then(|s| s.to_str())
        {
            let header = if path.is_dir() {
                file_name.to_uppercase()
            } else {
                format!("  - {}", file_name)
            };
            section_headers.push(header);
        }
    }

    section_headers.sort();

    let file_name =
        dir_path.file_name().and_then(|s| s.to_str()).unwrap_or("");
    let duration = start_time.elapsed();
    println!("\n❯ Generating the `{}` directory content.\n", file_name);
    for header in section_headers {
        println!("{}", header);
    }
    println!("\n❯ Done in {} microseconds.\n", duration.as_micros());

    debug!("Section headers printed for '{}'", dir_path.display());
    Ok(())
}
