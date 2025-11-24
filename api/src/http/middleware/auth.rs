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
use domain::auth::Claims;
use infrastructure::state::AppState;
use std::sync::Arc;
use uuid::Uuid;

// Wrapper to implement FromRequestParts locally
pub struct ClaimsExtractor(pub Claims);

impl ClaimsExtractor {
    pub fn get_user_id(&self) -> Uuid {
        self.0.sub
    }
}

impl<S> FromRequestParts<S> for ClaimsExtractor
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Claims>()
            .cloned()
            .map(ClaimsExtractor)
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}

pub async fn auth_middleware(
    State(app_state): State<Arc<AppState>>,
    auth_header: Option<TypedHeader<Authorization<Bearer>>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = match auth_header {
        Some(TypedHeader(auth)) => auth.token().to_owned(),
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let token_data = app_state
        .auth_service
        .jwt_provider
        .decode_token(&token)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let now = chrono::Utc::now().timestamp();

    if token_data.exp < now {
        return Err(StatusCode::UNAUTHORIZED);
    }

    request.extensions_mut().insert(token_data);

    Ok(next.run(request).await)
}
