use fs_err as fs;
use serde::Serialize;
use serde_json::Value;

pub fn serialize_to_json<T: Serialize>(contents: T) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(&contents)
}

pub fn deserialize_json(json: fs::File) {
    todo!()
}

pub(crate) fn parse_response(response: &str) -> Value {
    match serde_json::from_str::<Value>(response) {
        Ok(data) => data,
        Err(e) => panic!("{e}"),
    }
}

pub(crate) fn extract_value(json_text: &Value, keys: &[&str]) -> String {
    keys.iter()
        .try_fold(json_text, |acc, &key| {
            if let Ok(index) = key.parse::<usize>() {
                acc.as_array().and_then(|val| val.get(index))
            } else {
                acc.as_object().and_then(|val| val.get(key))
            }
        })
        .and_then(|val| match val {
            Value::String(s) => Some(s.to_owned()),
            _ => None,
        })
        .unwrap_or_else(|| {
            let key_path: Vec<_> = keys.iter().map(|k| k.to_owned()).collect();
            panic!(
                "Failed to extract value from returned json: {}",
                key_path.join("\"][\"")
            )
        })
}
