use crate::http::dtos::*;
use crate::http::middleware::auth::{ClaimsExtractor, auth_middleware};
use axum::extract::*;
use axum::middleware::from_fn_with_state;
use domain::entities::{
    CreateTableMemberCommand, TableRequestStatus, Update, UpdateTableRequestCommand,
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
    tags = ["Requests"],
    security(("auth" = [])),
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
    path = "/{request_id}/accept",
    tags = ["Requests"],
    summary = "Accepts a player request",
    security(("auth" = [])),
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
    path = "/{request_id}/reject",
    tags = ["Requests"],
    security(("auth" = [])),
    summary = "Rejects a player request"
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
    path = "/{request_id}",
    tags = ["Requests"],
    security(("auth" = [])),
    summary = "Delete a request"
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
