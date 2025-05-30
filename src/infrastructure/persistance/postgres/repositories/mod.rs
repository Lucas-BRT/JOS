pub mod user;
use std::sync::Arc;

use sqlx::PgPool;

pub struct PostgresUserRepository {
    pool: Arc<PgPool>,
}

impl<'a> PostgresUserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}
