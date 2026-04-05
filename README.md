# C++ Reference Manager for Algorithm Competition

A Rust-based tool for algorithm competition participants to manage C++ reference documentation from [cppreference.com](https://en.cppreference.com/).

## Features

- **Extract C++ References**: Automatically extracts C++ reference URLs from Markdown files
- **Download HTML Pages**: Downloads reference pages from cppreference.com
- **Multi-language Support**: Supports both English and Chinese (zh) versions
- **Process HTML**: Removes unnecessary navigation elements for cleaner printing
- **Concatenate Files**: Combines multiple HTML files into a single printable document
- **Syntax Highlighting Control**: Supports both colored and flattened output modes

## Installation

### Prerequisites

- Rust 1.70 or later
- Cargo

### Build

```bash
git clone <repository-url>
cd algcmp
cargo build --release
```

## Usage

### Directory Structure

```
algcmp/
├── contents/           # Markdown files containing C++ reference links
│   ├── Binary_search.md
│   ├── Permutation.md
│   └── ...
├── cppreference_en/    # Downloaded English HTML files (created by download command)
│   ├── std::midpoint.html
│   ├── std::next_permutation.html
│   └── ...
├── cppreference_zh/    # Downloaded Chinese HTML files (created by download command)
│   ├── std::midpoint.html
│   ├── std::next_permutation.html
│   └── ...
├── cppreference_en_print.html           # Generated English printable HTML (flattened)
├── cppreference_en_print_colored.html   # Generated English printable HTML (colored)
├── cppreference_zh_print.html           # Generated Chinese printable HTML (flattened)
└── cppreference_zh_print_colored.html   # Generated Chinese printable HTML (colored)
```

### Commands

#### Download C++ References

Extracts C++ reference URLs from Markdown files in `./contents` and downloads the corresponding HTML pages.

```bash
# Download English version (default)
cargo run -- ref download
cargo run -- ref download --lang en

# Download Chinese version
cargo run -- ref download --lang zh

# Re-download all files (overwrite existing)
cargo run -- ref download --overwrite
cargo run -- ref download --lang zh --overwrite
```

**Note for Chinese version**: If you encounter redirect issues to English pages, you can add browser request headers (including Cookie) in `src/commands/download.rs`.

#### Generate Printable HTML

Concatenates all downloaded HTML files into a single printable document.

```bash
# Generate English version (default)
cargo run -- ref print
cargo run -- ref print --lang en

# Generate Chinese version
cargo run -- ref print --lang zh

# Generate colored output (preserves syntax highlighting)
cargo run -- ref print --colored
cargo run -- ref print --lang zh --colored
```

### Markdown Format

The tool expects C++ references in Markdown tables with the following format:

```markdown
| Category | Reference | Description |
|----------|-----------|-------------|
| Algorithm | [`std::sort`](https://en.cppreference.com/w/cpp/algorithm/sort) | Sorts elements |
| Algorithm | [`std::binary_search`](https://en.cppreference.com/w/cpp/algorithm/binary_search) (C++20) | Binary search |
```

The regex pattern matches:
- Table rows with C++ references
- Optional version information like `(C++20)`
- Links to `https://en.cppreference.com/w/cpp/...`

### Sorting Order

Files are sorted using recursive dictionary order on `::` split:

- `std::binary_search` < `std::sort` < `std::vector`
- `std::chrono::duration` < `std::chrono::time_point`
- `std::vector` < `std::vector::iterator`

## Architecture

```
src/
├── main.rs           # Entry point and CLI definition
├── commands/
│   ├── mod.rs        # Command module exports
│   ├── download.rs   # Download command implementation
│   └── print.rs      # Print command implementation
├── html/
│   ├── mod.rs        # HTML module exports
│   └── processing.rs # HTML processing functions
├── references.rs     # Reference extraction and management
├── errors.rs         # Error type definitions
└── utils.rs          # Utility functions (file system operations)
```

### Key Components

- **`references.rs`**: Handles extraction, deduplication, and sorting of C++ references
- **`html/processing.rs`**: HTML processing functions (remove navigation, flatten code blocks)
- **`commands/download.rs`**: Downloads HTML pages and processes them
- **`commands/print.rs`**: Concatenates HTML files with optional syntax highlighting removal
- **`errors.rs`**: Defines application-specific error types

## Dependencies

- **clap**: Command-line argument parsing (with derive feature)
- **scraper**: HTML parsing and manipulation
- **markup5ever**: HTML tree manipulation
- **reqwest**: HTTP client for downloading pages
- **tokio**: Async runtime
- **regex**: Pattern matching for reference extraction
- **thiserror**: Error handling
- **log** / **env_logger**: Logging

## Error Handling

The tool provides detailed error messages for common issues:

- Missing files: Reports which HTML files are missing
- URL conflicts: Detects when the same C++ name has different URLs
- Invalid format: Reports file and line number for parsing errors

## Development

### Run Tests

```bash
cargo test
```

### Check Code

```bash
cargo check
cargo clippy
```

### Format Code

```bash
cargo fmt
```

## License

MIT License
