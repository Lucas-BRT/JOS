use super::api_doc::ApiDoc;
use axum::{Router, Json};
use utoipa::OpenApi;

const OPENAPI_JSON_ROUTE: &str = "/api-docs/openapi.json";
    
    
pub fn routes() -> Router {
    Router::new().route(OPENAPI_JSON_ROUTE, axum::routing::get(openapi_json))
}

async fn openapi_json() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}
