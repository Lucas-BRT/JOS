use crate::{
    Error, Result,
    interfaces::http::{
        auth::dtos::{LoginDto, SignupDto, UserSignupResponse},
        error::ValidationError,
    },
    state::AppState,
};
use axum::{Json, Router, extract::State, routing::post};
use std::sync::Arc;
use validator::Validate;

#[axum::debug_handler]
async fn signup(
    State(app_state): State<Arc<AppState>>,
    Json(new_user_payload): Json<SignupDto>,
) -> Result<UserSignupResponse> {
    if let Err(sanitization_error) = new_user_payload.validate() {
        return Err(Error::Validation(ValidationError::Other(
            sanitization_error,
        )));
    }

    if new_user_payload.password != new_user_payload.confirm_password {
        return Err(Error::Validation(ValidationError::PasswordMismatch));
    }

    let user = app_state
        .user_service
        .signup(&new_user_payload.into())
        .await?;

    Ok(user.into())
}

#[axum::debug_handler]
async fn login(
    State(app_state): State<Arc<AppState>>,
    Json(login_payload): Json<LoginDto>,
) -> Result<String> {
    if let Err(sanitization_error) = login_payload.validate() {
        return Err(Error::Validation(ValidationError::Other(
            sanitization_error,
        )));
    }

    let jwt_token = app_state
        .user_service
        .login(
            &login_payload.into(),
            &app_state.config.jwt_secret,
            app_state.config.jwt_expiration_duration,
        )
        .await?;

    Ok(jwt_token)
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .with_state(state.clone())
}
