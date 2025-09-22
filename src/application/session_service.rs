use crate::{Result, domain::entities::Session, domain::repositories::SessionRepository};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct SessionService {
    session_repository: std::sync::Arc<dyn SessionRepository>,
}

impl SessionService {
    pub fn new(session_repository: std::sync::Arc<dyn SessionRepository>) -> Self {
        Self { session_repository }
    }

    pub async fn get_sessions(&self, filters: &SessionFilters) -> Result<SessionListResponse> {
        // TODO: Implement session filtering and pagination
        let sessions = self.session_repository.get_all().await?;
        let sessions = sessions.iter().map(SessionResponse::from).collect();

        Ok(SessionListResponse {
            sessions,
            total: sessions.len() as u64,
            page: filters.page.unwrap_or(1),
            limit: filters.limit.unwrap_or(10),
        })
    }

    pub async fn create_session(
        &self,
        user_id: &Uuid,
        command: &CreateSessionCommand,
    ) -> Result<SessionResponse> {
        let session = Session {
            id: Uuid::new_v4(),
            title: command.title.clone(),
            description: command.description.clone(),
            table_id: command.table_id,
            gm_id: *user_id,
            scheduled_at: command.scheduled_at,
            max_players: command.max_players,
            status: "pending".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.session_repository.create(&session).await?;
        Ok(session.into())
    }

    pub async fn get_session_by_id(&self, session_id: &Uuid) -> Result<SessionResponse> {
        let session = self.session_repository.get_by_id(session_id).await?;
        Ok(session.into())
    }

    pub async fn update_session(
        &self,
        session_id: &Uuid,
        user_id: &Uuid,
        command: &UpdateSessionCommand,
    ) -> Result<SessionResponse> {
        // TODO: Implement session update logic
        let session = self.session_repository.get_by_id(session_id).await?;
        Ok(session.into())
    }

    pub async fn delete_session(&self, session_id: &Uuid, user_id: &Uuid) -> Result<()> {
        // TODO: Implement session deletion logic
        self.session_repository.delete(session_id).await?;
        Ok(())
    }

    pub async fn join_session(
        &self,
        session_id: &Uuid,
        user_id: &Uuid,
        character_name: &str,
    ) -> Result<()> {
        // TODO: Implement join session logic
        Ok(())
    }

    pub async fn leave_session(&self, session_id: &Uuid, user_id: &Uuid) -> Result<()> {
        // TODO: Implement leave session logic
        Ok(())
    }

    pub async fn confirm_session(&self, session_id: &Uuid, user_id: &Uuid) -> Result<()> {
        // TODO: Implement confirm session logic
        Ok(())
    }

    pub async fn decline_session(&self, session_id: &Uuid, user_id: &Uuid) -> Result<()> {
        // TODO: Implement decline session logic
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct SessionFilters {
    pub table_id: Option<Uuid>,
    pub status: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSessionCommand {
    pub title: String,
    pub description: String,
    pub table_id: Uuid,
    pub scheduled_at: DateTime<Utc>,
    pub max_players: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSessionCommand {
    pub title: Option<String>,
    pub description: Option<String>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub max_players: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub table_id: Uuid,
    pub gm_id: Uuid,
    pub scheduled_at: DateTime<Utc>,
    pub max_players: Option<u32>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct SessionListResponse {
    pub sessions: Vec<SessionResponse>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
}

impl From<Session> for SessionResponse {
    fn from(session: Session) -> Self {
        Self {
            id: session.id,
            title: session.title,
            description: session.description,
            table_id: session.table_id,
            gm_id: session.gm_id,
            scheduled_at: session.scheduled_at,
            max_players: session.max_players,
            status: session.status,
            created_at: session.created_at,
            updated_at: session.updated_at,
        }
    }
}

impl From<&Session> for SessionResponse {
    fn from(session: &Session) -> Self {
        Self {
            id: session.id,
            title: session.title.clone(),
            description: session.description.clone(),
            table_id: session.table_id,
            gm_id: session.gm_id,
            scheduled_at: session.scheduled_at,
            max_players: session.max_players,
            status: session.status.clone(),
            created_at: session.created_at,
            updated_at: session.updated_at,
        }
    }
}
