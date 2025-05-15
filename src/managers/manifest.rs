use crate::constants::{BANGBANG93, DownloadSource, MINECRAFT_ROOT, OFFICIAL};
use crate::error_handling::DownloadError;
use crate::managers::version;
use crate::utils::downloader::{CLIENT, Downloader, FileInfo};

use std::borrow::Cow;
use std::path::Path;

/// Downloads the manifest which contains metadata of all the Minecraft versions.
pub async fn download_version_manifest(src: DownloadSource) -> Result<Vec<String>, DownloadError> {
    let manifest_path = format!("{}/versions", MINECRAFT_ROOT);
    let manifest_name = "version_manifest_v2.json";

    let url = format!(
        "{}/mc/game/version_manifest_v2.json",
        match src {
            DownloadSource::Official => OFFICIAL,
            DownloadSource::Bangbang93 => BANGBANG93,
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

    let vers = version::read_version_manifest().await?;
    let mut versions = Vec::with_capacity(vers.len());

    // Versions are already descendingly ordered according to release time from original manifest.
    vers.keys().for_each(|v| versions.push(v.to_owned()));

    Ok(versions)
}
