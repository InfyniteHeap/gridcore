use super::THREAD_COUNT;
use crate::json;
use crate::path::MINECRAFT_ROOT;
use Category::*;
use DownloadSource::*;

use std::env::consts::OS;
use std::path::Path;
use std::sync::{Arc, Mutex as StdMutex};
use std::thread;

use lazy_static::lazy_static;
use serde_json::{Map, Value};
use tokio::runtime::Runtime;
use tokio::sync::Mutex as TokioMutex;

const OFFICIAL: &str = "https://piston-meta.mojang.com";
const BANGBANG93: &str = "https://bmclapi2.bangbang93.com";
const ASSETS_DOWNLOAD_OFFICIAL: &str = "https://resources.download.minecraft.net";
const ASSETS_DOWNLOAD_BANGBANG93: &str = "https://bmclapi2.bangbang93.com/assets";

lazy_static! {
    // This status variable is used by async functions, so we use `Mutex<T>` here.
    static ref DOWNLOAD_SOURCE: TokioMutex<DownloadSource> = TokioMutex::new(DownloadSource::Void);
}

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

fn select_category(category: &Category) -> &'static str {
    match category {
        Client => "client",
        Server => "server,",
    }
}

/// Download the manifest which contains metadata of all of the Minecraft versions.
pub async fn download_version_manifest() -> anyhow::Result<()> {
    let url = format!(
        "{}/mc/game/version_manifest_v2.json",
        match *DOWNLOAD_SOURCE.lock().await {
            Official => OFFICIAL,
            Bangbang93 => BANGBANG93,
            _ => unreachable!(),
        }
    );

    let manifest_path = format!("{}/versions", MINECRAFT_ROOT);
    let manifest_path = Path::new(&manifest_path);
    let manifest_name = "version_manifest_v2.json";

    if !manifest_path.join(manifest_name).exists() {
        super::download_file(manifest_path, manifest_name, &url).await?;
    }

    Ok(())
}

/// Read contents in `version_manifest_v2.json`.
pub(crate) fn read_version_manifest() -> anyhow::Result<Vec<Map<String, Value>>> {
    let manifest_path = format!("{}/versions", MINECRAFT_ROOT);
    let manifest_path = Path::new(&manifest_path);
    let manifest_name = "version_manifest_v2.json";

    let data = json::read(manifest_path, manifest_name)?;

    let mut manifest = Vec::new();

    match &data["versions"] {
        Value::Array(arr) => {
            for element in arr {
                if let Value::Object(obj) = element {
                    manifest.push(obj.to_owned())
                }
            }
        }
        _ => return Err(anyhow::Error::msg("Failed to read version manifest!")),
    }

    Ok(manifest)
}

/// Download the manifest which contains metadata of a specific Minecraft version.
pub async fn download_specific_version_manifest(version: &str) -> anyhow::Result<()> {
    let manifest_path = format!("{}/versions/{}", MINECRAFT_ROOT, version);
    let manifest_path = Path::new(&manifest_path);

    let manifest = read_version_manifest()?;

    for ver in manifest {
        if match ver.get("id") {
            Some(id) => id,
            None => return Err(anyhow::Error::msg("Failed to get id!")),
        } == version
        {
            match ver.get("url") {
                Some(Value::String(url)) => {
                    let mut url = url.to_owned();

                    match *DOWNLOAD_SOURCE.lock().await {
                        Official => (),
                        Bangbang93 => {
                            let len = "https://piston-meta.mojang.com/".len();
                            url = format!("{}/{}", BANGBANG93, &url[len..]);
                        }
                        _ => unreachable!(),
                    }

                    let manifest_name = format!("{}.json", version);

                    if !manifest_path.join(&manifest_name).exists() {
                        super::download_file(manifest_path, &manifest_name, &url).await?;
                    }
                }
                _ => return Err(anyhow::Error::msg("Failed to get download url!")),
            }
        }
    }

    Ok(())
}

/// Download Minecraft.
pub async fn download_files(version: &str, category: Category) -> anyhow::Result<()> {
    let manifest_path = format!("{}/versions/{}", MINECRAFT_ROOT, version);
    let manifest_path = Path::new(&manifest_path);
    let manifest_name = format!("{}.json", version);

    let data = json::read(manifest_path, &manifest_name)?;

    download_jar(version, &data, category).await?;
    download_libraries(&data).await?;
    download_assets(&data).await?;

    Ok(())
}

async fn download_jar(version: &str, data: &Value, category: Category) -> anyhow::Result<()> {
    if let Value::String(url) = &data["downloads"][select_category(&category)]["url"] {
        let mut url = url.to_owned();

        match *DOWNLOAD_SOURCE.lock().await {
            Official => (),
            Bangbang93 => {
                let idx = "https://piston-data.mojang.com/".len();
                url = format!("{}/{}", BANGBANG93, &url[idx..]);
            }
            _ => unreachable!(),
        }

        let file_path = format!("{}/versions/{}", MINECRAFT_ROOT, version);
        let file_path = Path::new(&file_path);
        let file_name = format!("{}.jar", version);

        if !file_path.join(&file_name).exists() {
            super::download_file(file_path, &file_name, &url).await?;
        }
    }

    Ok(())
}

async fn download_libraries(data: &Value) -> anyhow::Result<()> {
    let mut paths = Vec::new();
    let mut urls = Vec::new();

    if let Value::Array(libs) = &data["libraries"] {
        for lib in libs {
            if [Value::Null, Value::String(OS.replace("macos", "osx"))]
                .contains(&lib["rules"][0]["os"]["name"])
            {
                if let (Value::String(path), Value::String(url)) = (
                    &lib["downloads"]["artifact"]["path"],
                    &lib["downloads"]["artifact"]["url"],
                ) {
                    let mut url = url.to_owned();

                    match *DOWNLOAD_SOURCE.lock().await {
                        Official => (),
                        Bangbang93 => {
                            let idx = "https://libraries.minecraft.net/".len();
                            url = format!("{}/maven/{}", BANGBANG93, &url[idx..]);
                        }
                        _ => unreachable!(),
                    }

                    paths.push(path.to_owned());
                    urls.push(url);
                }
            }
        }
    }

    let urls = Arc::new(StdMutex::new(urls));
    let paths = Arc::new(StdMutex::new(paths));
    let mut handles = Vec::with_capacity(*THREAD_COUNT.lock().await);

    for _ in 0..*THREAD_COUNT.lock().await {
        let (urls, paths) = (urls.clone(), paths.clone());
        // We have to create a runtime because of
        // issue #62290 <https://github.com/rust-lang/rust/issues/62290>,
        // even if this will yield huge overheads.
        let rt = Runtime::new().unwrap();

        let handle = thread::spawn(move || loop {
            let (url, path) = {
                let mut urls = urls.lock().unwrap();
                let mut paths = paths.lock().unwrap();

                if urls.is_empty() || paths.is_empty() {
                    break;
                }

                (urls.pop().unwrap(), paths.pop().unwrap())
            };

            let file_name;
            let file_path = format!("{}/libraries/{}", MINECRAFT_ROOT, {
                let mut path = path.split('/').collect::<Vec<&str>>();
                file_name = path.pop().unwrap();
                path.join("/")
            });
            let file_path = Path::new(&file_path);

            if !file_path.join(file_name).exists() {
                rt.block_on(super::download_file(file_path, file_name, &url))
                    .unwrap();
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}

async fn download_assets(data: &Value) -> anyhow::Result<()> {
    let mut hashes = Vec::new();
    let mut urls = Vec::new();

    if let (Value::String(id), Value::String(url)) =
        (&data["assetIndex"]["id"], &data["assetIndex"]["url"])
    {
        let mut url = url.to_owned();

        match *DOWNLOAD_SOURCE.lock().await {
            Official => (),
            Bangbang93 => {
                let len = "https://piston-meta.mojang.com/".len();
                url = format!("{}/{}", BANGBANG93, &url[len..])
            }
            _ => unreachable!(),
        }

        let file_path = format!("{}/assets/indexes", MINECRAFT_ROOT);
        let file_path = Path::new(&file_path);
        let file_name = format!("{}.json", id);

        if !file_path.join(&file_name).exists() {
            super::download_file(file_path, &file_name, &url).await?
        }

        let data = json::read(file_path, &file_name)?;

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

                    hashes.push(hash.to_owned());
                    urls.push(url);
                }
            }
        }
    }

    let urls = Arc::new(StdMutex::new(urls));
    let hashes = Arc::new(StdMutex::new(hashes));
    let mut handles = Vec::with_capacity(*THREAD_COUNT.lock().await);

    for _ in 0..*THREAD_COUNT.lock().await {
        let (urls, hashes) = (urls.clone(), hashes.clone());
        // We have to create a runtime because of
        // issue #62290 <https://github.com/rust-lang/rust/issues/62290>,
        // even if this will yield huge overheads.
        let rt = Runtime::new().unwrap();

        let handle = thread::spawn(move || loop {
            let (url, hash) = {
                let mut urls = urls.lock().unwrap();
                let mut hashes = hashes.lock().unwrap();

                if urls.is_empty() || hashes.is_empty() {
                    break;
                }

                (urls.pop().unwrap(), hashes.pop().unwrap())
            };

            let file_path = format!("{}/assets/objects/{}", MINECRAFT_ROOT, &hash[0..2]);
            let file_path = Path::new(&file_path);
            let file_name = hash;

            if !file_path.join(&file_name).exists() {
                rt.block_on(super::download_file(file_path, &file_name, &url))
                    .unwrap();
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
