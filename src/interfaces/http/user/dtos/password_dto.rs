use serde::Deserialize;
use validator::{Validate, ValidationError};

const MIN_PASSWORD_LENGTH: u64 = 8;
const MAX_PASSWORD_LENGTH: u64 = 100;

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(transparent)]
pub struct PasswordDto {
    #[validate(length(
        min = MIN_PASSWORD_LENGTH,
        max = MAX_PASSWORD_LENGTH,
        message = "Password must be between 8 and 100 characters"
    ))]
    #[validate(custom(function = "validate_password"))]
    pub value: String,
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 {
        return Err(ValidationError::new("too_short"));
    }

    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(ValidationError::new("no_lowercase"));
    }
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(ValidationError::new("no_uppercase"));
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(ValidationError::new("no_digit"));
    }
    if !password.chars().any(|c| "@$!%*?&".contains(c)) {
        return Err(ValidationError::new("no_special_char"));
    }

    Ok(())
}
