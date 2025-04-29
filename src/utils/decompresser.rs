//! # Decompress
//!
//! This module is used when decompressing native libraries.

use crate::error_handling::DecompressError;
use crate::file_system;

use std::io::Read;
use std::path::Path;

use zip::ZipArchive;

pub async fn decompress_file(
    file_path: &Path,
    file_name: &str,
    extract_path: &Path,
) -> Result<(), DecompressError> {
    let file = file_system::open_file(file_path, file_name).await?;
    let mut archive = ZipArchive::new(file.into_std().await)?;

    for idx in 0..archive.len() {
        let mut file = archive.by_index(idx)?;

        let out_path = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if let Some(file_name) = out_path.file_name() {
            let file_name = file_name.to_str().unwrap();

            // TODO: Add support for Linux and MacOS platform.
            if file_name.ends_with(".dll") {
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)?;
                file_system::write_into_file(extract_path, file_name, &buffer).await?;
            }
        }
    }

    Ok(())
}
