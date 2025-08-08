use crate::{
    Error, Result,
    interfaces::http::{
        auth::dtos::{LoginDto, SignupDto, UserSignupResponse},
        error::ValidationError,
        openapi::schemas::{ValidationErrorResponse, SignupDto as OpenApiSignupDto, UserSignupResponse as OpenApiUserSignupResponse, ErrorResponse},
    },
    core::state::AppState,
    domain::user::{dtos::{CreateUserCommand, LoginUserCommand}, entity::User},
};
use axum::{Json, Router, extract::State, routing::{post, get}, response::IntoResponse, http::StatusCode};
use std::sync::Arc;
use validator::Validate;
use serde_json::json;

/// Create a new user account
#[utoipa::path(
    post,
    path = "/v1/auth/signup",
    tag = "auth",
    request_body = OpenApiSignupDto,
    responses(
        (status = 201, description = "User created successfully", body = OpenApiUserSignupResponse),
        (status = 400, description = "Bad request", body = ValidationErrorResponse)
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
    path = "/v1/auth/login",
    tag = "auth",
    request_body = LoginDto,
    responses(
        (status = 200, description = "Login successful", body = String),
        (status = 401, description = "Invalid credentials", body = ErrorResponse)
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

/// Get password requirements
#[utoipa::path(
    get,
    path = "/v1/auth/password-requirements",
    tag = "auth",
    responses(
        (status = 200, description = "Password requirements", body = crate::interfaces::http::openapi::schemas::PasswordRequirementsResponse)
    )
)]
#[axum::debug_handler]
async fn get_password_requirements(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>> {
    let requirements = state.password_service.get_requirements().await;
    
    Ok(Json(json!({
        "requirements": requirements
    })))
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/password-requirements", get(get_password_requirements))
        .with_state(state.clone())
}

// Implementations for conversions
impl From<SignupDto> for CreateUserCommand {
    fn from(dto: SignupDto) -> Self {
        CreateUserCommand {
            name: dto.name,
            email: dto.email,
            password: dto.password,
        }
    }
}

impl From<LoginDto> for LoginUserCommand {
    fn from(dto: LoginDto) -> Self {
        LoginUserCommand {
            email: dto.email,
            password: dto.password,
        }
    }
}

impl From<User> for UserSignupResponse {
    fn from(user: User) -> Self {
        UserSignupResponse {
            id: user.id.to_string(),
            name: user.name,
            email: user.email,
        }
    }
}

impl IntoResponse for UserSignupResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}
