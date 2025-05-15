use crate::constants::{BANGBANG93, DownloadSource, MINECRAFT_ROOT};
use crate::error_handling::DownloadError;
use crate::managers::version;
use crate::utils::downloader::{CLIENT, Downloader, FileInfo};

use std::borrow::Cow;
use std::path::Path;

use serde_json::Value;

/// Downloads the manifest which contains metadata of a specific Minecraft version.
pub async fn download_specific_version_manifest(
    ver: &str,
    src: DownloadSource,
) -> Result<(), DownloadError> {
    let manifest_path = format!("{}/versions/{}", MINECRAFT_ROOT, ver);
    let manifest_name = format!("{}.json", ver);

    let manifest = version::read_version_manifest().await?;

    let ver = manifest[ver].clone();

    if let (Value::String(url), Value::String(sha1)) = (&ver["url"], &ver["sha1"]) {
        let mut url = url.to_owned();

        if src == DownloadSource::Bangbang93 {
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
