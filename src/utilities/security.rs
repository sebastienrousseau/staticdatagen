// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Security utilities for file and path handling
//!
//! This module provides security-focused utilities for handling files and paths,
//! including path sanitization, directory validation, and other security checks.
//!
//! # Security Features
//!
//! - Path traversal prevention
//! - Directory validation
//! - Path sanitization
//! - Secure file handling
//! - Input validation

use anyhow::{Context, Result};
use commons::path::safe_join;
use log::warn;
use std::path::{Path, PathBuf};

/// Checks an untrusted relative path and returns it in normal form, or
/// refuses it.
///
/// The path is accepted only if:
/// - it is valid UTF-8 and not empty;
/// - every character is alphanumeric or one of `/`, `.`, `-`, `_`;
/// - it is relative (no root, drive or share prefix);
/// - no `..` climbs above the directory it is resolved against, even
///   transiently (`a/../../b`).
///
/// Containment follows the shared rule in
/// [`commons::path::safe_join`]: `.` components are dropped, each `..`
/// removes the component before it, and names that merely contain dots,
/// such as `a..b.md` or `.well-known`, are ordinary names.
///
/// # Errors
///
/// Returns an error naming the reason for any path that fails a rule
/// above, or that has no components left once `.` and `..` are resolved.
/// A bad path is never rewritten into a different one.
///
/// # Examples
///
/// ```
/// use std::path::{Path, PathBuf};
/// use staticdatagen::utilities::security::sanitize_path;
///
/// let path = Path::new("content/./blog/post.md");
/// assert_eq!(
///     sanitize_path(path).unwrap(),
///     PathBuf::from("content/blog/post.md")
/// );
///
/// assert!(sanitize_path(Path::new("content/../../etc/passwd")).is_err());
/// assert!(sanitize_path(Path::new("/etc/passwd")).is_err());
/// ```
pub fn sanitize_path(path: &Path) -> Result<PathBuf> {
    let path_str = path.to_str().context("Invalid path encoding")?;

    if path_str.is_empty() {
        return Err(anyhow::anyhow!("Empty path provided"));
    }

    if let Some(bad) = path_str.chars().find(|&c| !is_allowed_char(c)) {
        return Err(anyhow::anyhow!(
            "Path contains a disallowed character {bad:?}: {path_str}"
        ));
    }

    let resolved = safe_join("", path)
        .map_err(|e| anyhow::anyhow!("Unsafe path {path_str}: {e}"))?;
    if resolved.as_os_str().is_empty() {
        return Err(anyhow::anyhow!(
            "Path has no components after resolving . and ..: {path_str}"
        ));
    }

    Ok(resolved)
}

/// Characters `sanitize_path` accepts in a path.
fn is_allowed_char(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '/' | '.' | '-' | '_')
}

/// Validates a directory for security and accessibility.
///
/// This function checks that:
/// - The directory exists
/// - It is actually a directory
/// - It has proper permissions
/// - It is within allowed bounds
///
/// # Arguments
///
/// * `dir` - The directory path to validate
/// * `purpose` - A description of the directory's purpose (for error messages)
///
/// # Returns
///
/// Returns Ok(()) if validation passes, or an error describing the problem.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use staticdatagen::utilities::security::validate_directory;
///
/// let dir = Path::new("content");
/// if let Err(e) = validate_directory(dir, "content") {
///     eprintln!("Directory validation failed: {}", e);
/// }
/// ```
pub fn validate_directory(dir: &Path, purpose: &str) -> Result<()> {
    // Check if directory exists
    if !dir.exists() {
        return Err(anyhow::anyhow!(
            "{} directory does not exist: {}",
            purpose,
            dir.display()
        ));
    }

    // Verify it's a directory
    if !dir.is_dir() {
        return Err(anyhow::anyhow!(
            "{} path exists but is not a directory: {}",
            purpose,
            dir.display()
        ));
    }

    // Check if directory is readable
    match dir.read_dir() {
        Ok(_) => Ok(()),
        Err(e) => {
            warn!("Directory access error: {}", e);
            Err(anyhow::anyhow!(
                "Cannot access {} directory: {}",
                purpose,
                dir.display()
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_sanitize_path_basic() {
        let path = Path::new("content/blog/post.md");
        let result = sanitize_path(path).unwrap();
        assert_eq!(result, PathBuf::from("content/blog/post.md"));
    }

    #[test]
    fn test_sanitize_path_resolves_dotdot_semantically() {
        // a/b/../c should resolve to a/c (not a/b/c)
        let path = Path::new("a/b/../c");
        let result = sanitize_path(path).unwrap();
        assert_eq!(result, PathBuf::from("a/c"));
    }

    #[test]
    fn test_sanitize_path_refuses_traversal_instead_of_rewriting() {
        // Used to come back as Ok("etc/passwd"): the guard dropped the
        // `..` it could not honour and handed back a different path.
        for input in
            ["content/../../../etc/passwd", "../file.txt", "a/../../b"]
        {
            assert!(
                sanitize_path(Path::new(input)).is_err(),
                "{input} must be refused"
            );
        }
    }

    #[test]
    fn test_sanitize_path_refuses_absolute() {
        // Used to come back as Ok("etc/passwd").
        assert!(sanitize_path(Path::new("/etc/passwd")).is_err());
    }

    #[test]
    fn test_sanitize_path_refuses_disallowed_characters() {
        // Used to strip the characters and return a different path.
        let path = Path::new("content/<script>alert(1)</script>/x.md");
        assert!(sanitize_path(path).is_err());
    }

    #[test]
    fn test_sanitize_path_accepts_dotted_names() {
        for input in ["a..b.md", "blog/..hidden", ".well-known/x"] {
            assert_eq!(
                sanitize_path(Path::new(input)).unwrap(),
                PathBuf::from(input)
            );
        }
    }

    #[test]
    fn test_sanitize_path_empty() {
        let path = Path::new("");
        assert!(sanitize_path(path).is_err());
    }

    #[test]
    fn test_validate_directory_exists() {
        let temp_dir = TempDir::new().unwrap();
        assert!(validate_directory(temp_dir.path(), "test").is_ok());
    }

    #[test]
    fn test_validate_directory_nonexistent() {
        let path = Path::new("nonexistent");
        assert!(validate_directory(path, "test").is_err());
    }

    #[test]
    fn test_validate_directory_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        std::fs::write(&file_path, "test").unwrap();
        assert!(validate_directory(&file_path, "test").is_err());
    }

    #[test]
    fn test_sanitize_path_only_dots_and_slashes() {
        // `..` climbs above the root on the first component.
        let path = Path::new("../../../");
        let result = sanitize_path(path);
        assert!(
            result.is_err(),
            "Should return error for path that becomes empty"
        );
    }

    #[test]
    fn test_sanitize_path_only_dots() {
        // Only `.` components: nothing is left to name.
        let path = Path::new("./././.");
        let result = sanitize_path(path);
        assert!(
            result.is_err(),
            "Should return error for dots-only path"
        );
    }

    #[test]
    fn test_sanitize_path_with_unicode() {
        let path = Path::new("content/日本語/test.md");
        let result = sanitize_path(path).unwrap();
        // Alphanumeric in any script is allowed, and kept as written.
        assert_eq!(result, PathBuf::from("content/日本語/test.md"));
    }

    #[test]
    fn test_validate_directory_error_message() {
        // Test error messages for directory validation
        let path = Path::new("definitely_not_existing_dir_12345");
        let result = validate_directory(path, "content");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("content"),
            "Error should mention purpose"
        );
    }

    #[test]
    #[cfg(unix)]
    fn test_validate_directory_unreadable() {
        use std::os::unix::fs::PermissionsExt;

        let temp_dir = TempDir::new().unwrap();
        let restricted = temp_dir.path().join("restricted");
        std::fs::create_dir(&restricted).unwrap();
        // Remove all permissions
        std::fs::set_permissions(
            &restricted,
            std::fs::Permissions::from_mode(0o000),
        )
        .unwrap();

        let result = validate_directory(&restricted, "test");
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Cannot access"));

        // Restore permissions for cleanup
        let _ = std::fs::set_permissions(
            &restricted,
            std::fs::Permissions::from_mode(0o755),
        );
    }

    mod proptests {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn sanitize_path_never_panics(
                s in ".*"
            ) {
                let path = Path::new(&s);
                // Should never panic, only return Ok or Err
                let _ = sanitize_path(path);
            }

            #[test]
            fn sanitize_path_never_contains_dotdot(
                s in "[a-zA-Z0-9/._-]{1,100}"
            ) {
                let path = Path::new(&s);
                if let Ok(result) = sanitize_path(path) {
                    prop_assert!(
                        !result.components().any(|c| {
                            c.as_os_str() == ".."
                        }),
                        "Sanitized path must not contain \
                         '..' component: {}",
                        result.display()
                    );
                }
            }

            #[test]
            fn sanitize_path_no_absolute_output(
                s in "[a-zA-Z0-9/._-]{1,100}"
            ) {
                let path = Path::new(&s);
                if let Ok(result) = sanitize_path(path) {
                    prop_assert!(
                        !result
                            .to_string_lossy()
                            .starts_with('/'),
                        "Sanitized path must be relative"
                    );
                }
            }
        }
    }
}
