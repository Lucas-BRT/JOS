use crate::domain::utils::Update;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateGameSystemCommand {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct GetGameSystemCommand {
    pub id: Option<Uuid>,
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateGameSystemCommand {
    pub name: Update<String>,
}

#[derive(Debug, Clone)]
pub struct DeleteGameSystemCommand {
    pub id: Uuid,
}
