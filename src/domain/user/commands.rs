use crate::{Result, application::PasswordService, domain::utils::update::Update};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateUserCommand {
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
}

impl CreateUserCommand {
    pub fn new(username: String, display_name: String, email: String, password: String) -> Self {
        Self {
            username,
            display_name,
            email,
            password,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LoginUserCommand {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Default, Clone)]
pub struct UpdateUserCommand<'a> {
    pub id: Uuid,
    pub display_name: Update<&'a str>,
    pub email: Update<&'a str>,
    pub password: Update<&'a str>,
}

#[derive(Debug, Clone)]
pub struct DeleteUserCommand {
    pub id: Uuid,
}
