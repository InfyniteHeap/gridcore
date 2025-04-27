use crate::managers::game::download::{BANGBANG93, CLIENT, DOWNLOAD_SOURCE, DownloadSource};
use crate::path::MINECRAFT_ROOT;
use crate::utils::downloader::{self, FileInfo};

use std::env::consts::OS;
use std::path::PathBuf;

use serde_json::Value;

pub(super) async fn download_libraries(data: &Value) -> anyhow::Result<()> {
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

                    if *DOWNLOAD_SOURCE.read().await == DownloadSource::Bangbang93 {
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
                        path: PathBuf::from(file_path),
                        name: file_name.to_string(),
                        url,
                        sha1: sha1.to_owned(),
                    };

                    files.push(file_info);

                    // This seems only be compatible with versions older than 1.18.
                } else if let (Value::String(path), Value::String(sha1), Value::String(url)) = (
                    &lib["downloads"]["classifiers"]
                        [&format!("natives-{}", OS.replace("macos", "osx"))]["path"],
                    &lib["downloads"]["classifiers"]
                        [&format!("natives-{}", OS.replace("macos", "osx"))]["sha1"],
                    &lib["downloads"]["classifiers"]
                        [&format!("natives-{}", OS.replace("macos", "osx"))]["url"],
                ) {
                    let mut url = url.to_owned();

                    if *DOWNLOAD_SOURCE.read().await == DownloadSource::Bangbang93 {
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
                        path: PathBuf::from(file_path),
                        name: file_name.to_string(),
                        url,
                        sha1: sha1.to_owned(),
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
        downloader::download_file(&CLIENT, file_info).await?;
        num -= 1;
    }

    Ok(())
}
