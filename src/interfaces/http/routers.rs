use crate::core::state::AppState;
use axum::{Router, Json, routing::get};
use std::sync::Arc;
use serde_json::json;

const HEALTH_ROUTE: &str = "/health";
const API_V1_PREFIX: &str = "/v1/";

/// Health check endpoint
async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "service": "JOS API",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

fn router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/auth", super::auth::routes::routes(app_state.clone()))
        .nest("/users", super::user::routes::routes(app_state.clone()))
        .nest("/tables", super::table::routes::routes(app_state.clone()))
        .nest("/table-requests", super::table_request::routes::routes(app_state.clone()))
}

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(HEALTH_ROUTE, get(health_check))
        .nest(API_V1_PREFIX, router(app_state.clone()))
        .merge(super::openapi::routes::routes())
}
