use crate::{Db, Result, core::error::ApplicationSetupError};
use sqlx::PgPool;
use std::sync::Arc;

pub async fn create_postgres_pool<T: AsRef<str>>(database_url: T) -> Result<Db> {
    let pool = PgPool::connect(database_url.as_ref())
        .await
        .map_err(|e| ApplicationSetupError::FailedToEstablishDatabaseConnection(e.to_string()))?;

    Ok(Arc::new(pool))
}
