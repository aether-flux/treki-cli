use serde_json::{self, Value};
use std::collections::HashMap;

pub fn parse_headers(header_str: &str) -> HashMap<String, String> {
    // Parse the header string as JSON
    let json_value: Value = serde_json::from_str(header_str).unwrap_or(Value::Object(Default::default()));

    // If it's a JSON object, convert it to a HashMap<String, String>
    json_value.as_object()
        .unwrap_or(&serde_json::Map::new())  // Default to an empty object if parsing fails
        .iter()
        .filter_map(|(key, value)| {
            value.as_str().map(|v| (key.clone(), v.to_string()))
        })
        .collect()
}
