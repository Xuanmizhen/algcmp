//! Error types for the application
//!
//! This module defines all error types used throughout the application using the
//! `thiserror` crate. Each error variant provides detailed context about what went wrong.

use thiserror::Error;

/// Application error types
///
/// This enum represents all possible errors that can occur during the execution
/// of the program. Each variant includes relevant context information.
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
    /// Invalid file format in a Markdown file
    #[error("Invalid file format in {file}:{line}")]
    InvalidFileFormat { file: String, line: usize },
    /// Duplicate entry with conflicting information
    #[error("Duplicate entry with conflicting information: {name} at {url1} and {url2}")]
    DuplicateConflict {
        name: String,
        url1: String,
        url2: String,
    },
    /// Missing required HTML files
    #[error("Missing {count} required HTML file(s): {files}")]
    MissingRequiredFiles { count: usize, files: String },
    /// HTML parsing error
    #[error("HTML parsing error in {file}: {reason}")]
    HtmlParsingError { file: String, reason: String },
}

impl AppError {
    /// Create a new MissingRequiredFiles error
    pub fn missing_files(files: &[String]) -> Self {
        let files_str = files.iter().map(|f| format!("{}.html", f)).collect::<Vec<_>>().join(", ");
        AppError::MissingRequiredFiles {
            count: files.len(),
            files: files_str,
        }
    }
}
