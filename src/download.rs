//! # Download
//!
//! This module is responsible to download Minecraft files and mod files.

pub mod game;
pub mod mods;

use crate::file_system;

use std::io::Write;
use std::path::Path;
use std::thread;

use lazy_static::lazy_static;
use tokio::sync::Mutex;

lazy_static! {
    pub(crate) static ref THREAD_COUNT: Mutex<usize> =
        Mutex::new(thread::available_parallelism().unwrap().get());
}

pub async fn download_file(file_path: &Path, file_name: &str, url: &str) -> anyhow::Result<()> {
    for times in 1..=3 {
        let mut file = file_system::create_file(file_path, file_name)?;

        match reqwest::get(url).await {
            Ok(response) => {
                if response.status().is_success() {
                    file.write_all(&response.bytes().await?)?;

                    break;
                } else if let reqwest::Result::Err(err) = response.error_for_status() {
                    if times == 3 {
                        file_system::remove_file(file_path, file_name)?;

                        return Err(anyhow::Error::msg(format!(
                            "Failed to download {}: {}",
                            file_name, err
                        )));
                    } else {
                        eprintln!(
                            "{}! Retrying for {} {}",
                            err,
                            times,
                            if times == 1 { "time" } else { "times" }
                        );

                        continue;
                    }
                }
            }
            Err(err) => {
                if times == 3 {
                    file_system::remove_file(file_path, file_name)?;

                    return Err(anyhow::Error::msg(format!(
                        "Failed to download {}: {}",
                        file_name, err
                    )));
                } else {
                    eprintln!(
                        "{}! Retrying for {} {}",
                        err,
                        times,
                        if times == 1 { "time" } else { "times" }
                    );

                    continue;
                }
            }
        }
    }

    Ok(())
}

/// Set thread count.
///
/// If you don't call this function,
/// the default thread count will usually depends on
/// numbers of logical CPU cores.
pub async fn set_thread_count(count: usize) {
    *THREAD_COUNT.lock().await = count;
}
