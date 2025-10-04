use crate::{domain::auth::Claims, infrastructure::state::AppState};
use axum::{
    extract::{FromRequestParts, Request, State},
    http::{StatusCode, request::Parts},
    middleware::Next,
    response::Response,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use std::sync::Arc;

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Claims>()
            .cloned()
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}

pub async fn auth_middleware(
    State(app_state): State<Arc<AppState>>,
    TypedHeader(auth_header): TypedHeader<Authorization<Bearer>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = auth_header.token();

    let token_data = app_state
        .auth_service
        .jwt_provider
        .decode_token(token)
        .await
        .or(Err(StatusCode::UNAUTHORIZED))?;

    let now = chrono::Utc::now().timestamp();

    if token_data.exp < now {
        return Err(StatusCode::UNAUTHORIZED);
    }

    request.extensions_mut().insert(token_data);

    Ok(next.run(request).await)
}
