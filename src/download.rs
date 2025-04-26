//! # Download
//!
//! This module is responsible to download Minecraft files and mod files.

// TODO: Implement multi-threaded downloading.

pub mod game;
pub mod mods;

use crate::checksum;
use crate::error_handling::DownloadError;
use crate::file_system;

use std::path::PathBuf;
use std::time::Duration;

use reqwest::Client;
use tokio::time;

/// The wait time for a single download task.
pub(crate) const DURATION: Duration = Duration::from_secs(10);
// const RETRY_INTERVAL: Duration = Duration::from_secs(5);

// /// The manager that manages download tasks.
// pub(crate) struct DownloadTaskManager<'c> {
//     client: &'c Client,
//     queue: Arc<Mutex<VecDeque<DownloadTask>>>,
//     semaphore: Arc<Semaphore>,
// }

// /// A single download task.
// pub(crate) struct DownloadTask {
//     file_info: FileInfo,
//     max_retries: u8,
//     current_retries: u8,
//     last_attempt: Option<Instant>,
// }

#[derive(Clone)]
pub(crate) struct FileInfo {
    /// The location where the file is stored.
    pub(crate) path: PathBuf,
    /// The file name.
    pub(crate) name: String,
    /// The source address that can download the file.
    pub(crate) url: String,
    /// The SHA1 hash that is used to check integrity of the file.
    pub(crate) sha1: String,
}

// impl<'c> DownloadTaskManager<'c> {
//     pub async fn new(client: &'c Client) -> Self {
//         Self {
//             client,
//             queue: Arc::new(Mutex::new(VecDeque::new())),
//             semaphore: Arc::new(Semaphore::new(*THREAD_COUNT.read().await)),
//         }
//     }

//     pub fn add_task(&mut self, file_info: FileInfo, max_retries: u8) {
//         let mut queue = self.queue.lock().unwrap();
//         queue.push_back(DownloadTask {
//             file_info,
//             max_retries,
//             current_retries: 0,
//             last_attempt: None,
//         });
//     }

//     pub async fn run_tasks(&self) {
//         let mut handles = vec![];
//         let queue = self.queue.clone();
//         let client = self.client.clone();
//         let semaphore = self.semaphore.clone();

//         while let Ok(permit) = semaphore.acquire().await {
//             let queue = queue.clone();
//             let client = client.clone();

//             let handle = tokio::spawn(async move {
//                 loop {
//                     let task = {
//                         let mut queue = queue.lock().unwrap();
//                         queue.pop_front()
//                     };

//                     let Some(mut task) = task else {
//                         break;
//                     };

//                     // Check retry interval
//                     if let Some(last) = task.last_attempt {
//                         if let Some(remaining) = RETRY_INTERVAL.checked_sub(last.elapsed()) {
//                             time::sleep(remaining).await;
//                         }
//                     }

//                     // Execute download
//                     let result = download_file(&client, &task.file_info).await;
//                     task.last_attempt = Some(Instant::now());

//                     match result {
//                         Ok(_) => {
//                             println!("Downloaded: {}", task.file_info.url);
//                         }
//                         Err(e) => {
//                             eprintln!("Download failed: {} ({})", task.file_info.url, e);
//                             task.current_retries += 1;

//                             if task.current_retries < task.max_retries {
//                                 let mut queue = queue.lock().unwrap();
//                                 queue.push_back(task);
//                             } else {
//                                 eprintln!("Permanently failed: {}", task.file_info.url);
//                             }
//                         }
//                     }
//                 }
//                 drop(permit);
//             });

//             handles.push(handle);
//         }

//         for handle in handles {
//             handle.await.unwrap();
//         }
//     }
// }

/// Downloads a single file.
///
/// It returns `Ok(())` when successfully downloaded a file and verified its integrity,
/// or `Err(DownloadError)` when there are some errors happened during downloading.
pub(crate) async fn download_file(
    client: &Client,
    file_info: FileInfo,
) -> Result<(), DownloadError> {
    // This function will fast return when these conditions are satisfied:
    // 1. The target file exists.
    // 2. Its corresponding SHA1 value is equal to the provided one.
    // HACK: There might has a better method to do so.
    if file_info.path.join(&file_info.name).exists()
        && (checksum::calculate_sha1(&file_info.path, &file_info.name).await? == file_info.sha1)
    {
        return Ok(());
    }

    let response = match time::timeout(DURATION, client.get(&file_info.url).send()).await {
        Ok(res) => res?,
        Err(_) => {
            return Err(DownloadError::InternetError(
                "Time out when waiting for response from remote server!".to_string(),
            ));
        }
    };

    if response.status().is_success() {
        // If success, we write raw bytes into a file.
        file_system::write_into_file(&file_info.path, &file_info.name, &response.bytes().await?)
            .await?;

        // To check whether the file is successfully downloaded,
        // we must verify its SHA1 value.
        // If the value is equal to the given one,
        // We can confirm that this file is successfully
        // downloaded!
        if checksum::calculate_sha1(&file_info.path, &file_info.name).await? == file_info.sha1 {
            Ok(())
        } else {
            Err(DownloadError::CheckIntegrityError)
        }
    } else {
        Err(DownloadError::InternetError(response.status().to_string()))
    }
}
