use crate::{
    application::{
        auth_service::AuthService, password_service::PasswordService,
        search_service::SearchService, session_service::SessionService,
        table_request_service::TableRequestService, table_service::TableService,
        user_service::UserService,
    },
    infrastructure::setup::config::Config,
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
