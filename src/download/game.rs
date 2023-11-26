use reqwest::get;

use crate::file_system::*;

const OFFICIAL: &str = "https://piston-meta.mojang.com/";
const BANGBANG93: &str = "https://bmclapi2.bangbang93.com/";
const MCBBS: &str = "https://download.mcbbs.net/";

pub enum McResDlAddr {
    Official,
    BangBang93,
    McBBS,
}

fn select_dl_addr(res: McResDlAddr) -> &'static str {
    match res {
        McResDlAddr::Official => OFFICIAL,
        McResDlAddr::BangBang93 => BANGBANG93,
        McResDlAddr::McBBS => MCBBS,
    }
}

pub async fn download_mc_version_manifest(
    res: McResDlAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    const ADDR_SUFFIX: &str = "mc/game/version_manifest_v2.json";

    let dl_addr = select_dl_addr(res);

    let response = get(dl_addr.to_string() + ADDR_SUFFIX).await?.text().await?;

    let dir = "./.minecraft/versions";
    let file_name = "version_manifest_v2.json";
    let mut file = create_file(dir, file_name)?;

    write_file(&mut file, response)?;
    Ok(())
}
