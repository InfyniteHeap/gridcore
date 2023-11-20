use std::io::Write;

use fs_err as fs;

// Create a directory.
pub fn create_dir(dir: &str) -> Result<(), std::io::Error> {
    fs::create_dir_all(dir)
}

// Create a file.
pub fn create_file(file_name: &str) -> Result<fs::File, std::io::Error> {
    fs::File::create(file_name)
}

// Open a file.
pub fn open_file(file_name: &str) -> Result<fs::File, std::io::Error> {
    fs::File::open(file_name)
}

// Write contents in an opened file. Should not be called before opening a file.
pub fn write_file(file_name: &mut fs::File, contents: String) -> Result<(), std::io::Error> {
    file_name.write_all(contents.as_bytes())
}

// Copy a file to another directory.
pub fn copy_file() {
    todo!()
}

pub fn delete_file(file_name: &str) {
    todo!()
}
