use crate::{
    Error, Result,
    interfaces::http::auth::dtos::{LoginDto, SignupDto, UserSignupResponse},
    core::state::AppState,
};
use axum::{Json, Router, extract::State, routing::{post, get}};
use std::sync::Arc;
use validator::Validate;
use serde_json::json;


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

    let user = app_state
        .user_service
        .signup(payload)
        .await?;

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
        .user_service
        .login(login_payload.into())
        .await?;

    Ok(jwt_token)
}

#[utoipa::path(
    get,
    path = "/v1/auth/password-requirements",
    tag = "auth",
    responses(
        (status = 200, description = "Password requirements", body = serde_json::Value)
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

