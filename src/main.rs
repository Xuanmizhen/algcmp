#![forbid(unsafe_code)]

//! C++ Reference Downloader
//!
//! This program extracts C++ reference URLs from Markdown files, downloads the corresponding
//! HTML pages, and processes them by removing specified elements. It supports checking for
//! existing files and only downloading missing ones, with an option to overwrite existing files.

use clap::{Parser, Subcommand};

// Import modules
mod commands;
mod errors;
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
