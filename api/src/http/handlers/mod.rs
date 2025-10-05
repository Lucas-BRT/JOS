use crate::{
    http::{
        handlers::{
            auth::auth_routes, health::health_check, search::search_routes,
            session::session_routes, table::table_routes, table_request::table_request_routes,
            user::user_routes,
        },
        middleware::{cors, tracing},
        open_api::ApiDoc,
    },
};
use infrastructure::state::AppState;
use axum::{Router, routing::get};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod auth;
pub mod docs;
pub mod health;
pub mod search;
pub mod session;
pub mod table;
pub mod table_request;
pub mod user;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let openapi_spec = ApiDoc::openapi();

    let v1_api_routes = build_v1_api_routes(&app_state);
    let system_routes = build_system_routes(openapi_spec);

    Router::new()
        .merge(system_routes)
        .nest("/v1", v1_api_routes)
        .layer(cors::cors_layer())
        .layer(axum::middleware::from_fn(tracing::trace_middleware))
}

fn build_v1_api_routes(app_state: &Arc<AppState>) -> Router {
    Router::new()
        .merge(auth_routes(app_state.clone()))
        .merge(table_routes(app_state.clone()))
        .merge(session_routes(app_state.clone()))
        .merge(table_request_routes(app_state.clone()))
        .merge(user_routes(app_state.clone()))
        .merge(search_routes(app_state.clone()))
}

fn build_system_routes(openapi_spec: utoipa::openapi::OpenApi) -> Router {
    let swagger_ui = SwaggerUi::new("/docs").url("/api-docs/openapi.json", openapi_spec);

    Router::new()
        .route("/health", get(health_check))
        .merge(swagger_ui)
}
