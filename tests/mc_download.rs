use tokio::runtime::Runtime;

use gridcore::download::game::*;

#[test]
fn download_mc() -> anyhow::Result<()> {
    let ins = McResDlAddr::Official;

    let tokio_rt = Runtime::new().unwrap();

    match tokio_rt.block_on(download_mc_version_manifest(ins)) {
        Ok(()) => (),
        Err(e) => panic!("{:#?}", e),
    }

    let result = list_versions()?;

    println!(
        "{:?}",
        result
            .into_iter()
            .filter(|r| r.1
                == MinecraftVersionType::minecraft_version_type(MinecraftVersionType::Release))
            .collect::<Vec<(String, String)>>()
    );

    match tokio_rt.block_on(download_specific_mc_version_manifest("1.21")) {
        Ok(()) => (),
        Err(e) => panic!("{:#?}", e),
    }

    Ok(())
}
