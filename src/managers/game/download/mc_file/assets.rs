use crate::managers::game::download::{
    ASSETS_BANGBANG93, ASSETS_OFFICIAL, BANGBANG93, CLIENT, DOWNLOAD_SOURCE, DownloadSource,
};
use crate::path::MINECRAFT_ROOT;
use crate::utils::downloader::{self, FileInfo};
use crate::utils::json_processer;

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

        downloader::download_file(&CLIENT, file_info).await?;

        let data = json_processer::read(Path::new(&file_path), &file_name).await?;

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
                        path: file_path.into(),
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

    let mut num = files.len();

    for file_info in files {
        println!("Remains {num} asset files");
        downloader::download_file(&CLIENT, file_info).await?;
        num -= 1;
    }

    Ok(())
}
