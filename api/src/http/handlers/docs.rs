use axum::Router;
use utoipa::openapi::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn docs_routes(api: OpenApi) -> Router {
    Router::new().merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", api))
}
