use domain::entities::*;
use domain::repositories::{SessionIntentRepository, SessionRepository, TableRepository};
use shared::Result;
use shared::error::{ApplicationError, DomainError, Error};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct SessionService {
    session_repository: Arc<dyn SessionRepository>,
    session_intent_repository: Arc<dyn SessionIntentRepository>,
    table_repository: Arc<dyn TableRepository>,
}

impl SessionService {
    pub fn new(
        session_repository: Arc<dyn SessionRepository>,
        session_intent_repository: Arc<dyn SessionIntentRepository>,
        table_repository: Arc<dyn TableRepository>,
    ) -> Self {
        Self {
            session_repository,
            session_intent_repository,
            table_repository,
        }
    }

    pub async fn schedule_session(
        &self,
        gm_id: Uuid,
        command: CreateSessionCommand,
    ) -> Result<Session> {
        let table = self.table_repository.find_by_id(&command.table_id).await?;

        if let Some(table) = table {
            if table.gm_id != gm_id {
                return Err(Error::Application(ApplicationError::Forbidden));
            }
        } else {
            return Err(Error::Domain(DomainError::EntityNotFound {
                entity_type: "Table",
                entity_id: command.table_id.to_string(),
            }));
        }

        self.session_repository.create(command).await
    }

    pub async fn submit_session_intent(&self, command: CreateSessionIntentCommand) -> Result<()> {
        self.session_intent_repository.create(command).await?;

        Ok(())
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
            Error::Domain(DomainError::EntityNotFound {
                entity_type: "Session",
                entity_id: id.to_string(),
            })
        })
    }

    pub async fn update(&self, command: UpdateSessionCommand) -> Result<Session> {
        self.session_repository.update(command).await
    }

    pub async fn delete(&self, command: DeleteSessionCommand) -> Result<Session> {
        self.session_repository.delete(command).await
    }
}
