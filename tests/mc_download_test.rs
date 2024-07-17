use tokio::runtime::Runtime;

use gridcore::download;
use gridcore::download::game::{self, Category, DownloadSource};

#[test]
fn download_mc() {
    let res = DownloadSource::Official;
    let version = "1.21";

    let tokio_rt = Runtime::new().unwrap();

    tokio_rt.block_on(download::set_thread_count(32));
    tokio_rt.block_on(game::select_download_source(&res));

    tokio_rt
        .block_on(game::download_version_manifest())
        .unwrap();
    tokio_rt
        .block_on(game::download_specific_version_manifest(version))
        .unwrap();
    tokio_rt
        .block_on(game::download_files(version, Category::Client))
        .unwrap();
}
