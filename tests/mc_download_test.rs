use gridcore::managers::game::download::{Category, DownloadSource, MinecraftDownloader};

#[tokio::test]
async fn download_mc() {
    let mc_downloader =
        MinecraftDownloader::new("1.21.5", DownloadSource::Official, Category::Client, None);
    mc_downloader.download_minecraft().await.unwrap();
}
