pub mod display_name;
pub mod username;

use display_name::DisplayName;
use sqlx::prelude::FromRow;
use username::Username;

#[derive(FromRow, Debug, PartialEq, Eq, Clone)]
#[sqlx(type_name = "users", rename_all = "lowercase")]
pub struct User {
    id: sqlx::types::Uuid,
    username: Username,
    display_name: DisplayName,
    // username: VARCHAR(30) UNIQUE NOT NULL CHECK (username ~ '^[a-zA-Z0-9_]{3,30}$'),
    // display_name VARCHAR(100) NOT NULL,
    // email VARCHAR(255) UNIQUE NOT NULL,
    // password_hash VARCHAR(255) NOT NULL,
    // user_role user_role DEFAULT 'user' NOT NULL,
    // gender user_gender DEFAULT 'other' NOT NULL,
    // created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    // updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
}
