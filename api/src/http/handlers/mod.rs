pub use crate::http::{
    handlers::{
        auth::auth_routes, health::health_check, session::session_routes, table::table_routes,
        table_request::table_request_routes, user::user_routes,
    },
    middleware::{cors, tracing},
    open_api::ApiDoc,
};
use axum::{Router, routing::get};
use infrastructure::state::AppState;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

pub mod auth;
pub mod docs;
pub mod health;
pub mod session;
pub mod table;
pub mod table_request;
pub mod user;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let openapi_spec = ApiDoc::openapi();
    let mut api = OpenApiRouter::new();

    api = api
        .merge(auth_routes(app_state.clone()).into())
        .merge(table_routes(app_state.clone()).into())
        .merge(session_routes(app_state.clone()).into())
        .merge(table_request_routes(app_state.clone()).into())
        .merge(user_routes(app_state.clone()).into());

    Router::new()
        .merge(build_system_routes(openapi_spec))
        .nest("/v1", api.into())
        .layer(cors::cors_layer())
        .layer(axum::middleware::from_fn(tracing::trace_middleware))
}

fn build_system_routes(openapi_spec: utoipa::openapi::OpenApi) -> Router {
    let swagger_ui = SwaggerUi::new("/docs").url("/api-docs/openapi.json", openapi_spec);

    Router::new()
        .route("/health", get(health_check))
        .merge(swagger_ui)
}
