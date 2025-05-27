use sqlx::PgPool;

use crate::{core::error::ApplicationSetupError, prelude::AppResult};

pub async fn run_postgres_migrations(pool: &PgPool) -> AppResult<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|err| ApplicationSetupError::FailedToRunDBMigrations(err.to_string()))?;

    Ok(())
}
