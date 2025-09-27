use axum::Router;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;
use crate::adapters::inbound::open_api::ApiDoc;

pub fn routes() -> Router {
    Router::new().merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
