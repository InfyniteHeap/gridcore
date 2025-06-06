use crate::constants::{BANGBANG93, DownloadSource, MINECRAFT_ROOT};
use crate::error_handling::DownloadError;
use crate::utils::downloader::{CLIENT, Downloader, FileInfo};

use std::borrow::Cow;
use std::env::consts::OS;
use std::path::PathBuf;

use serde_json::Value;

pub(super) async fn download_libraries(
    data: &Value,
    src: DownloadSource,
) -> Result<(), DownloadError> {
    let mut files = Vec::new();

    if let Value::Array(libs) = &data["libraries"] {
        for lib in libs {
            if [Value::Null, Value::String(OS.replace("macos", "osx"))]
                .contains(&lib["rules"][0]["os"]["name"])
            {
                if let (Value::String(path), Value::String(sha1), Value::String(url)) = (
                    &lib["downloads"]["artifact"]["path"],
                    &lib["downloads"]["artifact"]["sha1"],
                    &lib["downloads"]["artifact"]["url"],
                ) {
                    let mut url = url.to_owned();

                    if src == DownloadSource::Bangbang93 {
                        let idx = "https://libraries.minecraft.net/".len();
                        url = format!("{}/maven/{}", BANGBANG93, &url[idx..]);
                    }

                    let file_name;
                    let file_path = format!("{}/libraries/{}", MINECRAFT_ROOT, {
                        let mut idx = path.len() - 1;

                        // We can ensure that `path` only contains ASCII characters,
                        // so this slice (index) is always valid.
                        while &path[idx..=idx] != "/" {
                            idx -= 1;
                        }

                        file_name = &path[idx + 1..];
                        &path[..idx]
                    });

                    let file_info = FileInfo {
                        path: Cow::from(PathBuf::from(&file_path)),
                        name: Cow::from(file_name),
                        url: url.into(),
                        sha1: Some(Cow::from(sha1)),
                    };

                    files.push(file_info);
                }
                if let (Value::String(path), Value::String(sha1), Value::String(url)) = (
                    &lib["downloads"]["classifiers"]
                        [&format!("natives-{}", OS.replace("macos", "osx"))]["path"],
                    &lib["downloads"]["classifiers"]
                        [&format!("natives-{}", OS.replace("macos", "osx"))]["sha1"],
                    &lib["downloads"]["classifiers"]
                        [&format!("natives-{}", OS.replace("macos", "osx"))]["url"],
                ) {
                    let mut url = url.to_owned();

                    if src == DownloadSource::Bangbang93 {
                        let idx = "https://libraries.minecraft.net/".len();
                        url = format!("{}/maven/{}", BANGBANG93, &url[idx..]);
                    }

                    let file_name;
                    let file_path = format!("{}/libraries/{}", MINECRAFT_ROOT, {
                        let mut idx = path.len() - 1;

                        while &path[idx..idx + 1] != "/" {
                            idx -= 1;
                        }

                        file_name = &path[idx + 1..];
                        &path[..idx]
                    });

                    let file_info = FileInfo {
                        path: Cow::from(PathBuf::from(&file_path)),
                        name: Cow::from(file_name),
                        url: url.into(),
                        sha1: Some(Cow::from(sha1)),
                    };

                    files.push(file_info);
                }
            } else if [Value::Null, Value::String(OS.replace("macos", "osx"))]
                .contains(&lib["rules"][1]["os"]["name"])
            {
                continue;
            }
        }
    }

    // TODO: Implement multi-threaded downloading.
    // let mut dtm = DownloadTaskManager::new(&CLIENT).await;
    // files.iter().for_each(|file_info| {
    //     let t = async || {};
    //     let dt = DownloadTask::new(t, 3);
    // });
    // dtm.run_tasks().await;

    let mut num = files.len();

    for file_info in files {
        println!("Remains {num} library files");

        let downloader = Downloader::new(&CLIENT, file_info);
        downloader.download_file().await?;

        num -= 1;
    }

    Ok(())
}
