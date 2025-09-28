use crate::{
    Result, domain::auth::Claims, domain::search::SearchQuery as DomainSearchQuery,
    infrastructure::state::AppState,
};
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
    let domain_query = DomainSearchQuery {
        q: query.q,
        r#type: query.r#type,
        page: query.page,
        limit: query.limit,
    };
    let domain_results = app_state.search_service.search(&domain_query).await?;

    // Convert domain results to local SearchResponse
    let results = SearchResponse {
        results: domain_results
            .results
            .into_iter()
            .map(|r| SearchResult {
                id: r.id.to_string(),
                title: r.title,
                description: r.description,
                r#type: r.r#type,
                created_at: r.created_at,
            })
            .collect(),
        total: domain_results.total,
        page: domain_results.page,
        limit: domain_results.limit,
    };

    Ok(Json(results))
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(search))
        .with_state(state.clone())
}
