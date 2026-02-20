use log::debug;
use regex::Regex;
use std::{collections::HashMap, fs, path::Path};

use crate::{errors::AppError, utils::find_markdown_files};

/// Represents a C++ reference entry
#[derive(Debug, Clone)]
pub struct CppReference {
    /// The name of the C++ function or class
    pub name: String,
    /// The URL to the cppreference.com page
    pub url: String,
}

/**
 * Get all required C++ references from markdown files
 *
 * This function finds all markdown files in the contents directory,
 * extracts C++ references from them, and deduplicates them.
 *
 * @return Result containing a HashMap of unique references by name
 */
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

/**
 * Extract C++ references from markdown files
 *
 * This function parses markdown files to find C++ reference entries in table format.
 * It extracts the function/class name and the corresponding cppreference.com URL.
 *
 * @param files Vector of PathBuf to markdown files
 * @return Result containing a vector of CppReference structs
 */
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

/**
 * Deduplicate C++ references
 *
 * This function removes duplicate references by name, checking for URL conflicts.
 * If two references with the same name have different URLs, it returns an error.
 *
 * @param references Vector of CppReference structs
 * @return Result containing a HashMap of unique references by name
 */
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

/**
 * Compare two C++ names using recursive dictionary order
 *
 * This function splits names by "::" and compares each component
 * using standard string dictionary order.
 *
 * @param a First name to compare
 * @param b Second name to compare
 * @return Ordering result
 */
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
}
