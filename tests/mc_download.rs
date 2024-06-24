use tokio::runtime::Runtime;

use gridcore::download::game::*;

#[test]
fn download_mc_version_manifest_test() -> anyhow::Result<()> {
    let ins = McResDlAddr::Official;

    let tokio_rt = Runtime::new().unwrap();

    match tokio_rt.block_on(download_mc_version_manifest(ins)) {
        Ok(()) => (),
        Err(e) => panic!("{:#?}", e),
    }

    let result = list_versions()?;

    println!("{:?}", result);

    Ok(())
}
