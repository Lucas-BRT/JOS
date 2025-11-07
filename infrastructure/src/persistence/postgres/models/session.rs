use domain::entities::{Session, SessionStatus};
use shared::prelude::Date;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "session_status", rename_all = "snake_case")]
pub enum ESessionStatus {
    Scheduled,
    InProgress,
    Completed,
    Cancelled,
}

impl From<ESessionStatus> for SessionStatus {
    fn from(value: ESessionStatus) -> Self {
        match value {
            ESessionStatus::Scheduled => SessionStatus::Scheduled,
            ESessionStatus::InProgress => SessionStatus::InProgress,
            ESessionStatus::Completed => SessionStatus::Completed,
            ESessionStatus::Cancelled => SessionStatus::Cancelled,
        }
    }
}

impl From<SessionStatus> for ESessionStatus {
    fn from(value: SessionStatus) -> Self {
        match value {
            SessionStatus::Scheduled => ESessionStatus::Scheduled,
            SessionStatus::InProgress => ESessionStatus::InProgress,
            SessionStatus::Completed => ESessionStatus::Completed,
            SessionStatus::Cancelled => ESessionStatus::Cancelled,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct SessionModel {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub table_id: Uuid,
    pub scheduled_for: Option<Date>,
    pub status: ESessionStatus,
    pub created_at: Date,
    pub updated_at: Date,
}

impl From<SessionModel> for Session {
    fn from(model: SessionModel) -> Self {
        Session {
            id: model.id,
            title: model.title,
            description: model.description,
            table_id: model.table_id,
            scheduled_for: model.scheduled_for,
            status: model.status.into(),
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
