use crate::{
    Error, Result,
    core::state::AppState,
    domain::auth::Authenticator,
    interfaces::http::auth::dtos::{LoginDto, SignupDto, UserSignupResponse},
};
use axum::{
    Json, Router,
    extract::State,
    routing::post,
};
use std::sync::Arc;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/v1/auth/signup",
    tag = "auth",
    request_body = SignupDto,
    responses(
        (status = 201, description = "User created successfully", body = UserSignupResponse),
        (status = 400, description = "Bad request", body = serde_json::Value)
    )
)]
#[axum::debug_handler]
async fn signup(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<SignupDto>,
) -> Result<UserSignupResponse> {
    if let Err(sanitization_error) = payload.validate() {
        return Err(Error::Validation(sanitization_error));
    }

    let user = app_state.auth_service.register(&mut payload.into()).await?;

    Ok(user.into())
}

#[utoipa::path(
    post,
    path = "/v1/auth/login",
    tag = "auth",
    request_body = LoginDto,
    responses(
        (status = 200, description = "Login successful", body = String),
        (status = 401, description = "Invalid credentials", body = serde_json::Value)
    )
)]
#[axum::debug_handler]
async fn login(
    State(app_state): State<Arc<AppState>>,
    Json(login_payload): Json<LoginDto>,
) -> Result<String> {
    if let Err(sanitization_error) = login_payload.validate() {
        return Err(Error::Validation(sanitization_error));
    }

    let jwt_token = app_state
        .auth_service
        .authenticate(&login_payload.into())
        .await?;

    Ok(jwt_token)
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .with_state(state.clone())
}
