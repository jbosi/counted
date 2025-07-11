//! This crate contains all shared fullstack server functions.
mod db;

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

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use sqlx::{FromRow, PgPool, Pool, QueryBuilder, Postgres};
#[cfg(feature = "server")]
use crate::db::get_db;
use shared::{Project, User, CreatableUser, CreatableProject, CreatableExpense};


// --- PROJECTS ---

#[server()]
pub async fn get_project(project_id: Uuid) -> Result<Project, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let projects: Project = sqlx::query_as("SELECT * FROM projects WHERE id = $1")
        .bind(project_id)
        .fetch_one(&pool)
        .await?;

    Ok(projects)
}

#[server()]
pub async fn add_project(project: CreatableProject) -> Result<Uuid, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let project_id: Uuid = sqlx::query_scalar!("INSERT INTO projects(name, description, currency) VALUES ($1, $2, $3) RETURNING id", project.name, project.description, "EUR")
        .fetch_one(&pool)
        .await?;

    Ok(project_id)
}

#[server()]
pub async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let projects: Vec<Project> = sqlx::query_as("SELECT id, name, created_at, currency, description FROM projects")
        .fetch_all(&pool)
        .await?;

    Ok(projects)
}

// --- USERS ---

#[server()]
pub async fn get_users() -> Result<Vec<User>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    // TODO Passer directement la struct en 1er argument
    let users: Vec<User> = sqlx::query_as("SELECT id, name, balance, created_at FROM users")
        .fetch_all(&pool)
        .await?;

    Ok(users)
}

#[server()]
pub async fn add_user(user: CreatableUser) -> Result<i32, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let user_id: i32 = sqlx::query_scalar!("INSERT INTO users(name) VALUES ($1) RETURNING id", user.name)
        .fetch_one(&pool)
        .await?;

    sqlx::query!("INSERT INTO user_projects(user_id, project_id) VALUES ($1, $2)", user_id, user.project_id)
        .execute(&pool)
        .await?;

    Ok(user_id)
}

#[server()]
pub async fn get_users_by_project_id(project_id: Uuid) -> Result<Vec<User>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let user_ids: Vec<i32> = sqlx::query!("SELECT user_id FROM user_projects WHERE project_id = $1", project_id)
        .fetch_all(&pool)
        .await?
        .into_iter()
        .map(|row| row.user_id)
        .collect();

    if user_ids.is_empty() {
        return Ok(Vec::new());
    }

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("SELECT id, name, balance, created_at FROM users WHERE id IN (");

    let mut separated = query_builder.separated(", ");
    for id in user_ids {
        separated.push_bind(id);
    }
    separated.push_unseparated(")");
        
    let query = query_builder.build_query_as::<User>();
    let users: Vec<User> = query
        .fetch_all(&pool)
        .await?;

    Ok(users)
}

// --- EXPENSES ---

#[server()]
pub async fn add_expense(expense: CreatableExpense) -> Result<i32, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let user_id: i32 = sqlx::query_scalar!("INSERT INTO users(name) VALUES ($1) RETURNING id", user.name)
        .fetch_one(&pool)
        .await?;

    sqlx::query!("INSERT INTO user_projects(user_id, project_id) VALUES ($1, $2)", user_id, user.project_id)
        .execute(&pool)
        .await?;

    Ok(user_id)
}