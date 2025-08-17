use dioxus::prelude::*;
use shared::{CreatableProject, ProjectDto, UpdatableProject};
use uuid::Uuid;

#[cfg(feature = "server")]
use crate::db::get_db;
use crate::sse::EventSSE;
#[cfg(feature = "server")]
use crate::sse::BROADCASTER;
#[cfg(feature = "server")]
use axum::response::sse::{Event, KeepAlive, Sse};
#[cfg(feature = "server")]
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
#[cfg(feature = "server")]
use sqlx::{FromRow, PgPool, Pool, Postgres, QueryBuilder};

#[server()]
pub async fn get_project(project_id: Uuid) -> Result<ProjectDto, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let projects: ProjectDto = sqlx::query_as("SELECT * FROM projects WHERE id = $1")
        .bind(project_id)
        .fetch_one(&pool)
        .await?;

    Ok(projects)
}

#[server()]
pub async fn get_projects() -> Result<Vec<ProjectDto>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let projects: Vec<ProjectDto> =
        sqlx::query_as("SELECT id, name, created_at, currency, description FROM projects")
            .fetch_all(&pool)
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

    BROADCASTER
        .broadcast(
            Event::default()
                .event::<String>(EventSSE::ProjectCreated.to_string())
                .data(EventSSE::ProjectCreated.to_string()),
        )
        .await;

    Ok(project_id)
}

#[server()]
pub async fn update_project_by_id(
    updatable_project: UpdatableProject,
) -> Result<ProjectDto, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let mut new_project =
        get_project(updatable_project.id).await.expect("Unable to find requested project_id");

    if updatable_project.name.is_some() {
        new_project.name = updatable_project.name.unwrap();
    }

    if updatable_project.description.is_some() {
        new_project.description = updatable_project.description;
    }

    if updatable_project.currency.is_some() {
        new_project.currency = updatable_project.currency.unwrap();
    }

    let update_project: ProjectDto = sqlx::query_as!(
        ProjectDto,
        "UPDATE projects SET name = $1, description = $2, currency = $3 WHERE id = $4 RETURNING id, name, created_at, currency, description",
        new_project.name,
        new_project.description,
        new_project.currency,
        new_project.id
    )
    .fetch_one(&pool)
    .await?;

    Ok(update_project)
}

#[server()]
pub async fn delete_project_by_id(project_id: Uuid) -> Result<(), ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    sqlx::query!("DELETE FROM projects WHERE id = $1", project_id).execute(&pool).await?;

    BROADCASTER
        .broadcast(
            Event::default()
                .event::<String>(EventSSE::ProjectDeleted.to_string())
                .data(EventSSE::ProjectDeleted.to_string()),
        )
        .await;

    Ok(())
}
