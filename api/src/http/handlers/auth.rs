use crate::http::dtos::*;
use crate::http::error::HttpError;
use crate::http::middleware::auth::ClaimsExtractor;
use crate::http::middleware::auth::auth_middleware;
use axum::middleware::from_fn_with_state;
use axum::{extract::State, http::StatusCode, routing::*, *};
use domain::services::auth_service::IAuthService;
use infrastructure::state::AppState;
use std::sync::Arc;

#[utoipa::path(
    post,
    path = "/v1/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses()
)]
#[axum::debug_handler]
async fn login(
    State(auth_service): State<Arc<dyn IAuthService>>,
    Json(login_payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<LoginResponse>), HttpError> {
    // let email = login_payload.email.clone();
    // let mut login_command = login_payload.into();
    //
    // let user = app_state
    //     .auth_service
    //     .user_repository
    //     .find_by_email(&email)
    //     .await?
    //     .ok_or(Error::Application(ApplicationError::InvalidCredentials))?;
    //
    // let jwt_token = app_state
    //     .auth_service
    //     .authenticate(&mut login_command)
    //     .await?;
    //
    // let refresh_token = app_state.auth_service.issue_refresh_token(&user.id).await?;
    //
    // let expires_in = app_state.config.jwt_expiration_duration.num_seconds();
    //
    // Ok((
    //     StatusCode::OK,
    //     Json(LoginResponse {
    //         user: user.into(),
    //         token: jwt_token,
    //         refresh_token,
    //         expires_in,
    //     }),
    // ))
    todo!()
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
    State(auth_service): State<Arc<dyn IAuthService>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<RegisterResponse>), HttpError> {
    // let user = auth_service.register(&mut payload.into()).await?;
    // let jwt_token = app_state
    //     .auth_service
    //     .jwt_provider
    //     .generate_token(&user.id)
    //     .await?;
    // let refresh_token = app_state.auth_service.issue_refresh_token(&user.id).await?;
    //
    // Ok((
    //     StatusCode::CREATED,
    //     Json(RegisterResponse {
    //         user: user.into(),
    //         token: jwt_token,
    //         refresh_token,
    //     }),
    // ))
    todo!()
}

#[utoipa::path(
    post,
    path = "/v1/auth/logout",
    tag = "auth",
    security(("auth" = [])),
    responses(
        (status = 200, description = "Logout successful", body = LogoutResponse),
        (status = 401, description = "Invalid token", body = serde_json::Value)
    )
)]
#[axum::debug_handler]
async fn logout(
    State(app_state): State<Arc<dyn IAuthService>>,
    claims: ClaimsExtractor,
) -> Result<LogoutResponse, HttpError> {
    todo!()
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
    State(app_state): State<Arc<dyn IAuthService>>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<RefreshTokenResponse, HttpError> {
    todo!()
}

#[utoipa::path(
    get,
    path = "/v1/auth/me",
    tag = "auth",
    security(("auth" = [])),
    responses(
        (status = 200, description = "User data retrieved", body = UserResponse),
        (status = 401, description = "Invalid token", body = serde_json::Value)
    )
)]
#[axum::debug_handler]
async fn me(
    State(app_state): State<Arc<dyn IAuthService>>,
    claims: ClaimsExtractor,
) -> Result<UserResponse, HttpError> {
    todo!()
}

#[utoipa::path(
    put,
    path = "/v1/auth/profile",
    tag = "auth",
    security(("auth" = [])),
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Profile updated successfully", body = UpdateProfileResponse),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse),
        (status = 409, description = "Username or email already exists", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn update_profile(
    claims: ClaimsExtractor,
    State(app_state): State<Arc<dyn IAuthService>>,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<UpdateProfileResponse>, HttpError> {
    todo!()
}

#[utoipa::path(
    put,
    path = "/v1/auth/password",
    tag = "auth",
    security(("auth" = [])),
    request_body = ChangePasswordRequest,
    responses()
)]
#[axum::debug_handler]
pub async fn change_password(
    claims: ClaimsExtractor,
    State(auth_service): State<Arc<dyn IAuthService>>,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<Json<ChangePasswordResponse>, HttpError> {
    todo!()
}

#[utoipa::path(
    delete,
    path = "/v1/auth/account",
    tag = "auth",
    security(("auth" = [])),
    request_body = DeleteAccountRequest,
    responses(
        (status = 200, description = "Account deleted successfully", body = DeleteAccountResponse),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse),
        (status = 403, description = "Password is incorrect", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn delete_account(
    claims: ClaimsExtractor,
    State(auth_service): State<Arc<dyn IAuthService>>,
    Json(payload): Json<DeleteAccountRequest>,
) -> Result<Json<DeleteAccountResponse>, HttpError> {
    todo!()
}

pub fn auth_routes(state: AppState) -> Router {
    let public = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh));

    let protected = Router::new()
        .route("/logout", post(logout))
        .route("/me", get(me))
        .route("/profile", put(update_profile))
        .route("/password", put(change_password))
        .route("/account", delete(delete_account))
        .layer(from_fn_with_state(state.clone(), auth_middleware));

    Router::new()
        .nest("/auth", Router::new().merge(public).merge(protected))
        .with_state(state)
}
