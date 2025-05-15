use gridcore::constants::{Category, DownloadSource};
use gridcore::managers::game::download::MinecraftDownloader;
use gridcore::managers::manifest;

#[tokio::test]
async fn download_mc() {
    // Contents will be listed on UI interface.
    let _versions = manifest::download_version_manifest(DownloadSource::Official)
        .await
        .unwrap();

    let mc_downloader =
        MinecraftDownloader::new("1.21.5", DownloadSource::Official, Category::Client, None);
    mc_downloader.download_minecraft().await.unwrap();
}
