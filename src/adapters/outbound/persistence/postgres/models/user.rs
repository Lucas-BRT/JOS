use crate::domain::user::User;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct UserModel {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<UserModel> for User {
    fn from(model: UserModel) -> Self {
        User {
            id: model.id,
            username: model.username,
            display_name: model.display_name,
            email: model.email,
            password: model.password,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
