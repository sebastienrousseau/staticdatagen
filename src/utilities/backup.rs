// Copyright © 2024 Shokunin Static Site Generator. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{
    fs::{self},
    io::{self},
    path::{Path, PathBuf},
};

/// Creates a backup of a file.
///
/// This function takes a reference to a `Path` object for a file and creates a
/// backup of the file with the extension ".src.html".
///
/// # Arguments
///
/// * `file_path` - A reference to a `Path` object for the file.
///
/// # Returns
///
/// * `Result<PathBuf, std::io::Error>` - A result containing a `PathBuf`
///    object for the backup file.
///     - `Ok(PathBuf)` if the backup file was created successfully.
///     - `Err(std::io::Error)` if the backup file could not be created.
///
pub fn backup_file(file_path: &Path) -> io::Result<PathBuf> {
    let backup_path = file_path.with_extension("src.html");
    let _ = fs::copy(file_path, &backup_path)?;
    Ok(backup_path)
}

#[cfg(test)]
mod tests {
    use super::backup_file;
    use std::fs::{self, File};
    use std::io::{self, Write};
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;
    use tempfile::tempdir;

    /// Tests a successful backup creation.
    #[test]
    fn test_backup_file_success() -> io::Result<()> {
        let dir = tempdir()?;
        let original_file_path = dir.path().join("test_file.txt");

        // Create an original file with some content.
        let mut file = File::create(&original_file_path)?;
        writeln!(file, "This is a test file.")?;

        // Perform the backup.
        let backup_path = backup_file(&original_file_path)?;

        // Check the backup file's existence and content.
        assert!(backup_path.exists());
        assert!(backup_path.ends_with("test_file.src.html"));

        let original_content = fs::read_to_string(&original_file_path)?;
        let backup_content = fs::read_to_string(&backup_path)?;
        assert_eq!(original_content, backup_content);

        Ok(())
    }

    /// Tests backup creation failure when the file does not exist.
    #[test]
    fn test_backup_file_nonexistent_file() {
        let nonexistent_path = Path::new("nonexistent_file.txt");
        let result = backup_file(nonexistent_path);
        assert!(result.is_err());
    }

    /// Tests backup creation failure due to lack of write permissions.
    #[test]
    fn test_backup_file_no_permission() -> io::Result<()> {
        let dir = tempdir()?;
        let original_file_path = dir.path().join("test_file.txt");

        // Create an original file to test permissions.
        let _ = File::create(&original_file_path);

        // Set directory to read-only to simulate permission error.
        let mut permissions = fs::metadata(dir.path())?.permissions();
        permissions.set_mode(0o400); // Read-only for the user
        fs::set_permissions(dir.path(), permissions)?;

        // Expect the backup to fail due to lack of permissions.
        let result = backup_file(&original_file_path);
        assert!(result.is_err());

        // Clean up by restoring user write permissions.
        let mut permissions = fs::metadata(dir.path())?.permissions();
        permissions.set_mode(0o700); // Read-write-execute for the user
        fs::set_permissions(dir.path(), permissions)?;

        Ok(())
    }
}
