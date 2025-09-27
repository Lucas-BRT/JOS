use axum::Router;
use utoipa_swagger_ui::SwaggerUi;

pub fn routes() -> Router {
    Router::new().merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
