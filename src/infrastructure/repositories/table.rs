use crate::infrastructure::entities::enums::ETableVisibility;
use crate::infrastructure::prelude::RepositoryError;
use crate::infrastructure::repositories::constraint_mapper;
use crate::Result;
use crate::domain::table::commands::{CreateTableCommand, DeleteTableCommand, UpdateTableCommand};
use crate::domain::table::entity::Table;
use crate::domain::table::search_filters::TableFilters;
use crate::domain::table::table_repository::TableRepository as TableRepositoryTrait;
use crate::domain::utils::pagination::Pagination;
use crate::infrastructure::entities::t_rpg_tables::Model as TableModel;
use chrono::Utc;
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
    async fn create(&self, command: &CreateTableCommand) -> Result<Table> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let visibility: ETableVisibility = command.visibility.into();

        let created_table = sqlx::query_as!(
            TableModel,
            r#"INSERT INTO t_rpg_tables 
                (id, 
                gm_id, 
                title, 
                visibility, 
                description, 
                game_system_id, 
                player_slots, 
                created_at, 
                updated_at)
            VALUES 
                ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING
                id,
                gm_id,
                title,
                visibility as "visibility: ETableVisibility",
                description,
                game_system_id,
                player_slots,
                created_at,
                updated_at
            "#,
            id,
            command.gm_id,
            command.title,
            visibility as _,
            command.description,
            command.game_system_id,
            command.player_slots as i32,
            now,
            now
        )
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(created_table.into())
    }

    async fn update(&self, command: &UpdateTableCommand) -> Result<Table> {
        todo!()
    }

    async fn delete(&self, command: &DeleteTableCommand) -> Result<Table> {
        let table = sqlx::query_as!(
            TableModel,
            r#"DELETE FROM t_rpg_tables 
                WHERE id = $1
                RETURNING
                    id,
                    gm_id,
                    title,
                    visibility as "visibility: ETableVisibility",
                    description,
                    game_system_id,
                    player_slots,
                    created_at,
                    updated_at
            "#,
            command.id
        )
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(constraint_mapper::map_database_error)?;

        match table {
            Some(table) => {
                Ok(table.into())
            }
            None => {
                return Err(RepositoryError::TableNotFound.into());
            }
        }
    }

    async fn get(&self, filters: &TableFilters, pagination: Pagination) -> Result<Vec<Table>> {
        todo!()
    }

    async fn find_by_id(&self, table_id: &Uuid) -> Result<Table> {
        todo!()
    }

    async fn find_by_gm_id(&self, gm_id: &Uuid) -> Result<Vec<Table>> {
        todo!()
    }
}
