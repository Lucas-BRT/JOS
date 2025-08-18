use crate::application::error::ApplicationError;
use crate::domain::table::commands::*;
use crate::domain::table::{
    commands::GetTableCommand, entity::Table, table_repository::TableRepository,
};
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

    pub async fn create(&self, new_table_data: &mut CreateTableCommand) -> Result<Table> {
        let created_table = self.table_repository.create(new_table_data).await?;

        Ok(created_table)
    }

    pub async fn delete(&self, command: &DeleteTableCommand) -> Result<Table> {
        let table = self.find_by_id(&command.id).await?;

        if table.gm_id != command.gm_id {
            return Err(Error::Application(ApplicationError::InvalidCredentials));
        }

        self.table_repository.delete(command).await
    }

    pub async fn find_by_id(&self, table_id: &Uuid) -> Result<Table> {
        self.table_repository.find_by_id(table_id).await
    }

    pub async fn get(&self, command: &GetTableCommand) -> Result<Vec<Table>> {
        self.table_repository.get(command).await
    }

    pub async fn update(&self, update_data: &UpdateTableCommand) -> Result<Table> {
        let updated_table = self.table_repository.update(update_data).await?;

        Ok(updated_table)
    }
}
