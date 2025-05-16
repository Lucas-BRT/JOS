use axum::{Json, extract::State};
use sqlx::PgPool;

use crate::infra::db::{
    postgres::{models::user::UserRow, repositories::PostgresRepository},
    repositories::user_repository::UserRepository,
};

pub async fn handle(State(pool): State<PgPool>) -> Json<Vec<UserRow>> {
    let usecase = PostgresRepository::new(pool);
    let users = usecase.get_all().await.unwrap();

    Json(users)
}
