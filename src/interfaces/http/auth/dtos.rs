use crate::domain::user::{
    dtos::{CreateUserCommand, LoginUserCommand},
    entity::User,
};
use axum::extract::Multipart;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

const MIN_USERNAME_LENGTH: u64 = 4;
const MAX_USERNAME_LENGTH: u64 = 100;

const MIN_PASSWORD_LENGTH: u64 = 8;
const MAX_PASSWORD_LENGTH: u64 = 200;

const MIN_NICKNAME_LENGTH: u64 = 4;
const MAX_NICKNAME_LENGTH: u64 = 100;

const MIN_BIO_LENGTH: u64 = 2;
const MAX_BIO_LENGTH: u64 = 200;

#[derive(Validate, Deserialize)]
pub struct LoginDto {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = MIN_PASSWORD_LENGTH, max = MAX_PASSWORD_LENGTH))]
    pub password: String,
}

impl Into<LoginUserCommand> for LoginDto {
    fn into(self) -> LoginUserCommand {
        LoginUserCommand {
            email: self.email,
            password: self.password,
        }
    }
}

#[derive(Validate)]
pub struct RecoveryDto {
    #[validate(email)]
    pub email: String,
}

#[derive(Validate, Deserialize)]
pub struct SignupDto {
    #[validate(length(min = MIN_USERNAME_LENGTH, max = MAX_USERNAME_LENGTH))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = MIN_PASSWORD_LENGTH, max = MAX_PASSWORD_LENGTH))]
    pub password: String,
    #[validate(length(min = MIN_PASSWORD_LENGTH, max = MAX_PASSWORD_LENGTH))]
    pub confirm_password: String,
}

#[derive(Debug, Deserialize)]
pub enum Gender {
    Male,
    Female,
    NonBinary,
    Other,
}

#[derive(Debug, Deserialize, Serialize, thiserror::Error)]
pub enum InvalidImageError {
    #[error("Image is too large: {0}")]
    TooLarge(String),
    #[error("The format {0} is not supported")]
    InvalidFormat(String),
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateProfile {
    #[validate(length(min = 2, max = 32))]
    pub nickname: Option<String>,
    #[validate(length(min = 2, max = 128))]
    pub bio: Option<String>,
    pub gender: Option<Gender>,
    #[validate(range(min = 0, max = 70))]
    pub years_playing: Option<u8>,
}

pub struct UpdateProfileWithAvatar {
    pub form: UpdateProfile,
    pub avatar: Option<Multipart>,
}

impl Into<CreateUserCommand> for SignupDto {
    fn into(self) -> CreateUserCommand {
        CreateUserCommand {
            name: self.name,
            email: self.email,
            password: self.password,
            confirm_password: self.confirm_password,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserSignupResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

impl From<User> for UserSignupResponse {
    fn from(user: User) -> Self {
        UserSignupResponse {
            id: user.id,
            name: user.name,
            email: user.email,
        }
    }
}
