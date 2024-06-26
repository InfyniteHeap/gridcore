use fs_err as fs;

use super::download_content;

use serde_json::{from_str, Map, Value};

const OFFICIAL: &str = "https://piston-meta.mojang.com/";
const BANGBANG93: &str = "https://bmclapi2.bangbang93.com/";

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
}

pub enum MinecraftVersionType {
    Release,
    Snapshot,
    OldAlpha,
}

impl MinecraftVersionType {
    pub fn minecraft_version_type(self) -> &'static str {
        match self {
            Self::Release => "release",
            Self::Snapshot => "snapshot",
            Self::OldAlpha => "old_alpha",
        }
    }
}

fn select_dl_addr(res: McResDlAddr) -> &'static str {
    match res {
        McResDlAddr::Official => OFFICIAL,
        McResDlAddr::Bangbang93 => BANGBANG93,
    }
}

pub async fn download_mc_version_manifest(res: McResDlAddr) -> anyhow::Result<()> {
    const ADDR_SUFFIX: &str = "mc/game/version_manifest_v2.json";

    let dl_addr = select_dl_addr(res);

    let dir = MINECRAFT_ROOT.to_string() + "/versions";
    let file_name = "version_manifest_v2.json";

    download_content(file_name, &dir, &(dl_addr.to_string() + ADDR_SUFFIX)).await?;

    Ok(())
}

pub fn read_version_manifest() -> anyhow::Result<Vec<Map<String, Value>>> {
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

    Ok(version_manifest)
}

/// The return contents will display on UI interface.
pub fn list_versions() -> anyhow::Result<Vec<(String, String)>> {
    let version_manifest = read_version_manifest()?;

    let mut versions = Vec::new();

    for e in version_manifest {
        if let (Some(Value::String(id)), Some(Value::String(ty))) = (e.get("id"), e.get("type")) {
            versions.push((id.clone(), ty.clone()))
        }
    }

    Ok(versions)
}

pub async fn download_specific_mc_version_manifest(version: &str) -> anyhow::Result<()> {
    let version_manifest_path = MINECRAFT_ROOT.to_string() + "/versions/" + version;
    let version_manifest = read_version_manifest()?;

    for e in version_manifest {
        if match e.get("id") {
            Some(id) => id,
            None => return Err(anyhow::Error::msg("No matched version!")),
        } == version
        {
            if let Some(Value::String(url)) = e.get("url") {
                let file_name = version.to_string() + ".json";
                download_content(&file_name, &version_manifest_path, url).await?;
            }
        }
    }

    Ok(())
}

pub async fn download_mc_files() -> anyhow::Result<()> {
    Ok(())
}
