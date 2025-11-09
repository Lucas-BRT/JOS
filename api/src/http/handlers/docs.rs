use axum::Router;
use utoipa::openapi::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn docs_routes(api: OpenApi) -> Router {
    let swagger_ui = SwaggerUi::new("/docs").url("/api-docs/openapi.json", api);
    Router::new().merge(swagger_ui)
}
