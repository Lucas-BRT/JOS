use std::sync::Arc;

use sqlx::PgPool;

use crate::{core::error::ApplicationSetupError, prelude::AppResult};

pub async fn run_postgres_migrations(pool: Arc<PgPool>) -> AppResult<()> {
    sqlx::migrate!("./migrations")
        .run(pool.as_ref())
        .await
        .map_err(|err| ApplicationSetupError::FailedToRunDBMigrations(err.to_string()))?;

    Ok(())
}
