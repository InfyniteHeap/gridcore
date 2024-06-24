use fs_err as fs;

use crate::file_system::*;

use reqwest::get;
use serde_json::{from_str, Value};

const OFFICIAL: &str = "https://piston-meta.mojang.com/";
const BANGBANG93: &str = "https://bmclapi2.bangbang93.com/";
const MCBBS: &str = "https://download.mcbbs.net/";

#[cfg(target_os = "windows")]
const MINECRAFT_ROOT: &str = "./.minecraft";
#[cfg(target_os = "macos")]
const MINECRAFT_ROOT: &str = "./minecraft";
#[cfg(target_os = "linux")]
const MINECRAFT_ROOT: &str = "./.minecraft";

#[derive(Default)]
pub enum McResDlAddr {
    #[default]
    Official,
    Bangbang93,
    Mcbbs,
}

fn select_dl_addr(res: McResDlAddr) -> &'static str {
    match res {
        McResDlAddr::Official => OFFICIAL,
        McResDlAddr::Bangbang93 => BANGBANG93,
        McResDlAddr::Mcbbs => MCBBS,
    }
}

pub async fn download_mc_version_manifest(res: McResDlAddr) -> anyhow::Result<()> {
    const ADDR_SUFFIX: &str = "mc/game/version_manifest_v2.json";

    let dl_addr = select_dl_addr(res);

    let dir = MINECRAFT_ROOT.to_string() + "/versions";
    let file_name = "version_manifest_v2.json";
    let mut file = create_file(&dir, file_name)?;

    let response = get(dl_addr.to_string() + ADDR_SUFFIX).await?.text().await?;

    write_file(&mut file, response)?;
    Ok(())
}

pub fn list_versions() -> anyhow::Result<Vec<String>> {
    let manifest_path = MINECRAFT_ROOT.to_string() + "/versions" + "/version_manifest_v2.json";
    let file = fs::read_to_string(manifest_path)?;
    let data = from_str::<Value>(&file)?;

    let mut version_manifest = Vec::new();

    if let Value::Array(arr) = &data["versions"] {
        for element in arr {
            if let Value::Object(obj) = element {
                version_manifest.push(obj.clone())
            }
        }
    }

    let mut versions = Vec::new();

    for e in &version_manifest {
        if let Some(Value::String(str)) = e.get("id") {
            versions.push(str.clone())
        }
    }

    Ok(versions)
}
