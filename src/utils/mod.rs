use std::{
    fs,
    path::{Path, PathBuf},
};

/**
 * Recursively find all markdown files in a directory
 *
 * @param dir The directory to start searching from
 * @return Result containing a vector of PathBuf to markdown files
 */
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

/**
 * Get all HTML files in a directory
 *
 * @param dir Directory to search
 * @return Result containing vector of HTML file paths
 */
pub fn get_html_files(dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension() == Some(std::ffi::OsStr::new("html")) {
            files.push(path);
        }
    }

    Ok(files)
}
