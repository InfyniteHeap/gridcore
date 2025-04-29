use crate::file_system;

use std::io;
use std::path::Path;

use sha1::{Digest, Sha1};
use tokio::io::AsyncReadExt;

// The average size of files is about 256 KiB.
const CAPACITY: usize = 0x4_0000;

pub async fn calculate_sha1(file_path: &Path, file_name: &str) -> io::Result<String> {
    let mut file = file_system::open_file(file_path, file_name).await?;

    let mut hasher = Sha1::new();
    let mut buffer = Vec::with_capacity(CAPACITY);

    file.read_to_end(&mut buffer).await?;

    hasher.update(&buffer);

    Ok(format!("{:x}", hasher.finalize()))
}

pub async fn check_sha1(
    file_path: &Path,
    file_name: &str,
    sha1: Option<&str>,
) -> Result<bool, io::Error> {
    if let Some(sha1) = sha1 {
        if calculate_sha1(file_path, file_name).await? == sha1 {
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        Ok(true)
    }
}
