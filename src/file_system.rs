//! # File System
//! For better developing quality, most of functions about file is simply wrapped here.

use std::{io::Write, path::Path};

use fs_err as fs;

/// Create a directory.
///
/// Although function `create_file` can automatically call this function, it is still meaningful
/// to keep this function public as there might are some situations that only creating directories is required.
pub(crate) fn create_dir(dir: &str) -> Result<(), std::io::Error> {
    fs::create_dir_all(dir)
}

/// Create a file.
///
/// This function can automatically create a directory if the target directory in which files will be stored later do not exist.
pub(crate) fn create_file(dir: &str, file_name: &str) -> Result<fs::File, std::io::Error> {
    if fs::metadata(dir).is_err() {
        create_dir(dir)?;
        let file_path = Path::new(dir).join(file_name);
        fs::File::create(file_path)
    } else {
        let file_path = Path::new(dir).join(file_name);
        fs::File::create(file_path)
    }
}

/// Open a file.
pub(crate) fn open_file(file_name: &str) -> Result<fs::File, std::io::Error> {
    fs::File::open(file_name)
}

/// Write contents in an opened file. Should not be called before opening a file.
pub(crate) fn write_file(file_name: &mut fs::File, contents: String) -> Result<(), std::io::Error> {
    file_name.write_all(contents.as_bytes())
}

/// Copy a file to another directory.
pub(crate) fn copy_file() {
    todo!()
}

pub(crate) fn delete_file(file_name: &str) {
    todo!()
}
