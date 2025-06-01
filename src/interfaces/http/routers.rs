use crate::core::state::AppState;
use axum::Router;

fn router(app_state: AppState) -> Router {
    Router::new()
        .nest("/users", super::user::routes::routes(&app_state))
        .nest("/tables", super::table::routes::routes(&app_state))
        .nest("/login", super::login::routes::routes(&app_state))
}

pub fn create_router(app_state: AppState) -> Router {
    Router::new().nest("/v1/", router(app_state))
}
