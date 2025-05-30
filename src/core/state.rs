use crate::{application::services::user_service::UserService, infrastructure::config::Config};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pg_pool: Arc<PgPool>,
    pub config: Arc<Config>,
    pub user_service: UserService,
}

impl AppState {
    pub fn new(pg_pool: Arc<PgPool>, config: Arc<Config>, user_service: UserService) -> Self {
        Self {
            pg_pool,
            config,
            user_service,
        }
    }
}
