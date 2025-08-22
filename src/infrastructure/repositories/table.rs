use crate::Result;
use crate::domain::table::commands::{
    CreateTableCommand, DeleteTableCommand, GetTableCommand, UpdateTableCommand,
};
use crate::domain::table::entity::Table;
use crate::domain::table::table_repository::TableRepository as TableRepositoryTrait;
use crate::domain::utils::update::Update;
use crate::infrastructure::entities::enums::ETableVisibility;
use crate::infrastructure::entities::t_rpg_tables::Model as TableModel;
use crate::infrastructure::prelude::RepositoryError;
use crate::infrastructure::repositories::constraint_mapper;
use chrono::Utc;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct PostgresTableRepository {
    pool: Arc<PgPool>,
}

impl PostgresTableRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TableRepositoryTrait for PostgresTableRepository {
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
        let now = Utc::now();

        let mut builder = sqlx::QueryBuilder::new("UPDATE t_rpg_tables SET ");

        let mut separated = builder.separated(", ");

        if let Update::Change(title) = &command.title {
            separated.push("title = ");
            separated.push_bind_unseparated(title);
        }

        if let Update::Change(description) = &command.description {
            separated.push("description = ");
            separated.push_bind_unseparated(description);
        }

        if let Update::Change(visibility) = &command.visibility {
            separated.push("visibility = ");
            separated.push_bind_unseparated(ETableVisibility::from(*visibility));
        }

        if let Update::Change(player_slots) = &command.player_slots {
            separated.push("player_slots = ");
            separated.push_bind_unseparated(*player_slots as i32);
        }

        if let Update::Change(game_system_id) = &command.game_system_id {
            separated.push("game_system_id = ");
            separated.push_bind_unseparated(game_system_id);
        }

        separated.push("updated_at = ");
        separated.push_bind_unseparated(now);

        builder.push(" WHERE id = ");
        builder.push_bind(command.id);

        builder.push(
            r#" RETURNING
                id,
                gm_id,
                title,
                visibility,
                description,
                game_system_id,
                player_slots,
                created_at,
                updated_at"#,
        );

        let updated_table = builder
            .build_query_as::<TableModel>()
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(constraint_mapper::map_database_error)?;

        Ok(updated_table.into())
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
            Some(table) => Ok(table.into()),
            None => {
                return Err(RepositoryError::TableNotFound.into());
            }
        }
    }

    async fn get(&self, command: &GetTableCommand) -> Result<Vec<Table>> {
        let mut builder = sqlx::QueryBuilder::new(
            r#"SELECT
                id,
                gm_id,
                title,
                visibility,
                description,
                game_system_id,
                player_slots,
                created_at,
                updated_at
            FROM t_rpg_tables"#,
        );

        let mut has_where = false;
        let mut push_filter_separator = |b: &mut sqlx::QueryBuilder<'_, sqlx::Postgres>| {
            if !has_where {
                b.push(" WHERE ");
                has_where = true;
            } else {
                b.push(" AND ");
            }
        };

        if let Some(id) = &command.filters.id {
            push_filter_separator(&mut builder);
            builder.push("id = ");
            builder.push_bind(id);
        }

        if let Some(gm_id) = &command.filters.gm_id {
            push_filter_separator(&mut builder);
            builder.push("gm_id = ");
            builder.push_bind(gm_id);
        }

        if let Some(title) = &command.filters.title {
            push_filter_separator(&mut builder);
            builder.push("title = ");
            builder.push_bind(title);
        }

        if let Some(visibility) = &command.filters.visibility {
            push_filter_separator(&mut builder);
            builder.push("visibility = ");
            builder.push_bind(ETableVisibility::from(*visibility));
        }

        if let Some(description) = &command.filters.description {
            push_filter_separator(&mut builder);
            builder.push("description = ");
            builder.push_bind(description);
        }

        if let Some(game_system_id) = &command.filters.game_system_id {
            push_filter_separator(&mut builder);
            builder.push("game_system_id = ");
            builder.push_bind(game_system_id);
        }

        if let Some(player_slots) = &command.filters.player_slots {
            push_filter_separator(&mut builder);
            builder.push("player_slots = ");
            builder.push_bind(*player_slots as i32);
        }

        if let Some(created_at) = &command.filters.created_at {
            push_filter_separator(&mut builder);
            builder.push("created_at = ");
            builder.push_bind(created_at);
        }

        if let Some(updated_at) = &command.filters.updated_at {
            push_filter_separator(&mut builder);
            builder.push("updated_at = ");
            builder.push_bind(updated_at);
        }

        let page = command.pagination.limit();
        let offset = command.pagination.offset();

        builder.push(" LIMIT ");
        builder.push_bind(page as i64);

        builder.push(" OFFSET ");
        builder.push_bind(offset as i64);

        let tables = builder
            .build_query_as::<TableModel>()
            .fetch_all(self.pool.as_ref())
            .await
            .map_err(constraint_mapper::map_database_error)?;

        Ok(tables.into_iter().map(|m| m.into()).collect())
    }

    async fn find_by_id(&self, table_id: &Uuid) -> Result<Table> {
        let table = sqlx::query_as!(
            TableModel,
            r#"SELECT
                id,
                gm_id,
                title,
                visibility as "visibility: ETableVisibility",
                description,
                game_system_id,
                player_slots,
                created_at,
                updated_at
            FROM t_rpg_tables
            WHERE id = $1"#,
            table_id
        )
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(constraint_mapper::map_database_error)?;

        match table {
            Some(table) => Ok(table.into()),
            None => Err(RepositoryError::TableNotFound.into()),
        }
    }

    async fn find_by_gm_id(&self, gm_id: &Uuid) -> Result<Vec<Table>> {
        let tables = sqlx::query_as!(
            TableModel,
            r#"SELECT
                id,
                gm_id,
                title,
                visibility as "visibility: ETableVisibility",
                description,
                game_system_id,
                player_slots,
                created_at,
                updated_at
            FROM t_rpg_tables
            WHERE gm_id = $1"#,
            gm_id
        )
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(tables.into_iter().map(|m| m.into()).collect())
    }
}
