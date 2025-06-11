use super::config::Config;
use crate::application::services::{table_service::TableService, user_service::UserService};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub user_service: UserService,
    pub table_service: TableService,
}

impl AppState {
    pub fn new(config: Config, user_service: UserService, table_service: TableService) -> Self {
        Self {
            config,
            user_service,
            table_service,
        }
    }
}
