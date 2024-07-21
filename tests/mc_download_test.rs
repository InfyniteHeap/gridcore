use gridcore::download;
use gridcore::download::game::{self, Category, DownloadSource};

#[tokio::test]
async fn download_mc() {
    let res = DownloadSource::Official;
    let version = "1.21";

    download::set_thread_count(32).await;
    game::select_download_source(&res).await;

    game::download_version_manifest().await.unwrap();
    game::download_specific_version_manifest(version)
        .await
        .unwrap();
    game::download_files(version, Category::Client)
        .await
        .unwrap();
}
