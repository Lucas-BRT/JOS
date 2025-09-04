use crate::{core::state::AppState, domain::auth::Claims, interfaces::http::error::AuthError};
use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use std::sync::Arc;

impl FromRequestParts<Arc<AppState>> for Claims {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> std::result::Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        state
            .auth_service
            .jwt_provider
            .decode_token(bearer.token())
            .await
            .map_err(|_| AuthError::InvalidToken)
    }
}
