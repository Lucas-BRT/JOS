use axum::http::StatusCode;

pub fn translate_db_error(err: &sqlx::Error) -> (StatusCode, String) {
    if err.as_database_error().is_none() {
        return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string());
    }

    let db_error = err.as_database_error();

    if db_error.is_none() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to get db error".to_string(),
        );
    }

    let db_error = db_error.expect("failed to get db error");

    let code = db_error.code();

    if code.is_none() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to get db error code".to_string(),
        );
    }

    let code = code.expect("failed to get db error code");

    match code.to_string().as_str() {
        "23505" => {
            let msg = match db_error.constraint().unwrap_or_default() {
                "users_username_key" => "Username already taken",
                "users_email_key" => "Email already taken",
                _ => "Unique constraint violated",
            };
            (StatusCode::CONFLICT, msg.to_string())
        }
        _ => return (StatusCode::INTERNAL_SERVER_ERROR, code.to_string()),
    }
}
