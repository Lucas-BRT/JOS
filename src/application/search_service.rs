use crate::{Result, domain::repositories::TableRepository, domain::repositories::UserRepository};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct SearchService {
    user_repository: std::sync::Arc<dyn UserRepository>,
    table_repository: std::sync::Arc<dyn TableRepository>,
}

impl SearchService {
    pub fn new(
        user_repository: std::sync::Arc<dyn UserRepository>,
        table_repository: std::sync::Arc<dyn TableRepository>,
    ) -> Self {
        Self {
            user_repository,
            table_repository,
        }
    }

    pub async fn search(&self, query: &SearchQuery) -> Result<SearchResponse> {
        let mut results = Vec::new();

        // Search users
        if query.r#type.is_none() || query.r#type.as_ref().unwrap() == "users" {
            let users = self.user_repository.search(&query.q).await?;
            for user in users {
                results.push(SearchResult {
                    id: user.id.to_string(),
                    title: user.username,
                    description: Some(user.display_name),
                    r#type: "user".to_string(),
                    created_at: user.created_at.to_rfc3339(),
                });
            }
        }

        // Search tables
        if query.r#type.is_none() || query.r#type.as_ref().unwrap() == "tables" {
            let tables = self.table_repository.search(&query.q).await?;
            for table in tables {
                results.push(SearchResult {
                    id: table.id.to_string(),
                    title: table.title,
                    description: Some(table.description),
                    r#type: "table".to_string(),
                    created_at: table.created_at.to_rfc3339(),
                });
            }
        }

        // TODO: Add session search when session repository is available

        Ok(SearchResponse {
            results,
            total: results.len() as u64,
            page: query.page.unwrap_or(1),
            limit: query.limit.unwrap_or(10),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub r#type: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub r#type: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
}
