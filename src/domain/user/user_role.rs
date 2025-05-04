use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum UserRole {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "user")]
    User,
}
