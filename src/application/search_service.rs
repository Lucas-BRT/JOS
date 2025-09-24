use crate::Result;
use crate::domain::repositories::*;
use crate::domain::search::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct SearchService {
    user_repository: Arc<dyn UserRepository>,
    table_repository: Arc<dyn TableRepository>,
}

impl SearchService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        table_repository: Arc<dyn TableRepository>,
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
                    description: Some(user.email),
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
        let total = results.len() as u64;
        Ok(SearchResponse {
            results,
            total,
            page: query.page.unwrap_or(1),
            limit: query.limit.unwrap_or(10),
        })
    }
}
