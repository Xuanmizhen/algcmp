use log::{debug, info, warn};
use markup5ever::interface::tree_builder::TreeSink;
use regex::Regex;
use reqwest;
use scraper::{Html, HtmlTreeSink, Selector};
use std::{collections::HashMap, fs, path::Path};
use tokio::time::Duration;

use crate::{errors::AppError, utils::find_markdown_files};

/// Represents a C++ reference entry
#[derive(Debug)]
pub struct CppReference {
    /// The name of the C++ function or class
    pub name: String,
    /// The URL to the cppreference.com page
    pub url: String,
    /// The Markdown file where this reference was found
    pub file: String,
    /// The line number in the Markdown file
    pub line: usize,
}

/**
 * Main function to download C++ references
 *
 * This function:
 * 1. Creates the output directory if it doesn't exist
 * 2. Finds all markdown files in the contents directory
 * 3. Extracts C++ references from the markdown files
 * 4. Deduplicates the references
 * 5. Downloads the references (only missing ones unless overwrite is true)
 *
 * @param overwrite Whether to overwrite existing files
 * @return Result indicating success or error
 */
pub async fn download_references(overwrite: bool) -> Result<(), AppError> {
    info!("Starting C++ reference downloader");

    // Create cppreference directory if it doesn't exist
    let output_dir = Path::new("./cppreference");
    if !output_dir.exists() {
        info!("Creating output directory: {:?}", output_dir);
        fs::create_dir_all(output_dir)?;
    }

    // Find all markdown files in contents directory
    let contents_dir = Path::new("./contents");
    let markdown_files = find_markdown_files(contents_dir)?;

    info!("Found {} markdown files", markdown_files.len());

    // Extract references from markdown files
    let references = extract_references(&markdown_files)?;

    info!("Extracted {} references", references.len());

    // Deduplicate references
    let unique_references = deduplicate_references(references)?;

    info!(
        "Found {} unique references after deduplication",
        unique_references.len()
    );

    // Download references
    download_files(unique_references, overwrite).await?;

    info!("Download completed successfully");
    Ok(())
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
        r#"\|\s*[^|]+\|\s*\[`(std::[^`]+)`\s*(?:\([^)]+\))?\]\((https://en.cppreference.com/w/cpp/[^\"]+)\)\s*\|"#,
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

                references.push(CppReference {
                    name,
                    url,
                    file: file_str.to_string(),
                    line: line_num + 1,
                });
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
 * Download C++ reference files
 *
 * This function downloads HTML files from cppreference.com for each reference,
 * skipping files that already exist unless overwrite is true.
 * It also processes each HTML file to remove specified elements.
 *
 * @param references HashMap of CppReference structs by name
 * @param overwrite Whether to overwrite existing files
 * @return Result indicating success or error
 */
async fn download_files(
    references: HashMap<String, CppReference>,
    overwrite: bool,
) -> Result<(), AppError> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .build()?;

    let output_dir = Path::new("./cppreference");

    for (name, ref_item) in references {
        let filename = format!("{}.html", name);
        let output_path = output_dir.join(filename);

        // Check if file already exists and skip if not overwriting
        if output_path.exists() && !overwrite {
            debug!("File already exists: {}.html, skipping download", name);
            continue;
        }

        info!("Downloading {} from {}", name, ref_item.url);

        let response = client.get(&ref_item.url).send().await?;
        let content = response.text().await?;

        // Process HTML to remove specified elements
        let processed_content = process_html(&content, &name)?;

        fs::write(output_path, processed_content)?;
        debug!("Saved {} to cppreference/{}.html", name, name);

        // Add a small delay to avoid overloading the server
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    Ok(())
}

/**
 * Process HTML content to remove specified elements
 *
 * This function removes elements with class "t-navbar" and id "mw-head" from the HTML content.
 * If either element count is not 1, it warns and returns the original content.
 *
 * @param content HTML content as a string
 * @param name Name of the C++ reference (for logging)
 * @return Result containing the processed HTML content
 */
pub fn process_html(content: &str, name: &str) -> Result<String, AppError> {
    // Parse HTML
    let html = Html::parse_document(content);

    // Create HtmlTreeSink for manipulation
    let tree_sink = HtmlTreeSink::new(html);

    // Remove t-navbar div
    let navbar_selector = Selector::parse(".t-navbar").unwrap();
    let navbar_found = {
        let html_ref = tree_sink.0.borrow();
        if let Some(elem) = html_ref.select(&navbar_selector).next() {
            let id = elem.id();
            drop(html_ref);
            tree_sink.remove_from_parent(&id);
            true
        } else {
            false
        }
    };

    // Remove mw-head element
    let head_selector = Selector::parse("#mw-head").unwrap();
    let head_found = {
        let html_ref = tree_sink.0.borrow();
        if let Some(elem) = html_ref.select(&head_selector).next() {
            let id = elem.id();
            drop(html_ref);
            tree_sink.remove_from_parent(&id);
            true
        } else {
            false
        }
    };

    // Check if either element count is not 1
    if !navbar_found || !head_found {
        warn!(
            "Unexpected element count for {}: t-navbar={}, mw-head={}. Skipping element removal.",
            name,
            if navbar_found { 1 } else { 0 },
            if head_found { 1 } else { 0 }
        );
        return Ok(content.to_string());
    }

    // Convert back to Html and then to string
    let modified_html = tree_sink.0.into_inner();
    let result = modified_html.html();

    Ok(result)
}
