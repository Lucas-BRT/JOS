use crate::{Db, Error, Result, infrastructure::SetupError};
use tracing::{error, info};

pub async fn health_check_database(pool: &Db) -> Result<()> {
    let result = sqlx::query("SELECT 1").execute(pool).await;

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
