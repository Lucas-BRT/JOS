use crate::http::handlers::{cors::cors_layer, tracing::trace_middleware};
pub use crate::http::{
    middleware::{cors, tracing},
    open_api::ApiDoc,
};
use axum::{Router, middleware::from_fn, routing::get};
use infrastructure::state::AppState;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa::openapi::OpenApi as OpenApiSpec;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

pub mod auth;
pub mod docs;
pub mod game_system;
pub mod health;
pub mod session;
pub mod session_checkin;
pub mod session_intent;
pub mod table;
pub mod table_request;
pub mod user;

pub use auth::auth_routes;
pub use game_system::game_system_routes;
pub use health::health_check;
pub use session::session_routes;
pub use session_intent::session_intent_routes;
pub use table::table_routes;
pub use table_request::table_request_routes;
pub use user::user_routes;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let open_api_router = OpenApiRouter::with_openapi(ApiDoc::openapi()).nest(
        "/v1",
        OpenApiRouter::new()
            .merge(auth_routes(app_state.clone()))
            .merge(table_routes(app_state.clone()))
            .merge(session_routes(app_state.clone()))
            .merge(table_request_routes(app_state.clone()))
            .merge(user_routes(app_state.clone()))
            .merge(game_system_routes(app_state.clone()))
            .merge(session_intent_routes(app_state.clone())),
    );

    let (router, api_doc) = open_api_router.split_for_parts();

    router
        .merge(build_system_routes(api_doc))
        .layer(cors_layer())
        .layer(from_fn(trace_middleware))
}

fn build_system_routes(openapi_spec: OpenApiSpec) -> Router {
    let swagger_ui = SwaggerUi::new("/docs").url("/api-docs/openapi.json", openapi_spec);

    Router::new()
        .route("/health", get(health_check))
        .merge(swagger_ui)
}
