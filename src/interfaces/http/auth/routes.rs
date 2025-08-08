use crate::{
    Error, Result,
    interfaces::http::{
        auth::dtos::{LoginDto, SignupDto, UserSignupResponse},
        error::ValidationError,
        openapi::{schemas::*, tags::AUTH_TAG},
    },
    state::AppState,
};
use axum::{Json, Router, extract::State, routing::post};
use std::sync::Arc;
use validator::Validate;

/// Create a new user account
#[utoipa::path(
    post,
    path = "/auth/signup",
    tag = AUTH_TAG,
    request_body = SignupDto,
    responses(
        (status = 201, description = "User created successfully", body = UserSignupResponse),
        (status = 400, description = "Validation error", body = ValidationErrorResponse),
        (status = 400, description = "Password mismatch", body = PasswordMismatchErrorResponse),
        (status = 409, description = "Email already taken", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
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

/// Authenticate user and get JWT token
#[utoipa::path(
    post,
    path = "/auth/login",
    tag = AUTH_TAG,
    request_body = LoginDto,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 400, description = "Validation error", body = ValidationErrorResponse),
        (status = 400, description = "Invalid credentials", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
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
        .login(&login_payload.into())
        .await?;

    Ok(jwt_token)
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .with_state(state.clone())
}
