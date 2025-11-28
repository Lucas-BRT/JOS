use chrono::{DateTime, Utc};
use shared::Result;
use uuid::Uuid;

pub const DEFAULT_PAGINATION_LIMIT: i64 = 50;
pub const MAX_PAGINATION_LIMIT: i64 = 100;
pub const DEFAULT_OFFSET: i64 = 0;

#[async_trait::async_trait]
pub trait Repository<Entity, CreateCommand, UpdateCommand, GetCommand, DeleteCommand>:
    Send + Sync
{
    async fn create(&self, command: CreateCommand) -> Result<Entity>;
    async fn read(&self, command: GetCommand) -> Result<Vec<Entity>>;
    async fn update(&self, command: UpdateCommand) -> Result<Entity>;
    async fn delete(&self, command: DeleteCommand) -> Result<Entity>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Entity>>;
}

pub trait AuditableEntity {
    fn created_at(&self) -> DateTime<Utc>;
    fn updated_at(&self) -> DateTime<Utc>;
}

#[derive(Debug, Clone)]
pub struct PaginationParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            limit: Some(DEFAULT_PAGINATION_LIMIT),
            offset: Some(DEFAULT_OFFSET),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SortDirection {
    Asc,
    Desc,
}

impl SortDirection {
    pub fn as_sql(&self) -> &'static str {
        match self {
            SortDirection::Asc => "ASC",
            SortDirection::Desc => "DESC",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SortParams {
    pub field: String,
    pub direction: SortDirection,
}

impl Default for SortParams {
    fn default() -> Self {
        Self {
            field: "created_at".to_string(),
            direction: SortDirection::Desc,
        }
    }
}
