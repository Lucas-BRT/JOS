use crate::core::state::AppState;
use crate::interfaces::http::auth::routes::routes as auth_routes;
use crate::interfaces::http::health::health_check;
use crate::interfaces::http::openapi::OpenApiRoutes;
use crate::interfaces::http::table::routes::routes as table_routes;
use crate::interfaces::http::table_request::routes::routes as table_request_routes;
use crate::interfaces::http::user::routes::routes as user_routes;
use crate::adapters::inbound::http::handlers::session::routes::routes as session_routes;
use crate::adapters::inbound::http::handlers::search::routes::routes as search_routes;
use axum::{Router, routing::get};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

fn router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/auth", auth_routes(app_state.clone()))
        .nest("/users", user_routes(app_state.clone()))
        .nest("/tables", table_routes(app_state.clone()))
        .nest("/sessions", session_routes(app_state.clone()))
        .nest("/requests", table_request_routes(app_state.clone()))
        .nest("/search", search_routes(app_state.clone()))
}

pub fn create_router(app_state: Arc<AppState>) -> Router {
    // TODO: Update later to be more specific (only allow requests from the frontend)
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health", get(health_check))
        .nest("/v1", router(app_state.clone()))
        .merge(OpenApiRoutes())
        .layer(cors)
}
