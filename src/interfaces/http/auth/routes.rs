use crate::{
    Error, Result, error::ValidationError, interfaces::http::auth::dtos::SignupDto, state::AppState,
};
use axum::{Json, Router, extract::State, routing::post};
use std::sync::Arc;
use validator::Validate;

#[axum::debug_handler]
async fn signup(
    State(app_state): State<Arc<AppState>>,
    Json(new_user_payload): Json<SignupDto>,
) -> Result<Json<String>> {
    new_user_payload
        .validate()
        .map_err(|err| Error::Validation(ValidationError::Other(err)))?;

    match app_state
        .user_service
        .create(&new_user_payload.into())
        .await
    {
        Ok(user_id) => Ok(Json(user_id.to_string())),
        Err(err) => Err(err),
    }
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/signup", post(signup))
        .with_state(state.clone())
}
