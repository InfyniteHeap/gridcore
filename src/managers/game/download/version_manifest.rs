use super::{BANGBANG93, CLIENT, DOWNLOAD_SOURCE, DownloadSource::*, OFFICIAL};
use crate::managers::version;
use crate::path::MINECRAFT_ROOT;
use crate::utils::downloader::{Downloader, FileInfo};

use std::borrow::Cow;
use std::path::Path;

use serde_json::Value;

/// Downloads the manifest which contains metadata of all the Minecraft versions.
pub async fn download_version_manifest() -> anyhow::Result<()> {
    let manifest_path = format!("{}/versions", MINECRAFT_ROOT);
    let manifest_name = "version_manifest_v2.json";

    let url = format!(
        "{}/mc/game/version_manifest_v2.json",
        match *DOWNLOAD_SOURCE.read().await {
            Official => OFFICIAL,
            Bangbang93 => BANGBANG93,
        }
    );

    // We always download this manifest regardless of the status of this file.
    // This is because: (1) we have no other ways to check integrity of this file,
    // and (2) we can fetch latest Minecraft information via this way.
    let file_info = FileInfo {
        path: Cow::from(Path::new(&manifest_path)),
        name: Cow::from(manifest_name),
        url: url.into(),
        sha1: None,
    };
    let downloader = Downloader::new(&CLIENT, file_info);
    downloader.download_file().await?;

    Ok(())
}

/// Downloads the manifest which contains metadata of a specific Minecraft version.
pub async fn download_specific_version_manifest(version: &str) -> anyhow::Result<()> {
    let manifest_path = format!("{}/versions/{}", MINECRAFT_ROOT, version);
    let manifest_name = format!("{}.json", version);

    let manifest = version::read_version_manifest().await?;

    let ver = manifest[version].clone();

    if let (Value::String(url), Value::String(sha1)) = (&ver["url"], &ver["sha1"]) {
        let mut url = url.to_owned();

        if *DOWNLOAD_SOURCE.read().await == Bangbang93 {
            let len = "https://piston-meta.mojang.com/".len();
            url = format!("{}/{}", BANGBANG93, &url[len..]);
        }

        let file_info = FileInfo {
            path: Cow::from(Path::new(&manifest_path)),
            name: manifest_name.into(),
            url: url.into(),
            sha1: Some(sha1.into()),
        };
        let downloader = Downloader::new(&CLIENT, file_info);
        downloader.download_file().await?;
    }

    Ok(())
}
