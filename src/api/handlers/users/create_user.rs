use crate::{
    domain::user::NewUser,
    infra::db::{
        postgres::repositories::PostgresRepository, repositories::user_repository::UserRepository,
    },
    prelude::AppResult,
};
use axum::{Json, extract::State};
use sqlx::PgPool;

pub async fn handle(State(pool): State<PgPool>, Json(user): Json<NewUser>) -> AppResult<String> {
    let usecase = PostgresRepository::new(pool);
    let user = usecase.create(&user).await?;

    Ok(user)
}
