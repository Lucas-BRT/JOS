use crate::persistence::Db;
use shared::{Error, error::InfrastructureError};

pub async fn create_postgres_pool(database_url: &str) -> Result<Db, Error> {
    let pool = Db::connect(database_url).await.map_err(|e| {
        Error::Infrastructure(InfrastructureError::FailedToEstablishDatabaseConnection(
            e.to_string(),
        ))
    })?;

    Ok(pool)
}
