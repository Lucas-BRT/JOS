use crate::http::dtos::*;
use crate::http::middleware::auth::ClaimsExtractor;
use axum::{extract::*, routing::*};
use infrastructure::state::AppState;
use shared::Result;
use shared::error::Error;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

#[utoipa::path(
    get,
    path = "/v1/requests/sent",
    tag = "table-requests",
    security(("auth" = [])),
    responses(
        (status = 200, description = "Sent requests retrieved successfully", body = Vec<SentRequestItem>),
        (status = 401, description = "Authentication required", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn get_sent_requests(
    _claims: ClaimsExtractor,
    State(_app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<SentRequestItem>>> {
    // TODO: Implement sent requests retrieval
    // For now, return empty list
    Ok(Json(vec![]))
}

#[utoipa::path(
    get,
    path = "/v1/requests/received",
    tag = "table-requests",
    security(("auth" = [])),
    responses(
        (status = 200, description = "Received requests retrieved successfully", body = Vec<ReceivedRequestItem>),
        (status = 401, description = "Authentication required", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn get_received_requests(
    _claims: ClaimsExtractor,
    State(_app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<ReceivedRequestItem>>> {
    // TODO: Implement received requests retrieval
    // For now, return empty list
    Ok(Json(vec![]))
}

#[utoipa::path(
    post,
    path = "/v1/tables/{id}/requests",
    tag = "table-requests",
    security(("auth" = [])),
    params(
        ("id" = Uuid, Path, description = "Table ID")
    ),
    request_body = CreateTableRequestRequest,
    responses(
        (status = 201, description = "Request created successfully", body = TableRequestResponse),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse),
        (status = 404, description = "Table not found", body = ErrorResponse),
        (status = 409, description = "Request already exists", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn create_table_request(
    claims: ClaimsExtractor,
    State(_app_state): State<Arc<AppState>>,
    Path(table_id): Path<Uuid>,
    Json(payload): Json<CreateTableRequestRequest>,
) -> Result<Json<TableRequestResponse>> {
    if let Err(validation_error) = payload.validate() {
        return Err(Error::Validation(validation_error));
    }

    // TODO: Implement table request creation logic
    // For now, return a placeholder response
    Ok(Json(TableRequestResponse {
        id: Uuid::new_v4(),
        table_id,
        player_id: claims.0.sub,
        message: payload.message,
        status: "pending".to_string(),
        request_date: chrono::Utc::now(),
    }))
}

#[utoipa::path(
    post,
    path = "/v1/requests/{id}/accept",
    tag = "table-requests",
    security(("auth" = [])),
    params(
        ("id" = Uuid, Path, description = "Request ID")
    ),
    responses(
        (status = 200, description = "Request accepted successfully", body = AcceptRequestResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse),
        (status = 403, description = "Not authorized to accept this request", body = ErrorResponse),
        (status = 404, description = "Request not found", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn accept_request(
    _claims: ClaimsExtractor,
    State(_app_state): State<Arc<AppState>>,
    Path(request_id): Path<Uuid>,
) -> Result<Json<AcceptRequestResponse>> {
    // TODO: Implement request acceptance logic
    Ok(Json(AcceptRequestResponse {
        message: format!("Request {} accepted successfully", request_id),
    }))
}

#[utoipa::path(
    post,
    path = "/v1/requests/{id}/reject",
    tag = "table-requests",
    security(("auth" = [])),
    params(
        ("id" = Uuid, Path, description = "Request ID")
    ),
    responses(
        (status = 200, description = "Request rejected successfully", body = RejectRequestResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse),
        (status = 403, description = "Not authorized to reject this request", body = ErrorResponse),
        (status = 404, description = "Request not found", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn reject_request(
    _claims: ClaimsExtractor,
    State(_app_state): State<Arc<AppState>>,
    Path(request_id): Path<Uuid>,
) -> Result<Json<RejectRequestResponse>> {
    // TODO: Implement request rejection logic
    Ok(Json(RejectRequestResponse {
        message: format!("Request {} rejected successfully", request_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/v1/requests/{id}",
    tag = "table-requests",
    security(("auth" = [])),
    params(
        ("id" = Uuid, Path, description = "Request ID")
    ),
    responses(
        (status = 200, description = "Request cancelled successfully", body = CancelRequestResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse),
        (status = 403, description = "Not authorized to cancel this request", body = ErrorResponse),
        (status = 404, description = "Request not found", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn cancel_request(
    _claims: ClaimsExtractor,
    State(_app_state): State<Arc<AppState>>,
    Path(request_id): Path<Uuid>,
) -> Result<Json<CancelRequestResponse>> {
    // TODO: Implement request cancellation logic
    Ok(Json(CancelRequestResponse {
        message: format!("Request {} cancelled successfully", request_id),
    }))
}

pub fn table_request_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .nest(
            "/requests",
            Router::new()
                .route("/sent", get(get_sent_requests))
                .route("/received", get(get_received_requests))
                .route("/{id}/accept", post(accept_request))
                .route("/{id}/reject", post(reject_request))
                .route("/{id}", delete(cancel_request)),
        )
        .with_state(state)
}
