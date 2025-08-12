use axum::{Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::interfaces::http::openapi::api_doc::ApiDoc;


pub fn routes() -> Router {
    Router::new()
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
}


