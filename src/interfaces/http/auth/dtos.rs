use serde::Deserialize;
use validator::Validate;

use crate::domain::user::dtos::CreateUserCommand;

const MIN_USERNAME_LENGTH: u64 = 4;
const MAX_USERNAME_LENGTH: u64 = 100;

const MIN_PASSWORD_LENGTH: u64 = 8;
const MAX_PASSWORD_LENGTH: u64 = 200;

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

impl Into<CreateUserCommand> for SignupDto {
    fn into(self) -> CreateUserCommand {
        CreateUserCommand {
            username: self.username,
            email: self.email,
            password: self.password,
        }
    }
}
