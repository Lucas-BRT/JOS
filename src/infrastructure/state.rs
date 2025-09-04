use super::config::Config;
use crate::application::{
    auth_service::AuthService, password_service::PasswordService,
    table_request_service::TableRequestService, table_service::TableService,
    user_service::UserService,
};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub user_service: UserService,
    pub table_service: TableService,
    pub table_request_service: TableRequestService,
    pub auth_service: AuthService,
    pub password_service: PasswordService,
}

impl AppState {
    pub fn new(
        config: Config,
        user_service: UserService,
        table_service: TableService,
        table_request_service: TableRequestService,
        auth_service: AuthService,
        password_service: PasswordService,
    ) -> Self {
        Self {
            config,
            user_service,
            table_service,
            table_request_service,
            auth_service,
            password_service,
        }
    }
}
