use serde::{Deserialize, Serialize};

pub const DEFAULT_PAGE: u32 = 1;
pub const DEFAULT_PAGE_SIZE: u32 = 20;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pagination {
    pub page: u32,
    pub page_size: u32,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: DEFAULT_PAGE,
            page_size: DEFAULT_PAGE_SIZE,
        }
    }
}

impl Pagination {
    pub fn offset(&self) -> u32 {
        (self.page - 1) * self.page_size
    }

    pub fn limit(&self) -> u32 {
        self.page_size
    }
}
