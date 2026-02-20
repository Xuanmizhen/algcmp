use log::{error, info};
use markup5ever::{
    LocalName, QualName,
    interface::{NodeOrText, TreeSink},
    tendril::StrTendril,
};
use scraper::{Html, HtmlTreeSink, Selector};
use std::{fs, path::Path};

use crate::{errors::AppError, utils::get_html_files};

/**
 * Print references by concatenating HTML files
 *
 * This function:
 * 1. Checks if all HTML files in ./cppreference are present
 * 2. If not, error out
 * 3. If yes, concatenate them in alphabetical order
 * 4. For non-colored output, flatten pre elements with class "de1"
 * 5. Save the result to the appropriate file
 *
 * @param colored Whether to include colored output
 * @return Result indicating success or error
 */
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

    // Get all HTML files in cppreference directory
    let html_files = get_html_files(cppreference_dir)?;

    if html_files.is_empty() {
        error!("No HTML files found in cppreference directory");
        return Err(AppError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No HTML files found in cppreference directory",
        )));
    }

    // Sort files alphabetically
    let mut sorted_files = html_files;
    sorted_files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    // Concatenate files
    let mut concatenated_content = String::new();

    for file in &sorted_files {
        let content = fs::read_to_string(file)?;
        concatenated_content.push_str(&content);
    }

    // Process content if not colored
    let processed_content = if colored {
        concatenated_content
    } else {
        process_for_printing(&concatenated_content)?
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

/**
 * Process HTML for printing (flatten pre elements)
 *
 * This function flattens pre elements with class "de1" by replacing them with
 * new pre elements containing only their text content, removing any child elements
 * that provide syntax highlighting.
 *
 * @param content HTML content
 * @return Result containing processed HTML
 */
pub fn process_for_printing(content: &str) -> Result<String, AppError> {
    // Parse HTML
    let html = Html::parse_document(content);

    // Create HtmlTreeSink for manipulation
    let tree_sink = HtmlTreeSink::new(html);

    // Find all pre elements with class "de1"
    let pre_selector = Selector::parse("pre.de1").unwrap();

    // Get all pre.de1 elements and their parent elements
    let pre_elements_with_parent: Vec<_> = {
        let html_ref = tree_sink.0.borrow();
        html_ref
            .select(&pre_selector)
            .map(|e| {
                let pre_id = e.id();
                let parent_id = e.parent().map(|p| p.id());
                (pre_id, parent_id)
            })
            .collect()
    };

    // Process each pre.de1 element
    for pre_id in pre_elements_with_parent.into_iter().map(|(id, _)| id) {
        // Get the text content of the pre element
        let text_content = {
            let html_ref = tree_sink.0.borrow();
            html_ref
                .select(&pre_selector)
                .find(|e| e.id() == pre_id)
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default()
        };

        // Remove all children from the pre element
        // We'll do this by creating a temporary node to hold the children
        // then removing that node
        let temp_id = {
            let temp_name = QualName::new(None, Default::default(), LocalName::from("temp"));
            tree_sink.create_element(temp_name, Vec::new(), Default::default())
        };

        // Move all children of pre_id to temp_id
        tree_sink.reparent_children(&pre_id, &temp_id);

        // Remove the temporary node (which now contains all the old children)
        tree_sink.remove_from_parent(&temp_id);

        // Add the text content as a new text node
        tree_sink.append(
            &pre_id,
            NodeOrText::AppendText(StrTendril::from(text_content)),
        );
    }

    // Convert back to HTML string
    let modified_html = tree_sink.0.into_inner();
    Ok(modified_html.html())
}
