use axum::Router;
use sqlx::PgPool;
mod user_routes;

pub fn create_router(app_state: PgPool) -> Router {
    Router::new().nest("/users", user_routes::routes(app_state))
}
