use fs_err as fs;
use serde::Serialize;

pub fn serialize_to_json<T: Serialize>(contents: T) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(&contents)
}

pub fn deserialize_json(json: fs::File) {
    todo!()
}
