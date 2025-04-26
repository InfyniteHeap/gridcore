//! # Download
//!
//! This module is responsible to download Minecraft files and mod files.

// TODO: Implement multi-threaded downloading.

pub mod game;
pub mod mods;

use crate::checksum;
use crate::error_handling::DownloadError;
use crate::file_system;

use std::path::{Path, PathBuf};
use std::time::Duration;

use reqwest::{Client, Response};
use tokio::time;

/// The wait time for a single download task.
pub(crate) const DURATION: Duration = Duration::from_secs(10);

#[derive(Clone)]
pub(crate) struct FileInfo {
    /// The location where the file is stored.
    pub(crate) path: PathBuf,
    /// The file name.
    pub(crate) name: String,
    /// The source address that can download the file.
    pub(crate) url: String,
    /// The SHA1 hash that is used to check integrity of the file.
    pub(crate) sha1: String,
}

/// Downloads a single file.
///
/// It returns `Ok(())` when successfully downloaded a file and verified its integrity,
/// or `Err(DownloadError)` when there are some errors happened during downloading.
pub(crate) async fn download_file(
    client: &Client,
    file_info: FileInfo,
) -> Result<(), DownloadError> {
    // This function will fast return when these conditions are satisfied:
    // 1. The target file exists.
    // 2. Its corresponding SHA1 value is equal to the provided one.
    // HACK: There might has a better method to do so.
    if file_info.path.join(&file_info.name).exists()
        && (checksum::calculate_sha1(&file_info.path, &file_info.name).await? == file_info.sha1)
    {
        return Ok(());
    }

    download_file_unchecked(client, &file_info.path, &file_info.name, &file_info.url).await?;

    // To check whether the file is successfully downloaded,
    // we must verify its SHA1 value.
    // If the value is equal to the given one,
    // We can confirm that this file is successfully
    // downloaded!
    if checksum::calculate_sha1(&file_info.path, &file_info.name).await? == file_info.sha1 {
        Ok(())
    } else {
        Err(DownloadError::CheckIntegrityError)
    }
}

pub(crate) async fn download_file_unchecked(
    client: &Client,
    path: &Path,
    name: &str,
    url: &str,
) -> Result<(), DownloadError> {
    let response = match time::timeout(DURATION, get_file_from_remote(client, url)).await {
        Ok(res) => res?,
        Err(e) => {
            return Err(DownloadError::InternetError(format!(
                r#"
Time out when waiting for response from remote server!
File details:
    Name: {},
    path: {},
    URL: {},
Error details: {},
                "#,
                name,
                path.to_string_lossy(),
                url,
                e
            )));
        }
    };

    if response.status().is_success() {
        file_system::write_into_file(path, name, &response.bytes().await?)
            .await
            .map_err(Into::into)
    } else {
        Err(DownloadError::InternetError(response.status().to_string()))
    }
}

async fn get_file_from_remote(client: &Client, url: &str) -> Result<Response, DownloadError> {
    client.get(url).send().await.map_err(Into::into)
}
