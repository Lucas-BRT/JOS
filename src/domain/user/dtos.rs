pub struct CreateUserCommand {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct UpdateUserCommand {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}
