use crate::persistence::Db;
use crate::persistence::postgres::create_postgres_pool;
use shared::Result;
use shared::error::Error;
use shared::error::SetupError;
use tracing::*;

pub async fn setup_database(database_url: &str) -> Result<Db> {
    let database = connect_to_database(database_url).await?;
    run_postgres_migrations(&database).await?;
    health_check_database(&database).await?;
    Ok(database)
}

async fn connect_to_database(database_url: &str) -> Result<Db> {
    info!("🔌 Establishing database connection...");
    let pool = create_postgres_pool(database_url).await?;
    info!("✅ Database connection established");

    Ok(pool)
}

async fn health_check_database(database: &Db) -> Result<()> {
    let result = sqlx::query("SELECT 1").execute(database).await;

    if result.is_err() {
        error!("❌ Database health check failed");
        return Err(Error::Setup(SetupError::DatabaseHealthCheckFailed(
            result.err().unwrap().to_string(),
        )));
    }

    info!("✅ Database health check passed");
    Ok(())
}

async fn run_postgres_migrations(database: &Db) -> Result<()> {
    info!("🔄 Running database migrations...");

    sqlx::migrate!("./migrations")
        .run(database)
        .await
        .map_err(|err| Error::Setup(SetupError::FailedToRunDBMigrations(err.to_string())))?;

    info!("✅ Database migrations completed");

    Ok(())
}
