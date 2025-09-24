use crate::Result;
use crate::adapters::outbound::postgres::constraint_mapper;
use crate::adapters::outbound::postgres::models::TableModel;
use crate::domain::entities::commands::*;
use crate::domain::entities::*;
use crate::domain::repositories::TableRepository;
use sqlx::PgPool;

pub struct PostgresTableRepository {
    pool: PgPool,
}

impl PostgresTableRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TableRepository for PostgresTableRepository {
    async fn create(&self, command: CreateTableCommand) -> Result<Table> {
        let created_table = sqlx::query_as!(
            TableModel,
            r#"INSERT INTO tables
                (
                gm_id,
                title,
                description,
                slots,
                game_system_id)
            VALUES
                ($1, $2, $3, $4, $5)
            RETURNING
                id,
                gm_id,
                title,
                description,
                slots,
                game_system_id,
                created_at,
                updated_at
            "#,
            command.gm_id,
            command.title,
            command.description,
            command.slots as i32,
            command.game_system_id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(created_table.into())
    }

    async fn update(&self, command: UpdateTableCommand) -> Result<Table> {
        let mut builder = sqlx::QueryBuilder::new("UPDATE tables SET ");
        let mut separated = builder.separated(", ");

        if let Update::Change(title) = &command.title {
            separated.push("title = ");
            separated.push_bind_unseparated(title);
        }

        if let Update::Change(description) = &command.description {
            separated.push("description = ");
            separated.push_bind_unseparated(description);
        }

        if let Update::Change(slots) = &command.slots {
            separated.push("slots = ");
            separated.push_bind_unseparated(*slots as i32);
        }

        if let Update::Change(game_system_id) = &command.game_system_id {
            separated.push("game_system_id = ");
            separated.push_bind_unseparated(game_system_id);
        }

        builder.push(" WHERE id = ");
        builder.push_bind(command.id);

        builder.push(
            r#" RETURNING
                id,
                gm_id,
                title,
                description,
                slots,
                game_system_id,
                created_at,
                updated_at"#,
        );

        let updated_table = builder
            .build_query_as::<TableModel>()
            .fetch_one(&self.pool)
            .await
            .map_err(constraint_mapper::map_database_error)?;

        Ok(updated_table.into())
    }

    async fn delete(&self, command: DeleteTableCommand) -> Result<Table> {
        let table = sqlx::query_as!(
            TableModel,
            r#"DELETE FROM tables
                WHERE id = $1
                RETURNING
                    id,
                    gm_id,
                    title,
                    description,
                    slots,
                    game_system_id,
                    created_at,
                    updated_at
            "#,
            command.id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(table.into())
    }

    async fn read(&self, command: GetTableCommand) -> Result<Vec<Table>> {
        let mut builder = sqlx::QueryBuilder::new(
            r#"SELECT
                id,
                gm_id,
                title,
                description,
                slots,
                game_system_id,
                created_at,
                updated_at
            FROM tables"#,
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

        if let Some(id) = &command.id {
            push_filter_separator(&mut builder);
            builder.push("id = ");
            builder.push_bind(id);
        }

        if let Some(gm_id) = &command.gm_id {
            push_filter_separator(&mut builder);
            builder.push("gm_id = ");
            builder.push_bind(gm_id);
        }

        if let Some(title) = &command.title {
            push_filter_separator(&mut builder);
            builder.push("title = ");
            builder.push_bind(title);
        }

        if let Some(game_system_id) = &command.game_system_id {
            push_filter_separator(&mut builder);
            builder.push("game_system_id = ");
            builder.push_bind(game_system_id);
        }

        if let Some(slots) = &command.slots {
            push_filter_separator(&mut builder);
            builder.push("slots = ");
            builder.push_bind(*slots as i32);
        }

        let tables = builder
            .build_query_as::<TableModel>()
            .fetch_all(&self.pool)
            .await
            .map_err(constraint_mapper::map_database_error)?;

        Ok(tables.into_iter().map(|m| m.into()).collect())
    }

    async fn search(&self, query: &str) -> Result<Vec<Table>> {
        let search_pattern = format!("%{}%", query);
        let tables = sqlx::query_as!(
            TableModel,
            r#"SELECT *
                FROM tables
                WHERE title ILIKE $1 OR description ILIKE $1
                LIMIT 10
            "#,
            search_pattern
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(tables.into_iter().map(|model| model.into()).collect())
    }
}
