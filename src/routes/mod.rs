use axum::Router;
use sqlx::PgPool;

pub mod users;

pub fn create_router(app_state: PgPool) -> Router {
    Router::new().nest("/users", users::routes(app_state))
}
