use crate::domain::user::entity::{AccessLevel, User};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub struct Model {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub access_level: AccessLevelModel,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub nickname: Option<String>,
    pub years_of_experience: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Into<User> for Model {
    fn into(self) -> User {
        User {
            id: self.id,
            name: self.name,
            email: self.email,
            password_hash: self.password_hash,
            access_level: self.access_level.into(),
            bio: self.bio,
            avatar_url: self.avatar_url,
            nickname: self.nickname,
            // guarantees that the years of experience is a positive number
            years_of_experience: self
                .years_of_experience
                .map(|years| years.try_into().unwrap_or_default()),
            account_creation_date: self.created_at,
            favorite_game_systems: None,
            statistics: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Type)]
#[sqlx(rename_all = "lowercase")]
#[sqlx(type_name = "access_level")]
pub enum AccessLevelModel {
    Admin,
    Moderator,
    User,
}

impl Default for AccessLevelModel {
    fn default() -> Self {
        AccessLevelModel::User
    }
}

impl Into<AccessLevel> for AccessLevelModel {
    fn into(self) -> AccessLevel {
        match self {
            AccessLevelModel::Admin => AccessLevel::Admin,
            AccessLevelModel::Moderator => AccessLevel::Moderator,
            AccessLevelModel::User => AccessLevel::User,
        }
    }
}

impl Into<AccessLevelModel> for AccessLevel {
    fn into(self) -> AccessLevelModel {
        match self {
            AccessLevel::Admin => AccessLevelModel::Admin,
            AccessLevel::Moderator => AccessLevelModel::Moderator,
            AccessLevel::User => AccessLevelModel::User,
        }
    }
}
