use sqlx::PgPool;

pub async fn create_postgres_pool<T: AsRef<str>>(database_url: T) -> Result<PgPool, String> {
    Ok(PgPool::connect(database_url.as_ref())
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?)
}
