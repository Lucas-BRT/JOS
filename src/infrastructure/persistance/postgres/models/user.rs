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

impl From<Model> for User {
    fn from(model: Model) -> Self {
        User {
            id: model.id,
            name: model.name,
            email: model.email,
            password_hash: model.password_hash,
            access_level: model.access_level.into(),
            bio: model.bio,
            avatar_url: model.avatar_url,
            nickname: model.nickname,
            // guarantees that the years of experience is a positive number
            years_of_experience: model
                .years_of_experience
                .map(|years| years.try_into().unwrap_or_default()),
            account_creation_date: model.created_at,
            favorite_game_systems: None,
            statistics: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Type, Default)]
#[sqlx(rename_all = "lowercase")]
#[sqlx(type_name = "access_level")]
pub enum AccessLevelModel {
    Admin,
    Moderator,
    #[default]
    User,
}

impl From<AccessLevelModel> for AccessLevel {
    fn from(value: AccessLevelModel) -> Self {
        match value {
            AccessLevelModel::Admin => AccessLevel::Admin,
            AccessLevelModel::Moderator => AccessLevel::Moderator,
            AccessLevelModel::User => AccessLevel::User,
        }
    }
}

impl From<AccessLevel> for AccessLevelModel {
    fn from(level: AccessLevel) -> Self {
        match level {
            AccessLevel::Admin => AccessLevelModel::Admin,
            AccessLevel::Moderator => AccessLevelModel::Moderator,
            AccessLevel::User => AccessLevelModel::User,
        }
    }
}
