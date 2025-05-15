use crate::constants::MINECRAFT_ROOT;
use crate::error_handling::JsonError;
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
