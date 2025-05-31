use serde::{Deserialize, Serialize};

use crate::domain::user::vo::{EmailVo, PhoneNumberVo};

use super::type_wraper::TypeWrapped;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ContactInfoTypeVo {
    Email(EmailVo),
    Phone(PhoneNumberVo),
    // TODO: Implement other contact info types and validate the size of the string
    Other(String),
}

impl ToString for ContactInfoTypeVo {
    fn to_string(&self) -> String {
        match self {
            ContactInfoTypeVo::Email(email) => email.raw(),
            ContactInfoTypeVo::Phone(phone) => phone.raw(),
            ContactInfoTypeVo::Other(other) => other.clone(),
        }
    }
}
