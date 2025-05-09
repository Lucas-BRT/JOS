use serde::{Deserialize, Serialize};

use super::{
    display_name::DisplayName,
    email::Email,
    password::{HashPassword, RawPassword},
    user_role::UserRole,
    username::Username,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub email: Email,
    pub username: Username,
    pub display_name: DisplayName,
    pub password: RawPassword,
}
