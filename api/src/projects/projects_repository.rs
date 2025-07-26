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
use crate::db::get_db;
use shared::{
    CreatableExpense, CreatableProject, CreatableUser, Expense, ExpenseType, NewPayment, Payment,
    Project, User, UserAmount,
};
#[cfg(feature = "server")]
use sqlx::{FromRow, PgPool, Pool, Postgres, QueryBuilder};

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

    let project_id: Uuid = sqlx::query_scalar!(
        "INSERT INTO projects(name, description, currency) VALUES ($1, $2, $3) RETURNING id",
        project.name,
        project.description,
        "EUR"
    )
    .fetch_one(&pool)
    .await?;

    Ok(project_id)
}

#[server()]
pub async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let projects: Vec<Project> =
        sqlx::query_as("SELECT id, name, created_at, currency, description FROM projects")
            .fetch_all(&pool)
            .await?;

    Ok(projects)
}
