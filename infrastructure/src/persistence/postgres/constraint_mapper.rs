use sqlx::Error as SqlxError;
use sqlx::error::DatabaseError;

use crate::persistence::postgres::RepositoryError;

const UNIQUE_CONSTRAINT_CODE: &str = "23505";
const FOREIGN_KEY_CONSTRAINT_CODE: &str = "23503";

pub fn map_constraint_violation(db_err: &dyn DatabaseError, constraint: &str) -> RepositoryError {
    let message = db_err.message();
    let is_referenced_not_found = message.contains("is not present in table");

    let extract_field_from_error = |field: &str| -> Option<String> {
        let pattern = format!("Key ({field})=(");
        if let Some(pos) = message.find(&pattern) {
            let after = &message[pos + pattern.len()..];
            if let Some(end) = after.find(')') {
                let value: String = after[..end]
                    .trim()
                    .trim_matches(|c| c == '\'' || c == '"')
                    .into();
                if !value.is_empty() {
                    return Some(value);
                }
            }
        }

        let alt_pattern = format!("({field})=(");
        if let Some(pos) = message.find(&alt_pattern) {
            let after = &message[pos + alt_pattern.len()..];
            if let Some(end) = after.find(')') {
                let value: String = after[..end]
                    .trim()
                    .trim_matches(|c| c == '\'' || c == '"')
                    .into();
                if !value.is_empty() {
                    return Some(value);
                }
            }
        }
        None
    };

    match constraint {
        // Unique constraints
        "users_username_key" => {
            tracing::debug!("Username already taken: {}", message);
            RepositoryError::UsernameAlreadyTaken
        }
        "users_email_key" => {
            tracing::debug!("Email already taken: {}", message);
            RepositoryError::EmailAlreadyTaken
        }
        "game_systems_name_key" => {
            tracing::debug!("Game system name already taken: {}", message);
            RepositoryError::GameSystemNameAlreadyTaken
        }
        "session_intents_user_id_session_id_key" => {
            tracing::debug!("User session intent already exists: {}", message);
            RepositoryError::UserSessionIntentAlreadyExists
        }
        "table_members_table_id_user_id_key" => {
            tracing::debug!("User already member of table: {}", message);
            RepositoryError::UserAlreadyMemberOfTable
        }

        // Foreign key constraints
        "tables_gm_id_fkey" => {
            if is_referenced_not_found {
                tracing::debug!("User not found for gm_id: {}", message);
                RepositoryError::UserNotFound
            } else {
                tracing::warn!("Foreign key violation for gm_id: {}", message);
                RepositoryError::ForeignKeyViolation {
                    table: "tables".into(),
                    field: "gm_id".into(),
                }
            }
        }
        "tables_game_system_id_fkey" => {
            if is_referenced_not_found {
                tracing::debug!("Game system not found: {}", message);
                let id =
                    extract_field_from_error("game_system_id").unwrap_or_else(|| "unknown".into());
                RepositoryError::GameSystemNotFound(id)
            } else {
                tracing::warn!("Foreign key violation for game_system_id: {}", message);
                RepositoryError::ForeignKeyViolation {
                    table: "tables".into(),
                    field: "game_system_id".into(),
                }
            }
        }
        "sessions_table_id_fkey" => {
            if is_referenced_not_found {
                tracing::debug!("Table not found: {}", message);
                let id = extract_field_from_error("table_id").unwrap_or_else(|| "unknown".into());
                RepositoryError::RpgTableNotFound(id)
            } else {
                tracing::warn!("Foreign key violation for table_id: {}", message);
                RepositoryError::ForeignKeyViolation {
                    table: "sessions".into(),
                    field: "table_id".into(),
                }
            }
        }
        "session_intents_user_id_fkey" => {
            if is_referenced_not_found {
                tracing::debug!("User not found for session intent: {}", message);
                RepositoryError::UserNotFound
            } else {
                tracing::warn!(
                    "Foreign key violation for user_id in session intents: {}",
                    message
                );
                RepositoryError::ForeignKeyViolation {
                    table: "session_intents".into(),
                    field: "user_id".into(),
                }
            }
        }
        "session_intents_session_id_fkey" => {
            if is_referenced_not_found {
                tracing::debug!("Session not found: {}", message);
                RepositoryError::SessionNotFound
            } else {
                tracing::warn!("Foreign key violation for session_id: {}", message);
                RepositoryError::ForeignKeyViolation {
                    table: "session_intents".into(),
                    field: "session_id".into(),
                }
            }
        }
        "session_checkins_session_intent_id_fkey" => {
            if is_referenced_not_found {
                tracing::debug!("Session intent not found: {}", message);
                let id = extract_field_from_error("session_intent_id")
                    .unwrap_or_else(|| "unknown".into());
                RepositoryError::SessionIntentNotFound(id)
            } else {
                tracing::warn!("Foreign key violation for session_intent_id: {}", message);
                RepositoryError::ForeignKeyViolation {
                    table: "session_checkins".into(),
                    field: "session_intent_id".into(),
                }
            }
        }
        "table_requests_user_id_fkey" => {
            if is_referenced_not_found {
                tracing::debug!("User not found for table request: {}", message);
                RepositoryError::UserNotFound
            } else {
                tracing::warn!(
                    "Foreign key violation for user_id in table requests: {}",
                    message
                );
                RepositoryError::ForeignKeyViolation {
                    table: "table_requests".into(),
                    field: "user_id".into(),
                }
            }
        }
        "table_requests_table_id_fkey" => {
            if is_referenced_not_found {
                tracing::debug!("Table not found for table request: {}", message);
                let id = extract_field_from_error("table_id").unwrap_or_else(|| "unknown".into());
                RepositoryError::RpgTableNotFound(id)
            } else {
                tracing::warn!(
                    "Foreign key violation for table_id in table requests: {}",
                    message
                );
                RepositoryError::ForeignKeyViolation {
                    table: "table_requests".into(),
                    field: "table_id".into(),
                }
            }
        }
        "table_members_table_id_fkey" => {
            if is_referenced_not_found {
                tracing::debug!("Table not found for table member: {}", message);
                let id = extract_field_from_error("table_id").unwrap_or_else(|| "unknown".into());
                RepositoryError::RpgTableNotFound(id)
            } else {
                tracing::warn!(
                    "Foreign key violation for table_id in table members: {}",
                    message
                );
                RepositoryError::ForeignKeyViolation {
                    table: "table_members".into(),
                    field: "table_id".into(),
                }
            }
        }
        "table_members_user_id_fkey" => {
            if is_referenced_not_found {
                tracing::debug!("User not found for table member: {}", message);
                RepositoryError::UserNotFound
            } else {
                tracing::warn!(
                    "Foreign key violation for user_id in table members: {}",
                    message
                );
                RepositoryError::ForeignKeyViolation {
                    table: "table_members".into(),
                    field: "user_id".into(),
                }
            }
        }
        _ => {
            tracing::error!("Unknown constraint violation: {} - {}", constraint, message);
            RepositoryError::UnknownConstraint(constraint.into())
        }
    }
}

pub fn map_database_error(error: SqlxError) -> RepositoryError {
    match error {
        SqlxError::RowNotFound => RepositoryError::NotFound,
        SqlxError::Database(db_err) => {
            if let Some(code) = db_err.code() {
                match code.as_ref() {
                    UNIQUE_CONSTRAINT_CODE | FOREIGN_KEY_CONSTRAINT_CODE => {
                        if let Some(constraint) = db_err.constraint() {
                            tracing::debug!(
                                "Mapping constraint violation: {} (code: {})",
                                constraint,
                                code
                            );
                            return map_constraint_violation(&*db_err, constraint);
                        }
                    }
                    "23514" => {
                        tracing::warn!("Check constraint violation: {}", db_err.message());
                        return RepositoryError::ValidationError(db_err.message().into());
                    }
                    "22P02" => {
                        tracing::warn!("Invalid input value: {}", db_err.message());
                        return RepositoryError::InvalidInput(db_err.message().into());
                    }
                    _ => {}
                }
            }
            tracing::error!("Unhandled database error: {:?}", db_err);
            RepositoryError::DatabaseError(SqlxError::Database(db_err))
        }
        _ => {
            tracing::error!("Unhandled general sqlx error: {:?}", error);
            RepositoryError::DatabaseError(error)
        }
    }
}
