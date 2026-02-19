use clap::{Command, ValueEnum};
use log::{debug, info};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Missing URL in {file}:{line}")]
    MissingUrl { file: String, line: usize },
    #[error("Invalid URL in {file}:{line}: {url}")]
    InvalidUrl { file: String, line: usize, url: String },
    #[error("Duplicate entry with conflicting information: {name} at {url1} and {url2}")]
    DuplicateConflict { name: String, url1: String, url2: String },
    #[error("Invalid file format in {file}:{line}")]
    InvalidFileFormat { file: String, line: usize },
}

#[derive(Clone, ValueEnum, Debug)]
enum CommandType {
    Ref,
}

#[derive(Clone, ValueEnum, Debug)]
enum RefSubcommand {
    Download,
}

#[derive(Debug)]
struct CppReference {
    name: String,
    url: String,
    file: String,
    line: usize,
}

async fn download_references() -> Result<(), AppError> {
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
    
    info!("Found {} unique references after deduplication", unique_references.len());
    
    // Download references
    download_files(unique_references).await?;
    
    info!("Download completed successfully");
    Ok(())
}

fn find_markdown_files(dir: &Path) -> Result<Vec<PathBuf>, io::Error> {
    let mut files = Vec::new();
    
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                files.extend(find_markdown_files(&path)?);
            } else if path.extension() == Some(std::ffi::OsStr::new("md")) {
                files.push(path);
            }
        }
    }
    
    Ok(files)
}

fn extract_references(files: &[PathBuf]) -> Result<Vec<CppReference>, AppError> {
    let mut references = Vec::new();
    
    // Regex to match C++ reference entries in markdown
    let regex = Regex::new(r#"\|\s*[^|]+\|\s*\[`(std::[^`]+)`\s*(?:\([^)]+\))?\]\((https://en.cppreference.com/w/cpp/[^"]+)\)\s*\|"#)?;
    
    for file in files {
        let file_str = file.to_str().unwrap_or_default();
        let content = fs::read_to_string(file)?;
        
        for (line_num, line) in content.lines().enumerate() {
            if let Some(captures) = regex.captures(line) {
                let name = captures.get(1).map(|m| m.as_str().trim().to_string())
                    .ok_or_else(|| AppError::InvalidFileFormat { 
                        file: file_str.to_string(), 
                        line: line_num + 1 
                    })?;
                
                let url = captures.get(2).map(|m| m.as_str().trim().to_string())
                    .ok_or_else(|| AppError::MissingUrl { 
                        file: file_str.to_string(), 
                        line: line_num + 1 
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

fn deduplicate_references(references: Vec<CppReference>) -> Result<HashMap<String, CppReference>, AppError> {
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

async fn download_files(references: HashMap<String, CppReference>) -> Result<(), AppError> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .build()?;
    
    let output_dir = Path::new("./cppreference");
    
    for (name, ref_item) in references {
        let filename = format!("{}.html", name);
        let output_path = output_dir.join(filename);
        
        info!("Downloading {} from {}", name, ref_item.url);
        
        let response = client.get(&ref_item.url).send().await?;
        let content = response.text().await?;
        
        fs::write(output_path, content)?;
        debug!("Saved {} to cppreference/{}.html", name, name);
        
        // Add a small delay to avoid overloading the server
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
    
    Ok(())
}

fn main() -> Result<(), AppError> {
    let matches = Command::new("cppreference-downloader")
        .about("Download C++ references from cppreference.com")
        .subcommand(
            Command::new("ref")
                .about("Reference operations")
                .subcommand(
                    Command::new("download")
                        .about("Download C++ references from markdown files")
                )
        )
        .get_matches();
    
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    
    if let Some(ref_subcommand) = matches.subcommand_matches("ref") {
        if ref_subcommand.subcommand_matches("download").is_some() {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            
            rt.block_on(download_references())
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}
