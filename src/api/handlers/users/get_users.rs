use axum::{Json, extract::State};
use sqlx::PgPool;

use crate::infra::db::{
    postgres::{models::user::UserRow, repositories::pg_user_repository::PostgresUserRepository},
    repositories::user_repository::UserRepository,
};

pub async fn handle(State(pool): State<PgPool>) -> Json<Vec<UserRow>> {
    let usecase = PostgresUserRepository::new(pool);
    let users = usecase.get_all().await.unwrap();

    Json(users)
}
