use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::game_system::GameSystem;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum AccessLevel {
    Admin,
    Moderator,
    User,
}

impl Default for AccessLevel {
    fn default() -> Self {
        AccessLevel::User
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct UserStatistics {
    pub total_games_played: u32,
    pub active_games_played: u32,
    pub completed_games_played: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub access_level: AccessLevel,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub nickname: Option<String>,
    pub years_of_experience: Option<u32>,
    pub account_creation_date: DateTime<Utc>,
    pub favorite_game_systems: Option<Vec<GameSystem>>,
    pub statistics: Option<Vec<UserStatistics>>,
}
