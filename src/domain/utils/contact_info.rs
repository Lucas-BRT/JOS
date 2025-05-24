use serde::{Deserialize, Serialize};

use crate::domain::user::phone_number::PhoneNumber;

use super::email::Email;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ContactInfo {
    Email(Email),
    Phone(PhoneNumber),
    // TODO: Implement other contact info types and validate the size of the string
    Other(String),
}
