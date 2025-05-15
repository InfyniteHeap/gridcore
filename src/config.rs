use crate::constants::{CONFIG_DIRECTORY, CONFIG_FILE_NAME};
use crate::file_system;

use std::io;

pub async fn create_config_file() -> io::Result<()> {
    let file = file_system::create_file(&CONFIG_DIRECTORY, CONFIG_FILE_NAME).await?;

    Ok(())
}
