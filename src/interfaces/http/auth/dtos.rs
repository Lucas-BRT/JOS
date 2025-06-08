use axum::extract::Multipart;
use serde::Deserialize;
use validator::Validate;

use crate::domain::user::dtos::CreateUserCommand;

const MIN_USERNAME_LENGTH: u64 = 4;
const MAX_USERNAME_LENGTH: u64 = 100;

const MIN_PASSWORD_LENGTH: u64 = 8;
const MAX_PASSWORD_LENGTH: u64 = 200;

const MIN_NICKNAME_LENGTH: u64 = 4;
const MAX_NICKNAME_LENGTH: u64 = 100;

const MIN_BIO_LENGTH: u64 = 2;
const MAX_BIO_LENGTH: u64 = 200;

#[derive(Validate)]
pub struct LoginDto {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = MIN_PASSWORD_LENGTH, max = MAX_PASSWORD_LENGTH))]
    pub password: String,
}

#[derive(Validate)]
pub struct RecoveryDto {
    #[validate(email)]
    pub email: String,
}

#[derive(Validate, Deserialize)]
pub struct SignupDto {
    #[validate(length(min = MIN_USERNAME_LENGTH, max = MAX_USERNAME_LENGTH))]
    pub username: String,
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
            username: self.username,
            email: self.email,
            password: self.password,
        }
    }
}
