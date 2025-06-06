use crate::{Result, core::error::ApplicationSetupError};
use sqlx::PgPool;
use std::sync::Arc;

pub async fn run_postgres_migrations(pool: Arc<PgPool>) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool.as_ref())
        .await
        .map_err(|err| ApplicationSetupError::FailedToRunDBMigrations(err.to_string()))?;

    Ok(())
}
