use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

static USERNAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-z][a-z0-9_]*[a-z0-9]$").unwrap());
const MAX_USERNAME_LENGTH: u64 = 30;
const MIN_USERNAME_LENGTH: u64 = 5;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
#[serde(transparent)]
pub struct UsernameDto {
    #[validate(length(
        min = MIN_USERNAME_LENGTH,
        max = MAX_USERNAME_LENGTH,
        message = "Username must be between 5 and 30 characters"
    ))]
    #[validate(regex(
        path = *USERNAME_REGEX,
        message = "Username can only contain lowercase letters and under&scores"
    ))]
    pub value: String,
}

static DISPLAY_NAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9_ ]{3,20}$").unwrap());
const MAX_DISPLAY_NAME_LENGTH: u64 = 30;
const MIN_DISPLAY_NAME_LENGTH: u64 = 5;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
#[serde(transparent)]
pub struct DisplayNameDto {
    #[validate(length(
        min = MIN_DISPLAY_NAME_LENGTH,
        max = MAX_DISPLAY_NAME_LENGTH,
        message = "Display name must be between 5 and 30 characters"
    ))]
    #[validate(regex(
        path = *DISPLAY_NAME_REGEX,
        message = "Display name can only contain letters, numbers, underscores and spaces"
    ))]
    pub value: String,
}

static PASSWORD_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,100}$").unwrap()
});
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
    #[validate(regex(
        path = *PASSWORD_REGEX,
        message = "Password must contain at least one lowercase letter, one uppercase letter, one number and one special character"
    ))]
    pub value: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateUserDto {
    pub username: UsernameDto,
    pub display_name: DisplayNameDto,
    #[validate(email)]
    pub email: String,
    pub password: PasswordDto,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateUserResponseDto {
    pub username: String,
}
