use super::{
    dtos::{TableSearchFilters, UpdateTableData},
    entity::Table,
};
use crate::domain::table::dtos::NewTableData;
use crate::domain::utils::pagination::Pagination;
use crate::prelude::AppResult;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TableRepository: Send + Sync {
    async fn create(&self, table_data: &NewTableData) -> AppResult<String>;

    async fn update(&self, table_id: &Uuid, update_data: &UpdateTableData) -> AppResult<()>;

    async fn delete(&self, table_id: &Uuid) -> AppResult<()>;

    async fn find_by_id(&self, table_id: &Uuid) -> AppResult<Option<Table>>;

    async fn find_by_gm_id(&self, gm_id: &Uuid, pagination: &Pagination) -> AppResult<Vec<Table>>;

    async fn search_public_tables(
        &self,
        filters: &TableSearchFilters,
        pagination: &Pagination,
    ) -> AppResult<Vec<Table>>;
}
