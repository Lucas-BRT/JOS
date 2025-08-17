use crate::domain::user::User;
use super::enums::ERoles;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct Model {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
    pub role: ERoles,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<Model> for User {
    fn from(model: Model) -> Self {
        User {
            id: model.id,
            username: model.username,
            display_name: model.display_name,
            email: model.email,
            password: model.password,
            role: model.role.into(),
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}