use crate::error_handling::DownloadError;
use crate::managers::game::download::CLIENT;
use crate::path::MINECRAFT_ROOT;
use crate::utils::downloader::{Downloader, FileInfo};

use std::borrow::Cow;
use std::path::Path;

use serde_json::Value;

pub(super) async fn download_logging_config(data: &Value) -> Result<(), DownloadError> {
    if let (Value::String(id), Value::String(sha1), Value::String(url)) = (
        &data["logging"]["client"]["file"]["id"],
        &data["logging"]["client"]["file"]["sha1"],
        &data["logging"]["client"]["file"]["url"],
    ) {
        let file_path = format!("{}/assets/log_configs", MINECRAFT_ROOT);

        let file_info = FileInfo {
            path: Cow::from(Path::new(&file_path)),
            name: Cow::from(id),
            url: Cow::from(url),
            sha1: Some(Cow::from(sha1)),
        };
        let downloader = Downloader::new(&CLIENT, file_info);
        downloader.download_file().await?;
    }

    Ok(())
}
