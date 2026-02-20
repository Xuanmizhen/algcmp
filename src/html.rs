//! HTML processing utilities
//!
//! This module provides functionality for processing HTML documents:
//!
//! - Removing navigation elements from cppreference pages
//! - Flattening code blocks for non-colored printing
//! - Concatenating multiple HTML documents

mod processing;

pub use processing::{flatten_code_blocks, remove_navigation_elements};
