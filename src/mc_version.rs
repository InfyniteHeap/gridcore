use crate::json;
use crate::path::MINECRAFT_ROOT;

use std::path::Path;

use serde_json::{Map, Value};

pub(crate) async fn read_version_manifest() -> anyhow::Result<Vec<Map<String, Value>>> {
/// Reads contents in `version_manifest_v2.json`.
    let manifest_path = format!("{}/versions", MINECRAFT_ROOT);
    let manifest_name = "version_manifest_v2.json";

    let data = json::read(Path::new(&manifest_path), manifest_name).await?;

    let mut manifest = Vec::new();

    if let Value::Array(arr) = &data["versions"] {
        arr.iter().for_each(|element| {
            if let Value::Object(obj) = element {
                manifest.push(obj.to_owned())
            }
        });
    }

    Ok(manifest)
}

/// Lists Minecraft versions and lets frontend display them on UI interface.
pub async fn list_versions() -> anyhow::Result<Vec<(String, String)>> {
    let version_manifest = read_version_manifest().await?;

    let mut versions = Vec::new();

    version_manifest.iter().for_each(|e| {
        if let (Some(Value::String(id)), Some(Value::String(ty))) = (e.get("id"), e.get("type")) {
            versions.push((id.clone(), ty.clone()))
        }
    });

    Ok(versions)
}
