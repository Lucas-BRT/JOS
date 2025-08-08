use super::config::Config;
use crate::application::services::{
    jwt_service::JwtService,
    password_service::PasswordService,
    table_request_service::TableRequestService,
    table_service::TableService,
    user_service::UserService,
};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub user_service: UserService,
    pub table_service: TableService,
    pub table_request_service: TableRequestService,
    pub jwt_service: JwtService,
    pub password_service: PasswordService,
}

impl AppState {
    pub fn new(
        config: Config,
        user_service: UserService,
        table_service: TableService,
        table_request_service: TableRequestService,
        jwt_service: JwtService,
        password_service: PasswordService,
    ) -> Self {
        Self {
            config,
            user_service,
            table_service,
            table_request_service,
            jwt_service,
            password_service,
        }
    }
}
