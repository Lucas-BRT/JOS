use crate::domain::table::dtos::{CreateTableCommand, TableFilters, UpdateTableCommand};
use crate::domain::table::entity::Table;
use crate::domain::table::table_repository::TableRepository;
use crate::domain::utils::pagination::Pagination;
use crate::{Db, Result};
use async_trait::async_trait;
use uuid::Uuid;

#[allow(unused)]
pub struct PostgresTableRepository {
    pool: Db,
}

impl PostgresTableRepository {
    pub fn new(pool: Db) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TableRepository for PostgresTableRepository {
    async fn create(&self, _table_data: &CreateTableCommand) -> Result<String> {
        todo!()
    }

    async fn update(&self, _table_id: &Uuid, _update_data: &UpdateTableCommand) -> Result<()> {
        todo!()
    }

    async fn delete(&self, _table_id: &Uuid) -> Result<()> {
        todo!()
    }

    async fn get(&self, _options: Option<TableFilters>) -> Result<Vec<Table>> {
        todo!()
    }

    async fn find_by_id(&self, _table_id: &Uuid) -> Result<Option<Table>> {
        todo!()
    }

    async fn find_by_gm_id(&self, _gm_id: &Uuid, _pagination: &Pagination) -> Result<Vec<Table>> {
        todo!()
    }
}
