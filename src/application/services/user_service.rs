use crate::{
    domain::{
        user::{NewUser, User, user_repository::UserRepository, username::Username},
        utils::type_wraper::TypeWrapped,
    },
    error::AppError,
    infra::db::postgres::{models::user::UserRow, repositories::PostgresRepository},
    prelude::AppResult,
};
use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::PgPool;

pub async fn create(State(pool): State<PgPool>, Json(user): Json<NewUser>) -> AppResult<String> {
    let usecase = PostgresRepository::new(pool);
    let user = usecase.create(&user).await?;

    Ok(user)
}

pub async fn find_by_username(
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

pub async fn get_all(State(pool): State<PgPool>) -> Json<Vec<UserRow>> {
    let usecase = PostgresRepository::new(pool);
    let users = usecase.get_all().await.unwrap();

    Json(users)
}

pub async fn update(
    State(pool): State<PgPool>,
    Json(user): Json<User>,
) -> Result<Json<()>, String> {
    let usecase = PostgresRepository::new(pool);
    let users = usecase.update(&user).await.map_err(|e| e.to_string())?;

    Ok(Json(users))
}
