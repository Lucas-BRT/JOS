use axum::Router;
use sqlx::PgPool;

mod users_router;

pub fn create_router(app_state: PgPool) -> Router {
    Router::new().nest("/users", users_router::routes(app_state))
}
