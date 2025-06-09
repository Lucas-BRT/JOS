pub struct CreateUserCommand {
    pub name: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

pub struct LoginUserCommand {
    pub email: String,
    pub password: String,
}

pub struct UpdateUserCommand {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}
