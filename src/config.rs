use crate::file_system;
use crate::path::{CONFIG_DIRECTORY, CONFIG_FILE_NAME};

use std::io;
use std::path::Path;

pub async fn create_config_file() -> io::Result<()> {
    let file = file_system::create_file(Path::new(CONFIG_DIRECTORY), CONFIG_FILE_NAME).await?;

    Ok(())
}
