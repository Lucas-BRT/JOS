use serde::{Deserialize, Serialize};

use crate::domain::email::Email;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ContactInfo {
    Email(Email),
    Phone(String),
    Address(String),
}
