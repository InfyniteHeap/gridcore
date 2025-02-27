use crate::download::game::{self, Category, DownloadSource, BANGBANG93, CLIENT, DOWNLOAD_SOURCE};
use crate::download::{self, FileInfo};
use crate::path::MINECRAFT_ROOT;

use std::path::PathBuf;

use serde_json::Value;

/// Download `<vision_number>.jar` on local machine, e.g. `1.21.4.jar`.
pub(super) async fn download_jar(
    version: &str,
    data: &Value,
    category: Category,
) -> anyhow::Result<()> {
    let file_path = format!("{}/versions/{}", MINECRAFT_ROOT, version);
    let file_name = format!("{}.jar", version);

    if let (Value::String(url), Value::String(sha1)) = (
        &data["downloads"][game::select_category(&category).await]["url"],
        &data["downloads"][game::select_category(&category).await]["sha1"],
    ) {
        let mut url = url.to_owned();

        if *DOWNLOAD_SOURCE.read().await == DownloadSource::Bangbang93 {
            let idx = "https://piston-data.mojang.com/".len();
            url = format!("{}/{}", BANGBANG93, &url[idx..]);
        }

        let file_info = FileInfo {
            path: PathBuf::from(file_path),
            name: file_name,
            url,
            sha1: sha1.to_owned(),
        };

        download::download_file(&CLIENT, file_info).await?;
    }

    Ok(())
}
