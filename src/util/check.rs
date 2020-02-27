pub fn validate_std_name(name: &str) -> bool {
    name.find(|c: char| !c.is_ascii_alphanumeric() && c != '_' && c != '-').is_none()
}

pub fn validate_url_json(url: &serde_json::Value) -> bool {
    if let serde_json::Value::Object(map) = url {
        for v in map.values() {
            if ! v.is_string() {
                return false
            }
        }
        true
    }else{
        false
    }
}