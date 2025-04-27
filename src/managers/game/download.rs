pub mod mc_file;
pub mod version_manifest;

use std::sync::{Arc, LazyLock};

use reqwest::Client;
use tokio::sync::RwLock;

const OFFICIAL: &str = "https://piston-meta.mojang.com";
const BANGBANG93: &str = "https://bmclapi2.bangbang93.com";
const ASSETS_OFFICIAL: &str = "https://resources.download.minecraft.net";
const ASSETS_BANGBANG93: &str = "https://bmclapi2.bangbang93.com/assets";

static DOWNLOAD_SOURCE: RwLock<DownloadSource> = RwLock::const_new(DownloadSource::Official);
/// The global-shared client.
static CLIENT: LazyLock<Arc<Client>> = LazyLock::new(|| Arc::new(Client::new()));

#[derive(Copy, Clone, PartialEq)]
pub enum DownloadSource {
    Official,
    Bangbang93,
}

pub enum Category {
    Client,
    Server,
}

pub async fn select_download_source(res: DownloadSource) {
    *DOWNLOAD_SOURCE.write().await = res;
}

async fn select_category(category: &Category) -> &'static str {
    match category {
        Category::Client => "client",
        Category::Server => "server",
    }
}
