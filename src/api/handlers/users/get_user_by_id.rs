use crate::{
    domain::user::User,
    infra::db::{
        postgres::repositories::pg_user_repository::PostgresUserRepository,
        repositories::user_repository::UserRepository,
    },
};
use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn handle(
    State(pool): State<PgPool>,
    Path(user): Path<Uuid>,
) -> Result<Json<User>, String> {
    let usecase = PostgresUserRepository::new(pool);
    let user_row = usecase.find_by_id(&user).await.map_err(|e| e.to_string())?;

    let user = User::try_from(user_row).map_err(|e| format!("failed to {}", e))?;

    Ok(Json(user))
}
