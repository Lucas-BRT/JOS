use shared::Result;
use domain::entities::*;
use domain::repositories::SessionCheckinRepository;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct SessionCheckinService {
    session_checkin_repository: Arc<dyn SessionCheckinRepository>,
}

impl SessionCheckinService {
    pub fn new(session_checkin_repository: Arc<dyn SessionCheckinRepository>) -> Self {
        Self {
            session_checkin_repository,
        }
    }

    pub async fn create(&self, command: CreateSessionCheckinCommand) -> Result<SessionCheckin> {
        self.session_checkin_repository.create(command).await
    }

    pub async fn get(&self, command: GetSessionCheckinCommand) -> Result<Vec<SessionCheckin>> {
        self.session_checkin_repository.read(command).await
    }

    pub async fn find_by_id(&self, id: &Uuid) -> Result<SessionCheckin> {
        let command = GetSessionCheckinCommand {
            id: Some(*id),
            ..Default::default()
        };
        let session_checkins = self.session_checkin_repository.read(command).await?;
        session_checkins.into_iter().next().ok_or_else(|| {
            shared::error::Error::Domain(shared::error::DomainError::EntityNotFound(
                format!("Session checkin not found: {}", id)
            ))
        })
    }

    pub async fn find_by_session_intent_id(
        &self,
        session_intent_id: &Uuid,
    ) -> Result<Vec<SessionCheckin>> {
        let command = GetSessionCheckinCommand {
            session_intent_id: Some(*session_intent_id),
            ..Default::default()
        };
        self.session_checkin_repository.read(command).await
    }

    pub async fn find_by_attendance(&self, attendance: bool) -> Result<Vec<SessionCheckin>> {
        let command = GetSessionCheckinCommand {
            attendance: Some(attendance),
            ..Default::default()
        };
        self.session_checkin_repository.read(command).await
    }

    pub async fn update(&self, command: UpdateSessionCheckinCommand) -> Result<SessionCheckin> {
        self.session_checkin_repository.update(command).await
    }

    pub async fn delete(&self, command: DeleteSessionCheckinCommand) -> Result<SessionCheckin> {
        self.session_checkin_repository.delete(command).await
    }
}
