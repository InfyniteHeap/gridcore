use gridcore::managers::game::download::{self, Category, DownloadSource};
use gridcore::utils::thread_count;

#[tokio::test]
async fn download_mc() {
    let version = "1.21.5";

    download::select_download_source(DownloadSource::Official).await;
    thread_count::set_thread_count(32).await;

    // TODO: These calls should be merged into a single function.
    download::version_manifest::download_version_manifest()
        .await
        .unwrap();
    download::version_manifest::download_specific_version_manifest(version)
        .await
        .unwrap();
    download::mc_file::download_files(version, Category::Client)
        .await
        .unwrap();
}
