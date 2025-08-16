use crate::application::error::ApplicationError;
use crate::domain::table::commands::DeleteTableCommand;
use crate::domain::table::search_filters::TableFilters;
use crate::domain::utils::pagination::Pagination;
use crate::domain::{
    auth::Claims,
    table::commands::{CreateTableCommand, UpdateTableCommand},
};
use crate::infrastructure::prelude::RepositoryError;
use crate::interfaces::http::{
    table::dtos::{AvaliableTableResponse, CreateTableDto, UpdateTableDto},
    table_request::dtos::UpdateTableRequestDto,
};
use crate::state::AppState;
use crate::{Error, Result};
use axum::extract::Query;
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, post},
};
use std::sync::Arc;
use uuid::Uuid;

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
    user: Claims,
    State(app_state): State<Arc<AppState>>,
    Query(filters): Query<TableFilters>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<AvaliableTableResponse>>> {
    let tables = app_state.table_service.get(&filters, pagination).await?;

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

pub async fn update_table(
    State(app_state): State<Arc<AppState>>,
    Path(table_id): Path<Uuid>,
    Json(update_payload): Json<UpdateTableDto>,
    user: Claims,
) -> Result<()> {
    // let update_command = UpdateTableCommand::from_dto(update_payload, user.sub);

    // app_state.table_service.update(&table_id, &update_command).await?;

    Ok(())
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(get_available_tables))
        .route("/", post(create_table))
        .route("/{id}", delete(delete_table))
        .with_state(state.clone())
}
