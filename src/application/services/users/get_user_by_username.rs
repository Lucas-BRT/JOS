use crate::{
    domain::{
        type_wraper::TypeWrapped,
        user::{User, username::Username},
    },
    error::AppError,
    infra::db::{
        postgres::repositories::PostgresRepository, repositories::user_repository::UserRepository,
    },
    prelude::AppResult,
};
use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::PgPool;

pub async fn handle(
    State(pool): State<PgPool>,
    Path(user): Path<Username>,
) -> AppResult<Json<User>> {
    let usecase = PostgresRepository::new(pool);
    let user_row = usecase.find_by_username(&user.raw()).await?;

    match user_row {
        Some(user_row) => Ok(Json(User::try_from(user_row)?)),
        None => Err(AppError::NotFound("User not found".to_string())),
    }
}
