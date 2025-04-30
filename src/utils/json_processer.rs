use crate::error_handling::JsonError;
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
pub async fn read<P: AsRef<Path>>(json_path: &P, json_name: &str) -> Result<Value, JsonError> {
    let json_file = file_system::read_file_to_string(json_path, json_name).await?;
    parse_from_string(&json_file).map_err(Into::into)
}

/// Convert an instance into serialized JSON data.
pub fn convert_to_string<S: Serialize>(json: &S) -> serde_json::Result<String> {
    serde_json::to_string_pretty(json)
}
