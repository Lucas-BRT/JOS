use crate::persistence::Db;
use shared::Result;
use shared::error::Error;
use shared::error::SetupError;

pub async fn create_postgres_pool(database_url: &str) -> Result<Db> {
    let pool = Db::connect(database_url).await.map_err(|e| {
        Error::Setup(SetupError::FailedToEstablishDatabaseConnection(
            e.to_string(),
        ))
    })?;

    Ok(pool)
}
