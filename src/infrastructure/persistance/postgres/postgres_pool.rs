use crate::{core::error::ApplicationSetupError, prelude::AppResult};
use sqlx::PgPool;

pub async fn create_postgres_pool<T: AsRef<str>>(database_url: T) -> AppResult<PgPool> {
    Ok(PgPool::connect(database_url.as_ref())
        .await
        .map_err(|e| ApplicationSetupError::FailedToEstablishDatabaseConnection(e.to_string()))?)
}
