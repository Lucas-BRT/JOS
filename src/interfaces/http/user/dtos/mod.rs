mod displayname_dto;
mod email_dto;
mod password_dto;
mod username_dto;

use email_dto::EmailDto;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub use displayname_dto::DisplayNameDto;
pub use password_dto::PasswordDto;
pub use username_dto::UsernameDto;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserDto {
    pub username: UsernameDto,
    pub display_name: DisplayNameDto,
    pub email: EmailDto,
    pub password: PasswordDto,
}

impl Validate for CreateUserDto {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        self.display_name.validate()?;
        self.username.validate()?;
        self.email.validate()?;
        self.password.validate()?;

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateUserResponseDto {
    pub username: String,
}
