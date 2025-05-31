pub mod error;
pub mod table;
pub mod user;

use crate::core::state::AppState;
use axum::Router;

fn router(app_state: AppState) -> Router {
    Router::new()
        .nest("/users", user::routes::routes(&app_state))
        .nest("/tables", table::routes::routes(&app_state))
}

pub fn create_router(app_state: AppState) -> Router {
    Router::new().nest("/v1/", router(app_state))
}
