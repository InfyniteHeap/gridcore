use crate::download::game::CLIENT;
use crate::download::{self, FileInfo};
use crate::path::MINECRAFT_ROOT;

use serde_json::Value;

pub(super) async fn download_logging_config(data: &Value) -> anyhow::Result<()> {
    if let (Value::String(id), Value::String(sha1), Value::String(url)) = (
        &data["logging"]["client"]["file"]["id"],
        &data["logging"]["client"]["file"]["sha1"],
        &data["logging"]["client"]["file"]["url"],
    ) {
        let file_path = format!("{}/assets/log_configs", MINECRAFT_ROOT);

        let file_info = FileInfo {
            path: file_path.into(),
            name: id.to_owned(),
            url: url.to_owned(),
            sha1: sha1.to_owned(),
        };

        download::download_file(&CLIENT, file_info).await?;
    }

    Ok(())
}
