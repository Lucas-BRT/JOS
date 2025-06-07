use crate::{interfaces::http::auth::dtos::SignupDto, state::AppState};
use axum::{Json, Router, extract::State, routing::post};
use std::sync::Arc;

#[axum::debug_handler]
async fn signup(
    State(app_state): State<Arc<AppState>>,
    Json(new_user_payload): Json<SignupDto>,
) -> Json<String> {
    match app_state
        .user_service
        .create(&new_user_payload.into())
        .await
    {
        Ok(user_id) => Json(user_id.to_string()),
        Err(err) => Json(err.to_string()),
    }
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/signup", post(signup))
        .with_state(state.clone())
}
