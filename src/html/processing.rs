//! HTML processing implementation
//!
//! This module contains the core HTML processing functions.

use log::warn;
use markup5ever::{
    LocalName, QualName,
    interface::{NodeOrText, TreeSink},
    tendril::StrTendril,
};
use scraper::{Html, HtmlTreeSink, Selector};

use crate::errors::AppError;

/// Remove navigation elements from HTML content
///
/// This function removes elements with class `t-navbar` and id `mw-head` from
/// the HTML content. These are navigation elements from cppreference.com that
/// are not needed for printing.
///
/// If either element is not found exactly once, a warning is logged and the
/// original content is returned unchanged.
///
/// # Arguments
///
/// * `content` - The HTML content as a string
/// * `name` - The name of the C++ reference (for logging purposes)
///
/// # Returns
///
/// The processed HTML content with navigation elements removed, or the original
/// content if processing fails.
pub fn remove_navigation_elements(content: &str, name: &str) -> Result<String, AppError> {
    let html = Html::parse_document(content);
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

    if !navbar_found || !head_found {
        warn!(
            "Unexpected element count for {}: t-navbar={}, mw-head={}. Skipping element removal.",
            name,
            if navbar_found { 1 } else { 0 },
            if head_found { 1 } else { 0 }
        );
        return Ok(content.to_string());
    }

    let modified_html = tree_sink.0.into_inner();
    Ok(modified_html.html())
}

/// Flatten code blocks in HTML for non-colored printing
///
/// This function flattens `pre` elements with class `de1` by replacing them with
/// new `pre` elements containing only their text content. This removes syntax
/// highlighting spans and other child elements that would not display correctly
/// in a non-colored printout.
///
/// # Arguments
///
/// * `content` - The HTML content to process
///
/// # Returns
///
/// The processed HTML content with flattened code blocks.
pub fn flatten_code_blocks(content: &str) -> Result<String, AppError> {
    let html = Html::parse_document(content);
    let tree_sink = HtmlTreeSink::new(html);

    let pre_selector = Selector::parse("pre.de1").unwrap();

    let pre_elements: Vec<_> = {
        let html_ref = tree_sink.0.borrow();
        html_ref.select(&pre_selector).map(|e| e.id()).collect()
    };

    for pre_id in pre_elements {
        let text_content = {
            let html_ref = tree_sink.0.borrow();
            html_ref
                .select(&pre_selector)
                .find(|e| e.id() == pre_id)
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default()
        };

        let temp_id = {
            let temp_name = QualName::new(None, Default::default(), LocalName::from("temp"));
            tree_sink.create_element(temp_name, Vec::new(), Default::default())
        };

        tree_sink.reparent_children(&pre_id, &temp_id);
        tree_sink.remove_from_parent(&temp_id);
        tree_sink.append(
            &pre_id,
            NodeOrText::AppendText(StrTendril::from(text_content)),
        );
    }

    let modified_html = tree_sink.0.into_inner();
    Ok(modified_html.html())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten_code_blocks_basic() {
        let html = r#"<!DOCTYPE html><html><body><pre class="de1"><span>code</span></pre></body></html>"#;
        let result = flatten_code_blocks(html).unwrap();
        assert!(result.contains("<pre class=\"de1\">code</pre>"));
    }

    #[test]
    fn test_flatten_code_blocks_no_change() {
        let html = r#"<!DOCTYPE html><html><body><p>text</p></body></html>"#;
        let result = flatten_code_blocks(html).unwrap();
        assert!(result.contains("<p>text</p>"));
    }
}
