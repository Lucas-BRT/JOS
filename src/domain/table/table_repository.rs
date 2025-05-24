use crate::domain::table::{
    new_table::NewTableData, rearch_result::TableSearchResult, search_filters::TableSearchFilters,
    table::Table, update::UpdateTableData,
};
use crate::domain::utils::pagination::Pagination;
use crate::prelude::AppResult;
use uuid::Uuid;

use super::table::TableAggregate;

pub trait TableRepository {
    async fn create(&self, table_data: NewTableData, genre_ids: &[i32]) -> AppResult<Table>;

    async fn update(
        &self,
        table_id: Uuid,
        update_data: UpdateTableData,
        genre_ids_to_set: Option<&[i32]>,
    ) -> AppResult<Table>;

    async fn delete(&self, table_id: Uuid) -> AppResult<()>;

    async fn find_by_id(&self, table_id: Uuid) -> AppResult<Option<TableAggregate>>;

    async fn find_by_gm_id(&self, gm_id: Uuid, pagination: Pagination) -> AppResult<Vec<Table>>;

    async fn search_public_tables(
        &self,
        filters: TableSearchFilters,
        pagination: Pagination,
    ) -> AppResult<Vec<TableSearchResult>>;
}
