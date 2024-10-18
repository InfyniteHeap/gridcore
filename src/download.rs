//! # Download
//!
//! This module is responsible to download Minecraft files and mod files.

pub mod game;
pub mod mods;

use crate::checksum;
use crate::error_handling::DownloadError;
use crate::file_system;

use std::path::PathBuf;
use std::sync::LazyLock;
use std::thread;
use std::time::Duration;

use reqwest::Client;
use tokio::sync::Mutex;
use tokio::time;

pub(crate) const DURATION: Duration = Duration::from_secs(10);

pub(crate) static THREAD_COUNT: LazyLock<Mutex<usize>> =
    LazyLock::new(|| Mutex::new(thread::available_parallelism().unwrap().get()));

#[derive(Clone)]
pub(crate) struct DownloadFileInfo {
    pub(crate) path: PathBuf,
    pub(crate) name: String,
    pub(crate) url: String,
    pub(crate) sha1: String,
}

pub(crate) async fn download_file(
    client: &Client,
    retry_times: u8,
    file_info: &DownloadFileInfo,
) -> Result<(), DownloadError> {
    // This function will fast return when these conditions are satisfied:
    // 1. The target file exists.
    // 2. Its corresponding SHA1 value is equal to the provided one.
    if file_info.path.join(&file_info.name).exists()
        && (checksum::calculate_sha1(&file_info.path, &file_info.name).await? == file_info.sha1)
    {
        return Ok(());
    }

    for times in 1..=retry_times {
        let response = match time::timeout(DURATION, client.get(&file_info.url).send()).await {
            Ok(Ok(res)) => res,
            Ok(Err(_)) | Err(_) => {
                eprintln!(
                    "Failed to download {}. Retrying {} {}",
                    file_info.name,
                    times,
                    if times == 1 { "time" } else { "times" }
                );
                continue;
            }
        };

        if response.status().is_success() {
            // If success, we write raw bytes into a file.
            file_system::write_into_file(
                &file_info.path,
                &file_info.name,
                &response.bytes().await.unwrap_or_default(),
            )
            .await?;

            // If the SHA1 value is equal to the given one,
            // We can confirm that this file is successfully
            // downloaded!
            if checksum::calculate_sha1(&file_info.path, &file_info.name).await? == file_info.sha1 {
                return Ok(());
            }
        }
    }

    // Function will return `Err(E)` after retrying 3 times with failure.
    Err(DownloadError::OtherError(format!(
        "Failed to download {}! Stop retrying!",
        file_info.name
    )))
}

/// Set thread count.
///
/// If you don't call this function,
/// the default thread count will usually depend on
/// numbers of logical CPU cores.
pub async fn set_thread_count(count: usize) {
    *THREAD_COUNT.lock().await = count;
}
