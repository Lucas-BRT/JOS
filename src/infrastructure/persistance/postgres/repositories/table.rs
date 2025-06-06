use crate::domain::table::dtos::{CreateTableCommand, TableSearchFilters, UpdateTableData};
use crate::domain::table::entity::Table;
use crate::domain::table::table_repository::TableRepository;
use crate::domain::utils::pagination::Pagination;
use crate::infrastructure::persistance::postgres::models::tables::TableRow;
use crate::{Db, Result};
use async_trait::async_trait;
use sqlx::query_scalar;
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
        let response = query_scalar!(
            r#"
                INSERT INTO tables (
                    gm_id,
                    title,
                    description,
                    game_system_id,
                    is_public,
                    player_slots,
                    occupied_slots,
                    bg_image_link
                )
                VALUES (
                    $1, $2, $3, $4, $5, $6, $7, $8
                )
                RETURNING title
            "#,
            table_data.gm_id,
            table_data.title,
            table_data.description,
            table_data.system_id,
            table_data.is_public,
            table_data.player_slots as i32,
            table_data.occupied_slots as i32,
            table_data.bg_image_link
        )
        .fetch_one(self.pool.as_ref())
        .await;

        let username = response.map_err(|e|   )?;

        Ok(response)
    }

    async fn update(&self, _table_id: &Uuid, _update_data: &UpdateTableData) -> Result<()> {
        todo!();
    }

    async fn delete(&self, _table_id: &Uuid) -> Result<()> {
        todo!();
    }

    async fn find_by_id(&self, _table_id: &Uuid) -> Result<Option<Table>> {
        todo!();
    }

    async fn find_by_gm_id(&self, _gm_id: &Uuid, _pagination: &Pagination) -> Result<Vec<Table>> {
        todo!();
    }

    async fn search_public_tables(
        &self,
        _filters: &TableSearchFilters,
        pagination: &Pagination,
    ) -> Result<Vec<Table>> {
        let result = sqlx::query_as!(
            TableRow,
            r#"
                SELECT * FROM tables
                WHERE is_public = TRUE
                ORDER BY created_at DESC
                LIMIT $1 OFFSET $2
            "#,
            pagination.limit() as i64,
            pagination.offset() as i64
        )
        .fetch_all(self.pool.as_ref())
        .await;

        let tables: Vec<Table> = result
            .into_iter()
            .filter_map(|row| Table::try_from(row).ok())
            .collect();

        Ok(tables)
    }
}
