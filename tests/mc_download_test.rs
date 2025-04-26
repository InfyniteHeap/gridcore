use gridcore::download::game::{self, Category, DownloadSource};
use gridcore::utils;

#[tokio::test]
async fn download_mc() {
    let version = "1.21.5";

    game::select_download_source(DownloadSource::Official).await;
    utils::set_thread_count(32).await;

    // TODO: These calls should be merged into a single function.
    game::version_manifest::download_version_manifest()
        .await
        .unwrap();
    game::version_manifest::download_specific_version_manifest(version)
        .await
        .unwrap();
    game::mc_file::download_files(version, Category::Client)
        .await
        .unwrap();
}
