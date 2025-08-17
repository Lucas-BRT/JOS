use crate::domain::table::entity::Visibility;
use crate::domain::table::{commands::CreateTableCommand, entity::Table};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Serialize, utoipa::ToSchema)]
pub enum TableVisibility {
    Private,
    Public,
}

impl From<TableVisibility> for Visibility {
    fn from(visibility: TableVisibility) -> Self {
        match visibility {
            TableVisibility::Private => Visibility::Private,
            TableVisibility::Public => Visibility::Public,
        }
    }
}

impl From<Visibility> for TableVisibility {
    fn from(visibility: Visibility) -> Self {
        match visibility {
            Visibility::Private => TableVisibility::Private,
            Visibility::Public => TableVisibility::Public,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Validate, utoipa::ToSchema)]
pub struct CreateTableDto {
    #[validate(length(min = 8, max = 60, message = "Title is empty"))]
    pub title: String,
    #[validate(length(
        min = 50,
        max = 1000,
        message = "Description must be between 50 and 1000 characters"
    ))]
    pub description: String,
    pub game_system_id: Uuid,
    pub visibility: TableVisibility,
    #[validate(range(min = 1, max = 20, message = "Max players must be between 1 and 20"))]
    pub max_players: u32,
    pub player_slots: u32,
    pub occupied_slots: u32,
}

impl CreateTableCommand {
    pub fn from_dto(dto: CreateTableDto, gm_id: Uuid) -> Self {
        Self {
            gm_id,
            title: dto.title,
            description: dto.description,
            visibility: dto.visibility.clone().into(),
            player_slots: dto.player_slots,
            game_system_id: dto.game_system_id,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Validate, utoipa::ToSchema)]
pub struct UpdateTableDto {
    #[validate(length(min = 8, max = 60, message = "Title is empty"))]
    pub title: Option<String>,
    #[validate(length(
        min = 50,
        max = 1000,
        message = "Description must be between 50 and 1000 characters"
    ))]
    pub description: Option<String>,
    pub is_public: Option<bool>,
    #[validate(range(min = 1, max = 20, message = "Max players must be between 1 and 20"))]
    pub max_players: Option<u32>,
    #[validate(range(min = 1, message = "Player slots must be greater than 0"))]
    pub player_slots: Option<u32>,
    #[validate(range(min = 1, message = "Occupied slots must be greater than 0"))]
    pub occupied_slots: Option<u32>,
}

#[derive(Debug, Clone, Serialize, utoipa::ToSchema)]
pub struct AvaliableTableResponse {
    pub id: Uuid,
    pub gm_id: Uuid,
    pub title: String,
    pub description: String,
    pub game_system_id: Uuid,
    pub visibility: TableVisibility,
    pub player_slots: u32,
    pub occupied_slots: u32,
}

impl From<&Table> for AvaliableTableResponse {
    fn from(table: &Table) -> Self {
        Self {
            id: table.id,
            gm_id: table.gm_id,
            title: table.title.clone(),
            description: table.description.clone(),
            game_system_id: table.game_system_id,
            visibility: table.visibility.into(),
            player_slots: table.player_slots,
            occupied_slots: 0,
        }
    }
}
