//! # Download
//!
//! This module is responsible to download Minecraft files and mod files.

// TODO: Implement multi-threaded downloading.

use crate::error_handling::DownloadError;
use crate::file_system;
use crate::utils::sha1_checker;

use std::borrow::Cow;
use std::path::Path;
use std::time::Duration;

use reqwest::{Client, Response};
use tokio::time;

/// The wait time for a single download task.
pub(crate) const DURATION: Duration = Duration::from_secs(10);

#[derive(Clone)]
pub(crate) struct FileInfo<'f> {
    /// The location where the file is stored.
    pub(crate) path: Cow<'f, Path>,
    /// The file name.
    pub(crate) name: Cow<'f, str>,
    /// The source address that can download the file.
    pub(crate) url: Cow<'f, str>,
    /// The SHA1 hash that is used to check integrity of the file.
    pub(crate) sha1: Option<Cow<'f, str>>,
}

pub(crate) struct Downloader<'d, 'f: 'd> {
    client: &'d Client,
    file_info: &'d FileInfo<'f>,
}

impl<'d, 'f: 'd> Downloader<'d, 'f> {
    pub(crate) fn new(client: &'d Client, file_info: &'f FileInfo) -> Self {
        Self { client, file_info }
    }

    /// Downloads a single file.
    ///
    /// It returns `Ok(())` when successfully downloaded the file and verified its integrity (if `sha1` exists),
    /// or `Err(DownloadError)` when there are some errors happened during downloading.
    ///
    /// If `sha1` doesn't exist, then downloader will also download the file, only except checking its integrity.
    pub(crate) async fn download_file(&self) -> Result<(), DownloadError> {
        // This function will fast return when these conditions are satisfied:
        // 1. The target file exists.
        // 2. Its corresponding SHA1 value is equal to the provided one.
        // Note: If `sha1` doesn't exist, then the second condition will be omitted.
        // HACK: Nested `if` expressions! There might has a better method to do so.
        if self.file_info.path.join(&*self.file_info.name).exists() {
            if let Some(sha1) = &self.file_info.sha1 {
                if &sha1_checker::calculate_sha1(&self.file_info.path, &self.file_info.name).await?
                    == sha1
                {
                    return Ok(());
                }
            }
        }

        self.download_file_inner().await?;

        // To check whether the file is successfully downloaded,
        // we must verify its SHA1 value.
        // If the value is equal to the given one,
        // We can confirm that this file is successfully
        // downloaded!
        if let Some(sha1) = &self.file_info.sha1 {
            if &sha1_checker::calculate_sha1(&self.file_info.path, &self.file_info.name).await?
                == sha1
            {
                Ok(())
            } else {
                Err(DownloadError::CheckIntegrityError)
            }
        } else {
            Ok(())
        }
    }

    async fn download_file_inner(&self) -> Result<(), DownloadError> {
        let response = match time::timeout(
            DURATION,
            get_file_from_remote(self.client, &self.file_info.url),
        )
        .await
        {
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
                    self.file_info.name,
                    self.file_info.path.to_string_lossy(),
                    self.file_info.url,
                    e
                )));
            }
        };

        if response.status().is_success() {
            file_system::write_into_file(
                &self.file_info.path,
                &self.file_info.name,
                &response.bytes().await?,
            )
            .await
            .map_err(Into::into)
        } else {
            Err(DownloadError::InternetError(response.status().to_string()))
        }
    }
}

/// Gets file stream from remote server.
///
/// Sometimes, people don't expect to save bytes on local machine;
/// they only want to temperately use them.
pub(crate) async fn get_file_from_remote(
    client: &Client,
    url: &str,
) -> Result<Response, DownloadError> {
    client.get(url).send().await.map_err(Into::into)
}
