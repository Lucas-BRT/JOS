use super::config::Config;
use crate::application::{
    auth_service::AuthService, password_service::PasswordService,
    search_service::SearchService, session_service::SessionService,
    table_request_service::TableRequestService, table_service::TableService,
    user_service::UserService,
};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub user_service: UserService,
    pub table_service: TableService,
    pub table_request_service: TableRequestService,
    pub session_service: SessionService,
    pub search_service: SearchService,
    pub auth_service: AuthService,
    pub password_service: PasswordService,
}

impl AppState {
    pub fn new(
        config: Config,
        user_service: UserService,
        table_service: TableService,
        table_request_service: TableRequestService,
        session_service: SessionService,
        search_service: SearchService,
        auth_service: AuthService,
        password_service: PasswordService,
    ) -> Self {
        Self {
            config,
            user_service,
            table_service,
            table_request_service,
            session_service,
            search_service,
            auth_service,
            password_service,
        }
    }
}
