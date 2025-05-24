use serde::{Deserialize, Serialize};

use crate::domain::{email::Email, phone_number::PhoneNumber};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ContactInfo {
    Email(Email),
    Phone(PhoneNumber),
    // TODO: Implement other contact info types and validate the size of the string
    Other(String),
}
