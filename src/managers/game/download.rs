mod mc_file;
mod version_manifest;

use crate::error_handling::DownloadError;

use std::fmt::Display;
use std::num::NonZero;
use std::sync::{Arc, LazyLock};
use std::thread;

use reqwest::Client;

const OFFICIAL: &str = "https://piston-meta.mojang.com";
const BANGBANG93: &str = "https://bmclapi2.bangbang93.com";
const ASSETS_OFFICIAL: &str = "https://resources.download.minecraft.net";
const ASSETS_BANGBANG93: &str = "https://bmclapi2.bangbang93.com/assets";

/// The global-shared client.
static CLIENT: LazyLock<Arc<Client>> = LazyLock::new(|| Arc::new(Client::new()));

pub struct MinecraftDownloader {
    version: &'static str,
    source: DownloadSource,
    category: Category,
    thread_count: usize,
}

#[derive(Copy, Clone, PartialEq)]
pub enum DownloadSource {
    Official,
    Bangbang93,
}

#[derive(Copy, Clone)]
pub enum Category {
    Client,
    Server,
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
        // We first download version manifest, which is fundamental to
        // subsequence Minecraft downloading operations.
        version_manifest::download_version_manifest(self.source).await?;
        version_manifest::download_specific_version_manifest(self.version, self.source).await?;
        mc_file::download_files(self.version, self.source, self.category).await?;

        Ok(())
    }

    pub fn get_thread_count(&self) -> usize {
        self.thread_count
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Client => String::from("client"),
                Self::Server => String::from("server"),
            }
        )
    }
}
