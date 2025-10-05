use domain::entities::*;
use domain::repositories::SessionRepository;
use shared::Result;
use shared::error::Error;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct SessionService {
    session_repository: Arc<dyn SessionRepository>,
}

impl SessionService {
    pub fn new(session_repository: Arc<dyn SessionRepository>) -> Self {
        Self { session_repository }
    }

    pub async fn create(&self, command: CreateSessionCommand) -> Result<Session> {
        self.session_repository.create(command).await
    }

    pub async fn get(&self, command: GetSessionCommand) -> Result<Vec<Session>> {
        self.session_repository.read(command).await
    }

    pub async fn find_by_id(&self, id: &Uuid) -> Result<Session> {
        let command = GetSessionCommand {
            id: Some(*id),
            ..Default::default()
        };
        let sessions = self.session_repository.read(command).await?;
        sessions.into_iter().next().ok_or_else(|| {
            Error::Domain(shared::error::DomainError::EntityNotFound(format!(
                "Session not found: {}",
                id
            )))
        })
    }

    pub async fn update(&self, command: UpdateSessionCommand) -> Result<Session> {
        self.session_repository.update(command).await
    }

    pub async fn delete(&self, command: DeleteSessionCommand) -> Result<Session> {
        self.session_repository.delete(command).await
    }
}
