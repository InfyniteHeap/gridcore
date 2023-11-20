use fs_err as fs;
use serde::Serialize;
use serde_json::Value;

pub fn serialize_to_json<T: Serialize>(contents: T) {
    todo!()
}

pub fn deserialize_json(json: fs::File) {
    todo!()
}

// Transfer responses into JSON text, fetch necessary fields and store them in the instance of structure.
// Use turbofish syntax to simplify data processing.
#[inline]
pub fn parse_response(response: &str) -> Result<Value, serde_json::Error> {
    serde_json::from_str::<Value>(response)
}

#[inline]
pub fn fetch_value(json_text: Value, key: &str) -> Option<String> {
    Some(json_text.get(key)?.to_string())
}
