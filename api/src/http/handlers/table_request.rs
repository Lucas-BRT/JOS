use crate::http::dtos::table_request::CreateTableRequestRequest;
use crate::http::dtos::*;
use crate::http::middleware::auth::{ClaimsExtractor, auth_middleware};
use axum::extract::*;
use axum::middleware::from_fn_with_state;
use domain::entities::{
    CreateTableMemberCommand, CreateTableRequestCommand, TableRequestStatus, Update,
    UpdateTableRequestCommand,
};
use infrastructure::state::AppState;
use shared::Result;
use shared::error::{DomainError, Error};
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/sent",
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
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<SentRequestItem>>> {
    let requests = app_state
        .table_request_service
        .find_by_user_id(&claims.0.sub)
        .await?;

    let requests = requests
        .into_iter()
        .filter(|request| request.user_id == claims.0.sub)
        .map(SentRequestItem::from)
        .collect::<Vec<SentRequestItem>>();

    Ok(Json(requests))
}

#[utoipa::path(
    post,
    path = "/{id}/accept",
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
    let requester_id = claims.0.sub;

    let session = app_state.session_service.find_by_id(&request_id).await?;
    let table = app_state
        .table_service
        .find_by_id(&session.table_id)
        .await?;

    if table.gm_id != requester_id {
        return Err(Error::Domain(DomainError::BusinessRuleViolation {
            message: "invalid credentials".to_owned(),
        }));
    }

    let requested_member_id = app_state
        .table_request_service
        .find_by_id(&request_id)
        .await?
        .user_id;

    let command = CreateTableMemberCommand {
        table_id: table.id,
        user_id: requested_member_id,
    };

    app_state.table_member_service.create(command).await?;

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
    path = "/{id}/reject",
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
    path = "/{id}",
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

#[utoipa::path(
    post,
    path = "/tables/{table_id}/requests",
    tag = "table-requests",
    security(("auth" = [])),
    request_body = CreateTableRequestRequest,
    responses(
        (status = 200, description = "Table request created successfully", body = CreateTableRequestResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse),
        (status = 404, description = "Table not found", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
async fn create_request(
    claims: ClaimsExtractor,
    State(app_state): State<Arc<AppState>>,
    Path(table_id): Path<Uuid>,
    Json(payload): Json<CreateTableRequestRequest>,
) -> Result<Json<CreateTableRequestResponse>> {
    let requester_id = claims.0.sub;

    let command = CreateTableRequestCommand {
        table_id,
        user_id: requester_id,
        message: payload.message,
    };

    let table_request = app_state.table_request_service.create(command).await?;

    Ok(Json(CreateTableRequestResponse {
        id: table_request.id,
    }))
}

pub fn table_request_routes(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .nest(
            "/requests",
            OpenApiRouter::new()
                .routes(routes!(get_sent_requests))
                .routes(routes!(accept_request))
                .routes(routes!(reject_request))
                .routes(routes!(cancel_request))
                .routes(routes!(create_request)),
        )
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
