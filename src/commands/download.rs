//! Download command implementation
//!
//! This module provides functionality to download C++ reference pages from
//! cppreference.com. It extracts URLs from Markdown files, downloads the
//! corresponding HTML pages, and processes them by removing navigation elements.

use log::{debug, info};
use std::{fs, path::Path};
use tokio::time::Duration;

use crate::{
    errors::AppError,
    html::remove_navigation_elements,
    references::get_required_references,
};

/// Download C++ reference pages from cppreference.com
///
/// This function:
/// 1. Creates the output directory (`./cppreference`) if it doesn't exist
/// 2. Gets all required C++ references from Markdown files in `./contents`
/// 3. Downloads the HTML pages (only missing ones unless `overwrite` is true)
/// 4. Processes each HTML file to remove navigation elements
///
/// # Arguments
///
/// * `overwrite` - Whether to overwrite existing files
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if something goes wrong.
///
/// # Errors
///
/// Returns an error if:
/// - The output directory cannot be created
/// - Reference extraction fails
/// - Download fails
/// - File writing fails
pub async fn download_references(overwrite: bool) -> Result<(), AppError> {
    info!("Starting C++ reference downloader");

    // Create cppreference directory if it doesn't exist
    let output_dir = Path::new("./cppreference");
    if !output_dir.exists() {
        info!("Creating output directory: {:?}", output_dir);
        fs::create_dir_all(output_dir)?;
    }

    // Get required references from markdown files
    let unique_references = get_required_references()?;

    info!(
        "Found {} unique references to download",
        unique_references.len()
    );

    // Download references
    download_files(unique_references, overwrite).await?;

    info!("Download completed successfully");
    Ok(())
}

/// Download HTML files from cppreference.com
///
/// This function downloads HTML files for each reference, skipping files that
/// already exist unless `overwrite` is true. It also processes each HTML file
/// to remove navigation elements.
///
/// # Arguments
///
/// * `references` - A HashMap of CppReference structs keyed by name
/// * `overwrite` - Whether to overwrite existing files
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if download or writing fails.
async fn download_files(
    references: std::collections::HashMap<String, crate::references::CppReference>,
    overwrite: bool,
) -> Result<(), AppError> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .build()?;

    let output_dir = Path::new("./cppreference");

    for (name, ref_item) in references {
        let filename = format!("{}.html", name);
        let output_path = output_dir.join(&filename);

        // Check if file already exists and skip if not overwriting
        if output_path.exists() && !overwrite {
            debug!("File already exists: {}.html, skipping download", name);
            continue;
        }

        info!("Downloading {} from {}", name, ref_item.url);

        let response = client.get(&ref_item.url).send().await?;
        let content = response.text().await?;

        // Process HTML to remove specified elements
        let processed_content = remove_navigation_elements(&content, &name)?;

        fs::write(output_path, processed_content)?;
        debug!("Saved {} to cppreference/{}.html", name, name);

        // Add a small delay to avoid overloading the server
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    Ok(())
}
