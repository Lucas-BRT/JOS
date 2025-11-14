use crate::http::dtos::*;
use crate::http::middleware::auth::ClaimsExtractor;
use axum::{extract::*, routing::*};
use domain::entities::{
    CreateTableRequestCommand, TableRequestStatus, Update, UpdateTableRequestCommand,
};
use infrastructure::state::AppState;
use shared::Result;
use shared::error::{DomainError, Error};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

#[utoipa::path(
    get,
    path = "/v1/tables/{id}/requests/sent",
    tag = "table-requests",
    security(("auth" = [])),
    responses(
        (status = 200, description = "Sent requests retrieved successfully", body = Vec<SentRequestItem>),
        (status = 401, description = "Authentication required", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn get_sent_requests(
    claims: ClaimsExtractor,
    Path(table_id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<SentRequestItem>>> {
    let requests = app_state
        .table_request_service
        .find_by_table_id(&table_id)
        .await?;

    let requests = requests
        .into_iter()
        .filter(|request| request.user_id == claims.0.sub)
        .map(SentRequestItem::from)
        .collect::<Vec<SentRequestItem>>();

    Ok(Json(requests))
}

#[utoipa::path(
    get,
    path = "/v1/tables/{id}/requests/received",
    tag = "table-requests",
    security(("auth" = [])),
    responses(
        (status = 200, description = "Received requests retrieved successfully", body = Vec<ReceivedRequestItem>),
        (status = 401, description = "Authentication required", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn get_received_requests(
    claims: ClaimsExtractor,
    Path(table_id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<ReceivedRequestItem>>> {
    let table = app_state.table_service.find_by_id(&table_id).await?;

    if table.gm_id != claims.0.sub {
        return Err(Error::Domain(DomainError::BusinessRuleViolation {
            message: "invalid credentials".to_owned(),
        }));
    }

    let requests = app_state
        .table_request_service
        .find_by_table_id(&table_id)
        .await?;

    let requests = requests
        .into_iter()
        .map(|request| ReceivedRequestItem {
            id: request.id,
            player_id: request.user_id,
            table_id: request.table_id,
            request_date: request.created_at,
            message: request.message,
        })
        .collect::<Vec<ReceivedRequestItem>>();
    Ok(Json(requests))
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
    State(app_state): State<Arc<AppState>>,
    Path(table_id): Path<Uuid>,
    Json(payload): Json<CreateTableRequestRequest>,
) -> Result<Json<TableRequestResponse>> {
    if let Err(validation_error) = payload.validate() {
        return Err(Error::Validation(validation_error));
    }

    let command = CreateTableRequestCommand {
        user_id: claims.0.sub,
        table_id,
        message: Some(payload.message.clone()),
    };

    let request = app_state.table_request_service.create(command).await?;

    Ok(Json(TableRequestResponse::from(request)))
}

#[utoipa::path(
    post,
    path = "/v1/requests/{id}/accept",
    tag = "table-requests",
    security(("auth" = [])),
    params(
        ("table_id" = Uuid, Path, description = "Table ID")
    ),
    responses(
        (status = 200, description = "Requests for the table retrieved successfully", body = Vec<TableRequestResponse>),
        (status = 401, description = "Authentication required", body = ErrorResponse),
        (status = 403, description = "Not authorized to view requests for this table", body = ErrorResponse),
        (status = 404, description = "Table not found", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn accept_request(
    claims: ClaimsExtractor,
    State(app_state): State<Arc<AppState>>,
    Path(request_id): Path<Uuid>,
) -> Result<Json<AcceptRequestResponse>> {
    let session = app_state.session_service.find_by_id(&request_id).await?;
    let table = app_state
        .table_service
        .find_by_id(&session.table_id)
        .await?;

    if table.gm_id != claims.0.sub {
        return Err(Error::Domain(DomainError::BusinessRuleViolation {
            message: "invalid credentials".to_owned(),
        }));
    }

    let command = UpdateTableRequestCommand {
        id: request_id,
        status: Update::Change(TableRequestStatus::Approved),
        message: Update::Keep,
    };

    app_state.table_request_service.update(command).await?;

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
    claims: ClaimsExtractor,
    State(app_state): State<Arc<AppState>>,
    Path(request_id): Path<Uuid>,
) -> Result<Json<RejectRequestResponse>> {
    let session = app_state.session_service.find_by_id(&request_id).await?;
    let table = app_state
        .table_service
        .find_by_id(&session.table_id)
        .await?;

    if table.gm_id != claims.0.sub {
        return Err(Error::Domain(DomainError::BusinessRuleViolation {
            message: "invalid credentials".to_owned(),
        }));
    }

    let command = UpdateTableRequestCommand {
        id: request_id,
        status: Update::Change(TableRequestStatus::Rejected),
        message: Update::Keep,
    };

    app_state.table_request_service.update(command).await?;

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
    claims: ClaimsExtractor,
    State(app_state): State<Arc<AppState>>,
    Path(request_id): Path<Uuid>,
) -> Result<Json<CancelRequestResponse>> {
    let session = app_state.session_service.find_by_id(&request_id).await?;
    let table = app_state
        .table_service
        .find_by_id(&session.table_id)
        .await?;

    if table.gm_id != claims.0.sub {
        return Err(Error::Domain(DomainError::BusinessRuleViolation {
            message: "invalid credentials".to_owned(),
        }));
    }

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
