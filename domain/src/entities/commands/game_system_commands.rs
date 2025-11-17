use crate::entities::Update;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateGameSystemCommand {
    pub name: String,
}

#[derive(Debug, Clone, Default)]
pub struct GetGameSystemCommand {
    pub id: Option<Uuid>,
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateGameSystemCommand {
    pub id: Uuid,
    pub name: Update<String>,
}

#[derive(Debug, Clone)]
pub struct DeleteGameSystemCommand {
    pub id: Uuid,
}
