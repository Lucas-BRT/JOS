use crate::Result;
use crate::domain::table::dtos::{CreateTableCommand, UpdateTableCommand};
use crate::domain::table::{entity::Table, table_repository::TableRepository};
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

    pub async fn create(&self, new_table_data: &CreateTableCommand) -> Result<String> {
        let created_table = self.table_repository.create(new_table_data).await?;

        Ok(created_table.to_string())
    }

    pub async fn find_by_id(&self, table_id: &Uuid) -> Result<Option<Table>> {
        self.table_repository.find_by_id(table_id).await
    }

    pub async fn get(&self) -> Result<Vec<Table>> {
        self.table_repository.get().await
    }

    pub async fn update(
        &self,
        table_id: &Uuid,
        table_to_update: &UpdateTableCommand,
    ) -> Result<()> {
        self.table_repository
            .update(table_id, table_to_update)
            .await?;

        Ok(())
    }
}
