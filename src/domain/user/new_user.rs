use super::display_name::DisplayName;
use super::password::HashPassword;
use super::{password::RawPassword, username::Username};
use crate::core::error::{AppError, ValidationError};
use crate::domain::utils::email::Email;
use crate::domain::utils::type_wraper::TypeWrapped;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub display_name: String,
    pub password: String,
}

pub struct ValidatedUser {
    pub username: Username,
    pub display_name: DisplayName,
    pub email: Email,
    pub password_hash: HashPassword,
}

impl TryFrom<NewUser> for ValidatedUser {
    type Error = AppError;

    fn try_from(user: NewUser) -> Result<Self, Self::Error> {
        let username = Username::parse(user.username)
            .map_err(|e| AppError::Validation(ValidationError::User(e)))?;

        let display_name = DisplayName::parse(user.display_name)
            .map_err(|e| AppError::Validation(ValidationError::User(e)))?;

        let email =
            Email::parse(user.email).map_err(|e| AppError::Validation(ValidationError::User(e)))?;

        let password_hash = RawPassword::parse(user.password)
            .map_err(|e| AppError::Validation(ValidationError::User(e)))?
            .hash()
            .map_err(|e| AppError::Validation(ValidationError::User(e)))?;

        Ok(Self {
            username,
            display_name,
            email,
            password_hash,
        })
    }
}
