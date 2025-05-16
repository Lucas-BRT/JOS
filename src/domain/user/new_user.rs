use super::super::type_wraper::TypeWrapped;
use super::password::HashPassword;
use super::{display_name::DisplayName, email::Email, password::RawPassword, username::Username};
use crate::error::{Error, ValidationError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub display_name: String,
    pub password: String,
}

pub struct ValidatedNewUser {
    pub username: Username,
    pub display_name: DisplayName,
    pub email: Email,
    pub password_hash: HashPassword,
}

impl TryFrom<NewUser> for ValidatedNewUser {
    type Error = Error;

    fn try_from(user: NewUser) -> Result<Self, Self::Error> {
        let username = Username::parse(user.username)
            .map_err(|e| Error::Validation(ValidationError::User(e)))?;

        let display_name = DisplayName::parse(user.display_name)
            .map_err(|e| Error::Validation(ValidationError::User(e)))?;

        let email =
            Email::parse(user.email).map_err(|e| Error::Validation(ValidationError::User(e)))?;

        let password_hash = RawPassword::parse(user.password)
            .map_err(|e| Error::Validation(ValidationError::User(e)))?
            .hash()
            .map_err(|e| Error::Validation(ValidationError::User(e)))?;

        Ok(Self {
            username,
            display_name,
            email,
            password_hash,
        })
    }
}
