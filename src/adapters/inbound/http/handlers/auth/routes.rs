use crate::{
    Error, Result,
    core::state::AppState,
    domain::auth::{Authenticator, Claims},
    interfaces::http::auth::dtos::{
        LoginDto, LoginResponse, SignupDto, UserResponse, UserSignupResponse,
    },
};
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
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
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials", body = serde_json::Value)
    )
)]
#[axum::debug_handler]
async fn login(
    State(app_state): State<Arc<AppState>>,
    Json(login_payload): Json<LoginDto>,
) -> Result<LoginResponse> {
    if let Err(sanitization_error) = login_payload.validate() {
        return Err(Error::Validation(sanitization_error));
    }

    let jwt_token = app_state
        .auth_service
        .authenticate(&login_payload.into())
        .await?;

    // Get user information
    let user = app_state
        .user_service
        .find_by_email(&login_payload.email)
        .await?;

    Ok(LoginResponse {
        user: user.into(),
        token: jwt_token,
    })
}

#[utoipa::path(
    post,
    path = "/v1/auth/register",
    tag = "auth",
    request_body = SignupDto,
    responses(
        (status = 201, description = "User created successfully", body = UserSignupResponse),
        (status = 400, description = "Bad request", body = serde_json::Value)
    )
)]
#[axum::debug_handler]
async fn register(
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
    path = "/v1/auth/logout",
    tag = "auth",
    responses(
        (status = 200, description = "Logout successful"),
        (status = 401, description = "Unauthorized", body = serde_json::Value)
    )
)]
#[axum::debug_handler]
async fn logout(_claims: Claims) -> Result<impl IntoResponse> {
    // In a stateless JWT implementation, logout is handled client-side
    // by removing the token. We could implement a blacklist here if needed.
    Ok((
        StatusCode::OK,
        Json(serde_json::json!({"message": "Logout successful"})),
    ))
}

#[utoipa::path(
    post,
    path = "/v1/auth/refresh",
    tag = "auth",
    responses(
        (status = 200, description = "Token refreshed successfully", body = LoginResponse),
        (status = 401, description = "Unauthorized", body = serde_json::Value)
    )
)]
#[axum::debug_handler]
async fn refresh(State(app_state): State<Arc<AppState>>, claims: Claims) -> Result<LoginResponse> {
    // Generate a new token with the same claims
    let new_token = app_state
        .auth_service
        .jwt_provider
        .generate_token(&claims.user_id)
        .await?;

    // Get user information
    let user = app_state.user_service.find_by_id(&claims.user_id).await?;

    Ok(LoginResponse {
        user: user.into(),
        token: new_token,
    })
}

#[utoipa::path(
    get,
    path = "/v1/auth/me",
    tag = "auth",
    responses(
        (status = 200, description = "User information", body = UserResponse),
        (status = 401, description = "Unauthorized", body = serde_json::Value)
    )
)]
#[axum::debug_handler]
async fn me(State(app_state): State<Arc<AppState>>, claims: Claims) -> Result<UserResponse> {
    let user = app_state.user_service.find_by_id(&claims.user_id).await?;
    Ok(user.into())
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/refresh", post(refresh))
        .route("/me", get(me))
        .with_state(state.clone())
}
