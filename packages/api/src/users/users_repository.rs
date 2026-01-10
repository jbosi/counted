use dioxus::{fullstack::Json, prelude::*};
use uuid::Uuid;

#[cfg(feature = "server")]
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

#[cfg(feature = "server")]
use crate::db::get_db;
#[cfg(feature = "server")]
use crate::payments::get_payments_by_user_id;
#[cfg(feature = "server")]
use crate::sse::BROADCASTER;
#[cfg(feature = "server")]
use anyhow::Context;
use shared::sse::EventSSE;
use shared::{CreatableUser, Payment, User};
#[cfg(feature = "server")]
use sqlx::{FromRow, PgPool, Pool, Postgres, QueryBuilder};

#[get("/api/users")]
pub async fn get_users() -> Result<Vec<User>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let users: Vec<User> = sqlx::query_as("SELECT id, name, balance, created_at FROM users")
        .fetch_all(&pool)
        .await
        .context("Failed to fetch users from database")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(users)
}

#[delete("/api/users/{user_id}")]
pub async fn delete_users(user_id: i32) -> Result<(), ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let payments =
        get_payments_by_user_id(user_id).await.map_err(|e| ServerFnError::new(e.to_string()))?;

    if !payments.is_empty() {
        // User still has payments → abort with a clear error
        return Err(ServerFnError::new(
            "User has existing payments in this project and cannot be removed",
        ));
    }

    // Anonymous user cannot be in more than one user_project
    sqlx::query("DELETE FROM user_projects WHERE user_id = $1")
        .bind(user_id)
        .execute(&pool)
        .await
        .context("Failed to delete user in user_projects table with specified id")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    sqlx::query_as!(User, "DELETE FROM users WHERE id = $1", user_id)
        .execute(&pool)
        .await
        .context("Failed to delete user in user table with specified id")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

#[post("/api/users")]
pub async fn add_user(Json(user): Json<CreatableUser>) -> Result<User, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let user_id: i32 =
        sqlx::query_scalar!("INSERT INTO users(name) VALUES ($1) RETURNING id", user.name)
            .fetch_one(&pool)
            .await
            .context("Failed to insert user into database")
            .map_err(|e| ServerFnError::new(e.to_string()))?;

    sqlx::query("INSERT INTO user_projects(user_id, project_id) VALUES ($1, $2)")
        .bind(user_id)
        .bind(user.project_id)
        .execute(&pool)
        .await
        .context("Failed to associate user with project")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    BROADCASTER
        .broadcast(
            axum::response::sse::Event::default()
                .event::<String>(EventSSE::UserCreated.to_string())
                .data(EventSSE::UserCreated.to_string()),
        )
        .await;

    let created_user = User { id: user_id, name: user.name, balance: None, created_at: None };

    Ok(created_user)
}

#[get("/api/projects/{project_id}/users")]
pub async fn get_users_by_project_id(project_id: Uuid) -> Result<Vec<User>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let user_ids: Vec<i32> =
        sqlx::query_scalar("SELECT user_id FROM user_projects WHERE project_id = $1")
            .bind(project_id)
            .fetch_all(&pool)
            .await
            .context("Failed to fetch user IDs for project")
            .map_err(|e| ServerFnError::new(e.to_string()))?;

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

    let query = query_builder.build_query_as::<User>();
    let users: Vec<User> = query
        .fetch_all(&pool)
        .await
        .context("Failed to fetch users by IDs")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(users)
}
