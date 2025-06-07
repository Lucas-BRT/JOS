use serde::Deserialize;
use validator::Validate;

use crate::domain::user::dtos::CreateUserCommand;

#[derive(Validate)]
pub struct LoginDto {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Validate)]
pub struct RecoveryDto {
    #[validate(email)]
    pub email: String,
}

#[derive(Validate, Deserialize)]
pub struct SignupDto {
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
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
