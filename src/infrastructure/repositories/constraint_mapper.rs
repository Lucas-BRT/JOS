use crate::infrastructure::repositories::error::RepositoryError;
use sqlx::Error as SqlxError;

/// Maps database constraint violations to specific RepositoryError variants
pub fn map_constraint_violation(error: &SqlxError, constraint: &str) -> RepositoryError {
    match constraint {
        // UNIQUE constraints
        "t_users_name_key" => {
            RepositoryError::UsernameAlreadyTaken
        }
        "t_users_email_key" => {
            RepositoryError::EmailAlreadyTaken
        }
        "t_game_system_name_key" => {
            let name = extract_field_from_error(error, "name").unwrap_or_else(|| "unknown".to_string());
            RepositoryError::GameSystemNameAlreadyTaken(name)
        }
        "t_session_intents_user_id_session_id_key" => {
            RepositoryError::UserSessionIntentAlreadyExists
        }
        
        // FOREIGN KEY constraints
        "t_rpg_tables_gm_id_fkey" => {
            RepositoryError::ForeignKeyViolation { 
                table: "t_rpg_tables".to_string(), 
                field: "gm_id".to_string() 
            }
        }
        "t_rpg_tables_game_system_id_fkey" => {
            RepositoryError::ForeignKeyViolation { 
                table: "t_rpg_tables".to_string(), 
                field: "game_system_id".to_string() 
            }
        }
        "t_sessions_table_id_fkey" => {
            RepositoryError::ForeignKeyViolation { 
                table: "t_sessions".to_string(), 
                field: "table_id".to_string() 
            }
        }
        "t_session_intents_user_id_fkey" => {
            RepositoryError::ForeignKeyViolation { 
                table: "t_session_intents".to_string(), 
                field: "user_id".to_string() 
            }
        }
        "t_session_intents_session_id_fkey" => {
            RepositoryError::ForeignKeyViolation { 
                table: "t_session_intents".to_string(), 
                field: "session_id".to_string() 
            }
        }
        "t_session_checkins_session_intent_id_fkey" => {
            RepositoryError::ForeignKeyViolation { 
                table: "t_session_checkins".to_string(), 
                field: "session_intent_id".to_string() 
            }
        }
        
        // Unknown constraints
        _ => RepositoryError::UnknownConstraint(constraint.to_string())
    }
}

/// Try to parse "Key (field)=(value)" from a given text
fn parse_key_value_from_text(text: &str, field: &str) -> Option<String> {
    // Most precise pattern first: "(field)=(" then value until ")"
    if let Some(pos) = text.find(&format!("({})=(", field)) {
        let after = &text[pos + field.len() + 4..]; // skip "(field)=("
        if let Some(end) = after.find(')') {
            let mut value = &after[..end];
            // Trim quotes if present
            value = value.trim_matches('\"').trim_matches('\'');
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }

    // Fallback: "(field)=" followed by optional '(' then value until ')' or whitespace/comma
    if let Some(pos) = text.find(&format!("({})=", field)) {
        let mut after = &text[pos + field.len() + 3..]; // skip "(field)="
        after = after.trim_start();
        if after.starts_with('(') {
            after = &after[1..];
        }
        if let Some(end) = after.find(|c: char| c == ')' || c.is_whitespace() || c == ',') {
            let mut value = &after[..end];
            value = value.trim_matches('\"').trim_matches('\'');
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }

    None
}

/// Helper function to extract field value from error message
fn extract_field_from_error(error: &SqlxError, field: &str) -> Option<String> {
    // Prefer driver message if available
    if let Some(db_err) = error.as_database_error() {
        let message = db_err.message();
        if let Some(val) = parse_key_value_from_text(message, field) {
            return Some(val);
        }
    }

    // Parse the full Display string, which usually includes DETAIL for Postgres
    let error_display = error.to_string();
    if let Some(val) = parse_key_value_from_text(&error_display, field) {
        return Some(val);
    }

    None
}

/// Maps sqlx::Error to RepositoryError, handling constraint violations
pub fn map_database_error(error: SqlxError) -> RepositoryError {

    if let Some(db_err) = error.as_database_error() {
        if let Some(code) = db_err.code() {
            if code == "23505" { // UNIQUE constraint violation
                if let Some(constraint) = db_err.constraint() {
                    tracing::debug!("Mapping constraint violation: {} -> {:?}", constraint, error);
                    return map_constraint_violation(&error, constraint);
                }
            }
        }
    }
    RepositoryError::DatabaseError(error)
}


