use crate::persistence::postgres::constraint_mapper;
use crate::persistence::postgres::models::TableMemberModel;
use domain::repositories::TableMemberRepository;
use domain::{entities::*, repositories::Repository};
use shared::Result;
use sqlx::PgPool;
use uuid::Uuid;

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
impl
    Repository<
        TableMember,
        CreateTableMemberCommand,
        UpdateTableMemberCommand,
        GetTableMemberCommand,
        DeleteTableMemberCommand,
    > for PostgresTableMemberRepository
{
    async fn create(&self, command: CreateTableMemberCommand) -> Result<TableMember> {
        let member = sqlx::query_as!(
            TableMemberModel,
            r#"INSERT INTO table_members (id, table_id, user_id, created_at, updated_at)
                  VALUES ($1, $2, $3, NOW(), NOW())
                  RETURNING * "#,
            command.id,
            command.table_id,
            command.user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(member.into())
    }

    async fn read(&self, command: GetTableMemberCommand) -> Result<Vec<TableMember>> {
        let members = sqlx::query_as!(
            TableMemberModel,
            r#"SELECT * FROM table_members
                  WHERE ($1::uuid IS NULL OR id = $1)
                    AND ($2::uuid IS NULL OR table_id = $2)
                    AND ($3::uuid IS NULL OR user_id = $3)"#,
            command.id,
            command.table_id,
            command.user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(members.into_iter().map(|m| m.into()).collect())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<TableMember>> {
        let member = sqlx::query_as!(
            TableMemberModel,
            "SELECT * FROM table_members WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(member.map(|m| m.into()))
    }

    async fn update(&self, command: UpdateTableMemberCommand) -> Result<TableMember> {
        let member = sqlx::query_as!(
            TableMemberModel,
            r#"UPDATE table_members
                  SET table_id = COALESCE($2, table_id),
                      user_id = COALESCE($3, user_id),
                      updated_at = NOW()
                  WHERE id = $1
                  RETURNING *"#,
            command.id,
            command.table_id,
            command.user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(member.into())
    }

    async fn delete(&self, command: DeleteTableMemberCommand) -> Result<TableMember> {
        let deleted = sqlx::query_as!(
            TableMemberModel,
            "DELETE FROM table_members WHERE id = $1 RETURNING *",
            command.id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(deleted.into())
    }
}

#[async_trait::async_trait]
impl TableMemberRepository for PostgresTableMemberRepository {
    async fn find_by_table_id(&self, table_id: Uuid) -> Result<Vec<TableMember>> {
        let members = sqlx::query_as!(
            TableMemberModel,
            "SELECT * FROM table_members WHERE table_id = $1",
            table_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(members.into_iter().map(|m| m.into()).collect())
    }

    async fn find_by_table_and_user(
        &self,
        table_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<TableMember>> {
        let member = sqlx::query_as!(
            TableMemberModel,
            "SELECT * FROM table_members WHERE table_id = $1 AND user_id = $2",
            table_id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(member.map(|m| m.into()))
    }
}
