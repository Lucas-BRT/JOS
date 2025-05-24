use crate::{
    domain::user::{NewUser, user_repository::UserRepository},
    infra::db::postgres::repositories::PostgresRepository,
    prelude::AppResult,
};
use axum::{Json, extract::State};
use sqlx::PgPool;

pub async fn handle(State(pool): State<PgPool>, Json(user): Json<NewUser>) -> AppResult<String> {
    let usecase = PostgresRepository::new(pool);
    let user = usecase.create(&user).await?;

    Ok(user)
}
