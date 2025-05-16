use crate::{
    domain::user::NewUser,
    infra::db::{
        postgres::repositories::PostgresRepository, repositories::user_repository::UserRepository,
    },
};
use axum::{Json, extract::State};
use sqlx::PgPool;

pub async fn handle(
    State(pool): State<PgPool>,
    Json(user): Json<NewUser>,
) -> Result<String, String> {
    let usecase = PostgresRepository::new(pool);
    let user = usecase.create(&user).await.map_err(|e| e.to_string())?;

    Ok(user)
}
