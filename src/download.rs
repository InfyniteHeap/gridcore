//! # Download
//!
//! This module is responsible to download Minecraft files and mod files.

pub mod game;
pub mod mods;

use crate::checksum;
use crate::file_system;

use std::path::Path;
use std::thread;

use lazy_static::lazy_static;
use tokio::sync::Mutex;

lazy_static! {
    pub(crate) static ref THREAD_COUNT: Mutex<usize> =
        Mutex::new(thread::available_parallelism().unwrap().get());
}

pub async fn download_file(
    file_path: &Path,
    file_name: &str,
    url: &str,
    sha1: Option<&str>,
) -> anyhow::Result<()> {
    if file_path.join(file_name).exists()
        && (sha1.is_none() || checksum::calculate_sha1(file_path, file_name)? == sha1.unwrap())
    {
        return Ok(());
    }

    let error_handling_helper = |times, err| {
        if times == 3 {
            panic!("{}", format!("Failed to download {}: {}", file_name, err));
        } else {
            eprintln!(
                "{}! Retrying for {} {}",
                err,
                times,
                if times == 1 { "time" } else { "times" }
            );
        }
    };

    for times in 1..=3 {
        match reqwest::get(url).await {
            Ok(response) => {
                if response.status().is_success() {
                    file_system::write_into_file(file_path, file_name, &response.bytes().await?)?;

                    if sha1.is_none()
                        || checksum::calculate_sha1(file_path, file_name)? == sha1.unwrap()
                    {
                        break;
                    } else {
                        continue;
                    }
                } else if let Err(err) = response.error_for_status() {
                    error_handling_helper(times, err);

                    continue;
                }
            }
            Err(err) => {
                error_handling_helper(times, err);

                continue;
            }
        }
    }

    Ok(())
}

/// Set thread count.
///
/// If you don't call this function,
/// the default thread count will usually depend on
/// numbers of logical CPU cores.
pub async fn set_thread_count(count: usize) {
    *THREAD_COUNT.lock().await = count;
}
