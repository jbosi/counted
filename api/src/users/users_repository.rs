use dioxus::prelude::*;
use uuid::Uuid;

#[cfg(feature = "server")]
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use crate::authentication::User;
#[cfg(feature = "server")]
use crate::db::get_db;
#[cfg(feature = "server")]
use crate::sse::BROADCASTER;
use shared::sse::EventSSE;
use shared::{CreatableRegisteredUser, CreatableUser, ProjectDto, UserDto};
#[cfg(feature = "server")]
use sqlx::{FromRow, PgPool, Pool, Postgres, QueryBuilder};

#[server()]
pub async fn get_users() -> Result<Vec<UserDto>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    // TODO Passer directement la struct en 1er argument
    let users: Vec<UserDto> =
        sqlx::query_as("SELECT id, name, balance, created_at FROM users").fetch_all(&pool).await?;

    Ok(users)
}

#[server()]
pub async fn get_user_by_id(user_id: i32) -> Result<User, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let user: User =
        sqlx::query_as("SELECT * FROM users WHERE id = $1").bind(user_id).fetch_one(&pool).await?;

    Ok(user)
}

#[server()]
pub async fn add_user(user: CreatableUser) -> Result<i32, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let user_id: i32 =
        sqlx::query_scalar!("INSERT INTO users(name) VALUES ($1) RETURNING id", user.name)
            .fetch_one(&pool)
            .await?;

    sqlx::query!(
        "INSERT INTO user_projects(user_id, project_id) VALUES ($1, $2)",
        user_id,
        user.project_id
    )
    .execute(&pool)
    .await?;

    BROADCASTER
        .broadcast(
            axum::response::sse::Event::default()
                .event::<String>(EventSSE::UserCreated.to_string())
                .data(EventSSE::UserCreated.to_string()),
        )
        .await;

    Ok(user_id)
}

#[server()]
pub async fn add_registered_user(user: CreatableRegisteredUser) -> Result<i32, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    Ok(0)
}

#[server()]
pub async fn get_users_by_project_id(project_id: Uuid) -> Result<Vec<UserDto>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let user_ids: Vec<i32> =
        sqlx::query!("SELECT user_id FROM user_projects WHERE project_id = $1", project_id)
            .fetch_all(&pool)
            .await?
            .into_iter()
            .map(|row| row.user_id)
            .collect();

    if user_ids.is_empty() {
        return Ok(Vec::new());
    }

    let mut query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("SELECT id, name, balance, created_at FROM users WHERE id IN (");

    let mut separated = query_builder.separated(", ");
    for id in user_ids {
        separated.push_bind(id);
    }
    separated.push_unseparated(")");

    let query = query_builder.build_query_as::<UserDto>();
    let users: Vec<UserDto> = query.fetch_all(&pool).await?;

    // TODO
    // const QUERY: &str = "SELECT u.id, u.name, u.balance, u.created_at
    //     FROM users u
    //     JOIN user_projects up ON up.user_id = u.id
    //     JOIN projects p ON p.id = up.project_id
    //     WHERE project_id = '553b5fc6-3e91-4c85-af6f-5d7a2e6bf9ff'";
    //
    // let users: Vec<User> = sqlx::query(QUERY)
    //     .fetch_all(&pool)
    //     .await?;

    Ok(users)
}
