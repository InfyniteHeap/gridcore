use crate::download::game::{
    DownloadSource, ASSETS_BANGBANG93, ASSETS_OFFICIAL, BANGBANG93, CLIENT, DOWNLOAD_SOURCE,
};
use crate::download::{self, FileInfo};
use crate::json;
use crate::path::MINECRAFT_ROOT;

use std::path::{Path, PathBuf};

use serde_json::Value;

pub(super) async fn download_assets(data: &Value) -> anyhow::Result<()> {
    let mut files = Vec::new();

    if let (Value::String(id), Value::String(sha1), Value::String(url)) = (
        &data["assetIndex"]["id"],
        &data["assetIndex"]["sha1"],
        &data["assetIndex"]["url"],
    ) {
        let mut url = url.to_owned();

        if *DOWNLOAD_SOURCE.read().await == DownloadSource::Bangbang93 {
            let len = "https://piston-meta.mojang.com/".len();
            url = format!("{}/{}", BANGBANG93, &url[len..])
        }

        let file_path = format!("{}/assets/indexes", MINECRAFT_ROOT);
        let file_name = format!("{}.json", id);

        let file_info = FileInfo {
            path: PathBuf::from(&file_path),
            name: file_name.clone(),
            url,
            sha1: sha1.to_owned(),
        };

        download::download_file(&CLIENT, file_info).await?;

        let data = json::read(Path::new(&file_path), &file_name).await?;

        if let Value::Object(obj) = &data["objects"] {
            for (_, key) in obj {
                if let Value::String(hash) = &key["hash"] {
                    let url = format!(
                        "{}/{}/{}",
                        match *DOWNLOAD_SOURCE.read().await {
                            DownloadSource::Official => ASSETS_OFFICIAL,
                            DownloadSource::Bangbang93 => ASSETS_BANGBANG93,
                        },
                        &hash[0..2],
                        hash
                    );

                    let file_path = format!("{}/assets/objects/{}", MINECRAFT_ROOT, &hash[0..2]);

                    let file_info = FileInfo {
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

    // TODO: Implement multi-threaded downloading.
    // let mut dtm = DownloadTaskManager::new(&CLIENT).await;
    // files
    //     .into_iter()
    //     .for_each(|file_info| dtm.add_task(file_info, 3));
    // dtm.run_tasks().await;

    let mut handles = Vec::new();

    files.into_iter().for_each(|file_info| {
        let handle = tokio::spawn(async move { download::download_file(&CLIENT, file_info).await });

        handles.push(handle);
    });

    for handle in handles {
        handle.await??;
    }

    Ok(())
}
