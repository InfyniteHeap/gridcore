use crate::constants::{
    ASSETS_BANGBANG93, ASSETS_OFFICIAL, BANGBANG93, DownloadSource, MINECRAFT_ROOT,
};
use crate::error_handling::DownloadError;
use crate::utils::downloader::{CLIENT, Downloader, FileInfo};
use crate::utils::json_processer;

use std::borrow::Cow;
use std::path::{Path, PathBuf};

use serde_json::Value;

pub(super) async fn download_assets(
    data: &Value,
    src: DownloadSource,
) -> Result<(), DownloadError> {
    let mut files = Vec::new();

    if let (Value::String(id), Value::String(sha1), Value::String(url)) = (
        &data["assetIndex"]["id"],
        &data["assetIndex"]["sha1"],
        &data["assetIndex"]["url"],
    ) {
        let mut url = url.to_owned();

        if src == DownloadSource::Bangbang93 {
            let len = "https://piston-meta.mojang.com/".len();
            url = format!("{}/{}", BANGBANG93, &url[len..])
        }

        let file_path = format!("{}/assets/indexes", MINECRAFT_ROOT);
        let file_name = format!("{}.json", id);

        let file_info = FileInfo {
            path: Cow::from(Path::new(&file_path)),
            name: Cow::from(&file_name),
            url: url.into(),
            sha1: Some(Cow::from(sha1)),
        };
        let downloader = Downloader::new(&CLIENT, file_info);
        downloader.download_file().await?;

        let data = json_processer::read(&file_path, &file_name).await?;

        if let Value::Object(obj) = &data["objects"] {
            for val in obj.values() {
                if let Value::String(hash) = &val["hash"] {
                    let url = format!(
                        "{}/{}/{}",
                        match src {
                            DownloadSource::Official => ASSETS_OFFICIAL,
                            DownloadSource::Bangbang93 => ASSETS_BANGBANG93,
                        },
                        &hash[0..2],
                        &hash
                    );

                    let file_path = format!("{}/assets/objects/{}", MINECRAFT_ROOT, &hash[0..2]);

                    let file_info = FileInfo {
                        path: Cow::from(PathBuf::from(&file_path)),
                        name: Cow::from(hash.clone()),
                        url: url.into(),
                        sha1: Some(Cow::from(hash.clone())),
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

        let downloader = Downloader::new(&CLIENT, file_info);
        downloader.download_file().await?;

        num -= 1;
    }

    Ok(())
}
