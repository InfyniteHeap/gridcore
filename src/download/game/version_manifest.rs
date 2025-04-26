use super::{DownloadSource::*, BANGBANG93, CLIENT, DOWNLOAD_SOURCE, OFFICIAL};
use crate::download::{self, FileInfo};
use crate::file_system;
use crate::mc_version;
use crate::path::MINECRAFT_ROOT;

use std::path::{Path, PathBuf};

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
    let response = CLIENT.get(&url).send().await?;

    if response.status().is_success() {
        file_system::write_into_file(
            Path::new(&manifest_path),
            manifest_name,
            &response.bytes().await?,
        )
        .await?;
    }

    Ok(())
}

/// Downloads the manifest which contains metadata of a specific Minecraft version.
pub async fn download_specific_version_manifest(version: &str) -> anyhow::Result<()> {
    let manifest_path = format!("{}/versions/{}", MINECRAFT_ROOT, version);
    let manifest_name = format!("{}.json", version);

    let manifest = mc_version::read_version_manifest().await?;

    let ver = manifest[version].clone();

    if let (Value::String(url), Value::String(sha1)) = (&ver["url"], &ver["sha1"]) {
        let mut url = url.to_owned();

        if *DOWNLOAD_SOURCE.read().await == Bangbang93 {
            let len = "https://piston-meta.mojang.com/".len();
            url = format!("{}/{}", BANGBANG93, &url[len..]);
        }

        let file_info = FileInfo {
            path: PathBuf::from(manifest_path),
            name: manifest_name,
            url,
            sha1: sha1.to_owned(),
        };

        download::download_file(&CLIENT, file_info).await?
    }

    Ok(())
}
