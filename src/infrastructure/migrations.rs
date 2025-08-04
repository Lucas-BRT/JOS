use crate::{Result, setup::SetupError};
use sqlx::PgPool;
use std::sync::Arc;

pub async fn run_postgres_migrations(pool: Arc<PgPool>) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool.as_ref())
        .await
        .map_err(|err| SetupError::FailedToRunDBMigrations(err.to_string()))?;

    Ok(())
}
