use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    domain::type_wraper::TypeWrapped, error::UserValidationError,
    infra::db::postgres::models::user::UserRow,
};

use super::{
    display_name::DisplayName, email::Email, password::HashPassword, user_role::UserRole,
    username::Username,
};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct User {
    id: Uuid,
    username: Username,
    display_name: DisplayName,
    email: Email,
    password_hash: HashPassword,
    user_role: UserRole,
}

impl TryFrom<UserRow> for User {
    type Error = UserValidationError;

    fn try_from(user_row: UserRow) -> Result<Self, Self::Error> {
        let user = User {
            id: user_row.id,
            username: Username::parse(user_row.username)?,
            display_name: DisplayName::parse(user_row.display_name)?,
            email: Email::parse(user_row.email)?,
            password_hash: HashPassword::parse(user_row.password_hash)?,
            user_role: UserRole::from(user_row.user_role),
        };

        Ok(user)
    }
}
