use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pagination {
    pub page: u32,
    pub page_size: u32,
}
