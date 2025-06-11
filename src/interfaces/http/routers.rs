use crate::core::state::AppState;
use axum::Router;
use std::sync::Arc;

fn router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/auth", super::auth::routes::routes(app_state.clone()))
        .nest("/users", super::user::routes::routes(app_state.clone()))
        .nest("/tables", super::table::routes::routes(app_state.clone()))
}

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new().nest("/v1/", router(app_state.clone()))
}
