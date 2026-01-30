use dioxus::prelude::*;
use uuid::Uuid;

#[cfg(feature = "server")]
use crate::db::get_db;
#[cfg(feature = "server")]
use anyhow::Context;
use shared::{CreatableUser, User};
#[cfg(feature = "server")]
use sqlx::{Pool, Postgres, QueryBuilder};

#[cfg(feature = "server")]
pub async fn get_users() -> Result<Vec<User>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let users: Vec<User> = sqlx::query_as("SELECT id, name, balance, created_at FROM users")
        .fetch_all(&pool)
        .await
        .context("Failed to fetch users from database")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(users)
}

#[cfg(feature = "server")]
pub async fn delete_users(user_id: i32) -> Result<(), ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    // Anonymous user cannot be in more than one user_project
    sqlx::query("DELETE FROM user_projects WHERE user_id = $1")
        .bind(user_id)
        .execute(&pool)
        .await
        .context("Failed to delete user in user_projects table with specified id")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
        .execute(&pool)
        .await
        .context("Failed to delete user in user table with specified id")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

#[cfg(feature = "server")]
pub async fn add_user(user: CreatableUser) -> Result<i32, ServerFnError> {
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

    Ok(user_id)
}

#[cfg(feature = "server")]
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
