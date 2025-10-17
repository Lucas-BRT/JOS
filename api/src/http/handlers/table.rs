use crate::http::dtos::*;
use crate::http::middleware::auth::{ClaimsExtractor, auth_middleware};
use axum::http::StatusCode;
use axum::{extract::*, routing::*};
use domain::entities::commands::table_commands::*;
use domain::entities::*;
use infrastructure::state::AppState;
use shared::Result;
use shared::error::{ApplicationError, Error};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

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
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CreateTableRequest>,
) -> Result<(StatusCode, Json<CreateTableResponse>)> {
    if let Err(validation_error) = payload.validate() {
        return Err(Error::Validation(validation_error));
    }

    let command = CreateTableCommand {
        title: payload.title,
        description: payload.description,
        slots: payload.max_players as u32,
        game_system_id: payload.system_id,
        gm_id: claims.0.sub,
    };

    let table = app_state.table_service.create(&command).await?;

    let response = CreateTableResponse { id: table.id };

    Ok((StatusCode::CREATED, Json(response)))
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
    State(app_state): State<Arc<AppState>>,
    Query(search): Query<SearchTablesQuery>,
) -> Result<(StatusCode, Json<Vec<Table>>)> {
    let result = match search.search {
        Some(search_term) if !search_term.is_empty() => {
            // TODO: Implement search with filters
            app_state.table_service.get_all().await?
        }
        _ => app_state.table_service.get_all().await?,
    };

    Ok((StatusCode::OK, Json(result)))
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
    State(app_state): State<Arc<AppState>>,
    Path(table_id): Path<Uuid>,
) -> Result<Json<TableDetails>> {
    let table = app_state.table_service.find_by_id(&table_id).await?;
    let game_master = app_state.user_service.find_by_id(&table.gm_id).await?;

    // This is a placeholder for now
    let game_system_name = "D&D 5e".to_string();

    let response = TableDetails {
        id: table.id,
        title: table.title,
        game_system: game_system_name,
        game_master: GameMasterInfo {
            id: game_master.id,
            username: game_master.username,
        },
        description: table.description,
        player_slots: table.player_slots as i32,
        players: vec![],              // TODO: Implement player retrieval
        status: "active".to_string(), // TODO: Map from table status
        sessions: vec![],             // TODO: Implement session retrieval
    };

    Ok(Json(response))
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
    State(app_state): State<Arc<AppState>>,
    Path(table_id): Path<Uuid>,
    Json(payload): Json<UpdateTableRequest>,
) -> Result<Json<TableDetails>> {
    if let Err(validation_error) = payload.validate() {
        return Err(Error::Validation(validation_error));
    }

    let table = app_state.table_service.find_by_id(&table_id).await?;
    if table.gm_id != claims.0.sub {
        return Err(Error::Application(ApplicationError::IncorrectPassword));
    }

    let command = UpdateTableCommand {
        id: table_id,
        title: payload.title.into(),
        description: payload.description.into(),
        slots: payload.max_players.map(|s| s as u32).into(),
        game_system_id: payload
            .system
            .map(|s| Uuid::parse_str(&s).unwrap_or_default())
            .into(),
    };

    app_state.table_service.update(&command).await?;

    let updated_table = app_state.table_service.find_by_id(&table_id).await?;
    let game_master = app_state
        .user_service
        .find_by_id(&updated_table.gm_id)
        .await?;
    let game_system_name = "D&D 5e".to_string(); // Placeholder

    let response = TableDetails {
        id: updated_table.id,
        title: updated_table.title,
        game_system: game_system_name,
        game_master: GameMasterInfo {
            id: game_master.id,
            username: game_master.username,
        },
        description: updated_table.description,
        player_slots: updated_table.player_slots as i32,
        players: vec![],
        status: "active".to_string(),
        sessions: vec![],
    };

    Ok(Json(response))
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
    claims: ClaimsExtractor,
    State(app_state): State<Arc<AppState>>,
    Path(table_id): Path<Uuid>,
) -> Result<Json<DeleteTableResponse>> {
    let command = DeleteTableCommand {
        id: table_id,
        gm_id: claims.0.sub,
    };

    app_state.table_service.delete(&command).await?;

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
                .route("/{id}", delete(delete_table))
                .layer(axum::middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                )),
        )
        .with_state(state)
}
