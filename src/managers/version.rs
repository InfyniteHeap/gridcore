use crate::error_handling::JsonError;
use crate::path::MINECRAFT_ROOT;
use crate::utils::json_processer;

use std::collections::HashMap;

use serde_json::{Map, Value};

/// Reads contents in `version_manifest_v2.json`.
pub(crate) async fn read_version_manifest() -> Result<HashMap<String, Map<String, Value>>, JsonError>
{
    let manifest_path = format!("{}/versions", MINECRAFT_ROOT);
    let manifest_name = "version_manifest_v2.json";

    let data = json_processer::read(&manifest_path, manifest_name).await?;

    let mut manifest = HashMap::new();

    if let Value::Array(arr) = &data["versions"] {
        arr.iter().for_each(|element| {
            if let Value::Object(obj) = element {
                manifest.insert(obj["id"].as_str().unwrap().to_owned(), obj.clone());
            }
        });
    }

    Ok(manifest)
}

/// Lists Minecraft versions and lets frontend display them on UI interface.
pub async fn list_versions() -> Result<Vec<(String, String)>, JsonError> {
    let version_manifest = read_version_manifest().await?;

    let mut versions = Vec::new();

    version_manifest.iter().for_each(|(_, e)| {
        if let (Some(Value::String(id)), Some(Value::String(ty))) = (e.get("id"), e.get("type")) {
            versions.push((id.clone(), ty.clone()))
        }
    });

    Ok(versions)
}
