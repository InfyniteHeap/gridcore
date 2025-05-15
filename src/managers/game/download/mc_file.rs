mod assets;
mod jar;
mod libraries;
mod logging_config;

use crate::constants::{Category, DownloadSource, MINECRAFT_ROOT};
use crate::error_handling::DownloadError;
use crate::utils::json_processer;

/// Downloads Minecraft.
pub async fn download_files(
    ver: &str,
    src: DownloadSource,
    category: Category,
) -> Result<(), DownloadError> {
    let manifest_path = format!("{}/versions/{}", MINECRAFT_ROOT, ver);
    let manifest_name = format!("{}.json", ver);

    let data = json_processer::read(&manifest_path, &manifest_name).await?;

    // let download_manifest = Vec::new();

    // HACK: We'd better first generate a manifest that stores all the files,
    // and then download them at once.
    // If we do so, we can monitor remaining files that have not been downloaded yet better.
    jar::download_jar(&data, ver, src, category).await?;
    libraries::download_libraries(&data, src).await?;
    assets::download_assets(&data, src).await?;
    logging_config::download_logging_config(&data).await?;

    Ok(())
}
