use crate::Result;
use crate::domain::table_request::dtos::*;
use crate::domain::table_request::entity::TableRequest;
use crate::domain::table_request::table_request_repository::TableRequestRepository as TableRequestRepositoryTrait;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct TableRequestRepository {
    pool: Arc<PgPool>,
}

impl TableRequestRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TableRequestRepositoryTrait for TableRequestRepository {
    async fn create(&self, request_data: &CreateTableRequestCommand) -> Result<String> {
        todo!()
    }
    
    async fn update(&self, request_id: &Uuid, update_data: &UpdateTableRequestCommand) -> Result<()> {
        todo!()
    }
    
    async fn delete(&self, request_id: &Uuid) -> Result<()> {
        todo!()
    }
    
    async fn get(&self) -> Result<Vec<TableRequest>> {
        todo!()
    }
    
    async fn find_by_id(&self, request_id: &Uuid) -> Result<Option<TableRequest>> {
        todo!()
    }
    
    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Vec<TableRequest>> {
        todo!()
    }
    
    async fn find_by_table_id(&self, table_id: &Uuid) -> Result<Vec<TableRequest>> {
        todo!()
    }
    
    async fn find_pending_by_table_id(&self, table_id: &Uuid) -> Result<Vec<TableRequest>> {
        todo!()
    }
}
