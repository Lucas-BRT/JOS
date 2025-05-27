use serde::{Deserialize, Serialize};

use crate::domain::user::vo::{EmailVo, PhoneNumberVo};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ContactInfoVo {
    Email(EmailVo),
    Phone(PhoneNumberVo),
    // TODO: Implement other contact info types and validate the size of the string
    Other(String),
}
