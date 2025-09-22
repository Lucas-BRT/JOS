use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

pub async fn auth_middleware<B>(
    State(auth_state): State<AppState>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let decoding_key = DecodingKey::from_secret(auth_state.jwt_secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);

    let token_data = decode::<Claims>(auth_header, &decoding_key, &validation)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let now = chrono::Utc::now().timestamp() as usize;
    if token_data.claims.exp < now {
        return Err(StatusCode::UNAUTHORIZED);
    }

    req.extensions_mut().insert(token_data.claims.sub);

    Ok(next.run(req).await)
}
