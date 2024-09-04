use gridcore::download;
use gridcore::download::game::{self, Category, DownloadSource};

#[tokio::test]
async fn download_mc() {
    let version = "1.21.1";

    game::select_download_source(&DownloadSource::Official).await;
    download::set_thread_count(32).await;

    game::download_version_manifest().await.unwrap();
    game::download_specific_version_manifest(version)
        .await
        .unwrap();
    game::download_files(version, Category::Client)
        .await
        .unwrap();
}
