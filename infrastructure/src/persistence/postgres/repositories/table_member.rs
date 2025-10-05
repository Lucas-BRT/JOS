use crate::persistence::postgres::models::TableMemberModel;
use crate::persistence::postgres::{RepositoryError, constraint_mapper};
use domain::entities::*;
use domain::repositories::TableMemberRepository;
use shared::Result;
use sqlx::PgPool;
use uuid::{NoContext, Uuid};

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
        let uuid = Uuid::new_v7(uuid::Timestamp::now(NoContext));

        let created_table_member = sqlx::query_as!(
            TableMemberModel,
            r#"INSERT INTO table_members
                (
                id,
                table_id,
                user_id,
                created_at,
                updated_at)
            VALUES
                ($1, $2, $3, NOW(), NOW())
            RETURNING
                *
            "#,
            uuid,
            &command.table_id,
            &command.user_id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(created_table_member.into())
    }

    async fn read(&self, command: GetTableMemberCommand) -> Result<Vec<TableMember>> {
        let table_members = sqlx::query_as!(
            TableMemberModel,
            r#"
            SELECT
                *
            FROM table_members
            WHERE ($1::uuid IS NULL OR id = $1)
              AND ($2::uuid IS NULL OR table_id = $2)
              AND ($3::uuid IS NULL OR user_id = $3)
            "#,
            command.id,
            command.table_id,
            command.user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;

        Ok(table_members
            .into_iter()
            .map(|model| model.into())
            .collect())
    }

    async fn update(&self, command: UpdateTableMemberCommand) -> Result<TableMember> {
        let has_table_id_update = matches!(command.table_id, Update::Change(_));
        let has_user_id_update = matches!(command.user_id, Update::Change(_));

        if !has_table_id_update && !has_user_id_update {
            return Err(shared::error::Error::Persistence(
                shared::error::PersistenceError::DatabaseError("Row not found".to_string()),
            ));
        }

        let table_id_value = match &command.table_id {
            Update::Change(table_id) => Some(*table_id),
            Update::Keep => None,
        };

        let user_id_value = match &command.user_id {
            Update::Change(user_id) => Some(*user_id),
            Update::Keep => None,
        };

        let updated_table_member = sqlx::query_as!(
            TableMemberModel,
            r#"
            UPDATE table_members 
            SET 
                table_id = COALESCE($2, table_id),
                user_id = COALESCE($3, user_id),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
            command.id,
            table_id_value,
            user_id_value
        )
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

    async fn find_by_id(&self, id: Uuid) -> Result<Option<TableMember>> {
        let table_member = sqlx::query_as!(
            TableMemberModel,
            r#"
            SELECT
                *
            FROM table_members
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;

        Ok(table_member.map(|model| model.into()))
    }

    async fn find_by_table_id(&self, table_id: Uuid) -> Result<Vec<TableMember>> {
        let table_members = sqlx::query_as!(
            TableMemberModel,
            r#"
            SELECT
                *
            FROM table_members
            WHERE table_id = $1
            "#,
            table_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;

        Ok(table_members
            .into_iter()
            .map(|model| model.into())
            .collect())
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<TableMember>> {
        let table_members = sqlx::query_as!(
            TableMemberModel,
            r#"
            SELECT
                *
            FROM table_members
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;

        Ok(table_members
            .into_iter()
            .map(|model| model.into())
            .collect())
    }
}
