use crate::http::dtos::*;
use crate::http::middleware::auth::{ClaimsExtractor, auth_middleware};
use axum::http::StatusCode;
use axum::{extract::*, routing::*};
use domain::entities::commands::session_commands::*;
use domain::entities::commands::table_commands::*;
use domain::entities::*;
use infrastructure::state::AppState;
use shared::Result;
use shared::error::{ApplicationError, Error};
use std::str::FromStr;
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
    let response = app_state.table_service.get(&search.into()).await?;

    Ok((StatusCode::OK, Json(response)))
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
    let game_system = app_state
        .game_system_service
        .find_by_id(table.game_system_id)
        .await?;
    let table_members = app_state
        .table_member_service
        .find_by_table_id(&table.id)
        .await?;

    let mut players: Vec<PlayerInfo> = Vec::new();
    for tm in table_members {
        if let Some(user) = app_state.user_service.find_by_id(&tm.user_id).await.ok() {
            players.push(PlayerInfo {
                id: user.id,
                username: user.username,
            });
        }
    }

    let sessions = app_state
        .session_service
        .get(GetSessionCommand {
            table_id: Some(table.id),
            ..Default::default()
        })
        .await?;

    let session_infos: Vec<SessionInfo> = sessions
        .into_iter()
        .map(|s| SessionInfo {
            id: s.id,
            title: s.name,
            description: s.description,
            status: s.status.to_string(), // Assuming SessionStatus has a Display impl or can be converted to String
            scheduled_at: s.scheduled_for.unwrap_or_default(), // Handle Option<Date>
        })
        .collect();

    let response = TableDetails {
        id: table.id,
        title: table.title,
        game_system: game_system.name, // Use the actual game system name
        game_master: GameMasterInfo {
            id: game_master.id,
            username: game_master.username,
        },
        description: table.description,
        player_slots: table.player_slots as i32,
        players,
        status: table.status.to_string(), // Map from table status
        sessions: session_infos,
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
        return Err(Error::Application(ApplicationError::Forbidden)); // Changed to Forbidden
    }

    let game_system_id = match payload.system {
        Some(s) => {
            let uuid = Uuid::parse_str(&s).map_err(|_| {
                Error::Application(ApplicationError::InvalidInput {
                    message: "system must be a valid UUID".to_string(),
                })
            })?;
            Some(uuid).into()
        }
        None => None.into(),
    };

    let status = match payload.status {
        Some(s) => {
            let status = TableStatus::from_str(&s).map_err(|_| {
                Error::Application(ApplicationError::InvalidInput {
                    message: "Invalid table status".to_string(),
                })
            })?;
            Some(status).into()
        }
        None => None.into(),
    };

    let command = UpdateTableCommand {
        id: table_id,
        title: payload.title.into(),
        description: payload.description.into(),
        slots: payload.max_players.map(|s| s as u32).into(),
        game_system_id,
        status,
    };

    app_state.table_service.update(&command).await?;

    let updated_table = app_state.table_service.find_by_id(&table_id).await?;
    let game_master = app_state
        .user_service
        .find_by_id(&updated_table.gm_id)
        .await?;
    let game_system = app_state
        .game_system_service
        .find_by_id(updated_table.game_system_id)
        .await?;
    let table_members = app_state
        .table_member_service
        .find_by_table_id(&updated_table.id)
        .await?;

    let mut players: Vec<PlayerInfo> = Vec::new();
    for tm in table_members {
        if let Some(user) = app_state.user_service.find_by_id(&tm.user_id).await.ok() {
            players.push(PlayerInfo {
                id: user.id,
                username: user.username,
            });
        }
    }

    let sessions = app_state
        .session_service
        .get(GetSessionCommand {
            table_id: Some(updated_table.id),
            ..Default::default()
        })
        .await?;

    let session_infos: Vec<SessionInfo> = sessions
        .into_iter()
        .map(|s| SessionInfo {
            id: s.id,
            title: s.name,
            description: s.description,
            status: s.status.to_string(),
            scheduled_at: s.scheduled_for.unwrap_or_default(),
        })
        .collect();

    let response = TableDetails {
        id: updated_table.id,
        title: updated_table.title,
        game_system: game_system.name, // Use the actual game system name
        game_master: GameMasterInfo {
            id: game_master.id,
            username: game_master.username,
        },
        description: updated_table.description,
        player_slots: updated_table.player_slots as i32,
        players,
        status: updated_table.status.to_string(), // Map from table status
        sessions: session_infos,
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
