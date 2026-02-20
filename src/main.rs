#![forbid(unsafe_code)]

//! C++ Reference Manager for Algorithm Competition
//!
//! This program helps algorithm competition participants manage C++ reference documentation
//! from cppreference.com. It provides two main functionalities:
//!
//! # Commands
//!
//! ## `ref download`
//! Extracts C++ reference URLs from Markdown files in `./contents`, downloads the corresponding
//! HTML pages from cppreference.com, and processes them by removing navigation elements.
//! Only downloads missing files unless `--overwrite` is specified.
//!
//! ## `ref print`
//! Concatenates all downloaded HTML files in `./cppreference` into a single file for printing.
//! Supports colored output (preserving syntax highlighting) or flattened output (removing
//! syntax highlighting for non-colored printing).
//!
//! # Usage
//!
//! ```bash
//! # Download C++ references
//! cargo run -- ref download
//! cargo run -- ref download --overwrite
//!
//! # Generate printable HTML
//! cargo run -- ref print           # Flattened output (no syntax highlighting)
//! cargo run -- ref print --colored # Colored output (with syntax highlighting)
//! ```
//!
//! # Directory Structure
//!
//! - `./contents/` - Markdown files containing C++ reference links
//! - `./cppreference/` - Downloaded HTML files from cppreference.com
//! - `./cppreference_print.html` - Generated printable HTML (flattened)
//! - `./cppreference_print_colored.html` - Generated printable HTML (colored)

use clap::{Parser, Subcommand};

// Import modules
mod commands;
mod errors;
mod html;
mod references;
mod utils;

use crate::commands::{download::download_references, print::print_references};
use crate::errors::AppError;

#[derive(Parser, Debug)]
#[command(name = "cppreference-downloader")]
#[command(about = "Download C++ references from cppreference.com")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Ref {
        #[command(subcommand)]
        subcommand: RefSubcommands,
    },
}

#[derive(Subcommand, Debug)]
enum RefSubcommands {
    Download {
        /// Overwrite existing files
        #[arg(long, default_value_t = false)]
        overwrite: bool,
    },
    Print {
        /// Include colored output
        #[arg(long, default_value_t = false)]
        colored: bool,
    },
}

/**
 * Main function
 *
 * This function parses command-line arguments, initializes the logger,
 * and runs the appropriate command based on user input.
 *
 * @return Result indicating success or error
 */
fn main() -> Result<(), AppError> {
    let cli = Cli::parse();

    // Initialize logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    match &cli.command {
        Commands::Ref { subcommand } => match subcommand {
            RefSubcommands::Download { overwrite } => {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();

                rt.block_on(download_references(*overwrite))
            }
            RefSubcommands::Print { colored } => print_references(*colored),
        },
    }
}
