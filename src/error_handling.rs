//! The place where all the errors are handled.
//!
//! Though [anyhow](https://crates.io/crates/anyhow) provides some ways
//! to simplify how the Rust programmers handle recoverable errors,
//! we still wish to provide concrate errors, so that we can exactly know
//! where an error happens.

use std::error::Error;
use std::fmt::Display;
use std::io;

macro_rules! derive_trait {
    ($src:ty, $dst:ty, $with:expr) => {
        impl From<$src> for $dst {
            fn from(value: $src) -> Self {
                $with(value.to_string())
            }
        }
    };
}

#[derive(Debug)]
pub enum DownloadError {
    InternetError(String),
    FileSystemError(String),
    OtherError(String),
}

impl Display for DownloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::InternetError(e) => format!("Internet error: {}", e),
                Self::FileSystemError(e) => format!("Failed to write contents to disk: {}", e),
                Self::OtherError(e) => e.to_owned(),
            }
        )
    }
}

impl Error for DownloadError {}

derive_trait!(io::Error, DownloadError, DownloadError::FileSystemError);
derive_trait!(reqwest::Error, DownloadError, DownloadError::InternetError);
