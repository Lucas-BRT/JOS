use axum::{extract::Request, middleware::Next, response::Response};
use tracing::debug;

pub async fn trace_middleware(request: Request, next: Next) -> Response {
    debug!("Processing request: {} {}", request.method(), request.uri());
    let response = next.run(request).await;
    debug!("Request completed with status: {}", response.status());
    response
}
