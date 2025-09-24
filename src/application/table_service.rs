use crate::application::error::ApplicationError;
use crate::domain::entities::*;
use crate::domain::error::*;
use crate::domain::repositories::TableRepository;
use crate::{Error, Result};
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
        let table = self.find_by_id(&command.id).await?;

        if table.gm_id != command.gm_id {
            return Err(Error::Application(ApplicationError::InvalidCredentials));
        }

        self.table_repository.delete(command).await
    }

    pub async fn find_by_id(&self, table_id: &Uuid) -> Result<Table> {
        let command = GetTableCommand {
            id: Some(*table_id),
            ..Default::default()
        };
        let tables = self.table_repository.read(command).await?;
        tables
            .into_iter()
            .next()
            .ok_or_else(|| Error::Domain(DomainError::Table(TableDomainError::TableNotFound(table_id.to_string()))))
    }

    pub async fn get(&self, command: GetTableCommand) -> Result<Vec<Table>> {
        self.table_repository.read(command).await
    }

    pub async fn update(&self, command: UpdateTableCommand) -> Result<Table> {
        self.table_repository.update(command).await
    }
}
