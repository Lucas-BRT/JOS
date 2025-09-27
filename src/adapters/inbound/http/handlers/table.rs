use crate::Result;
use axum::extract::Query;
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, post, put},
};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/v1/tables",
    tag = "tables",
    security(
        ("bearer_auth" = [])
    ),
    request_body = CreateTableDto,
    responses(
        (status = 201, description = "Table created successfully", body = String),
        (status = 400, description = "Bad request", body = Value),
        (status = 401, description = "Unauthorized", body = Value)
    )
)]
#[axum::debug_handler]
pub async fn create_table(
    user: Claims,
    State(app_state): State<Arc<AppState>>,
    Json(new_table_payload): Json<CreateTableDto>,
) -> Result<Json<String>> {
    let mut table = CreateTableCommand::from_dto(new_table_payload, user.sub);
    let table_id = app_state.table_service.create(&mut table).await?.id;

    Ok(Json(table_id.to_string()))
}

#[utoipa::path(
    get,
    path = "/v1/tables",
    tag = "tables",
    responses(
        (status = 200, description = "List of available tables", body = Vec<AvaliableTableResponse>)
    )
)]
#[axum::debug_handler]
pub async fn get_available_tables(
    _: Claims,
    State(app_state): State<Arc<AppState>>,
    Query(filters): Query<TableFilters>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<AvaliableTableResponse>>> {
    let tables = app_state
        .table_service
        .get(&GetTableCommand::new(filters, pagination))
        .await?;

    let tables = tables.iter().map(AvaliableTableResponse::from).collect();

    Ok(Json(tables))
}

#[utoipa::path(
    params(
        ("id" = Uuid, Path, description = "Table ID")
    ),
    delete,
    path = "/v1/tables/{id}",
    tag = "tables",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Table deleted successfully", body = ()),
        (status = 400, description = "Bad request", body = Value),
        (status = 401, description = "Unauthorized", body = Value)
    )
)]
#[axum::debug_handler]
pub async fn delete_table(
    user: Claims,
    State(app_state): State<Arc<AppState>>,
    Path(table_id): Path<Uuid>,
) -> Result<Json<()>> {
    let command = DeleteTableCommand {
        id: table_id,
        gm_id: user.sub,
    };

    app_state.table_service.delete(&command).await?;

    Ok(Json(()))
}

#[utoipa::path(
    get,
    path = "/v1/tables/{id}",
    tag = "tables",
    params(
        ("id" = Uuid, Path, description = "Table ID")
    ),
    responses(
        (status = 200, description = "Table details", body = AvaliableTableResponse),
        (status = 404, description = "Table not found", body = serde_json::Value)
    )
)]
pub async fn get_table_by_id(
    State(app_state): State<Arc<AppState>>,
    Path(table_id): Path<Uuid>,
) -> Result<Json<AvaliableTableResponse>> {
    let table = app_state.table_service.get_by_id(&table_id).await?;
    Ok(Json(table.into()))
}

#[utoipa::path(
    put,
    path = "/v1/tables/{id}",
    tag = "tables",
    params(
        ("id" = Uuid, Path, description = "Table ID")
    ),
    security(
        ("bearer_auth" = [])
    ),
    request_body = UpdateTableDto,
    responses(
        (status = 200, description = "Table updated successfully", body = AvaliableTableResponse),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 403, description = "Forbidden", body = serde_json::Value)
    )
)]
pub async fn update_table(
    State(app_state): State<Arc<AppState>>,
    Path(table_id): Path<Uuid>,
    Json(update_payload): Json<UpdateTableDto>,
    claims: Claims,
) -> Result<Json<AvaliableTableResponse>> {
    if let Err(validation_error) = update_payload.validate() {
        return Err(crate::Error::Validation(validation_error));
    }

    let table = app_state
        .table_service
        .update_table(&table_id, &claims.user_id, &update_payload.into())
        .await?;
    Ok(Json(table.into()))
}

#[utoipa::path(
    get,
    path = "/v1/tables/my-tables",
    tag = "tables",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "User's tables", body = Vec<AvaliableTableResponse>)
    )
)]
pub async fn get_my_tables(
    State(app_state): State<Arc<AppState>>,
    claims: Claims,
) -> Result<Json<Vec<AvaliableTableResponse>>> {
    let tables = app_state
        .table_service
        .get_user_tables(&claims.user_id)
        .await?;
    let tables = tables.iter().map(AvaliableTableResponse::from).collect();
    Ok(Json(tables))
}

#[utoipa::path(
    get,
    path = "/v1/tables/{id}/players",
    tag = "tables",
    params(
        ("id" = Uuid, Path, description = "Table ID")
    ),
    responses(
        (status = 200, description = "Table players", body = Vec<serde_json::Value>)
    )
)]
pub async fn get_table_players(
    State(app_state): State<Arc<AppState>>,
    Path(table_id): Path<Uuid>,
) -> Result<Json<Vec<serde_json::Value>>> {
    let players = app_state.table_service.get_table_players(&table_id).await?;
    Ok(Json(players))
}

#[utoipa::path(
    post,
    path = "/v1/tables/{id}/join",
    tag = "tables",
    params(
        ("id" = Uuid, Path, description = "Table ID")
    ),
    security(
        ("bearer_auth" = [])
    ),
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Joined table successfully"),
        (status = 401, description = "Unauthorized", body = serde_json::Value)
    )
)]
pub async fn join_table(
    State(app_state): State<Arc<AppState>>,
    Path(table_id): Path<Uuid>,
    Json(payload): Json<serde_json::Value>,
    claims: Claims,
) -> Result<Json<serde_json::Value>> {
    let message = payload
        .get("message")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    app_state
        .table_service
        .join_table(&table_id, &claims.user_id, message)
        .await?;
    Ok(Json(
        serde_json::json!({"message": "Joined table successfully"}),
    ))
}

#[utoipa::path(
    delete,
    path = "/v1/tables/{id}/leave",
    tag = "tables",
    params(
        ("id" = Uuid, Path, description = "Table ID")
    ),
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Left table successfully"),
        (status = 401, description = "Unauthorized", body = serde_json::Value)
    )
)]
pub async fn leave_table(
    State(app_state): State<Arc<AppState>>,
    Path(table_id): Path<Uuid>,
    claims: Claims,
) -> Result<Json<serde_json::Value>> {
    app_state
        .table_service
        .leave_table(&table_id, &claims.user_id)
        .await?;
    Ok(Json(
        serde_json::json!({"message": "Left table successfully"}),
    ))
}

#[utoipa::path(
    delete,
    path = "/v1/tables/{id}/players/{player_id}",
    tag = "tables",
    params(
        ("id" = Uuid, Path, description = "Table ID"),
        ("player_id" = Uuid, Path, description = "Player ID")
    ),
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Player removed successfully"),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 403, description = "Forbidden", body = serde_json::Value)
    )
)]
pub async fn remove_player(
    State(app_state): State<Arc<AppState>>,
    Path((table_id, player_id)): Path<(Uuid, Uuid)>,
    claims: Claims,
) -> Result<Json<serde_json::Value>> {
    app_state
        .table_service
        .remove_player(&table_id, &player_id, &claims.user_id)
        .await?;
    Ok(Json(
        serde_json::json!({"message": "Player removed successfully"}),
    ))
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(get_available_tables))
        .route("/", post(create_table))
        .route("/my-tables", get(get_my_tables))
        .route("/{id}", get(get_table_by_id))
        .route("/{id}", put(update_table))
        .route("/{id}", delete(delete_table))
        .route("/{id}/players", get(get_table_players))
        .route("/{id}/join", post(join_table))
        .route("/{id}/leave", delete(leave_table))
        .route("/{id}/players/{player_id}", delete(remove_player))
        .with_state(state.clone())
}
