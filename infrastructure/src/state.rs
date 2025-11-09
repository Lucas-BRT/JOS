use crate::config::AppConfig;
use axum::extract::FromRef;
use domain::services::*;
use std::sync::Arc;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub user_service: Arc<dyn IUserService>,
    pub table_service: Arc<dyn ITableService>,
    pub table_request_service: Arc<dyn ITableRequestService>,
    pub session_service: Arc<dyn ISessionService>,
    pub auth_service: Arc<dyn IAuthService>,
    pub password_service: Arc<dyn IPasswordService>,
    pub game_system_service: Arc<dyn IGameSystemService>,
    pub table_member_service: Arc<dyn ITableMemberService>,
}
