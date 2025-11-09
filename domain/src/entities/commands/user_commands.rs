use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateUserCommand<'a> {
    pub id: Uuid,
    pub username: &'a str,
    pub email: &'a str,
    pub password: String,
}

#[derive(Debug, Default, Clone)]
pub struct UpdateUserCommand<'a> {
    pub user_id: Uuid,
    pub username: Option<&'a str>,
    pub email: Option<&'a str>,
    pub password: Option<&'a str>,
}

#[derive(Debug, Default, Clone)]
pub struct UpdatePasswordCommand<'a> {
    pub user_id: Uuid,
    pub current_password: &'a str,
    pub new_password: &'a str,
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
pub struct DeleteAccountCommand<'a> {
    pub user_id: Uuid,
    pub password: &'a str,
}

#[derive(Debug, Clone)]
pub struct LoginUserCommand<'a> {
    pub email: &'a str,
    pub password: &'a str,
}
