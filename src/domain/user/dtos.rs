use crate::domain::utils::update::Update;
use uuid::Uuid;

pub struct CreateUserCommand {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub struct LoginUserCommand {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Default)]
pub struct UpdateUserCommand {
    pub id: Uuid,
    pub name: Update<String>,
    pub email: Update<String>,
    pub password: Update<String>,
    pub bio: Update<Option<String>>,
    pub avatar_url: Update<Option<String>>,
    pub nickname: Update<Option<String>>,
}
