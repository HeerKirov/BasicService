pub fn validate_std_name(name: &str) -> bool {
    name.find(|c: char| !c.is_ascii_alphanumeric()).is_none()
}