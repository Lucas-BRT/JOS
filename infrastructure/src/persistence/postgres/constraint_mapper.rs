use shared::Error;
use shared::error::InfrastructureError;
use sqlx::Error as SqlxError;
use sqlx::error::DatabaseError;

const UNIQUE_CONSTRAINT_CODE: &str = "23505";
const FOREIGN_KEY_CONSTRAINT_CODE: &str = "23503";

pub fn map_constraint_violation(
    db_err: &dyn DatabaseError,
    constraint: &str,
) -> InfrastructureError {
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
            InfrastructureError::UsernameAlreadyTaken
        }
        "users_email_key" => {
            tracing::debug!("Email already taken: {}", message);
            InfrastructureError::EmailAlreadyTaken
        }
        "game_systems_name_key" => {
            tracing::debug!("Game system name already taken: {}", message);
            InfrastructureError::GameSystemNameAlreadyTaken
        }
        "session_intents_user_id_session_id_key" => {
            tracing::debug!("User session intent already exists: {}", message);
            InfrastructureError::UserSessionIntentAlreadyExists
        }
        "table_members_table_id_user_id_key" => {
            tracing::debug!("User already member of table: {}", message);
            InfrastructureError::UserAlreadyMemberOfTable
        }

        // Foreign key constraints
        "tables_gm_id_fkey" => {
            if is_referenced_not_found {
                tracing::debug!("User not found for gm_id: {}", message);
                let id = extract_field_from_error("gm_id").unwrap_or_else(|| "unknown".into());
                InfrastructureError::UserNotFound(id.parse().unwrap_or_default())
            } else {
                tracing::warn!("Foreign key violation for gm_id: {}", message);
                InfrastructureError::ForeignKeyViolation {
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
                InfrastructureError::GameSystemNotFound(id.parse().unwrap_or_default())
            } else {
                tracing::warn!("Foreign key violation for game_system_id: {}", message);
                InfrastructureError::ForeignKeyViolation {
                    table: "tables".into(),
                    field: "game_system_id".into(),
                }
            }
        }
        "sessions_table_id_fkey" => {
            if is_referenced_not_found {
                tracing::debug!("Table not found: {}", message);
                let id = extract_field_from_error("table_id").unwrap_or_else(|| "unknown".into());
                InfrastructureError::RpgTableNotFound(id.parse().unwrap_or_default())
            } else {
                tracing::warn!("Foreign key violation for table_id: {}", message);
                InfrastructureError::ForeignKeyViolation {
                    table: "sessions".into(),
                    field: "table_id".into(),
                }
            }
        }
        "session_intents_user_id_fkey" => {
            if is_referenced_not_found {
                tracing::debug!("User not found for session intent: {}", message);
                let id = extract_field_from_error("user_id").unwrap_or_else(|| "unknown".into());
                InfrastructureError::UserNotFound(id.parse().unwrap_or_default())
            } else {
                tracing::warn!(
                    "Foreign key violation for user_id in session intents: {}",
                    message
                );
                InfrastructureError::ForeignKeyViolation {
                    table: "session_intents".into(),
                    field: "user_id".into(),
                }
            }
        }
        "session_intents_session_id_fkey" => {
            if is_referenced_not_found {
                tracing::debug!("Session not found: {}", message);
                let id = extract_field_from_error("session_id").unwrap_or_else(|| "unknown".into());
                InfrastructureError::SessionNotFound(id.parse().unwrap_or_default())
            } else {
                tracing::warn!("Foreign key violation for session_id: {}", message);
                InfrastructureError::ForeignKeyViolation {
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
                InfrastructureError::SessionIntentNotFound(id.parse().unwrap_or_default())
            } else {
                tracing::warn!("Foreign key violation for session_intent_id: {}", message);
                InfrastructureError::ForeignKeyViolation {
                    table: "session_checkins".into(),
                    field: "session_intent_id".into(),
                }
            }
        }
        "table_requests_user_id_fkey" => {
            if is_referenced_not_found {
                tracing::debug!("User not found for table request: {}", message);
                let id = extract_field_from_error("user_id").unwrap_or_else(|| "unknown".into());
                InfrastructureError::UserNotFound(id.parse().unwrap_or_default())
            } else {
                tracing::warn!(
                    "Foreign key violation for user_id in table requests: {}",
                    message
                );
                InfrastructureError::ForeignKeyViolation {
                    table: "table_requests".into(),
                    field: "user_id".into(),
                }
            }
        }
        "table_requests_table_id_fkey" => {
            if is_referenced_not_found {
                tracing::debug!("Table not found for table request: {}", message);
                let id = extract_field_from_error("table_id").unwrap_or_else(|| "unknown".into());
                InfrastructureError::RpgTableNotFound(id.parse().unwrap_or_default())
            } else {
                tracing::warn!(
                    "Foreign key violation for table_id in table requests: {}",
                    message
                );
                InfrastructureError::ForeignKeyViolation {
                    table: "table_requests".into(),
                    field: "table_id".into(),
                }
            }
        }
        "table_members_table_id_fkey" => {
            if is_referenced_not_found {
                tracing::debug!("Table not found for table member: {}", message);
                let id = extract_field_from_error("table_id").unwrap_or_else(|| "unknown".into());
                InfrastructureError::RpgTableNotFound(id.parse().unwrap_or_default())
            } else {
                tracing::warn!(
                    "Foreign key violation for table_id in table members: {}",
                    message
                );
                InfrastructureError::ForeignKeyViolation {
                    table: "table_members".into(),
                    field: "table_id".into(),
                }
            }
        }
        "table_members_user_id_fkey" => {
            if is_referenced_not_found {
                tracing::debug!("User not found for table member: {}", message);
                let id = extract_field_from_error("user_id").unwrap_or_else(|| "unknown".into());
                InfrastructureError::UserNotFound(id.parse().unwrap_or_default())
            } else {
                tracing::warn!(
                    "Foreign key violation for user_id in table members: {}",
                    message
                );
                InfrastructureError::ForeignKeyViolation {
                    table: "table_members".into(),
                    field: "user_id".into(),
                }
            }
        }
        _ => {
            tracing::error!("Unknown constraint violation: {} - {}", constraint, message);
            InfrastructureError::UnknownConstraint(constraint.into())
        }
    }
}

pub fn map_database_error(error: SqlxError) -> Error {
    match error {
        SqlxError::RowNotFound => Error::Infrastructure(InfrastructureError::NotFound),
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
                            return Error::Infrastructure(map_constraint_violation(
                                &*db_err, constraint,
                            ));
                        }
                    }
                    "23514" => {
                        tracing::warn!("Check constraint violation: {}", db_err.message());
                        return Error::Infrastructure(InfrastructureError::ValidationError(
                            db_err.message().into(),
                        ));
                    }
                    "22P02" => {
                        tracing::warn!("Invalid input value: {}", db_err.message());
                        return Error::Infrastructure(InfrastructureError::InvalidInput(
                            db_err.message().into(),
                        ));
                    }
                    _ => {}
                }
            }
            tracing::error!("Unhandled database error: {:?}", db_err);
            Error::Infrastructure(InfrastructureError::Database(
                SqlxError::Database(db_err).to_string(),
            ))
        }
        _ => {
            tracing::error!("Unhandled general sqlx error: {:?}", error);
            Error::Infrastructure(InfrastructureError::DatabaseError(error.to_string()))
        }
    }
}
