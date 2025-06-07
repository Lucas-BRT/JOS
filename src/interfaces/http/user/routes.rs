use crate::state::AppState;
use axum::Router;
use std::sync::Arc;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new().with_state(state.clone())
}
