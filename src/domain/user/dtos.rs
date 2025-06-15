use uuid::Uuid;

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

#[derive(Debug, Default)]
pub struct UpdateUserCommand {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub nickname: Option<String>,
    pub years_of_experience: Option<u32>,
}
