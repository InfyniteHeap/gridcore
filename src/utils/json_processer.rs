use crate::file_system;

use std::path::Path;

use serde::Serialize;
use serde_json::Value;

/// Parse JSON from string.
///
/// This is usually used when parsing JSON directly got from Internet.
pub fn parse_from_string(json: &str) -> serde_json::Result<Value> {
    serde_json::from_str::<Value>(json)
}

/// Parse JSON from string.
///
/// This is usually used when parsing JSON stored on local machine.
pub async fn read(json_path: &Path, json_name: &str) -> anyhow::Result<Value> {
    let json_file = file_system::read_file_to_string(json_path, json_name).await?;
    Ok(parse_from_string(&json_file)?)
}

/// Convert an instance into serialized JSON data.
pub fn convert_to_string(json: impl Serialize) -> serde_json::Result<String> {
    serde_json::to_string_pretty(&json)
}
