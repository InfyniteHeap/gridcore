//! # File System
//!
//! For better developing experience and more convenience,
//! most functions about files are simply wrapped here.

use std::path::Path;

use tokio::fs::{self, File};
use tokio::io::{self, AsyncWriteExt};

/// Create a directory.
///
/// Although function `create_file()` can automatically call this function, it is still meaningful
/// to keep this function public as there might are some situations that only creating directories is required.
pub async fn create_dir(path: &Path) -> io::Result<()> {
    fs::create_dir_all(path).await
}

/// Opens a file in write-only mode, and return its handle.
///
/// This function can automatically create a directory
/// if the target directory in which files will be stored later does not exist.
pub async fn create_file(file_path: &Path, file_name: &str) -> io::Result<File> {
    if fs::metadata(file_path).await.is_err() {
        create_dir(file_path).await?;
        File::create(file_path.join(file_name)).await
    } else {
        File::create(file_path.join(file_name)).await
    }
}

pub async fn remove_dir(path: &Path) -> io::Result<()> {
    fs::remove_dir_all(path).await
}

pub async fn remove_file(file_path: &Path, file_name: &str) -> io::Result<()> {
    fs::remove_file(file_path.join(file_name)).await
}

/// Opens a file in read-only mode, and return its handle.
pub async fn open_file(file_path: &Path, file_name: &str) -> io::Result<File> {
    File::open(file_path.join(file_name)).await
}

/// Writes contents into a file.
///
/// This function will automatically create a file
/// if it doesn't exist.
pub async fn write_into_file(file_path: &Path, file_name: &str, contents: &[u8]) -> io::Result<()> {
    let mut file = create_file(file_path, file_name).await?;
    file.write_all(contents).await?;

    Ok(())
}

/// Read the entire contents of a file into a string.
pub async fn read_file_to_string(file_path: &Path, file_name: &str) -> io::Result<String> {
    fs::read_to_string(file_path.join(file_name)).await
}
