use crate::{domain::utils::type_wraper::TypeWrapped, interfaces::http::user::dtos::CreateUserDto};
use serde::{Deserialize, Serialize};

use super::{
    error::UserDomainError,
    vo::{DisplayNameVo, EmailVo, Hashed, PasswordVo, UsernameVo},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewUser {
    pub username: UsernameVo,
    pub display_name: DisplayNameVo,
    pub email: EmailVo,
    pub password: PasswordVo<Hashed>,
}

impl TryFrom<&CreateUserDto> for NewUser {
    type Error = UserDomainError;
    fn try_from(value: &CreateUserDto) -> Result<Self, Self::Error> {
        let username = UsernameVo::parse(value.username.value.clone())?;
        let display_name = DisplayNameVo::parse(value.display_name.value.clone())?;
        let email = EmailVo::parse(value.email.value.clone())?;
        let password = PasswordVo::parse(value.password.value.clone())?.hash()?;

        Ok(NewUser {
            username,
            display_name,
            email,
            password,
        })
    }
}
