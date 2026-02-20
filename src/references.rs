//! C++ reference extraction and management
//!
//! This module provides functionality for extracting C++ reference information
//! from Markdown files. It handles parsing, deduplication, and sorting of
//! references from cppreference.com.
//!
//! # Main Components
//!
//! - [`CppReference`] - A struct representing a C++ reference entry
//! - [`get_required_references`] - Extract all required references from Markdown files
//! - [`compare_cpp_names`] - Compare C++ names using recursive dictionary order

use log::debug;
use regex::Regex;
use std::{collections::HashMap, fs, path::Path};

use crate::{errors::AppError, utils::find_markdown_files};

/// A C++ reference entry extracted from a Markdown file
///
/// This struct represents a single C++ reference, containing both the
/// function/class name (e.g., `std::vector`) and the corresponding
/// cppreference.com URL.
#[derive(Debug, Clone)]
pub struct CppReference {
    /// The name of the C++ function or class (e.g., `std::vector`)
    pub name: String,
    /// The URL to the cppreference.com page
    pub url: String,
}

/// Get all required C++ references from Markdown files
///
/// This function finds all Markdown files in the `./contents` directory,
/// extracts C++ references from them, and deduplicates them.
///
/// # Returns
///
/// A `HashMap` mapping reference names to their `CppReference` structs.
///
/// # Errors
///
/// Returns an error if:
/// - The contents directory cannot be read
/// - A Markdown file cannot be parsed
/// - Duplicate references have conflicting URLs
pub fn get_required_references() -> Result<HashMap<String, CppReference>, AppError> {
    // Find all markdown files in contents directory
    let contents_dir = Path::new("./contents");
    let markdown_files = find_markdown_files(contents_dir)?;

    // Extract references from markdown files
    let references = extract_references(&markdown_files)?;

    // Deduplicate references
    let unique_references = deduplicate_references(references)?;

    Ok(unique_references)
}

/// Extract C++ references from Markdown files
///
/// This function parses Markdown files to find C++ reference entries in table format.
/// It extracts the function/class name and the corresponding cppreference.com URL.
///
/// # Expected Format
///
/// The function looks for entries in this format:
/// ```markdown
/// | ... | [`std::function_name`](https://en.cppreference.com/w/cpp/...) | ... |
/// ```
///
/// # Arguments
///
/// * `files` - A slice of `PathBuf` pointing to Markdown files
///
/// # Returns
///
/// A vector of `CppReference` structs.
///
/// # Errors
///
/// Returns an error if:
/// - A file cannot be read
/// - A reference entry has an invalid format
pub fn extract_references(files: &[std::path::PathBuf]) -> Result<Vec<CppReference>, AppError> {
    let mut references = Vec::new();

    // Regex to match C++ reference entries in markdown table format
    let regex = Regex::new(
        r#"\|\s*[^|]+\|\s*\[`(std::[^`]+)`\s*(?:\([^)]+\))?\]\((https://en.cppreference.com/w/cpp/[^"]+)\)\s*\|"#,
    )?;

    for file in files {
        let file_str = file.to_str().unwrap_or_default();
        let content = fs::read_to_string(file)?;

        for (line_num, line) in content.lines().enumerate() {
            if let Some(captures) = regex.captures(line) {
                let name = captures
                    .get(1)
                    .map(|m| m.as_str().trim().to_string())
                    .ok_or_else(|| AppError::InvalidFileFormat {
                        file: file_str.to_string(),
                        line: line_num + 1,
                    })?;

                let url = captures
                    .get(2)
                    .map(|m| m.as_str().trim().to_string())
                    .ok_or_else(|| AppError::MissingUrl {
                        file: file_str.to_string(),
                        line: line_num + 1,
                    })?;

                references.push(CppReference { name, url });
            }
        }
    }

    Ok(references)
}

/// Deduplicate C++ references
///
/// This function removes duplicate references by name, checking for URL conflicts.
/// If two references with the same name have different URLs, it returns an error.
///
/// # Arguments
///
/// * `references` - A vector of `CppReference` structs
///
/// # Returns
///
/// A `HashMap` mapping reference names to their `CppReference` structs.
///
/// # Errors
///
/// Returns an error if duplicate references have different URLs.
pub fn deduplicate_references(
    references: Vec<CppReference>,
) -> Result<HashMap<String, CppReference>, AppError> {
    let mut unique: HashMap<String, CppReference> = HashMap::new();

    for ref_item in references {
        if let Some(existing) = unique.get(&ref_item.name) {
            // Check for URL conflict
            if existing.url != ref_item.url {
                return Err(AppError::DuplicateConflict {
                    name: ref_item.name.clone(),
                    url1: existing.url.clone(),
                    url2: ref_item.url.clone(),
                });
            }
            // Same URL, no conflict
            debug!("Duplicate entry found but no conflict: {}", ref_item.name);
        } else {
            unique.insert(ref_item.name.clone(), ref_item);
        }
    }

    Ok(unique)
}

/// Compare two C++ names using recursive dictionary order
///
/// This function splits names by `::` and compares each component
/// using standard string dictionary order. This provides a more intuitive
/// sorting for C++ names than simple ASCII comparison.
///
/// # Examples
///
/// ```
/// use algcmp::references::compare_cpp_names;
/// use std::cmp::Ordering;
///
/// assert_eq!(compare_cpp_names("std::vector", "std::list"), Ordering::Greater);
/// assert_eq!(compare_cpp_names("std::list", "std::vector"), Ordering::Less);
/// assert_eq!(compare_cpp_names("std::vector", "std::vector"), Ordering::Equal);
/// assert_eq!(compare_cpp_names("std::vector", "std::vector::iterator"), Ordering::Less);
/// ```
///
/// # Arguments
///
/// * `a` - First name to compare
/// * `b` - Second name to compare
///
/// # Returns
///
/// An `Ordering` result indicating the relative order of the names.
pub fn compare_cpp_names(a: &str, b: &str) -> std::cmp::Ordering {
    let a_parts: Vec<&str> = a.split("::").collect();
    let b_parts: Vec<&str> = b.split("::").collect();

    // Compare each component
    for (a_part, b_part) in a_parts.iter().zip(b_parts.iter()) {
        match a_part.cmp(b_part) {
            std::cmp::Ordering::Equal => continue,
            other => return other,
        }
    }

    // If all compared components are equal, shorter name comes first
    a_parts.len().cmp(&b_parts.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_cpp_names() {
        // Test basic comparison
        assert_eq!(
            compare_cpp_names("std::vector", "std::list"),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            compare_cpp_names("std::list", "std::vector"),
            std::cmp::Ordering::Less
        );
        assert_eq!(
            compare_cpp_names("std::vector", "std::vector"),
            std::cmp::Ordering::Equal
        );

        // Test nested names
        assert_eq!(
            compare_cpp_names("std::chrono::duration", "std::chrono::time_point"),
            std::cmp::Ordering::Less
        );

        // Test different depths
        assert_eq!(
            compare_cpp_names("std::vector", "std::vector::iterator"),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn test_deduplicate_references_no_conflict() {
        let refs = vec![
            CppReference {
                name: "std::vector".to_string(),
                url: "https://example.com/vector".to_string(),
            },
            CppReference {
                name: "std::vector".to_string(),
                url: "https://example.com/vector".to_string(),
            },
        ];
        let result = deduplicate_references(refs).unwrap();
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_deduplicate_references_with_conflict() {
        let refs = vec![
            CppReference {
                name: "std::vector".to_string(),
                url: "https://example.com/vector1".to_string(),
            },
            CppReference {
                name: "std::vector".to_string(),
                url: "https://example.com/vector2".to_string(),
            },
        ];
        let result = deduplicate_references(refs);
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::DuplicateConflict { name, .. } => {
                assert_eq!(name, "std::vector");
            }
            _ => panic!("Expected DuplicateConflict error"),
        }
    }

    #[test]
    fn test_extract_references_from_string() {
        let markdown = r#"| Algorithm | [`std::sort`](https://en.cppreference.com/w/cpp/algorithm/sort) | Sorts elements |
| Algorithm | [`std::find`](https://en.cppreference.com/w/cpp/algorithm/find) (C++20) | Finds element |"#;

        // Create a temporary file for testing
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_file = temp_dir.path().join("test.md");
        fs::write(&temp_file, markdown).unwrap();

        let refs = extract_references(&[temp_file]).unwrap();
        assert_eq!(refs.len(), 2);
        assert_eq!(refs[0].name, "std::sort");
        assert_eq!(refs[1].name, "std::find");
    }
}
