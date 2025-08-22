use crate::domain::{
    session_intent::IntentStatus, table::entity::Visibility,
    table_request::entity::TableRequestStatus, user::Role,
};

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "e_intent_status", rename_all = "snake_case")]
pub enum EIntentStatus {
    Yes,
    No,
    Maybe,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "e_roles", rename_all = "lowercase")]
pub enum ERoles {
    Admin,
    Moderator,
    User,
}

impl From<ERoles> for Role {
    fn from(role: ERoles) -> Self {
        match role {
            ERoles::Admin => Role::Admin,
            ERoles::Moderator => Role::Moderator,
            ERoles::User => Role::User,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "e_table_visibility", rename_all = "snake_case")]
pub enum ETableVisibility {
    Private,
    Public,
}

impl From<ETableVisibility> for Visibility {
    fn from(value: ETableVisibility) -> Self {
        match value {
            ETableVisibility::Private => Visibility::Private,
            ETableVisibility::Public => Visibility::Public,
        }
    }
}

impl From<Visibility> for ETableVisibility {
    fn from(value: Visibility) -> Self {
        match value {
            Visibility::Private => ETableVisibility::Private,
            Visibility::Public => ETableVisibility::Public,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "e_table_request_status", rename_all = "lowercase")]
pub enum ETableRequestStatus {
    Pending,
    Approved,
    Rejected,
}

impl From<TableRequestStatus> for ETableRequestStatus {
    fn from(status: TableRequestStatus) -> Self {
        match status {
            TableRequestStatus::Pending => ETableRequestStatus::Pending,
            TableRequestStatus::Approved => ETableRequestStatus::Approved,
            TableRequestStatus::Rejected => ETableRequestStatus::Rejected,
        }
    }
}

impl From<ETableRequestStatus> for TableRequestStatus {
    fn from(status: ETableRequestStatus) -> Self {
        match status {
            ETableRequestStatus::Pending => TableRequestStatus::Pending,
            ETableRequestStatus::Approved => TableRequestStatus::Approved,
            ETableRequestStatus::Rejected => TableRequestStatus::Rejected,
        }
    }
}

impl From<EIntentStatus> for IntentStatus {
    fn from(status: EIntentStatus) -> Self {
        match status {
            EIntentStatus::Yes => IntentStatus::Yes,
            EIntentStatus::No => IntentStatus::No,
            EIntentStatus::Maybe => IntentStatus::Maybe,
        }
    }
}

impl From<IntentStatus> for EIntentStatus {
    fn from(status: IntentStatus) -> Self {
        match status {
            IntentStatus::Yes => EIntentStatus::Yes,
            IntentStatus::No => EIntentStatus::No,
            IntentStatus::Maybe => EIntentStatus::Maybe,
        }
    }
}
