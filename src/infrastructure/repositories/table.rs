use crate::Result;
use crate::domain::table::dtos::*;
use crate::domain::table::entity::Table;
use crate::domain::table::table_repository::TableRepository as TableRepositoryTrait;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct TableRepository {
    pool: Arc<PgPool>,
}

impl TableRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TableRepositoryTrait for TableRepository {
    async fn create(&self, table_data: &CreateTableCommand) -> Result<String> {
        todo!()
    }
    async fn update(&self, table_id: &Uuid, update_data: &UpdateTableCommand) -> Result<()> {
        todo!()
    }
    async fn get(&self) -> Result<Vec<Table>> {
        todo!()
    }
    async fn delete(&self, table_id: &Uuid) -> Result<()> {
        todo!()
    }
    async fn find_by_id(&self, table_id: &Uuid) -> Result<Option<Table>> {
        todo!()
    }
    async fn find_by_gm_id(&self, gm_id: &Uuid) -> Result<Vec<Table>> {
        todo!()
    }
}
