//! # File System
//!
//! For better developing experience and more convenience,
//! most functions about files are simply wrapped here.

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

/// Create a directory.
///
/// Although function `create_file()` can automatically call this function, it is still meaningful
/// to keep this function public as there might are some situations that only creating directories is required.
pub fn create_dir(path: &Path) -> io::Result<()> {
    fs::create_dir_all(path)
}

/// Opens a file in write-only mode, and return its handle.
///
/// This function can automatically create a directory
/// if the target directory in which files will be stored later does not exist.
pub fn create_file(file_path: &Path, file_name: &str) -> io::Result<File> {
    if fs::metadata(file_path).is_err() {
        create_dir(file_path)?;
        File::create(file_path.join(file_name))
    } else {
        File::create(file_path.join(file_name))
    }
}

pub fn remove_dir(path: &Path) -> io::Result<()> {
    fs::remove_dir_all(path)
}

pub fn remove_file(file_path: &Path, file_name: &str) -> io::Result<()> {
    fs::remove_file(file_path.join(file_name))
}

/// Opens a file in read-only mode, and return its handle.
pub fn open_file(file_path: &Path, file_name: &str) -> io::Result<File> {
    File::open(file_path.join(file_name))
}

/// Writes contents into a file.
pub fn write_into_file(file_path: &Path, file_name: &str, contents: &[u8]) -> io::Result<()> {
    let mut file = create_file(file_path, file_name)?;
    file.write_all(contents)?;

    Ok(())
}

/// Read the entire contents of a file into a string.
pub fn read_file_to_string(file_path: &Path, file_name: &str) -> io::Result<String> {
    fs::read_to_string(file_path.join(file_name))
}
