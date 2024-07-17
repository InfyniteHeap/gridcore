use crate::file_system;

use std::io::{self, BufReader, Read};
use std::path::Path;

use sha1::{Digest, Sha1};

pub fn calculate_sha1(file_path: &Path, file_name: &str) -> io::Result<String> {
    let file = file_system::open_file(file_path, file_name)?;

    let mut reader = BufReader::new(file);
    let mut hasher = Sha1::new();
    let mut buffer = [0; 1024];

    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}
