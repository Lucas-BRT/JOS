use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use std::sync::Arc;

use crate::{
    core::state::AppState,
    domain::jwt::Claims,
    interfaces::http::error::AuthError,
};

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
            .jwt_service
            .decode_token(bearer.token())
            .await
            .map_err(|_| AuthError::InvalidToken)
    }
}
