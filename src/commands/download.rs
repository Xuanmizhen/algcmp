//! Download command implementation
//!
//! This module provides functionality to download C++ reference pages from
//! cppreference.com. It extracts URLs from Markdown files, downloads the
//! corresponding HTML pages, and processes them by removing navigation elements.

use log::{debug, info};
use std::{fs, path::Path};
use tokio::time::Duration;

use crate::{
    errors::AppError, html::remove_navigation_elements, references::get_required_references,
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
pub async fn download_references(overwrite: bool, lang: &str) -> Result<(), AppError> {
    info!("Starting C++ reference downloader (language: {})", lang);

    let output_dir_name = format!("./cppreference_{}", lang);
    let output_dir = Path::new(&output_dir_name);
    if !output_dir.exists() {
        info!("Creating output directory: {:?}", output_dir);
        fs::create_dir_all(output_dir)?;
    }

    let unique_references = get_required_references()?;

    info!(
        "Found {} unique references to download",
        unique_references.len()
    );

    download_files(unique_references, overwrite, lang).await?;

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
    lang: &str,
) -> Result<(), AppError> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .build()?;

    let output_dir_name = format!("./cppreference_{}", lang);
    let output_dir = Path::new(&output_dir_name);

    for (name, ref_item) in references {
        let filename = format!("{}.html", name);
        let output_path = output_dir.join(&filename);

        if output_path.exists() && !overwrite {
            debug!("File already exists: {}.html, skipping download", name);
            continue;
        }

        // URL 转换
        //
        // 中文版：
        // 1. 域名：en.cppreference.com → cppreference.cn
        // 2. 后缀：移除 .html（中文版 URL 没有 .html 后缀）
        //
        // 示例：
        //   英文: https://en.cppreference.com/w/cpp/numeric/bit_floor.html
        //   中文: https://cppreference.cn/w/cpp/numeric/bit_floor
        let url = if lang == "zh" {
            ref_item.url.replace("en.cppreference.com", "cppreference.cn").trim_end_matches(".html").to_string()
        } else {
            ref_item.url.clone()
        };

        info!("Downloading {} from {}", name, url);

        let request_builder = client.get(&url);
        if lang == "zh" {
            // 可选：添加浏览器请求头以避免被重定向到英文版
            // 如需使用，请取消注释并根据需要填写
            // request_builder = request_builder
            //     .header("Accept-Language", "zh-CN,zh-Hans;q=0.9")
            //     .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
            //     .header("Sec-Fetch-Dest", "document")
            //     .header("Sec-Fetch-Mode", "navigate")
            //     .header("Sec-Fetch-Site", "none")
            //     .header("Cookie", "在此处填入你的浏览器 Cookie");
        }
        let response = request_builder.send().await?;
        let final_url = response.url().to_string();

        // 重定向检查
        //
        // 此检查确保我们下载的 URL 与请求的 URL 完全一致
        // 主要用途：
        // 1. 中文版下载时：防止被重定向到 en.cppreference.com 英文版
        // 2. 确保下载到的是我们真正期望的页面，而不是其他语言或错误页面
        //
        // 注意：如果正常的重定向（如旧 URL 重定向到新 URL）也会被阻止
        // 如需允许重定向，请注释掉此检查
        if final_url != url {
            return Err(AppError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("URL {} redirected to {}", url, final_url),
            )));
        }

        let content = response.text().await?;

        let processed_content = remove_navigation_elements(&content, &name)?;

        fs::write(output_path, processed_content)?;
        debug!("Saved {} to {}/{}.html", name, output_dir_name, name);

        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    Ok(())
}
