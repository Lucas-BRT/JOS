use axum::Json;
use chrono::Utc;
use serde_json::Value;
use serde_json::json;

#[utoipa::path(get, path = "/health", summary = "Get API status", tag = "health")]
pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": Utc::now().to_rfc3339(),
        "service": "JOS API",
        "version": env!("CARGO_PKG_VERSION")
    }))
}
