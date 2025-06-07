use crate::domain::table::dtos::{CreateTableCommand, TableFilters, UpdateTableCommand};
use crate::domain::table::entity::Table;
use crate::domain::table::table_repository::TableRepository;
use crate::domain::utils::pagination::Pagination;
use crate::{Db, Result};
use async_trait::async_trait;
use uuid::Uuid;

pub struct PostgresTableRepository {
    pool: Db,
}

impl<'a> PostgresTableRepository {
    pub fn new(pool: Db) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TableRepository for PostgresTableRepository {
    async fn create(&self, table_data: &CreateTableCommand) -> Result<String> {
        todo!()
    }

    async fn update(&self, table_id: &Uuid, update_data: &UpdateTableCommand) -> Result<()> {
        todo!()
    }

    async fn delete(&self, table_id: &Uuid) -> Result<()> {
        todo!()
    }

    async fn get(&self, options: Option<TableFilters>) -> Result<Vec<Table>> {
        todo!()
    }

    async fn find_by_id(&self, table_id: &Uuid) -> Result<Option<Table>> {
        todo!()
    }

    async fn find_by_gm_id(&self, gm_id: &Uuid, pagination: &Pagination) -> Result<Vec<Table>> {
        todo!()
    }
}
