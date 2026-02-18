// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Directory operation macros for the static site generator
//!
//! This module provides macros for common directory operations, including:
//! - Checking and creating directories
//! - Cleaning up directories
//! - Creating multiple directories at once
//!
//! The macros in this module are designed to provide a convenient and safe way
//! to perform common directory operations while maintaining proper error handling
//! and logging.

/// Checks if a directory exists and creates it if necessary.
///
/// # Arguments
///
/// * `_dir` - The path to check/create (as a `std::path::Path`)
/// * `_name` - A string literal representing the directory name for error messages
///
/// # Returns
///
/// Returns a `Result<(), anyhow::Error>` indicating success or failure.
///
/// # Example
///
/// ```rust
/// use staticdatagen::macro_check_directory;
/// use std::path::Path;
/// use std::fs;
///
/// let path = Path::new("logs");
/// macro_check_directory!(path, "logs").unwrap();
///
/// // Ensure the directory is removed after the test
/// if path.exists() {
///     fs::remove_dir_all(path).expect("Failed to remove logs directory");
/// }
/// ```
#[macro_export]
macro_rules! macro_check_directory {
    ($_dir:expr, $_name:expr) => {{
        use std::path::Path;
        let directory: &Path = $_dir;
        let name = $_name;

        if directory.exists() {
            if !directory.is_dir() {
                log::error!("'{}' is not a directory.", name);
                Err(anyhow::anyhow!("'{}' is not a directory.", name))
            } else {
                Ok(())
            }
        } else {
            match std::fs::create_dir_all(directory) {
                Ok(_) => {
                    log::info!("Created directory: {}", name);
                    Ok(())
                }
                Err(e) => {
                    log::error!(
                        "Cannot create '{}' directory: {}",
                        name,
                        e
                    );
                    Err(anyhow::anyhow!(
                        "Cannot create '{}' directory: {}",
                        name,
                        e
                    ))
                }
            }
        }
    }};
}

/// Cleans up (removes) multiple directories.
///
/// # Arguments
///
/// * `$path` - The path to the directory to clean up
///
/// # Returns
///
/// Returns a `Result<(), anyhow::Error>` indicating success or failure.
///
/// # Example
///
/// ```rust
/// use staticdatagen::macro_cleanup_directories;
/// use std::path::Path;
///
/// let path = Path::new("temp_dir");
/// if let Err(e) = macro_cleanup_directories!(path) {
///     eprintln!("Failed to clean up directory: {}", e);
/// }
/// ```
#[macro_export]
macro_rules! macro_cleanup_directories {
    ($path:expr) => {{
        use anyhow::Context;
        std::fs::remove_dir_all($path).with_context(|| {
            format!("Failed to clean up directory: {:?}", $path)
        })
    }};
}

/// Creates multiple directories at once.
///
/// # Arguments
///
/// * `$($path:expr),+` - One or more directory paths to create
///
/// # Returns
///
/// Returns a `Result<(), anyhow::Error>` indicating success or failure.
///
/// # Example
///
/// ```rust
/// use staticdatagen::macro_create_directories;
/// use std::path::Path;
/// use std::fs;
///
/// let path1 = Path::new("dir1");
/// let path2 = Path::new("dir2");
///
/// // Attempt to create directories
/// if let Err(e) = macro_create_directories!(path1, path2) {
///     eprintln!("Failed to create directories: {}", e);
/// }
///
/// // Ensure the directories are removed after the test
/// if path1.exists() {
///     fs::remove_dir_all(path1).expect("Failed to remove dir1");
/// }
/// if path2.exists() {
///     fs::remove_dir_all(path2).expect("Failed to remove dir2");
/// }
/// ```
#[macro_export]
macro_rules! macro_create_directories {
    ($($path:expr),+) => {
        {
            use anyhow::{Result, Context};
            (|| -> Result<()> {
                $(
                    std::fs::create_dir_all($path)
                        .with_context(|| format!("Failed to create directory: {:?}", $path))?;
                    log::info!("✓ Created directory: {:?}", $path);
                )+
                Ok(())
            })()
        }
    };
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    #[test]
    fn test_macro_check_directory() {
        let temp_dir = TempDir::new().unwrap();
        let test_path = temp_dir.path().join("test_dir");

        macro_check_directory!(&test_path, "test_dir").unwrap();
        assert!(test_path.exists());
        assert!(test_path.is_dir());
    }

    #[test]
    fn test_macro_check_directory_not_a_dir() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("a_file");
        std::fs::write(&file_path, "data").unwrap();

        let result: Result<(), anyhow::Error> =
            macro_check_directory!(&file_path, "a_file");
        assert!(result.is_err());
    }

    #[test]
    fn test_macro_cleanup_directories() {
        let temp_dir = TempDir::new().unwrap();
        let test_path = temp_dir.path().join("test_dir");
        std::fs::create_dir(&test_path).unwrap();

        assert!(macro_cleanup_directories!(&test_path).is_ok());
        assert!(!test_path.exists());
    }

    #[test]
    fn test_macro_create_directories() {
        let temp_dir = TempDir::new().unwrap();
        let test_path1 = temp_dir.path().join("dir1");
        let test_path2 = temp_dir.path().join("dir2");

        assert!(
            macro_create_directories!(&test_path1, &test_path2).is_ok()
        );
        assert!(test_path1.exists());
        assert!(test_path2.exists());
    }

    #[test]
    fn test_macro_check_directory_existing() {
        let temp_dir = TempDir::new().unwrap();
        let test_path = temp_dir.path().join("existing_dir");
        std::fs::create_dir(&test_path).unwrap();

        let result: Result<(), anyhow::Error> =
            macro_check_directory!(&test_path, "existing_dir");
        assert!(result.is_ok());
    }

    #[test]
    fn test_macro_cleanup_directories_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let nonexistent = temp_dir.path().join("does_not_exist");

        let result = macro_cleanup_directories!(&nonexistent);
        assert!(result.is_err());
    }

    #[test]
    fn test_macro_create_directories_nested() {
        let temp_dir = TempDir::new().unwrap();
        let nested = temp_dir.path().join("a/b/c");

        assert!(macro_create_directories!(&nested).is_ok());
        assert!(nested.exists());
    }

    #[test]
    fn test_macro_create_directories_single() {
        let temp_dir = TempDir::new().unwrap();
        let single = temp_dir.path().join("single_dir");

        assert!(macro_create_directories!(&single).is_ok());
        assert!(single.exists());
    }

    #[test]
    fn test_macro_check_directory_creates_nested() {
        let temp_dir = TempDir::new().unwrap();
        let nested = temp_dir.path().join("x/y/z");

        let result: Result<(), anyhow::Error> =
            macro_check_directory!(&nested, "nested");
        assert!(result.is_ok());
        assert!(nested.exists());
        assert!(nested.is_dir());
    }

    #[test]
    fn test_macro_cleanup_then_verify_gone() {
        let temp_dir = TempDir::new().unwrap();
        let test_path = temp_dir.path().join("cleanup_target");
        std::fs::create_dir(&test_path).unwrap();
        // Add a file inside
        std::fs::write(test_path.join("file.txt"), "data").unwrap();

        assert!(macro_cleanup_directories!(&test_path).is_ok());
        assert!(!test_path.exists());
    }

    #[test]
    fn test_macro_check_directory_create_fails() {
        let temp_dir = TempDir::new().unwrap();
        // Create a file that blocks directory creation at a path component
        let blocker = temp_dir.path().join("blocker_file");
        std::fs::write(&blocker, "data").unwrap();
        // Try to create a directory inside the file — create_dir_all will fail
        let nested = blocker.join("subdir");
        let result: Result<(), anyhow::Error> =
            macro_check_directory!(&nested, "blocked");
        assert!(result.is_err());
    }
}
