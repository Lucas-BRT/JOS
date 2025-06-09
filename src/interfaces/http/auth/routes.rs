use crate::{
    Error, Result,
    error::ValidationError,
    interfaces::http::auth::dtos::{LoginDto, SignupDto, UserSignupResponse},
    state::AppState,
};
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use std::sync::Arc;
use validator::Validate;

#[axum::debug_handler]
async fn signup(
    State(app_state): State<Arc<AppState>>,
    Json(new_user_payload): Json<SignupDto>,
) -> impl IntoResponse {
    if let Err(sanitization_error) = new_user_payload.validate() {
        return Err(Error::Validation(ValidationError::Other(
            sanitization_error,
        )));
    }

    let user = app_state
        .user_service
        .signup(&new_user_payload.into())
        .await;

    match user {
        Ok(user) => {
            let response = UserSignupResponse {
                id: user.id,
                name: user.name,
                email: user.email,
            };
            Ok((StatusCode::CREATED, Json(response)).into_response())
        }
        Err(err) => Err(err),
    }
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/signup", post(signup))
        .with_state(state.clone())
}
