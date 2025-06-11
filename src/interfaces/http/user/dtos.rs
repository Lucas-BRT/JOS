use crate::domain::{
    game_system::GameSystem,
    user::entity::{AccessLevel, User, UserStatistics},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct MeResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub access_level: AccessLevel,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub nickname: Option<String>,
    pub years_of_experience: Option<u32>,
    pub account_creation_date: DateTime<Utc>,
    pub favorite_game_systems: Option<Vec<GameSystem>>,
    pub statistics: Option<Vec<UserStatistics>>,
}

impl From<User> for MeResponse {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            email: value.email,
            name: value.name,
            access_level: value.access_level,
            bio: value.bio,
            avatar_url: value.avatar_url,
            nickname: value.nickname,
            years_of_experience: value.years_of_experience,
            account_creation_date: value.account_creation_date,
            favorite_game_systems: value.favorite_game_systems,
            statistics: value.statistics,
        }
    }
}
