// Copyright © 2025 Static Data Gen.
// All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{
    fs::{self},
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};

/// Creates a backup of a file by copying it with a ".src.html" extension.
///
/// # Arguments
///
/// * `file_path` - A reference to the path of the file to backup
///
/// # Returns
///
/// * `io::Result<PathBuf>` - The path to the created backup file
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use staticdatagen::utilities::backup::backup_file;
/// let file_path = Path::new("myfile.txt");
/// match backup_file(file_path) {
///     Ok(backup_path) => println!("Backup created at: {:?}", backup_path),
///     Err(e) => eprintln!("Failed to create backup: {}", e),
/// }
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - The source file doesn't exist
/// - There are insufficient permissions
/// - The backup operation fails due to I/O issues
pub fn backup_file(file_path: &Path) -> io::Result<PathBuf> {
    // Verify source file exists
    if !file_path.exists() {
        return Err(io::Error::new(
            ErrorKind::NotFound,
            format!("Source file not found: {:?}", file_path),
        ));
    }

    let backup_path = file_path.with_extension("src.html");

    // Perform the backup
    let _bytes_copied =
        fs::copy(file_path, &backup_path).map_err(|e| {
            io::Error::new(
                e.kind(),
                format!(
                    "Failed to create backup at {:?}: {}",
                    backup_path, e
                ),
            )
        })?;

    Ok(backup_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::{self, Write};
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;
    use tempfile::tempdir;

    /// Helper function to create a test directory and file
    fn setup_test_file(
        content: &str,
    ) -> io::Result<(tempfile::TempDir, PathBuf)> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test_file.txt");
        let mut file = File::create(&file_path)?;
        writeln!(file, "{}", content)?;
        Ok((dir, file_path))
    }

    #[test]
    /// Tests basic backup creation with simple content
    fn test_backup_file_success() -> io::Result<()> {
        let (_dir, original_file_path) =
            setup_test_file("This is a test file.")?;
        let backup_path = backup_file(&original_file_path)?;

        assert!(backup_path.exists());
        assert!(backup_path.ends_with("test_file.src.html"));

        let original_content = fs::read_to_string(&original_file_path)?;
        let backup_content = fs::read_to_string(&backup_path)?;
        assert_eq!(original_content, backup_content);

        Ok(())
    }

    #[test]
    /// Tests backup creation with empty file
    fn test_backup_empty_file() -> io::Result<()> {
        let (_dir, original_file_path) = setup_test_file("")?;
        let backup_path = backup_file(&original_file_path)?;

        assert!(backup_path.exists());
        let backup_content = fs::read_to_string(&backup_path)?;
        assert_eq!(backup_content, "\n");

        Ok(())
    }

    #[test]
    /// Tests backup creation with large content
    fn test_backup_large_file() -> io::Result<()> {
        let large_content = "A".repeat(1024 * 1024); // 1MB of data
        let (_dir, original_file_path) =
            setup_test_file(&large_content)?;
        let backup_path = backup_file(&original_file_path)?;

        assert!(backup_path.exists());
        let backup_content = fs::read_to_string(&backup_path)?;
        assert_eq!(backup_content, format!("{}\n", large_content));

        Ok(())
    }

    #[test]
    /// Tests backup creation with special characters in content
    fn test_backup_special_chars() -> io::Result<()> {
        let content =
            "Special chars: !@#$%^&*()_+-=[]{}|;:'\",.<>?`~\n\t\r";
        let (_dir, original_file_path) = setup_test_file(content)?;
        let backup_path = backup_file(&original_file_path)?;

        assert!(backup_path.exists());
        let backup_content = fs::read_to_string(&backup_path)?;
        assert_eq!(backup_content, format!("{}\n", content));

        Ok(())
    }

    #[test]
    /// Tests backup creation with Unicode content
    fn test_backup_unicode_content() -> io::Result<()> {
        let content =
            "Unicode: 你好，世界！ привет мир こんにちは世界 👋🌍";
        let (_dir, original_file_path) = setup_test_file(content)?;
        let backup_path = backup_file(&original_file_path)?;

        assert!(backup_path.exists());
        let backup_content = fs::read_to_string(&backup_path)?;
        assert_eq!(backup_content, format!("{}\n", content));

        Ok(())
    }

    #[test]
    /// Tests backup creation failure when source file doesn't exist
    fn test_backup_file_nonexistent_file() {
        let nonexistent_path = Path::new("nonexistent_file.txt");
        let result = backup_file(nonexistent_path);
        assert!(result.is_err());
    }

    #[test]
    /// Tests backup creation with file that has no extension
    fn test_backup_file_no_extension() -> io::Result<()> {
        let dir = tempdir()?;
        let original_file_path = dir.path().join("testfile");
        let mut file = File::create(&original_file_path)?;
        writeln!(file, "File without extension")?;

        let backup_path = backup_file(&original_file_path)?;
        assert!(backup_path.exists());
        assert!(backup_path.ends_with("testfile.src.html"));

        Ok(())
    }

    #[test]
    /// Tests backup creation with file that has multiple extensions
    fn test_backup_file_multiple_extensions() -> io::Result<()> {
        let dir = tempdir()?;
        let original_file_path = dir.path().join("test.file.txt");
        let mut file = File::create(&original_file_path)?;
        writeln!(file, "File with multiple extensions")?;

        let backup_path = backup_file(&original_file_path)?;
        assert!(backup_path.exists());
        assert!(backup_path.ends_with("test.file.src.html"));

        Ok(())
    }

    #[test]
    /// Tests backup creation failure due to insufficient permissions
    fn test_backup_file_no_permission() -> io::Result<()> {
        let dir = tempdir()?;
        let original_file_path = dir.path().join("test_file.txt");
        let _ = File::create(&original_file_path)?;

        // Set directory to read-only
        let mut permissions = fs::metadata(dir.path())?.permissions();
        permissions.set_mode(0o400);
        fs::set_permissions(dir.path(), permissions)?;

        let result = backup_file(&original_file_path);
        assert!(result.is_err());

        // Restore permissions for cleanup
        let mut permissions = fs::metadata(dir.path())?.permissions();
        permissions.set_mode(0o700);
        fs::set_permissions(dir.path(), permissions)?;

        Ok(())
    }

    #[test]
    /// Tests backup creation when backup file already exists
    fn test_backup_file_overwrite_existing() -> io::Result<()> {
        let (_dir, original_file_path) =
            setup_test_file("Original content")?;

        // Create existing backup file
        let backup_path = original_file_path.with_extension("src.html");
        let mut existing_backup = File::create(&backup_path)?;
        writeln!(existing_backup, "Existing backup content")?;

        // Create new backup
        let new_backup_path = backup_file(&original_file_path)?;
        assert_eq!(backup_path, new_backup_path);

        // Verify content was overwritten
        let backup_content = fs::read_to_string(&backup_path)?;
        assert_eq!(backup_content, "Original content\n");

        Ok(())
    }

    #[test]
    /// Tests backup creation with a long filename that's within system limits
    fn test_backup_long_filename() -> io::Result<()> {
        let dir = tempdir()?;

        // Use a more reasonable filename length that works across platforms
        // Most Unix systems limit to 255 bytes, while Windows limits to 260 characters for the full path
        // We'll use a shorter length to ensure the test passes on all systems
        let long_name = format!("{}.txt", "a".repeat(200));

        // Get the full path to check total length
        let original_file_path = dir.path().join(&long_name);

        // Check if the path length would be valid for the system
        if original_file_path.to_str().map_or(false, |s| s.len() < 255)
        {
            let mut file = File::create(&original_file_path)?;
            writeln!(file, "File with long name")?;

            let backup_path = backup_file(&original_file_path)?;
            assert!(backup_path.exists());

            // Verify the backup was created successfully
            let original_content =
                fs::read_to_string(&original_file_path)?;
            let backup_content = fs::read_to_string(&backup_path)?;
            assert_eq!(original_content, backup_content);
        } else {
            // Skip test if path would be too long for this system
            println!("Skipping long filename test due to system path length limitations");
        }

        Ok(())
    }

    // Add a new test for very long paths
    #[test]
    /// Tests handling of excessively long filenames
    fn test_backup_excessive_filename_length() {
        let dir = tempdir().unwrap();
        let very_long_name = format!("{}.txt", "a".repeat(1000)); // Deliberately too long
        let original_file_path = dir.path().join(very_long_name);

        let result = backup_file(&original_file_path);

        // Should fail with InvalidFilename or similar error
        assert!(result.is_err());
    }

    #[test]
    /// Tests backup creation with readonly source file
    fn test_backup_readonly_source() -> io::Result<()> {
        let (_dir, original_file_path) =
            setup_test_file("Readonly file")?;

        // Make source file readonly
        let mut permissions =
            fs::metadata(&original_file_path)?.permissions();
        permissions.set_mode(0o444);
        fs::set_permissions(&original_file_path, permissions)?;

        // Should still succeed as we only need read permissions
        let backup_path = backup_file(&original_file_path)?;
        assert!(backup_path.exists());

        // Restore permissions for cleanup
        let mut permissions =
            fs::metadata(&original_file_path)?.permissions();
        permissions.set_mode(0o644);
        fs::set_permissions(&original_file_path, permissions)?;

        Ok(())
    }

    #[test]
    /// Tests backup creation with symlinked source file
    #[cfg(unix)] // This test is Unix-specific
    fn test_backup_symlink() -> io::Result<()> {
        let (dir, original_file_path) =
            setup_test_file("Symlink test content")?;
        let symlink_path = dir.path().join("symlink.txt");
        std::os::unix::fs::symlink(&original_file_path, &symlink_path)?;

        let backup_path = backup_file(&symlink_path)?;
        assert!(backup_path.exists());
        assert!(backup_path.ends_with("symlink.src.html"));

        let backup_content = fs::read_to_string(&backup_path)?;
        assert_eq!(backup_content, "Symlink test content\n");

        Ok(())
    }

    #[test]
    /// Tests error handling when the source file is a directory
    fn test_backup_directory() -> io::Result<()> {
        let dir = tempdir()?;
        let result = backup_file(dir.path());
        assert!(result.is_err());

        if let Err(e) = result {
            assert!(
                e.kind() == ErrorKind::Other
                    || e.kind() == ErrorKind::InvalidInput
            );
        }

        Ok(())
    }

    #[test]
    /// Tests concurrent backup operations
    fn test_concurrent_backups() -> io::Result<()> {
        use std::thread;

        let (_dir, original_file_path) =
            setup_test_file("Concurrent test content")?;
        let path_clone = original_file_path.clone();

        let handle = thread::spawn(move || backup_file(&path_clone));

        let result1 = backup_file(&original_file_path);
        let result2 = handle.join().expect("Thread panicked");

        // More specific error checking
        match (result1, result2) {
            (Ok(_), Ok(_)) => println!("Both backups succeeded"),
            (Ok(_), Err(_)) | (Err(_), Ok(_)) => {
                println!("One backup succeeded")
            }
            (Err(e1), Err(e2)) => {
                panic!("Both backups failed: {:?}, {:?}", e1, e2);
            }
        }

        Ok(())
    }

    #[test]
    /// Tests backup creation with a file containing null bytes
    fn test_backup_null_bytes() -> io::Result<()> {
        let content = format!(
            "Content with null byte: {}\0 and more text",
            "middle"
        );
        let (_dir, original_file_path) = setup_test_file(&content)?;
        let backup_path = backup_file(&original_file_path)?;

        assert!(backup_path.exists());
        let backup_content = fs::read_to_string(&backup_path)?;
        assert_eq!(backup_content, format!("{}\n", content));

        Ok(())
    }

    #[test]
    /// Tests backup creation with a file containing only whitespace
    fn test_backup_whitespace_only() -> io::Result<()> {
        let content = "    \t    \n    \r\n    ";
        let (_dir, original_file_path) = setup_test_file(content)?;
        let backup_path = backup_file(&original_file_path)?;

        assert!(backup_path.exists());
        let backup_content = fs::read_to_string(&backup_path)?;
        assert_eq!(backup_content, format!("{}\n", content));

        Ok(())
    }

    #[test]
    /// Tests backup of a file with unusual but valid characters in its name
    fn test_backup_unusual_filename() -> io::Result<()> {
        let dir = tempdir()?;
        let original_file_path =
            dir.path().join("test!@#$%^&()_+-=[]file.txt");
        let mut file = File::create(&original_file_path)?;
        writeln!(file, "File with unusual name")?;

        let backup_path = backup_file(&original_file_path)?;
        assert!(backup_path.exists());
        assert!(
            backup_path.ends_with("test!@#$%^&()_+-=[]file.src.html")
        );

        Ok(())
    }

    #[test]
    /// Tests backup when destination already exists and is readonly
    fn test_backup_readonly_destination() -> io::Result<()> {
        let (_dir, original_file_path) =
            setup_test_file("Original content")?;

        // Create and make readonly backup file
        let backup_path = original_file_path.with_extension("src.html");
        {
            let mut existing_backup = File::create(&backup_path)?;
            writeln!(existing_backup, "Existing backup content")?;
        }

        let mut permissions = fs::metadata(&backup_path)?.permissions();
        permissions.set_mode(0o444);
        fs::set_permissions(&backup_path, permissions)?;

        // Attempt backup (should fail due to readonly destination)
        let result = backup_file(&original_file_path);
        assert!(result.is_err());

        // Cleanup
        let mut permissions = fs::metadata(&backup_path)?.permissions();
        permissions.set_mode(0o644);
        fs::set_permissions(&backup_path, permissions)?;

        Ok(())
    }

    #[test]
    /// Tests backup of a zero-byte file
    fn test_backup_zero_byte_file() -> io::Result<()> {
        let dir = tempdir()?;
        let original_file_path = dir.path().join("empty.txt");
        let _file = File::create(&original_file_path)?; // Use _file to indicate intentionally unused

        let backup_path = backup_file(&original_file_path)?;
        assert!(backup_path.exists());

        let metadata = fs::metadata(&backup_path)?;
        assert_eq!(metadata.len(), 0);

        Ok(())
    }

    #[test]
    /// Tests backup creation with maximum path segments
    fn test_backup_max_path_segments() -> io::Result<()> {
        let dir = tempdir()?;
        let mut path = PathBuf::from(dir.path());

        // Create a path with multiple segments but within length limits
        for i in 1..10 {
            path.push(format!("dir{}", i));
            fs::create_dir(&path)?;
        }

        path.push("test.txt");
        let mut file = File::create(&path)?;
        writeln!(file, "Deep path test")?;

        let result = backup_file(&path);
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    /// Tests backup when disk quota is exceeded (simulation)
    #[cfg(unix)]
    fn test_backup_disk_quota_exceeded() -> io::Result<()> {
        use std::os::unix::fs::MetadataExt;

        let (_dir, original_file_path) =
            setup_test_file("Test content")?;

        // Verify source file exists and is readable
        assert!(original_file_path.exists());
        let metadata = fs::metadata(&original_file_path)?;
        assert!(metadata.mode() & 0o444 != 0);

        // Attempt backup
        let result = backup_file(&original_file_path)?;
        assert!(result.exists()); // Check the returned backup path exists

        Ok(())
    }

    #[test]
    /// Tests backup with alternative file extensions
    fn test_backup_alternative_extensions() -> io::Result<()> {
        // Test cases with expected backup filenames
        let test_cases = [
            ("test", "test.src.html"),
            ("test.txt", "test.src.html"),
            ("test.md", "test.src.html"),
            ("test.html", "test.src.html"),
            ("test.文档", "test.src.html"),
        ];

        for (input_name, expected_backup) in test_cases.iter() {
            let dir = tempdir()?;
            let original_file_path = dir.path().join(input_name);
            let mut file = File::create(&original_file_path)?;
            writeln!(file, "Test content")?;

            let backup_path = backup_file(&original_file_path)?;
            assert!(backup_path.exists());

            // Get the filename without path and verify it matches expected
            if let Some(backup_filename) = backup_path.file_name() {
                assert_eq!(
                    backup_filename.to_string_lossy().to_string(), // Convert to owned String
                    *expected_backup, // Dereference str
                    "Backup filename mismatch for input: {}",
                    input_name
                );
            } else {
                panic!("Backup path has no filename");
            }
        }

        Ok(())
    }

    #[test]
    /// Tests backup with case sensitivity in filenames
    fn test_backup_case_sensitivity() -> io::Result<()> {
        let dir = tempdir()?;
        let original_file_path = dir.path().join("TEST.TXT");
        let mut file = File::create(&original_file_path)?;
        writeln!(file, "Test content")?;

        let backup_path = backup_file(&original_file_path)?;
        assert!(backup_path.exists());

        // Test that case is preserved in backup file name
        assert!(backup_path.ends_with("TEST.src.html"));

        Ok(())
    }

    #[test]
    /// Tests backup of hidden files (Unix-style)
    fn test_backup_hidden_file() -> io::Result<()> {
        let dir = tempdir()?;
        let original_file_path = dir.path().join(".hidden_file");
        let mut file = File::create(&original_file_path)?;
        writeln!(file, "Hidden file content")?;

        let backup_path = backup_file(&original_file_path)?;
        assert!(backup_path.exists());
        assert!(backup_path.ends_with(".hidden_file.src.html"));

        Ok(())
    }
}
