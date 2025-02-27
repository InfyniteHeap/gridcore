mod assets;
mod jar;
mod libraries;
mod logging_config;

use super::Category;
use crate::json;
use crate::path::MINECRAFT_ROOT;

use std::path::Path;

/// Download Minecraft.
pub async fn download_files(version: &str, category: Category) -> anyhow::Result<()> {
    let manifest_path = format!("{}/versions/{}", MINECRAFT_ROOT, version);
    let manifest_name = format!("{}.json", version);

    let data = json::read(Path::new(&manifest_path), &manifest_name).await?;

    jar::download_jar(version, &data, category).await?;
    libraries::download_libraries(&data).await?;
    assets::download_assets(&data).await?;
    logging_config::download_logging_config(&data).await?;

    Ok(())
}
