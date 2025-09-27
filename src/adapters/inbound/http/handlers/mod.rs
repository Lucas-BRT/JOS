use axum::{
    Router,
    routing::get,
};
use std::sync::Arc;

use crate::{
    infrastructure::state::AppState,
    adapters::inbound::http::{
        middleware::{cors, tracing},
    },
};

pub mod auth;
pub mod docs;
pub mod health;
pub mod search;
pub mod session;
pub mod table;
pub mod table_request;
pub mod user;

pub fn create_router(app_state: Arc<AppState>) -> Router {
        Router::new()
            .route("/health", get(health::health_check))
            .nest(
                "/v1",
                Router::new()
                    .nest("/auth", auth::routes(app_state.clone()))
                    .nest("/tables", table::routes(app_state.clone()))
                    .nest("/sessions", session::routes(app_state.clone()))
                    .nest("/requests", table_request::routes(app_state.clone()))
                    .nest("/user", user::routes(app_state.clone()))
                    .nest("/search", search::routes(app_state.clone())),
            )
                .layer(cors::cors_layer())
                .layer(axum::middleware::from_fn(tracing::trace_middleware))
}
