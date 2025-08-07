use super::config::Config;
use crate::application::services::{table_service::TableService, table_request_service::TableRequestService, user_service::UserService};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub user_service: UserService,
    pub table_service: TableService,
    pub table_request_service: TableRequestService,
}

impl AppState {
    pub fn new(config: Config, user_service: UserService, table_service: TableService, table_request_service: TableRequestService) -> Self {
        Self {
            config,
            user_service,
            table_service,
            table_request_service,
        }
    }
}
