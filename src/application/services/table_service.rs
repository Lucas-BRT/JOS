use crate::{
    domain::{
        table::{
            dtos::{NewTableData, TableSearchFilters, UpdateTableData},
            entity::Table,
            table_repository::TableRepository,
        },
        utils::pagination::Pagination,
    },
    prelude::AppResult,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct TableService {
    table_repository: Arc<dyn TableRepository>,
}

impl TableService {
    pub fn new(table_repository: Arc<dyn TableRepository>) -> Self {
        Self { table_repository }
    }

    pub async fn create_table(&self, new_table_data: &NewTableData) -> AppResult<String> {
        let created_table = self.table_repository.create(new_table_data).await?;

        Ok(created_table.to_string())
    }

    pub async fn find_table_by_id(&self, table_id: &Uuid) -> AppResult<Option<Table>> {
        Ok(self.table_repository.find_by_id(table_id).await?)
    }

    pub async fn get_avaliable(&self) -> AppResult<Vec<Table>> {
        let tables = self
            .table_repository
            .search_public_tables(&TableSearchFilters::default(), &Pagination::default())
            .await?;

        Ok(tables)
    }

    pub async fn update_table(
        &self,
        table_id: &Uuid,
        table_to_update: &UpdateTableData,
    ) -> AppResult<()> {
        self.table_repository
            .update(table_id, table_to_update)
            .await?;

        Ok(())
    }
}
