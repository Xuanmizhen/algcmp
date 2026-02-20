//! Print command implementation
//!
//! This module provides functionality to concatenate HTML files from the
//! `./cppreference` directory into a single printable HTML file. It supports
//! two output modes:
//!
//! - **Colored**: Preserves syntax highlighting from the original pages
//! - **Flattened**: Removes syntax highlighting for non-colored printing

use log::{error, info};
use markup5ever::{
    Attribute, LocalName, QualName,
    interface::{NodeOrText, TreeSink},
    tendril::StrTendril,
};
use scraper::{Html, HtmlTreeSink, Selector};
use std::{collections::HashSet, fs, path::Path};

use crate::{
    errors::AppError,
    html::flatten_code_blocks,
    references::{compare_cpp_names, get_required_references},
};

/// Concatenate HTML files for printing
///
/// This function:
/// 1. Checks if all required HTML files in `./cppreference` are present
/// 2. If not, errors out with details about missing files
/// 3. If yes, concatenates them in sorted order by manipulating DOM elements
/// 4. For non-colored output, flattens `pre` elements with class `de1`
/// 5. Saves the result to the appropriate file
///
/// # Arguments
///
/// * `colored` - Whether to include colored output (preserve syntax highlighting)
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if something goes wrong.
///
/// # Errors
///
/// Returns an error if:
/// - The cppreference directory does not exist
/// - Required HTML files are missing
/// - File reading or writing fails
/// - HTML parsing fails
///
/// # Output Files
///
/// - `./cppreference_print.html` - Flattened output (no syntax highlighting)
/// - `./cppreference_print_colored.html` - Colored output (with syntax highlighting)
pub fn print_references(colored: bool) -> Result<(), AppError> {
    info!("Starting reference printer");

    // Check if cppreference directory exists
    let cppreference_dir = Path::new("./cppreference");
    if !cppreference_dir.exists() {
        error!("cppreference directory does not exist");
        return Err(AppError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "cppreference directory does not exist",
        )));
    }

    // Get required references from markdown files
    let unique_references = get_required_references()?;

    // Get the set of required file names (without extension)
    let required_names: HashSet<String> = unique_references.keys().cloned().collect();

    info!("Found {} required references", required_names.len());

    // Get all HTML files in cppreference directory
    let html_files: Vec<_> = fs::read_dir(cppreference_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().extension().map_or(false, |ext| ext == "html")
        })
        .map(|entry| entry.path())
        .collect();

    // Get the set of existing file names (without extension)
    let existing_names: HashSet<String> = html_files
        .iter()
        .filter_map(|path| {
            path.file_stem()
                .and_then(|stem| stem.to_str())
                .map(|s| s.to_string())
        })
        .collect();

    // Check for missing files
    let missing_files: Vec<_> = required_names.difference(&existing_names).cloned().collect();
    if !missing_files.is_empty() {
        error!("Missing required HTML files:");
        for name in &missing_files {
            error!("  - {}.html", name);
        }
        return Err(AppError::missing_files(&missing_files));
    }

    // Filter HTML files to only include required ones, then sort
    let mut sorted_files: Vec<_> = html_files
        .into_iter()
        .filter(|path| {
            path.file_stem()
                .and_then(|stem| stem.to_str())
                .map(|s| required_names.contains(s))
                .unwrap_or(false)
        })
        .collect();

    // Sort files using recursive lexicographic order on :: split
    sorted_files.sort_by(|a, b| {
        let a_name = a.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        let b_name = b.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        compare_cpp_names(a_name, b_name)
    });

    // Process files by manipulating DOM elements
    let processed_content = {
        // Create an iterator over the sorted files
        let mut files_iter = sorted_files.into_iter();

        if let Some(first_file) = files_iter.next() {
            // Parse the first file as the root document
            let root_html = Html::parse_document(&fs::read_to_string(first_file)?);
            let tree_sink = HtmlTreeSink::new(root_html);

            // Get the body element of the root document
            let body_selector = Selector::parse("body").unwrap();
            let body_id = {
                let html_ref = tree_sink.0.borrow();
                html_ref
                    .select(&body_selector)
                    .next()
                    .map(|e| e.id())
                    .ok_or_else(|| {
                        AppError::HtmlParsingError {
                            file: "root document".to_string(),
                            reason: "Could not find body element".to_string(),
                        }
                    })
            }?;

            // Process remaining files
            for file in files_iter {
                // Parse the current file
                let current_html = Html::parse_document(&fs::read_to_string(file)?);

                // Get all elements from the current file's body
                let current_body_selector = Selector::parse("body").unwrap();
                if let Some(current_body) = current_html.select(&current_body_selector).next() {
                    // Create a temporary container element
                    let container_id = {
                        let container_name =
                            QualName::new(None, Default::default(), LocalName::from("div"));
                        tree_sink.create_element(container_name, Vec::new(), Default::default())
                    };

                    // Add the container to the root body
                    tree_sink.append(&body_id, NodeOrText::AppendNode(container_id));

                    // Add all children of the current body to the container
                    for child in current_body.children() {
                        match *child.value() {
                            scraper::node::Node::Element(_) => {
                                // For elements, we need to recreate them in the tree sink
                                if let Some(child_element) = scraper::ElementRef::wrap(child) {
                                    add_element_to_tree(&tree_sink, &container_id, &child_element);
                                }
                            }
                            scraper::node::Node::Text(ref text_node) => {
                                // For text nodes, add directly
                                let mut tendril = StrTendril::new();
                                tendril.push_slice(&text_node.text);
                                tree_sink.append(&container_id, NodeOrText::AppendText(tendril));
                            }
                            _ => {
                                // Skip other node types
                            }
                        }
                    }
                }
            }

            // Convert back to HTML string
            let root_html = tree_sink.0.into_inner();
            let concatenated_content = root_html.html();

            // Process content if not colored
            if colored {
                concatenated_content
            } else {
                flatten_code_blocks(&concatenated_content)?
            }
        } else {
            // No files found
            error!("No HTML files found in cppreference directory");
            return Err(AppError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No HTML files found in cppreference directory",
            )));
        }
    };

    // Save to appropriate file
    let output_file = if colored {
        Path::new("./cppreference_print_colored.html")
    } else {
        Path::new("./cppreference_print.html")
    };

    fs::write(output_file, processed_content)?;
    info!("Saved concatenated references to {:?}", output_file);

    Ok(())
}

/// Recursively add an element and its children to the tree sink
///
/// This function creates a deep copy of an element and all its children
/// in the target tree sink.
///
/// # Arguments
///
/// * `tree_sink` - The HtmlTreeSink to add elements to
/// * `parent_id` - The ID of the parent element
/// * `element` - The element to add
fn add_element_to_tree(
    tree_sink: &HtmlTreeSink,
    parent_id: &<HtmlTreeSink as TreeSink>::Handle,
    element: &scraper::ElementRef,
) {
    // Create the element
    let element_id = {
        let element_name = QualName::new(
            None,
            Default::default(),
            LocalName::from(element.value().name()),
        );

        // Create attributes
        let mut attrs = Vec::new();
        for attr in element.value().attrs() {
            attrs.push(Attribute {
                name: QualName::new(None, Default::default(), LocalName::from(attr.0)),
                value: StrTendril::from(attr.1),
            });
        }

        tree_sink.create_element(element_name, attrs, Default::default())
    };

    // Add the element to the parent
    tree_sink.append(parent_id, NodeOrText::AppendNode(element_id));

    // Add children
    for child in element.children() {
        match *child.value() {
            scraper::node::Node::Element(_) => {
                if let Some(child_element) = scraper::ElementRef::wrap(child) {
                    add_element_to_tree(tree_sink, &element_id, &child_element);
                }
            }
            scraper::node::Node::Text(ref text_node) => {
                let mut tendril = StrTendril::new();
                tendril.push_slice(&text_node.text);
                tree_sink.append(&element_id, NodeOrText::AppendText(tendril));
            }
            _ => {
                // Skip other node types
            }
        }
    }
}
