use crate::{
    domain::user::User,
    infra::db::{
        postgres::repositories::pg_user_repository::PostgresUserRepository,
        repositories::user_repository::UserRepository,
    },
};
use axum::{Json, extract::State};
use sqlx::PgPool;

pub async fn handle(
    State(pool): State<PgPool>,
    Json(user): Json<User>,
) -> Result<Json<()>, String> {
    let usecase = PostgresUserRepository::new(pool);
    let users = usecase.update(&user).await.map_err(|e| e.to_string())?;

    Ok(Json(users))
}
