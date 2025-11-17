use crate::entities::Update;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateUserCommand {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Default, Clone)]
pub struct UpdateUserCommand {
    pub user_id: Uuid,
    pub username: Update<String>,
    pub email: Update<String>,
    pub password: Update<String>,
}

#[derive(Debug, Default, Clone)]
pub struct UpdatePasswordCommand {
    pub user_id: Uuid,
    pub current_password: String,
    pub new_password: String,
}

#[derive(Debug, Clone, Default)]
pub struct GetUserCommand {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DeleteUserCommand {
    pub id: Uuid,
}

#[derive(Debug, Clone)]
pub struct DeleteAccountCommand {
    pub user_id: Uuid,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct LoginUserCommand {
    pub email: String,
    pub password: String,
}
