use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

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
