use crate::{Db, Error, Result, infrastructure::SetupError};

pub async fn run_postgres_migrations(pool: Db) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|err| Error::Setup(SetupError::FailedToRunDBMigrations(err.to_string())))?;

    Ok(())
}
