use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
#[serde(transparent)]
pub struct EmailDto {
    #[validate(email)]
    pub value: String,
}
