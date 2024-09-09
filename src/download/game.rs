use super::{DownloadFileInfo, THREAD_COUNT};
use crate::file_system;
use crate::json;
use crate::path::MINECRAFT_ROOT;
use DownloadSource::*;

use std::env::consts::OS;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock};

use reqwest::Client;
use serde_json::{Map, Value};
use tokio::sync::{Mutex, Semaphore};
use tokio::task;

const OFFICIAL: &str = "https://piston-meta.mojang.com";
const BANGBANG93: &str = "https://bmclapi2.bangbang93.com";
const ASSETS_DOWNLOAD_OFFICIAL: &str = "https://resources.download.minecraft.net";
const ASSETS_DOWNLOAD_BANGBANG93: &str = "https://bmclapi2.bangbang93.com/assets";

const RETRY_TIMES: u8 = 5;

static DOWNLOAD_SOURCE: Mutex<DownloadSource> = Mutex::const_new(Void);
static CLIENT: LazyLock<Arc<Client>> = LazyLock::new(|| {
    Arc::new(
        Client::builder()
            .https_only(true)
            .build()
            .unwrap_or_default(),
    )
});

#[derive(PartialEq)]
pub enum DownloadSource {
    // This is only for initializing `DOWNLOAD_SOURCE`.
    Void,
    Official,
    Bangbang93,
}

pub enum Category {
    Client,
    Server,
}

pub async fn select_download_source(res: &DownloadSource) {
    match res {
        Official => *DOWNLOAD_SOURCE.lock().await = Official,
        Bangbang93 => *DOWNLOAD_SOURCE.lock().await = Bangbang93,
        _ => unreachable!(),
    }
}

async fn select_category(category: &Category) -> &'static str {
    match category {
        Category::Client => "client",
        Category::Server => "server,",
    }
}

/// Download the manifest which contains metadata of all the Minecraft versions.
pub async fn download_version_manifest() -> anyhow::Result<()> {
    let manifest_path = format!("{}/versions", MINECRAFT_ROOT);
    let manifest_name = "version_manifest_v2.json";

    let url = format!(
        "{}/mc/game/version_manifest_v2.json",
        match *DOWNLOAD_SOURCE.lock().await {
            Official => OFFICIAL,
            Bangbang93 => BANGBANG93,
            _ => unreachable!(),
        }
    );

    // We always download this manifest regardless of the status of this file.
    // This is because: (1) we have no other ways to check integrity of this file,
    // and (2) we can fetch latest Minecraft information via this way.
    let response = CLIENT.get(&url).send().await?;

    if response.status().is_success() {
        file_system::write_into_file(
            Path::new(&manifest_path),
            manifest_name,
            &response.bytes().await?,
        )
        .await?;
    }

    Ok(())
}

/// Read contents in `version_manifest_v2.json`.
pub(crate) async fn read_version_manifest() -> anyhow::Result<Vec<Map<String, Value>>> {
    let manifest_path = format!("{}/versions", MINECRAFT_ROOT);
    let manifest_name = "version_manifest_v2.json";

    let data = json::read(Path::new(&manifest_path), manifest_name).await?;

    let mut manifest = Vec::new();

    if let Value::Array(arr) = &data["versions"] {
        arr.iter().for_each(|element| {
            if let Value::Object(obj) = element {
                manifest.push(obj.to_owned())
            }
        });
    }

    Ok(manifest)
}

/// Download the manifest which contains metadata of a specific Minecraft version.
pub async fn download_specific_version_manifest(version: &str) -> anyhow::Result<()> {
    let manifest_path = format!("{}/versions/{}", MINECRAFT_ROOT, version);
    let manifest_name = format!("{}.json", version);

    let manifest = read_version_manifest().await?;

    for ver in manifest {
        if ver["id"] == version {
            if let (Value::String(url), Value::String(sha1)) = (&ver["url"], &ver["sha1"]) {
                let mut url = url.to_owned();

                if *DOWNLOAD_SOURCE.lock().await == Bangbang93 {
                    let len = "https://piston-meta.mojang.com/".len();
                    url = format!("{}/{}", BANGBANG93, &url[len..]);
                }

                let file_info = DownloadFileInfo {
                    path: PathBuf::from(manifest_path),
                    name: manifest_name,
                    url,
                    sha1: sha1.to_owned(),
                };

                super::download_file(&CLIENT, RETRY_TIMES, &file_info).await?;
            }

            break;
        }
    }

    Ok(())
}

/// Download Minecraft.
pub async fn download_files(version: &str, category: Category) -> anyhow::Result<()> {
    let manifest_path = format!("{}/versions/{}", MINECRAFT_ROOT, version);
    let manifest_name = format!("{}.json", version);

    let data = json::read(Path::new(&manifest_path), &manifest_name).await?;

    download_jar(version, &data, category).await?;
    download_libraries(&data).await?;
    download_assets(&data).await?;
    download_logging_config(&data).await?;

    Ok(())
}

async fn download_jar(version: &str, data: &Value, category: Category) -> anyhow::Result<()> {
    let file_path = format!("{}/versions/{}", MINECRAFT_ROOT, version);
    let file_name = format!("{}.jar", version);

    if let (Value::String(url), Value::String(sha1)) = (
        &data["downloads"][select_category(&category).await]["url"],
        &data["downloads"][select_category(&category).await]["sha1"],
    ) {
        let mut url = url.to_owned();

        if *DOWNLOAD_SOURCE.lock().await == Bangbang93 {
            let idx = "https://piston-data.mojang.com/".len();
            url = format!("{}/{}", BANGBANG93, &url[idx..]);
        }

        let file_info = DownloadFileInfo {
            path: PathBuf::from(file_path),
            name: file_name,
            url,
            sha1: sha1.to_owned(),
        };

        super::download_file(&CLIENT, RETRY_TIMES, &file_info).await?;
    }

    Ok(())
}

async fn download_libraries(data: &Value) -> anyhow::Result<()> {
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

                    if *DOWNLOAD_SOURCE.lock().await == Bangbang93 {
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

                    let file_info = DownloadFileInfo {
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

                    if *DOWNLOAD_SOURCE.lock().await == Bangbang93 {
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

                    let file_info = DownloadFileInfo {
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

    let mut tasks = Vec::with_capacity(files.len());

    let semaphore = Arc::new(Semaphore::new(*THREAD_COUNT.lock().await));

    for file_info in files {
        let permit = semaphore.clone().acquire_owned().await.unwrap();

        let task = task::spawn(async move {
            let _permit = permit;

            super::download_file(&CLIENT, RETRY_TIMES, &file_info).await
        });

        tasks.push(task);
    }

    for task in tasks {
        task.await??;
    }

    Ok(())
}

async fn download_assets(data: &Value) -> anyhow::Result<()> {
    let mut files = Vec::new();

    if let (Value::String(id), Value::String(sha1), Value::String(url)) = (
        &data["assetIndex"]["id"],
        &data["assetIndex"]["sha1"],
        &data["assetIndex"]["url"],
    ) {
        let mut url = url.to_owned();

        if *DOWNLOAD_SOURCE.lock().await == Bangbang93 {
            let len = "https://piston-meta.mojang.com/".len();
            url = format!("{}/{}", BANGBANG93, &url[len..])
        }

        let file_path = format!("{}/assets/indexes", MINECRAFT_ROOT);
        let file_name = format!("{}.json", id);

        let file_info = DownloadFileInfo {
            path: PathBuf::from(&file_path),
            name: file_name.clone(),
            url,
            sha1: sha1.to_owned(),
        };

        super::download_file(&CLIENT, RETRY_TIMES, &file_info).await?;

        let data = json::read(Path::new(&file_path), &file_name).await?;

        if let Value::Object(obj) = &data["objects"] {
            for (_, key) in obj {
                if let Value::String(hash) = &key["hash"] {
                    let url = format!(
                        "{}/{}/{}",
                        match *DOWNLOAD_SOURCE.lock().await {
                            Official => ASSETS_DOWNLOAD_OFFICIAL,
                            Bangbang93 => ASSETS_DOWNLOAD_BANGBANG93,
                            _ => unreachable!(),
                        },
                        &hash[0..2],
                        hash
                    );

                    let file_path = format!("{}/assets/objects/{}", MINECRAFT_ROOT, &hash[0..2]);

                    let file_info = DownloadFileInfo {
                        path: PathBuf::from(file_path),
                        name: hash.to_owned(),
                        url,
                        sha1: hash.to_owned(),
                    };

                    files.push(file_info);
                }
            }
        }
    }

    let mut tasks = Vec::with_capacity(files.len());

    let semaphore = Arc::new(Semaphore::new(*THREAD_COUNT.lock().await));

    for file_info in files {
        let permit = semaphore.clone().acquire_owned().await.unwrap();

        let task = task::spawn(async move {
            let _permit = permit;

            super::download_file(&CLIENT, RETRY_TIMES, &file_info).await
        });

        tasks.push(task);
    }

    for task in tasks {
        task.await??;
    }

    Ok(())
}

async fn download_logging_config(data: &Value) -> anyhow::Result<()> {
    if let (Value::String(id), Value::String(sha1), Value::String(url)) = (
        &data["logging"]["client"]["file"]["id"],
        &data["logging"]["client"]["file"]["sha1"],
        &data["logging"]["client"]["file"]["url"],
    ) {
        let file_path = format!("{}/assets/log_configs", MINECRAFT_ROOT);

        let file_info = DownloadFileInfo {
            path: PathBuf::from(file_path),
            name: id.to_owned(),
            url: url.to_owned(),
            sha1: sha1.to_owned(),
        };

        super::download_file(&CLIENT, RETRY_TIMES, &file_info).await?;
    }

    Ok(())
}
