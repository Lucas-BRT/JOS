use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{delete, get, patch, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/v1/table-requests",
    tag = "table-requests",
    security(
        ("bearer_auth" = [])
    ),
    request_body = CreateTableRequestDto,
    responses(
        (status = 201, description = "Request created successfully", body = String),
        (status = 400, description = "Bad request", body = Value),
        (status = 401, description = "Unauthorized", body = Value)
    )
)]
#[axum::debug_handler]
pub async fn create_table_request(
    user: Claims,
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CreateTableRequestDto>,
) -> Result<Json<String>> {
    let mut request = CreateTableRequestCommand::from_dto(payload, user.sub);
    let request = app_state.table_request_service.create(&mut request).await?;

    Ok(Json(request.id.to_string()))
}

#[utoipa::path(
    get,
    path = "/v1/table-requests",
    tag = "table-requests",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "List of table requests", body = Vec<TableRequestResponse>),
        (status = 401, description = "Unauthorized", body = Value)
    )
)]
#[axum::debug_handler]
pub async fn get_table_requests(
    _: Claims,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<TableRequestResponse>>> {
    let requests = app_state
        .table_request_service
        .get(&GetTableRequestCommand::default())
        .await?;
    let requests = requests.iter().map(TableRequestResponse::from).collect();

    Ok(Json(requests))
}

#[utoipa::path(
    get,
    path = "/v1/table-requests/table/{id}",
    tag = "table-requests",
    params(
        ("id" = Uuid, Path, description = "Table ID")
    ),
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "List of table requests of a table", body = Vec<TableRequestResponse>),
        (status = 401, description = "Unauthorized", body = Value)
    )
)]
#[axum::debug_handler]
pub async fn get_table_requests_by_table_id(
    user: Claims,
    State(app_state): State<Arc<AppState>>,
    Path(table_id): Path<Uuid>,
) -> Result<Json<Vec<TableRequestResponse>>> {
    let requests = app_state
        .table_request_service
        .get_requests_by_table_id(&table_id, &user.sub)
        .await?
        .iter()
        .map(TableRequestResponse::from)
        .collect();

    Ok(Json(requests))
}

#[utoipa::path(
    patch,
    path = "/v1/table-requests/{id}",
    tag = "table-requests",
    params(
        ("id" = Uuid, Path, description = "Table request ID")
    ),
    request_body = UpdateTableRequestDto,
    responses(
        (status = 200, description = "Table request updated successfully", body = ()),
        (status = 400, description = "Bad request", body = Value),
        (status = 404, description = "Table request not found", body = Value)
    )
)]
#[axum::debug_handler]
pub async fn update_table_request(
    State(app_state): State<Arc<AppState>>,
    Path(request_id): Path<Uuid>,
    Json(update_payload): Json<UpdateTableRequestDto>,
) -> Result<()> {
    let update_command = UpdateTableRequestCommand {
        id: request_id,
        status: update_payload.status,
        message: update_payload.message,
    };

    app_state
        .table_request_service
        .update(&update_command)
        .await?;

    Ok(())
}

#[utoipa::path(
    delete,
    path = "/v1/table-requests/{id}",
    tag = "table-requests",
    params(
        ("id" = Uuid, Path, description = "Table request ID")
    ),
    responses(
        (status = 200, description = "Table request deleted successfully", body = ()),
        (status = 404, description = "Table request not found", body = Value)
    )
)]
#[axum::debug_handler]
pub async fn delete_table_request(
    State(app_state): State<Arc<AppState>>,
    Path(request_id): Path<Uuid>,
    user: Claims,
) -> Result<()> {
    let command = DeleteTableRequestCommand {
        id: request_id,
        gm_id: user.sub,
    };
    app_state.table_request_service.delete(&command).await?;

    Ok(())
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RequestFilters {
    pub r#type: Option<String>,
    pub status: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RequestListResponse {
    pub requests: Vec<TableRequestResponse>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AcceptRequestResponse {
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RejectRequestResponse {
    pub message: String,
}

#[utoipa::path(
    get,
    path = "/v1/requests",
    tag = "requests",
    params(
        ("type" = Option<String>, Query, description = "Filter by type: sent or received"),
        ("status" = Option<String>, Query, description = "Filter by status"),
        ("page" = Option<u32>, Query, description = "Page number"),
        ("limit" = Option<u32>, Query, description = "Items per page")
    ),
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "List of requests", body = RequestListResponse),
        (status = 401, description = "Unauthorized", body = serde_json::Value)
    )
)]
pub async fn get_requests(
    State(app_state): State<Arc<AppState>>,
    Query(filters): Query<RequestFilters>,
    claims: Claims,
) -> Result<Json<RequestListResponse>> {
    let requests = app_state
        .table_request_service
        .get_requests(&filters, &claims.user_id)
        .await?;
    Ok(Json(requests))
}

#[utoipa::path(
    post,
    path = "/v1/tables/{id}/requests",
    tag = "requests",
    params(
        ("id" = Uuid, Path, description = "Table ID")
    ),
    security(
        ("bearer_auth" = [])
    ),
    request_body = CreateTableRequestDto,
    responses(
        (status = 201, description = "Request created successfully", body = String),
        (status = 400, description = "Bad request", body = serde_json::Value),
        (status = 401, description = "Unauthorized", body = serde_json::Value)
    )
)]
pub async fn create_table_request_by_table_id(
    State(app_state): State<Arc<AppState>>,
    Path(table_id): Path<Uuid>,
    Json(payload): Json<CreateTableRequestDto>,
    claims: Claims,
) -> Result<Json<String>> {
    let mut request = CreateTableRequestCommand {
        user_id: claims.user_id,
        table_id,
        message: payload.message,
    };
    let request = app_state.table_request_service.create(&mut request).await?;
    Ok(Json(request.id.to_string()))
}

#[utoipa::path(
    post,
    path = "/v1/requests/{id}/accept",
    tag = "requests",
    params(
        ("id" = Uuid, Path, description = "Request ID")
    ),
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Request accepted successfully", body = AcceptRequestResponse),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 403, description = "Forbidden", body = serde_json::Value)
    )
)]
pub async fn accept_request(
    State(app_state): State<Arc<AppState>>,
    Path(request_id): Path<Uuid>,
    claims: Claims,
) -> Result<Json<AcceptRequestResponse>> {
    app_state
        .table_request_service
        .accept_request(&request_id, &claims.user_id)
        .await?;
    Ok(Json(AcceptRequestResponse {
        message: "Request accepted successfully".to_string(),
    }))
}

#[utoipa::path(
    post,
    path = "/v1/requests/{id}/reject",
    tag = "requests",
    params(
        ("id" = Uuid, Path, description = "Request ID")
    ),
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Request rejected successfully", body = RejectRequestResponse),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 403, description = "Forbidden", body = serde_json::Value)
    )
)]
pub async fn reject_request(
    State(app_state): State<Arc<AppState>>,
    Path(request_id): Path<Uuid>,
    claims: Claims,
) -> Result<Json<RejectRequestResponse>> {
    app_state
        .table_request_service
        .reject_request(&request_id, &claims.user_id)
        .await?;
    Ok(Json(RejectRequestResponse {
        message: "Request rejected successfully".to_string(),
    }))
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(get_table_requests))
        .route("/", post(create_table_request))
        .route("/{id}", patch(update_table_request))
        .route("/{id}", delete(delete_table_request))
        .route("/requests", get(get_requests))
        .route(
            "/tables/{id}/requests",
            post(create_table_request_by_table_id),
        )
        .route("/requests/{id}/accept", post(accept_request))
        .route("/requests/{id}/reject", post(reject_request))
        .with_state(state.clone())
}
