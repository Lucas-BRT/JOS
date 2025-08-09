use crate::Result;
use crate::infrastructure::repositories::{error::RepositoryError, constraint_mapper};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

#[derive(Clone)]
pub struct GameSystemRepository {
    pool: Arc<PgPool>,
}

impl GameSystemRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

// Exemplo de implementação usando o novo sistema de mapeamento
impl GameSystemRepository {
    pub async fn create_game_system(&self, name: &str) -> Result<Uuid> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO t_game_system (id, name, created_at, updated_at)
            VALUES ($1, $2, $3, $4)
            "#
        )
        .bind(id)
        .bind(name)
        .bind(now)
        .bind(now)
        .execute(self.pool.as_ref())
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(id)
    }

    pub async fn find_by_name(&self, name: &str) -> Result<Option<Uuid>> {
        let result = sqlx::query_scalar::<_, Uuid>(
            "SELECT id FROM t_game_system WHERE name = $1"
        )
        .bind(name)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(RepositoryError::DatabaseError)?;

        Ok(result)
    }
}
