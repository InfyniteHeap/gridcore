mod mc_file;
mod version_manifest;

use crate::constants::{Category, DownloadSource};
use crate::error_handling::DownloadError;

use std::num::NonZero;
use std::thread;

pub struct MinecraftDownloader {
    version: &'static str,
    source: DownloadSource,
    category: Category,
    thread_count: usize,
}

impl MinecraftDownloader {
    pub fn new(
        ver: &'static str,
        src: DownloadSource,
        category: Category,
        count: Option<usize>,
    ) -> Self {
        let count = count.unwrap_or(
            thread::available_parallelism()
                .unwrap_or(NonZero::new(8).unwrap())
                .get(),
        );

        Self {
            version: ver,
            source: src,
            category,
            thread_count: count,
        }
    }

    pub async fn download_minecraft(&self) -> Result<(), DownloadError> {
        version_manifest::download_specific_version_manifest(self.version, self.source).await?;
        mc_file::download_files(self.version, self.source, self.category).await?;

        Ok(())
    }

    pub fn get_thread_count(&self) -> usize {
        self.thread_count
    }
}
