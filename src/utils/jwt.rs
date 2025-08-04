use crate::{Error, Result, config::JWT_SECRET, domain::user::role::Role};
use axum::{
    Json, RequestPartsExt,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
    pub iat: usize,
    pub role: Role,
}

impl Claims {
    pub fn create_jwt(
        user_id: Uuid,
        jwt_secret: &str,
        jwt_expiration_duration: Duration,
        user_role: Role,
    ) -> Result<String> {
        let expiration = Utc::now()
            .checked_add_signed(jwt_expiration_duration)
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user_id,
            exp: expiration as usize,
            iat: Utc::now().timestamp() as usize,
            role: user_role,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_ref()),
        )
        .map_err(|e| {
            tracing::error!("failed to generate JWT token {}", e);
            Error::InternalServerError
        })
    }
}

#[derive(Deserialize, Serialize)]
pub struct AuthUser {
    pub user_id: Uuid,
}

#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        tracing::debug!("Extracting claims from request parts");
        let (status, error_message) = match self {
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}
