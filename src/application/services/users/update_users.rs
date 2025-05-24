use crate::{domain::user::User, infra::db::postgres::repositories::PostgresRepository};
use axum::{Json, extract::State};
use sqlx::PgPool;

pub async fn handle(
    State(pool): State<PgPool>,
    Json(user): Json<User>,
) -> Result<Json<()>, String> {
    let usecase = PostgresRepository::new(pool);
    let users = usecase.update(&user).await.map_err(|e| e.to_string())?;

    Ok(Json(users))
}
