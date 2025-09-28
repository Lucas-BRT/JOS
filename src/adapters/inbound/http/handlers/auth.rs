use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::{
    domain::auth::{Authenticator, Claims},
    domain::entities::commands::{CreateUserCommand, LoginUserCommand},
    dtos::auth::*,
    infrastructure::state::AppState,
    shared::{Error, Result},
};

// Conversion implementations
impl From<LoginRequest> for LoginUserCommand {
    fn from(req: LoginRequest) -> Self {
        LoginUserCommand {
            email: req.email,
            password: req.password,
        }
    }
}

impl From<RegisterRequest> for CreateUserCommand {
    fn from(req: RegisterRequest) -> Self {
        CreateUserCommand {
            username: req.username,
            email: req.email,
            password: req.password,
        }
    }
}

#[utoipa::path(
    post,
    path = "/v1/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials", body = serde_json::Value),
        (status = 400, description = "Missing required fields", body = serde_json::Value)
    )
)]
#[axum::debug_handler]
async fn login(
    State(app_state): State<Arc<AppState>>,
    Json(login_payload): Json<LoginRequest>,
) -> Result<LoginResponse> {
    if let Err(sanitization_error) = login_payload.validate() {
        return Err(Error::Validation(sanitization_error));
    }

    let email = login_payload.email.clone();
    let mut login_command = login_payload.into();
    let jwt_token = app_state
        .auth_service
        .authenticate(&mut login_command)
        .await?;

    let user = app_state
        .auth_service
        .user_repository
        .find_by_email(&email)
        .await?
        .ok_or(Error::Application(
            crate::application::error::ApplicationError::InvalidCredentials,
        ))?;

    Ok(LoginResponse {
        user: user.into(),
        token: jwt_token,
        refresh_token: "refresh_token_placeholder".to_string(), // TODO: Implement refresh token
        expires_in: 86400,                                      // 24 hours in seconds
    })
}

#[utoipa::path(
    post,
    path = "/v1/auth/register",
    tag = "auth",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User created successfully", body = RegisterResponse),
        (status = 400, description = "Validation error", body = serde_json::Value),
        (status = 409, description = "Email or username already exists", body = serde_json::Value)
    )
)]
#[axum::debug_handler]
async fn register(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<RegisterResponse> {
    if let Err(sanitization_error) = payload.validate() {
        return Err(Error::Validation(sanitization_error));
    }

    // Validate password confirmation
    if payload.password != payload.confirm_password {
        return Err(Error::Validation(validator::ValidationErrors::new()));
    }

    let user = app_state.auth_service.register(&mut payload.into()).await?;
    let jwt_token = app_state
        .auth_service
        .jwt_provider
        .generate_token(&user.id)
        .await?;

    Ok(RegisterResponse {
        user: user.into(),
        token: jwt_token,
        refresh_token: "refresh_token_placeholder".to_string(), // TODO: Implement refresh token
        expires_in: 86400,                                      // 24 hours in seconds
    })
}

#[utoipa::path(
    post,
    path = "/v1/auth/logout",
    tag = "auth",
    responses(
        (status = 200, description = "Logout successful", body = LogoutResponse),
        (status = 401, description = "Invalid token", body = serde_json::Value)
    )
)]
#[axum::debug_handler]
async fn logout(_claims: Claims) -> Result<LogoutResponse> {
    Ok(LogoutResponse {
        message: "Logout successful".to_string(),
    })
}

#[utoipa::path(
    post,
    path = "/v1/auth/refresh",
    tag = "auth",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = RefreshTokenResponse),
        (status = 401, description = "Invalid refresh token", body = serde_json::Value)
    )
)]
#[axum::debug_handler]
async fn refresh(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<RefreshTokenResponse> {
    // TODO: Implement proper refresh token validation
    let new_token = app_state
        .auth_service
        .jwt_provider
        .generate_token(&Uuid::new_v4()) // TODO: Extract user_id from refresh token
        .await?;

    Ok(RefreshTokenResponse {
        token: new_token,
        refresh_token: "new_refresh_token_placeholder".to_string(), // TODO: Generate new refresh token
        expires_in: 86400,                                          // 24 hours in seconds
    })
}

#[utoipa::path(
    get,
    path = "/v1/auth/me",
    tag = "auth",
    responses(
        (status = 200, description = "User data retrieved", body = UserResponse),
        (status = 401, description = "Invalid token", body = serde_json::Value)
    )
)]
#[axum::debug_handler]
async fn me(State(app_state): State<Arc<AppState>>, claims: Claims) -> Result<UserResponse> {
    let user = app_state
        .auth_service
        .user_repository
        .find_by_id(&claims.sub)
        .await?
        .ok_or(Error::Application(
            crate::application::error::ApplicationError::InvalidCredentials,
        ))?;
    Ok(user.into())
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/refresh", post(refresh))
        .route("/me", get(me))
        .with_state(state)
}
