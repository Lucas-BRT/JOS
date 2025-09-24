use crate::domain::utils::update::Update;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateUserCommand {
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Default, Clone)]
pub struct UpdateUserCommand {
    pub user_id: Uuid,
    pub display_name: Update<String>,
    pub email: Update<String>,
    pub password: Update<String>,
}

#[derive(Debug, Default, Clone)]
pub struct UpdatePasswordCommand {
    pub user_id: Uuid,
    pub new_password: String,
}

#[derive(Debug, Clone, Default)]
pub struct GetUserCommand {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DeleteUserCommand {
    pub id: Uuid,
}

#[derive(Debug, Clone)]
pub struct LoginUserCommand {
    pub email: String,
    pub password: String,
}
