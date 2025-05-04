use crate::api::handlers::users;
use axum::{Router, routing::get};
use sqlx::PgPool;

pub fn routes(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(users::get_users::handle))
        .with_state(pool)
}
