use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{delete, get, post, put},
};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::http::dtos::*;
use crate::http::middleware::auth::ClaimsExtractor;
use infrastructure::state::AppState;
use shared::Result;
use shared::error::Error;

#[utoipa::path(
    post,
    path = "/v1/tables",
    tag = "tables",
    request_body = CreateTableRequest,
    responses(
        (status = 201, description = "Table created successfully", body = TableDetails),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn create_table(
    claims: ClaimsExtractor,
    State(_app_state): State<Arc<AppState>>,
    Json(payload): Json<CreateTableRequest>,
) -> Result<Json<TableDetails>> {
    if let Err(validation_error) = payload.validate() {
        return Err(Error::Validation(
            shared::error::ValidationError::ValidationFailed(validation_error.to_string()),
        ));
    }

    // TODO: Implement table creation logic
    // For now, return a placeholder response
    Ok(Json(TableDetails {
        id: Uuid::new_v4(),
        title: payload.title,
        game_system: payload.system,
        game_master: GameMasterInfo {
            id: claims.0.sub,
            username: "placeholder".to_string(),
        },
        description: payload.description,
        player_slots: payload.max_players,
        players: vec![],
        status: "active".to_string(),
        sessions: vec![],
    }))
}

#[utoipa::path(
    get,
    path = "/v1/tables",
    tag = "tables",
    responses(
        (status = 200, description = "Tables retrieved successfully", body = Vec<TableListItem>),
        (status = 401, description = "Authentication required", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn get_tables(
    _claims: ClaimsExtractor,
    State(_app_state): State<Arc<AppState>>,
    Query(_search): Query<SearchTablesQuery>,
) -> Result<Json<Vec<TableListItem>>> {
    // TODO: Implement table listing logic with search functionality
    // For now, return empty list
    Ok(Json(vec![]))
}

#[utoipa::path(
    get,
    path = "/v1/tables/{id}",
    tag = "tables",
    params(
        ("id" = Uuid, Path, description = "Table ID")
    ),
    responses(
        (status = 200, description = "Table details retrieved", body = TableDetails),
        (status = 404, description = "Table not found", body = ErrorResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn get_table_details(
    _claims: ClaimsExtractor,
    State(_app_state): State<Arc<AppState>>,
    Path(table_id): Path<Uuid>,
) -> Result<Json<TableDetails>> {
    // TODO: Implement table details retrieval
    // For now, return a placeholder response
    Ok(Json(TableDetails {
        id: table_id,
        title: "Placeholder Table".to_string(),
        game_system: "D&D 5e".to_string(),
        game_master: GameMasterInfo {
            id: Uuid::new_v4(),
            username: "placeholder".to_string(),
        },
        description: "Placeholder description".to_string(),
        player_slots: 4,
        players: vec![],
        status: "active".to_string(),
        sessions: vec![],
    }))
}

#[utoipa::path(
    put,
    path = "/v1/tables/{id}",
    tag = "tables",
    params(
        ("id" = Uuid, Path, description = "Table ID")
    ),
    request_body = UpdateTableRequest,
    responses(
        (status = 200, description = "Table updated successfully", body = TableDetails),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse),
        (status = 403, description = "Not authorized to update this table", body = ErrorResponse),
        (status = 404, description = "Table not found", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn update_table(
    claims: ClaimsExtractor,
    State(_app_state): State<Arc<AppState>>,
    Path(table_id): Path<Uuid>,
    Json(payload): Json<UpdateTableRequest>,
) -> Result<Json<TableDetails>> {
    if let Err(validation_error) = payload.validate() {
        return Err(Error::Validation(
            shared::error::ValidationError::ValidationFailed(validation_error.to_string()),
        ));
    }

    // TODO: Implement table update logic
    // For now, return a placeholder response
    Ok(Json(TableDetails {
        id: table_id,
        title: payload.title.unwrap_or("Updated Table".to_string()),
        game_system: payload.system.unwrap_or("D&D 5e".to_string()),
        game_master: GameMasterInfo {
            id: claims.0.sub,
            username: "placeholder".to_string(),
        },
        description: payload
            .description
            .unwrap_or("Updated description".to_string()),
        player_slots: payload.max_players.unwrap_or(4),
        players: vec![],
        status: payload.status.unwrap_or("active".to_string()),
        sessions: vec![],
    }))
}

#[utoipa::path(
    delete,
    path = "/v1/tables/{id}",
    tag = "tables",
    params(
        ("id" = Uuid, Path, description = "Table ID")
    ),
    responses(
        (status = 200, description = "Table deleted successfully", body = DeleteTableResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse),
        (status = 403, description = "Not authorized to delete this table", body = ErrorResponse),
        (status = 404, description = "Table not found", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn delete_table(
    _claims: ClaimsExtractor,
    State(_app_state): State<Arc<AppState>>,
    Path(table_id): Path<Uuid>,
) -> Result<Json<DeleteTableResponse>> {
    // TODO: Implement table deletion logic
    Ok(Json(DeleteTableResponse {
        message: format!("Table {} deleted successfully", table_id),
    }))
}

pub fn table_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .nest(
            "/tables",
            Router::new()
                .route("/", get(get_tables))
                .route("/", post(create_table))
                .route("/{id}", get(get_table_details))
                .route("/{id}", put(update_table))
                .route("/{id}", delete(delete_table)),
        )
        .with_state(state)
}
