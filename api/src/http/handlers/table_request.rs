use crate::http::dtos::*;
use crate::http::middleware::auth::{ClaimsExtractor, auth_middleware};
use axum::extract::*;
use axum::middleware::from_fn_with_state;
use infrastructure::state::AppState;
use shared::Result;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/sent",
    tag = "table-request",
    security(("auth" = [])),
)]
#[axum::debug_handler]
pub async fn get_sent_requests(
    claims: ClaimsExtractor,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<SentRequestItem>>> {
    let requests = app_state
        .table_request_service
        .get_sent_requests(claims.get_user_id())
        .await?;

    let requests = requests
        .into_iter()
        .map(SentRequestItem::from)
        .collect::<Vec<SentRequestItem>>();

    Ok(Json(requests))
}

#[utoipa::path(
    post,
    path = "/{request_id}/accept",
    tag = "table-request",
    summary = "Accepts a player request",
    security(("auth" = [])),
)]
#[axum::debug_handler]
pub async fn accept_request(
    claims: ClaimsExtractor,
    State(app_state): State<Arc<AppState>>,
    Path(request_id): Path<Uuid>,
) -> Result<Json<AcceptRequestResponse>> {
    app_state
        .table_request_service
        .accept_request(request_id, claims.get_user_id())
        .await?;

    Ok(Json(AcceptRequestResponse {
        message: format!("Request {} accepted successfully", request_id),
    }))
}

#[utoipa::path(
    post,
    path = "/{request_id}/reject",
    tag = "table-request",
    security(("auth" = [])),
    summary = "Rejects a player request"
)]
#[axum::debug_handler]
pub async fn reject_request(
    claims: ClaimsExtractor,
    State(app_state): State<Arc<AppState>>,
    Path(request_id): Path<Uuid>,
) -> Result<Json<RejectRequestResponse>> {
    app_state
        .table_request_service
        .reject_request(request_id, claims.get_user_id())
        .await?;

    Ok(Json(RejectRequestResponse {
        message: format!("Request {} rejected successfully", request_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/{request_id}",
    tag = "table-request",
    security(("auth" = [])),
    summary = "Delete a request"
)]
#[axum::debug_handler]
pub async fn cancel_request(
    claims: ClaimsExtractor,
    State(app_state): State<Arc<AppState>>,
    Path(request_id): Path<Uuid>,
) -> Result<Json<CancelRequestResponse>> {
    app_state
        .table_request_service
        .cancel_request(request_id, claims.get_user_id())
        .await?;

    Ok(Json(CancelRequestResponse {
        message: format!("Request {} cancelled successfully", request_id),
    }))
}

pub fn table_request_routes(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .nest(
            "/requests",
            OpenApiRouter::new()
                .routes(routes!(accept_request))
                .routes(routes!(reject_request))
                .routes(routes!(cancel_request)),
        )
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
