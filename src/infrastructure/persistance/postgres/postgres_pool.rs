use std::sync::Arc;

use crate::{core::error::ApplicationSetupError, prelude::AppResult};
use sqlx::PgPool;

pub async fn create_postgres_pool<T: AsRef<str>>(database_url: T) -> AppResult<Arc<PgPool>> {
    let pool = PgPool::connect(database_url.as_ref())
        .await
        .map_err(|e| ApplicationSetupError::FailedToEstablishDatabaseConnection(e.to_string()))?;

    Ok(Arc::new(pool))
}
