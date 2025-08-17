use axum::Json;
use serde_json::Value;
use serde_json::json;

#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "Service is healthy", body = Value)
    )
)]
pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "service": "JOS API",
        "version": env!("CARGO_PKG_VERSION")
    }))
}
