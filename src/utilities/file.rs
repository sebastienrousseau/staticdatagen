// Copyright © 2024 Shokunin Static Site Generator. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::models::data::FileData;
use quick_xml::escape::escape;
use std::{fs, io, path::Path};

/// Reads all files in a directory specified by the given path and returns a vector of FileData.
///
/// Each file is represented as a `FileData` struct containing the name and content of the file.
///
/// # Arguments
///
/// * `path` - A `Path` representing the directory containing the files to be read.
///
/// # Returns
///
/// A `Result` containing a vector of `FileData` structs representing all files in the directory,
/// or an `io::Error` if the directory cannot be read.
pub fn add(path: &Path) -> io::Result<Vec<FileData>> {
    let files = fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() {
                let file_name =
                    path.file_name()?.to_string_lossy().to_string();
                if file_name == ".DS_Store" {
                    return None;
                }
                let content = fs::read_to_string(&path)
                    .map_err(|e| {
                        eprintln!(
                            "Error reading file {:?}: {}",
                            path, e
                        );
                        e
                    })
                    .ok()?;
                Some((file_name, content))
            } else {
                None
            }
        })
        .map(|(file_name, content)| {
            let rss = escape(&content).to_string();
            let json =
                serde_json::to_string(&content).unwrap_or_else(|e| {
                    eprintln!(
                        "Error serializing JSON for file {}: {}",
                        file_name, e
                    );
                    String::new()
                });
            let cname = escape(&content).to_string();
            let keyword = escape(&content).to_string();
            let human = content.clone();
            let security = content.clone();
            let sitemap = escape(&content).to_string();
            let sitemap_news = escape(&content).to_string();
            let txt = content.clone();

            FileData {
                cname,
                content,
                json,
                human,
                keyword,
                name: file_name,
                rss,
                security,
                sitemap,
                sitemap_news,
                // tags,
                txt,
            }
        })
        .collect::<Vec<FileData>>();

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::add;
    use std::fs::{self, File};
    use std::io::{self, Write};
    use std::path::Path;
    use tempfile::tempdir;

    /// Tests that `add` reads all valid files in the directory and excludes invalid files.
    #[test]
    fn test_add_valid_files() -> io::Result<()> {
        let dir = tempdir()?;
        let file1_path = dir.path().join("test1.txt");
        let file2_path = dir.path().join("test2.txt");
        let ds_store_path = dir.path().join(".DS_Store");

        // Create test files with content
        File::create(&file1_path)?.write_all(b"Content for file 1")?;
        File::create(&file2_path)?.write_all(b"Content for file 2")?;
        let _ = File::create(&ds_store_path)?; // .DS_Store file should be ignored

        // Run the `add` function
        let files = add(dir.path())?;

        // Verify the correct files are read
        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|file| file.name == "test1.txt"));
        assert!(files.iter().any(|file| file.name == "test2.txt"));

        Ok(())
    }

    /// Tests that `add` handles an empty directory correctly.
    #[test]
    fn test_add_empty_directory() -> io::Result<()> {
        let dir = tempdir()?;

        // Run the `add` function on an empty directory
        let files = add(dir.path())?;

        // Verify that no files are found
        assert!(files.is_empty());

        Ok(())
    }

    /// Tests that `add` returns an error when given a nonexistent directory.
    #[test]
    fn test_add_nonexistent_directory() {
        let nonexistent_dir = Path::new("nonexistent_directory");

        // Run the `add` function on a nonexistent directory
        let result = add(nonexistent_dir);

        // Verify that an error is returned
        assert!(result.is_err());
    }

    /// Tests that `add` correctly escapes special characters in file content.
    #[test]
    fn test_add_escapes_special_characters() -> io::Result<()> {
        let dir = tempdir()?;
        let special_chars_file_path =
            dir.path().join("special_chars.txt");

        // Create a file with special characters
        File::create(&special_chars_file_path)?
            .write_all(b"Content with < & > characters")?;

        // Run the `add` function
        let files = add(dir.path())?;

        // Verify that the content is properly escaped
        let file_data = files
            .iter()
            .find(|f| f.name == "special_chars.txt")
            .unwrap();
        assert_eq!(
            file_data.rss,
            "Content with &lt; &amp; &gt; characters"
        );

        Ok(())
    }

    /// Tests that `add` correctly serializes file content to JSON.
    #[test]
    fn test_add_serializes_to_json() -> io::Result<()> {
        let dir = tempdir()?;
        let json_file_path = dir.path().join("json_file.txt");

        // Create a file with simple text content
        File::create(&json_file_path)?
            .write_all(b"JSON content test")?;

        // Run the `add` function
        let files = add(dir.path())?;

        // Verify that the content is correctly serialized to JSON
        let file_data =
            files.iter().find(|f| f.name == "json_file.txt").unwrap();
        assert_eq!(file_data.json, "\"JSON content test\"");

        Ok(())
    }

    /// Tests that `add` skips over non-file entries (e.g., subdirectories).
    #[test]
    fn test_add_skips_directories() -> io::Result<()> {
        let dir = tempdir()?;
        let subdir_path = dir.path().join("subdir");
        let file_path = dir.path().join("test_file.txt");

        // Create a subdirectory and a test file in the main directory
        fs::create_dir(&subdir_path)?;
        File::create(&file_path)?.write_all(b"Content in file")?;

        // Run the `add` function
        let files = add(dir.path())?;

        // Verify that only the file is read, and the subdirectory is ignored
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].name, "test_file.txt");

        Ok(())
    }
}
