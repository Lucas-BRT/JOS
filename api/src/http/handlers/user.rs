use crate::http::middleware::auth::auth_middleware;
use axum::{Router, middleware::from_fn_with_state};
use infrastructure::state::AppState;
use std::sync::Arc;

pub fn user_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .nest(
            "/user",
            Router::new().layer(from_fn_with_state(state.clone(), auth_middleware)),
        )
        .with_state(state)
}
