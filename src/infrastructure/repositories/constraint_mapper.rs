use crate::infrastructure::repositories::error::RepositoryError;
use sqlx::Error as SqlxError;

const UNIQUE_CONSTRAINT_CODE: &str = "23505";
const FOREIGN_KEY_CONSTRAINT_CODE: &str = "23503";

pub fn map_constraint_violation(error: &SqlxError, constraint: &str) -> RepositoryError {
    let db_err = error
        .as_database_error()
        .expect("Expected a database error");
    let message = db_err.message();
    let is_referenced_not_found = message.contains("is not present in table");

    match constraint {
        "t_users_username_key" => {
            tracing::error!("username already taken: {}", message);
            RepositoryError::UsernameAlreadyTaken
        }
        "t_users_email_key" => {
            tracing::error!("email already taken: {}", message);
            RepositoryError::EmailAlreadyTaken
        }
        "t_game_system_name_key" => {
            tracing::error!("game system name already taken: {}", message);
            RepositoryError::GameSystemNameAlreadyTaken
        }
        "t_session_intents_user_id_session_id_key" => {
            tracing::error!("user session intent already exists: {}", message);
            RepositoryError::UserSessionIntentAlreadyExists
        }

        "t_rpg_tables_gm_id_fkey" => {
            if is_referenced_not_found {
                let id = extract_field_from_error(error, "gm_id")
                    .unwrap_or_else(|| "unknown".to_string());
                RepositoryError::UserNotFound(id)
            } else {
                RepositoryError::ForeignKeyViolation {
                    table: "t_rpg_tables".to_string(),
                    field: "gm_id".to_string(),
                }
            }
        }
        "t_rpg_tables_game_system_id_fkey" => {
            if is_referenced_not_found {
                let id = extract_field_from_error(error, "game_system_id")
                    .unwrap_or_else(|| "unknown".to_string());
                RepositoryError::GameSystemNotFound(id)
            } else {
                RepositoryError::ForeignKeyViolation {
                    table: "t_rpg_tables".to_string(),
                    field: "game_system_id".to_string(),
                }
            }
        }
        "t_sessions_table_id_fkey" => {
            if is_referenced_not_found {
                let id = extract_field_from_error(error, "table_id")
                    .unwrap_or_else(|| "unknown".to_string());
                RepositoryError::RpgTableNotFound(id)
            } else {
                RepositoryError::ForeignKeyViolation {
                    table: "t_sessions".to_string(),
                    field: "table_id".to_string(),
                }
            }
        }
        "t_session_intents_user_id_fkey" => {
            if is_referenced_not_found {
                let id = extract_field_from_error(error, "user_id")
                    .unwrap_or_else(|| "unknown".to_string());
                RepositoryError::UserNotFound(id)
            } else {
                RepositoryError::ForeignKeyViolation {
                    table: "t_session_intents".to_string(),
                    field: "user_id".to_string(),
                }
            }
        }
        "t_session_intents_session_id_fkey" => {
            if is_referenced_not_found {
                let id = extract_field_from_error(error, "session_id")
                    .unwrap_or_else(|| "unknown".to_string());
                RepositoryError::SessionNotFound(id)
            } else {
                RepositoryError::ForeignKeyViolation {
                    table: "t_session_intents".to_string(),
                    field: "session_id".to_string(),
                }
            }
        }
        "t_session_checkins_session_intent_id_fkey" => {
            if is_referenced_not_found {
                let id = extract_field_from_error(error, "session_intent_id")
                    .unwrap_or_else(|| "unknown".to_string());
                RepositoryError::SessionIntentNotFound(id)
            } else {
                RepositoryError::ForeignKeyViolation {
                    table: "t_session_checkins".to_string(),
                    field: "session_intent_id".to_string(),
                }
            }
        }
        "t_table_requests_user_id_fkey" => {
            if is_referenced_not_found {
                let id = extract_field_from_error(error, "user_id")
                    .unwrap_or_else(|| "unknown".to_string());
                RepositoryError::UserNotFound(id)
            } else {
                RepositoryError::ForeignKeyViolation {
                    table: "t_table_requests".to_string(),
                    field: "user_id".to_string(),
                }
            }
        }
        _ => {
            tracing::error!("unknown constraint violation: {}", constraint);
            RepositoryError::UnknownConstraint(constraint.to_string())
        }
    }
}

fn parse_key_value_from_text(text: &str, field: &str) -> Option<String> {
    if let Some(pos) = text.find(&format!("({field})=(")) {
        let after = &text[pos + field.len() + 4..];
        if let Some(end) = after.find(')') {
            let mut value = &after[..end];

            value = value.trim_matches('\"').trim_matches('\'');
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }

    if let Some(pos) = text.find(&format!("({field})=")) {
        let mut after = &text[pos + field.len() + 3..];
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

fn extract_field_from_error(error: &SqlxError, field: &str) -> Option<String> {
    if let Some(db_err) = error.as_database_error() {
        let message = db_err.message();
        if let Some(val) = parse_key_value_from_text(message, field) {
            return Some(val);
        }
    }

    let error_display = error.to_string();
    if let Some(val) = parse_key_value_from_text(&error_display, field) {
        return Some(val);
    }

    None
}

pub fn map_database_error(error: SqlxError) -> RepositoryError {
    if matches!(&error, SqlxError::RowNotFound) {
        return RepositoryError::TableNotFound;
    }

    if let Some(db_err) = error.as_database_error()
        && let Some(code) = db_err.code()
        && (code == UNIQUE_CONSTRAINT_CODE || code == FOREIGN_KEY_CONSTRAINT_CODE)
        && let Some(constraint) = db_err.constraint()
    {
        tracing::debug!(
            "Mapping constraint violation: {} -> {:?}",
            constraint,
            error
        );
        return map_constraint_violation(&error, constraint);
    }
    RepositoryError::DatabaseError(error)
}
