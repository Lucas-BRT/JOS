use super::api_doc::ApiDoc;
use axum::{Router, Json};
use utoipa::OpenApi;

    
pub fn routes() -> Router {
    Router::new().route("/api-docs/openapi.json", axum::routing::get(openapi_json))
}

async fn openapi_json() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}
