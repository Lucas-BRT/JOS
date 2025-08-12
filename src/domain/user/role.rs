use std::fmt::Display;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, ToSchema)]
pub enum Role {
    Admin,
    Moderator,
    #[default]
    User,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => write!(f, "admin"),
            Role::Moderator => write!(f, "moderator"),
            Role::User => write!(f, "user"),
        }
    }
}