use thiserror::Error;

/// Error types for the application
#[derive(Error, Debug)]
pub enum AppError {
    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    /// Regex error
    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),
    /// HTTP error
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    /// Missing URL in a Markdown file
    #[error("Missing URL in {file}:{line}")]
    MissingUrl { file: String, line: usize },
    /// Invalid URL in a Markdown file
    #[error("Invalid URL in {file}:{line}: {url}")]
    InvalidUrl {
        file: String,
        line: usize,
        url: String,
    },
    /// Duplicate entry with conflicting information
    #[error("Duplicate entry with conflicting information: {name} at {url1} and {url2}")]
    DuplicateConflict {
        name: String,
        url1: String,
        url2: String,
    },
    /// Invalid file format in a Markdown file
    #[error("Invalid file format in {file}:{line}")]
    InvalidFileFormat { file: String, line: usize },
}
