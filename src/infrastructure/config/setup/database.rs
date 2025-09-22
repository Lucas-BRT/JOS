use std::sync::Arc;
use tracing::{error, info};

use crate::{Error, Result, infrastructure::SetupError};
use sqlx::PgPool;

pub async fn health_check_database(pool: &Arc<PgPool>) -> Result<()> {
    let result = sqlx::query("SELECT 1").execute(pool.as_ref()).await;

    match result {
        Ok(_) => {
            info!("✅ Database health check passed");
            Ok(())
        }
        Err(e) => {
            error!("❌ Database health check failed: {}", e);
            Err(Error::Setup(SetupError::DatabaseHealthCheckFailed(
                e.to_string(),
            )))
        }
    }
}
