use axum::{extract::Request, middleware::Next, response::Response};
use tracing::info;

pub async fn trace_middleware(request: Request, next: Next) -> Response {
    info!("Processing request: {} {}", request.method(), request.uri());
    let response = next.run(request).await;
    info!("Request completed with status: {}", response.status());
    response
}
