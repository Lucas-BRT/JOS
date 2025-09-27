use crate::{Result, domain::auth::Claims, state::AppState};
use axum::{
    Json, Router,
    extract::{Query, State},
    routing::get,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct SearchQuery {
    pub q: String,
    pub r#type: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub r#type: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
}

#[utoipa::path(
    get,
    path = "/v1/search",
    tag = "search",
    params(
        ("q" = String, Query, description = "Search query"),
        ("type" = Option<String>, Query, description = "Filter by type: tables, sessions, users"),
        ("page" = Option<u32>, Query, description = "Page number"),
        ("limit" = Option<u32>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "Search results", body = SearchResponse)
    )
)]
pub async fn search(
    State(app_state): State<Arc<AppState>>,
    Query(query): Query<SearchQuery>,
    _: Claims,
) -> Result<Json<SearchResponse>> {
    let results = app_state.search_service.search(&query).await?;
    Ok(Json(results))
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(search))
        .with_state(state.clone())
}
