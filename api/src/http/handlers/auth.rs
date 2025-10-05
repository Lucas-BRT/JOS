use crate::http::middleware::auth::ClaimsExtractor;
use crate::{http::dtos::auth::*, http::middleware::auth::auth_middleware};
use axum::middleware::from_fn_with_state;
use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use domain::auth::Authenticator;
use domain::entities::commands::{CreateUserCommand, LoginUserCommand};
use infrastructure::state::AppState;
use shared::Result;
use shared::error::Error;
use std::sync::Arc;
use tracing::info;
use validator::Validate;

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
        return Err(Error::Validation(
            shared::error::ValidationError::ValidationFailed(sanitization_error.to_string()),
        ));
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
            shared::error::ApplicationError::InvalidCredentials,
        ))?;

    let refresh_token = app_state.auth_service.issue_refresh_token(&user.id).await?;

    Ok(LoginResponse {
        user: user.into(),
        token: jwt_token,
        refresh_token,
        expires_in: 86400,
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
        return Err(Error::Validation(
            shared::error::ValidationError::ValidationFailed(sanitization_error.to_string()),
        ));
    }

    let user = app_state.auth_service.register(&mut payload.into()).await?;
    let jwt_token = app_state
        .auth_service
        .jwt_provider
        .generate_token(&user.id)
        .await?;
    let refresh_token = app_state.auth_service.issue_refresh_token(&user.id).await?;

    Ok(RegisterResponse {
        user: user.into(),
        token: jwt_token,
        refresh_token,
    })
}

#[utoipa::path(
    post,
    path = "/v1/auth/logout",
    tag = "auth",
    security(("bearer" = [])),
    responses(
        (status = 200, description = "Logout successful", body = LogoutResponse),
        (status = 401, description = "Invalid token", body = serde_json::Value)
    )
)]
#[axum::debug_handler]
async fn logout(_claims: ClaimsExtractor) -> Result<LogoutResponse> {
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
    let (new_refresh_token, user_id) = app_state
        .auth_service
        .rotate_refresh_token(&payload.refresh_token)
        .await?;

    let new_jwt = app_state
        .auth_service
        .jwt_provider
        .generate_token(&user_id)
        .await?;

    Ok(RefreshTokenResponse {
        token: new_jwt,
        refresh_token: new_refresh_token,
        expires_in: 86400,
    })
}

#[utoipa::path(
    get,
    path = "/v1/auth/me",
    tag = "auth",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "User data retrieved", body = UserResponse),
        (status = 401, description = "Invalid token", body = serde_json::Value)
    )
)]
#[axum::debug_handler]
async fn me(
    State(app_state): State<Arc<AppState>>,
    claims: ClaimsExtractor,
) -> Result<UserResponse> {
    let user = app_state
        .auth_service
        .user_repository
        .find_by_id(&claims.0.sub)
        .await?
        .ok_or(Error::Application(
            shared::error::ApplicationError::InvalidCredentials,
        ))?;

    info!("user id: {}", claims.0.sub);

    Ok(user.into())
}

pub fn auth_routes(state: Arc<AppState>) -> Router {
    let public = Router::new()
        .route("/register", post(register))
        .route("/login", post(login));

    let protected = Router::new()
        .route("/logout", post(logout))
        .route("/me", get(me))
        .route("/refresh", post(refresh))
        .layer(from_fn_with_state(state.clone(), auth_middleware));

    Router::new()
        .nest("/auth", Router::new().merge(public).merge(protected))
        .with_state(state)
}
