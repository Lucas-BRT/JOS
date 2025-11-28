use domain::entities::*;
use domain::repositories::TableRepository;
use shared::Result;
use shared::error::Error;
use shared::error::{ApplicationError, DomainError};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct TableService {
    table_repository: Arc<dyn TableRepository>,
}

impl TableService {
    pub fn new(table_repository: Arc<dyn TableRepository>) -> Self {
        Self { table_repository }
    }

    pub async fn create(&self, command: CreateTableCommand) -> Result<Table> {
        self.table_repository.create(command).await
    }

    pub async fn delete(&self, command: DeleteTableCommand) -> Result<Table> {
        self.table_repository.delete(command).await
    }

    pub async fn find_by_id(&self, table_id: &Uuid) -> Result<Table> {
        let table = self.table_repository.find_by_id(*table_id).await?;
        table.ok_or_else(|| {
            Error::Domain(DomainError::EntityNotFound {
                entity_type: "Table",
                entity_id: table_id.to_string(),
            })
        })
    }

    pub async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Vec<Table>> {
        self.table_repository.find_by_user_id(user_id).await
    }

    pub async fn find_by_session_id(&self, session_id: &Uuid) -> Result<Option<Table>> {
        self.table_repository.find_by_session_id(session_id).await
    }

    pub async fn update(&self, command: UpdateTableCommand) -> Result<Table> {
        self.table_repository.update(command).await
    }

    pub async fn get_table_sessions(&self, table_id: Uuid, user_id: Uuid) -> Result<Vec<Session>> {
        let table = self.find_by_id(&table_id).await?;

        if table.gm_id != user_id {
            return Err(Error::Application(ApplicationError::InvalidCredentials));
        }

        // This would need to be injected or called through SessionService
        // For now, return empty vec as placeholder
        Ok(Vec::new())
    }

    pub async fn verify_table_ownership(&self, table_id: Uuid, user_id: Uuid) -> Result<Table> {
        let table = self.find_by_id(&table_id).await?;

        if table.gm_id != user_id {
            return Err(Error::Application(ApplicationError::InvalidCredentials));
        }

        Ok(table)
    }

    pub async fn get_table_requests(&self, table_id: Uuid, user_id: Uuid) -> Result<Table> {
        self.verify_table_ownership(table_id, user_id).await
    }
}
