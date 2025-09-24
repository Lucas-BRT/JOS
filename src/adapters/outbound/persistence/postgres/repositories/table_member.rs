use crate::Result;
use crate::adapters::outbound::postgres::models::TableMemberModel;
use crate::adapters::outbound::postgres::{RepositoryError, constraint_mapper};
use crate::domain::entities::*;
use crate::domain::repositories::TableMemberRepository;
use sqlx::PgPool;

#[derive(Clone)]
pub struct PostgresTableMemberRepository {
    pool: PgPool,
}

impl PostgresTableMemberRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TableMemberRepository for PostgresTableMemberRepository {
    async fn create(&self, command: CreateTableMemberCommand) -> Result<TableMember> {
        let created_table_member = sqlx::query_as!(
            TableMemberModel,
            r#"INSERT INTO table_members
                (
                table_id,
                user_id)
            VALUES
                ($1, $2)
            RETURNING
                *
            "#,
            &command.table_id,
            &command.user_id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(created_table_member.into())
    }

    async fn read(&self, command: GetTableMemberCommand) -> Result<Vec<TableMember>> {
        let mut query = sqlx::QueryBuilder::new(
            r#"SELECT
                id,
                table_id,
                user_id,
                joined_at,
                created_at,
                updated_at
            FROM table_members
            "#,
        );

        let mut conditions = Vec::new();

        if let Some(id) = &command.id {
            conditions.push("id = ");
            query.push_bind(id);
        }

        if let Some(table_id) = &command.table_id {
            conditions.push("table_id = ");
            query.push_bind(table_id);
        }

        if let Some(user_id) = &command.user_id {
            conditions.push("user_id = ");
            query.push_bind(user_id);
        }

        if !conditions.is_empty() {
            query.push(" WHERE ");
            for (i, condition) in conditions.iter().enumerate() {
                if i > 0 {
                    query.push(" AND ");
                }
                query.push(condition);
            }
        }

        let table_members = query
            .build_query_as::<TableMemberModel>()
            .fetch_all(&self.pool)
            .await
            .map_err(RepositoryError::DatabaseError)?;

        Ok(table_members
            .into_iter()
            .map(|model| model.into())
            .collect())
    }

    async fn update(&self, command: UpdateTableMemberCommand) -> Result<TableMember> {
        let mut builder = sqlx::QueryBuilder::new("UPDATE table_members SET ");
        let mut separated = builder.separated(", ");

        if let Update::Change(table_id) = &command.table_id {
            separated.push("table_id = ");
            separated.push_bind_unseparated(table_id);
        }

        if let Update::Change(user_id) = &command.user_id {
            separated.push("user_id = ");
            separated.push_bind_unseparated(user_id);
        }

        builder.push(" WHERE id = ");
        builder.push_bind(command.id);

        builder.push(" RETURNING *");

        let updated_table_member = builder
            .build_query_as::<TableMemberModel>()
            .fetch_one(&self.pool)
            .await
            .map_err(constraint_mapper::map_database_error)?;

        Ok(updated_table_member.into())
    }

    async fn delete(&self, command: DeleteTableMemberCommand) -> Result<TableMember> {
        let table_member = sqlx::query_as!(
            TableMemberModel,
            r#"DELETE FROM table_members
                WHERE id = $1
                RETURNING
                    *
            "#,
            &command.id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;

        Ok(table_member.into())
    }
}
