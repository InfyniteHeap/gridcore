pub mod game;
pub mod mods;

use reqwest::get;

use crate::file_system::{create_file, write_file};

pub async fn download_content(file_name: &str, dir: &str, url: &str) -> anyhow::Result<()> {
    let mut file = create_file(dir, file_name)?;

    let response = get(url).await?.text().await?;

    write_file(&mut file, response)?;

    Ok(())
}
