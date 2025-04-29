//! # Error Handling
//!
//! The place where all the errors are handled.
//!
//! Though [anyhow](https://crates.io/crates/anyhow) provides some ways
//! to simplify how the Rust programmers handle recoverable errors,
//! we still wish to provide concrete errors, so that we can exactly know
//! where an error happens.

use std::error::Error;
use std::fmt::Display;
use std::io;

use regex;
use serde_json;
use zip::result;

macro_rules! derive_trait {
    ($src:ty, $dst:ty, $with:expr) => {
        impl From<$src> for $dst {
            fn from(value: $src) -> Self {
                $with(value.to_string())
            }
        }
    };
    ($src:ty, $dst:ty, $with:expr, $method:expr) => {
        impl From<$src> for $dst {
            fn from(value: $src) -> Self {
                $with($method(value))
            }
        }
    };
}

#[derive(Debug)]
pub enum JsonError {
    FileSystemError(io::Error),
    JsonParseError(serde_json::Error),
}

#[derive(Debug)]
pub enum DownloadError {
    JsonError(String),
    InternetError(String),
    FileSystemError(String),
    CheckIntegrityError,
    OtherError(String),
}

#[derive(Debug)]
pub enum DecompressError {
    FileSystemError(io::Error),
    ZipError(result::ZipError),
}

#[derive(Debug)]
pub enum AuthError {
    InternetError(String),
    FileSystemError(String),
}

#[derive(Debug)]
pub enum LaunchError {
    JsonError(String),
    RegexError(String),
}

impl Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::FileSystemError(fse) => fse.to_string(),
                Self::JsonParseError(je) => je.to_string(),
            }
        )
    }
}

impl Error for JsonError {}

derive_trait!(io::Error, JsonError, JsonError::FileSystemError, |fse| fse);
derive_trait!(
    serde_json::Error,
    JsonError,
    JsonError::JsonParseError,
    |je| je
);

impl Display for DownloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::JsonError(e) => format!("Json parse error: {}", e),
                Self::InternetError(e) => format!("Internet error: {}", e),
                Self::FileSystemError(e) => format!("Failed to write contents to disk: {}", e),
                Self::CheckIntegrityError => "Downloaded file is incomplete!".to_string(),
                Self::OtherError(e) => e.to_owned(),
            }
        )
    }
}

impl Error for DownloadError {}

derive_trait!(JsonError, DownloadError, DownloadError::JsonError);
derive_trait!(io::Error, DownloadError, DownloadError::FileSystemError);
derive_trait!(reqwest::Error, DownloadError, DownloadError::InternetError);

impl Display for DecompressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::FileSystemError(fse) => fse.to_string(),
                Self::ZipError(ze) => ze.to_string(),
            }
        )
    }
}

impl Error for DecompressError {}

derive_trait!(
    io::Error,
    DecompressError,
    DecompressError::FileSystemError,
    |v| v
);
derive_trait!(
    result::ZipError,
    DecompressError,
    DecompressError::ZipError,
    |v| v
);

impl Display for LaunchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::JsonError(je) => je.to_string(),
                Self::RegexError(re) => re.to_string(),
            }
        )
    }
}

impl Error for LaunchError {}

derive_trait!(JsonError, LaunchError, LaunchError::JsonError);
derive_trait!(regex::Error, LaunchError, LaunchError::RegexError);
