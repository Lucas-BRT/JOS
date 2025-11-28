use chrono::{DateTime, Utc};
use domain::entities::session_checkin::{
    SessionCheckinData, SessionFinalizationData, SessionFinalizationResult,
};
use domain::entities::*;
use domain::repositories::{SessionRepository, TableRepository};
use shared::Result;
use shared::error::{ApplicationError, DomainError, Error};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct SessionService {
    session_repository: Arc<dyn SessionRepository>,
    table_repository: Arc<dyn TableRepository>,
}

impl SessionService {
    pub fn new(
        session_repository: Arc<dyn SessionRepository>,
        table_repository: Arc<dyn TableRepository>,
    ) -> Self {
        Self {
            session_repository,
            table_repository,
        }
    }

    pub async fn schedule_session(
        &self,
        gm_id: Uuid,
        command: CreateSessionCommand,
    ) -> Result<Session> {
        let table = self.table_repository.find_by_id(command.table_id).await?;

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

    pub async fn update_session_with_validation(
        &self,
        gm_id: Uuid,
        session_id: Uuid,
        title: Option<String>,
        description: Option<String>,
        scheduled_for: Option<DateTime<Utc>>,
        status: Option<SessionStatus>,
    ) -> Result<Session> {
        let table = self
            .table_repository
            .find_by_session_id(&session_id)
            .await?
            .ok_or(Error::Domain(DomainError::EntityNotFound {
                entity_type: "Session",
                entity_id: session_id.to_string(),
            }))?;

        if table.gm_id != gm_id {
            return Err(Error::Domain(DomainError::BusinessRuleViolation {
                message: "User is not the GM of the table".to_string(),
            }));
        }

        let session = self.find_by_id(&session_id).await?;

        let command = UpdateSessionCommand {
            id: session.id,
            title,
            description,
            scheduled_for,
            status,
        };

        self.update(command).await
    }

    pub async fn delete_session_with_validation(
        &self,
        gm_id: Uuid,
        session_id: Uuid,
    ) -> Result<()> {
        let table = self
            .table_repository
            .find_by_session_id(&session_id)
            .await?
            .ok_or(Error::Domain(DomainError::EntityNotFound {
                entity_type: "Session",
                entity_id: session_id.to_string(),
            }))?;

        if gm_id != table.gm_id {
            return Err(Error::Application(ApplicationError::InvalidCredentials));
        }

        self.delete(DeleteSessionCommand { id: session_id }).await?;
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

    pub async fn start_session(&self, gm_id: Uuid, session_id: Uuid) -> Result<Session> {
        let table = match self
            .table_repository
            .find_by_session_id(&session_id)
            .await?
        {
            Some(table) => table,
            None => {
                return Err(Error::Domain(DomainError::EntityNotFound {
                    entity_type: "table",
                    entity_id: session_id.to_string(),
                }));
            }
        };

        if gm_id != table.gm_id {
            return Err(Error::Application(ApplicationError::InvalidCredentials));
        }

        if self
            .session_repository
            .find_by_id(session_id)
            .await?
            .is_none()
        {
            return Err(Error::Domain(DomainError::EntityNotFound {
                entity_type: "session",
                entity_id: session_id.to_string(),
            }));
        };

        let update_command = UpdateSessionCommand {
            id: session_id,
            title: None,
            description: None,
            scheduled_for: None,
            status: Some(SessionStatus::InProgress),
        };

        self.session_repository.update(update_command).await
    }

    pub async fn finalize_session_with_checkins(
        &self,
        gm_id: Uuid,
        session_id: Uuid,
        checkins: Vec<SessionCheckinData>,
    ) -> Result<SessionFinalizationResult> {
        let session = self
            .session_repository
            .find_by_id(session_id)
            .await?
            .ok_or(Error::Domain(DomainError::EntityNotFound {
                entity_type: "Session",
                entity_id: session_id.to_string(),
            }))?;

        let table = self
            .table_repository
            .find_by_id(session.table_id)
            .await?
            .ok_or(Error::Domain(DomainError::EntityNotFound {
                entity_type: "Table",
                entity_id: session.table_id.to_string(),
            }))?;

        if table.gm_id != gm_id {
            return Err(Error::Application(ApplicationError::InvalidCredentials));
        }

        if session.status != SessionStatus::InProgress {
            return Err(Error::Domain(DomainError::BusinessRuleViolation {
                message: "Only sessions in progress can be finalized".into(),
            }));
        }

        let finalization_data = SessionFinalizationData {
            session_id,
            checkins,
        };

        let result = self
            .session_repository
            .finalize_session_with_checkins(finalization_data)
            .await?;

        let update_command = UpdateSessionCommand {
            id: session_id,
            title: None,
            description: None,
            scheduled_for: None,
            status: Some(SessionStatus::Completed),
        };

        self.session_repository.update(update_command).await?;

        Ok(result)
    }

    pub async fn get_table_sessions(&self, table_id: Uuid, user_id: Uuid) -> Result<Vec<Session>> {
        let table = self
            .table_repository
            .find_by_id(table_id)
            .await?
            .ok_or_else(|| {
                Error::Domain(DomainError::EntityNotFound {
                    entity_type: "Table",
                    entity_id: table_id.to_string(),
                })
            })?;

        if table.gm_id != user_id {
            return Err(Error::Application(ApplicationError::InvalidCredentials));
        }

        let command = GetSessionCommand {
            table_id: Some(table_id),
            ..Default::default()
        };

        self.session_repository.read(command).await
    }
}
