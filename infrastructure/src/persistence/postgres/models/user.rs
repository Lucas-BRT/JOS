use chrono::{DateTime, Utc};
use domain::entities::User;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserModel {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<UserModel> for User {
    fn from(model: UserModel) -> Self {
        User {
            id: model.id,
            username: model.username,
            email: model.email,
            password: model.password,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
