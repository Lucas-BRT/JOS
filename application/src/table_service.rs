use domain::entities::*;
use domain::repositories::TableRepository;
use shared::Result;
use shared::error::{ApplicationError, DomainError, Error};
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

    pub async fn create(&self, command: &CreateTableCommand) -> Result<Table> {
        self.table_repository.create(command).await
    }

    pub async fn delete(&self, command: &DeleteTableCommand) -> Result<Table> {
        let table = self.find_by_id(&command.id).await?;

        if table.gm_id != command.gm_id {
            return Err(Error::Application(ApplicationError::Forbidden));
        }

        self.table_repository.delete(command).await
    }

    pub async fn find_by_id(&self, table_id: &Uuid) -> Result<Table> {
        let tables = self.table_repository.find_by_id(table_id).await?;
        tables.into_iter().next().ok_or_else(|| {
            Error::Domain(DomainError::EntityNotFound {
                entity_type: "Table",
                entity_id: table_id.to_string(),
            })
        })
    }

    pub async fn find_by_session_id(&self, session_id: &Uuid) -> Result<Option<Table>> {
        let table = self.table_repository.find_by_session_id(session_id).await?;

        Ok(table)
    }

    pub async fn get_all(&self) -> Result<Vec<Table>> {
        self.table_repository.get_all().await
    }

    pub async fn update(&self, command: &UpdateTableCommand) -> Result<Table> {
        self.table_repository.update(command).await
    }
}
