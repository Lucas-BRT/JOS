#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "e_intent_status")]
#[sqlx(rename_all = "snake_case")]
pub enum EIntentStatus {
    Yes,
    No,
    Maybe,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "e_roles")]
#[sqlx(rename_all = "snake_case")]
pub enum ERoles {
    Admin,
    Moderator,
    User,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "e_table_visibility")]
#[sqlx(rename_all = "snake_case")]
pub enum ETableVisibility {
    Private,
    Public,
}
