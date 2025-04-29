use crate::error_handling::DownloadError;
use crate::managers::game::download::{BANGBANG93, CLIENT, Category, DownloadSource};
use crate::path::MINECRAFT_ROOT;
use crate::utils::downloader::{Downloader, FileInfo};

use std::borrow::Cow;
use std::path::Path;

use serde_json::Value;

/// Downloads `<vision_number>.jar` on local machine, e.g. `1.21.5.jar`.
pub(super) async fn download_jar(
    data: &Value,
    ver: &str,
    src: DownloadSource,
    category: Category,
) -> Result<(), DownloadError> {
    let file_path = format!("{}/versions/{}", MINECRAFT_ROOT, ver);
    let file_name = format!("{}.jar", ver);

    if let (Value::String(url), Value::String(sha1)) = (
        &data["downloads"][category.to_string()]["url"],
        &data["downloads"][category.to_string()]["sha1"],
    ) {
        let mut url = url.to_owned();

        if src == DownloadSource::Bangbang93 {
            let idx = "https://piston-data.mojang.com/".len();
            url = format!("{}/{}", BANGBANG93, &url[idx..]);
        }

        let file_info = FileInfo {
            path: Cow::from(Path::new(&file_path)),
            name: file_name.into(),
            url: url.into(),
            sha1: Some(Cow::from(sha1)),
        };
        let downloader = Downloader::new(&CLIENT, file_info);
        downloader.download_file().await?;
    }

    Ok(())
}
