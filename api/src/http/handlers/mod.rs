use crate::http::handlers::{cors::cors_layer, tracing::trace_middleware};
pub use crate::http::{
    middleware::{cors, tracing},
    open_api::ApiDoc,
};
use axum::{Router, middleware::from_fn, routing::get};
use infrastructure::state::AppState;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

pub mod auth;
pub mod docs;
pub mod game_system;
pub mod health;
pub mod session;
pub mod table;
pub mod table_members;
pub mod table_request;
pub mod user;

pub use auth::auth_routes;
pub use game_system::game_system_routes;
pub use health::health_check;
pub use session::session_routes;
pub use table::table_routes;
pub use table_request::table_request_routes;
pub use user::user_routes;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let openapi_spec = ApiDoc::openapi();
    let mut api = OpenApiRouter::new();

    api = api
        .merge(auth_routes(app_state.clone()).into())
        .merge(table_routes(app_state.clone()).into())
        .merge(session_routes(app_state.clone()).into())
        .merge(table_request_routes(app_state.clone()).into())
        .merge(user_routes(app_state.clone()).into())
        .merge(game_system_routes(app_state.clone()).into());

    Router::new()
        .merge(build_system_routes(openapi_spec))
        .nest("/v1", api.into())
        .layer(cors_layer())
        .layer(from_fn(trace_middleware))
}

fn build_system_routes(openapi_spec: utoipa::openapi::OpenApi) -> Router {
    let swagger_ui = SwaggerUi::new("/docs").url("/api-docs/openapi.json", openapi_spec);

    Router::new()
        .route("/health", get(health_check))
        .merge(swagger_ui)
}
