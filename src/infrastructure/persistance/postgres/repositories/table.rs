use crate::core::error::AppError;
use crate::domain::error::DomainError;
use crate::domain::table::error::TableDomainError;
use crate::domain::table::vo::{DescriptionVo, TitleVo};
use crate::domain::utils::type_wraper::TypeWrapped;
use crate::infrastructure::persistance::postgres::models::tables::TableRow;
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
use async_trait::async_trait;
use sqlx::{PgPool, query_scalar};
use std::num::TryFromIntError;
use std::sync::Arc;
use uuid::Uuid;

pub struct PostgresTableRepository {
    pool: Arc<PgPool>,
}

impl<'a> PostgresTableRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

impl TryFrom<TableRow> for Table {
    type Error = AppError;

    fn try_from(row: TableRow) -> Result<Self, Self::Error> {
        let title = TitleVo::parse(row.title.clone()).map_err(|e| AppError::Domain(e.into()))?;
        let description = DescriptionVo::parse(row.description.clone())
            .map_err(|e| AppError::Domain(e.into()))?;
        let game_system_id = row.game_system_id;
        let is_public = row.is_public;
        let player_slots = row.player_slots.try_into().map_err(|e: TryFromIntError| {
            AppError::Domain(DomainError::Table(TableDomainError::FailedToParseDbData(
                e.to_string(),
            )))
        })?;
        let occupied_slots = row
            .occupied_slots
            .try_into()
            .map_err(|e: TryFromIntError| {
                AppError::Domain(DomainError::Table(TableDomainError::FailedToParseDbData(
                    e.to_string(),
                )))
            })?;
        let bg_image_link = row.bg_image_link;

        Ok(Self {
            id: row.id,
            gm_id: row.gm_id,
            title,
            description,
            game_system_id,
            is_public,
            player_slots,
            occupied_slots,
            bg_image_link,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}

#[async_trait]
impl TableRepository for PostgresTableRepository {
    async fn create(&self, table_data: &NewTableData) -> AppResult<String> {
        let description = table_data
            .description
            .as_ref()
            .map(|description| description.raw());

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
            table_data.title.raw(),
            description,
            table_data.system_id,
            table_data.is_public,
            table_data.player_slots as i32,
            table_data.occupied_slots as i32,
            table_data.bg_image_link
        )
        .fetch_one(self.pool.as_ref())
        .await?;

        Ok(response)
    }

    async fn update(&self, _table_id: &Uuid, _update_data: &UpdateTableData) -> AppResult<()> {
        todo!();
    }

    async fn delete(&self, _table_id: &Uuid) -> AppResult<()> {
        todo!();
    }

    async fn find_by_id(&self, _table_id: &Uuid) -> AppResult<Option<Table>> {
        todo!();
    }

    async fn find_by_gm_id(
        &self,
        _gm_id: &Uuid,
        _pagination: &Pagination,
    ) -> AppResult<Vec<Table>> {
        todo!();
    }

    async fn search_public_tables(
        &self,
        _filters: &TableSearchFilters,
        pagination: &Pagination,
    ) -> AppResult<Vec<Table>> {
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
        .await?;

        let tables: Vec<Table> = result
            .into_iter()
            .filter_map(|row| Table::try_from(row).ok())
            .collect();

        Ok(tables)
    }
}
