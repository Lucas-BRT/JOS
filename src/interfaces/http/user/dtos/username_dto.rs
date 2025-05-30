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
