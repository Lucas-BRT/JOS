use validator::ValidationError;

lazy_static::lazy_static! {
    pub static ref USERNAME_REGEX: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9_]+$").expect("Failed to compile username regex");
}

pub fn validate_username(username: &str) -> Result<(), ValidationError> {
    if USERNAME_REGEX.is_match(username) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_username"))
    }
}
