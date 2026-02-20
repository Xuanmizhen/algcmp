//! Utility functions for file system operations
//!
//! This module provides helper functions for finding files
//! in directories, such as Markdown files.

use std::{
    fs,
    path::{Path, PathBuf},
};

/// Recursively find all Markdown files in a directory
///
/// This function traverses a directory tree and collects all files
/// with the `.md` extension.
///
/// # Arguments
///
/// * `dir` - The directory to start searching from
///
/// # Returns
///
/// A vector of `PathBuf` pointing to all found Markdown files.
///
/// # Errors
///
/// Returns an error if directory traversal fails.
pub fn find_markdown_files(dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut files = Vec::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                files.extend(find_markdown_files(&path)?);
            } else if path.extension() == Some(std::ffi::OsStr::new("md")) {
                files.push(path);
            }
        }
    }

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_markdown_files_empty_dir() {
        let result = find_markdown_files(Path::new("/nonexistent"));
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }
}
