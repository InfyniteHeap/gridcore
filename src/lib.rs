pub mod auth;
pub mod checksum;
pub mod decompress;
pub mod download;
pub mod error_handling;
pub mod file_system;
pub mod http;
pub mod json;
pub mod launch;
pub mod path;

use download::game;

use serde_json::Value;

/// The return contents will display on UI interface.
pub async fn list_versions() -> anyhow::Result<Vec<(String, String)>> {
    let version_manifest = game::read_version_manifest().await?;

    let mut versions = Vec::new();

    for e in version_manifest {
        if let (Some(Value::String(id)), Some(Value::String(ty))) = (e.get("id"), e.get("type")) {
            versions.push((id.clone(), ty.clone()))
        }
    }

    Ok(versions)
}
