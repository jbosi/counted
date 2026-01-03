use dioxus::logger::tracing;
use dioxus::{fullstack::Json, prelude::*};
use shared::sse::EventSSE;
use shared::{CreatableProject, EditableProject, ProjectDto};
use uuid::Uuid;

#[cfg(feature = "server")]
use crate::db::get_db;
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
    Router,
};
#[cfg(feature = "server")]
use sqlx::{FromRow, PgPool, Pool, Postgres, QueryBuilder};

#[get("/api/projects/{project_id}")]
pub async fn get_project(project_id: Uuid) -> Result<ProjectDto, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let projects: ProjectDto = sqlx::query_as("SELECT * FROM projects WHERE id = $1")
        .bind(project_id)
        .fetch_one(&pool)
        .await
        .context("Failed get project with specified id")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(projects)
}

#[get("/api/projects")]
pub async fn get_projects() -> Result<Vec<ProjectDto>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let projects: Vec<ProjectDto> =
        sqlx::query_as("SELECT id, name, created_at, currency, description FROM projects")
            .fetch_all(&pool)
            .await
            .context("Failed to get projects")
            .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(projects)
}

#[post("/api/projects")]
pub async fn add_project(Json(project): Json<CreatableProject>) -> Result<Uuid, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let project_id: Uuid = sqlx::query_scalar!(
        "INSERT INTO projects(name, description, currency) VALUES ($1, $2, $3) RETURNING id",
        project.name,
        project.description,
        "EUR"
    )
    .fetch_one(&pool)
    .await
    .context("Failed to add project")
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    BROADCASTER
        .broadcast(
            Event::default()
                .event::<String>(EventSSE::ProjectCreated.to_string())
                .data(EventSSE::ProjectCreated.to_string()),
        )
        .await;

    Ok(project_id)
}

#[put("/api/projects")]
pub async fn update_project_by_id(
    Json(editable_project): Json<EditableProject>,
) -> Result<ProjectDto, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let mut new_project =
        get_project(editable_project.id).await.expect("Unable to find requested project_id");

    if editable_project.name.is_some() {
        new_project.name = editable_project.name.unwrap();
    }

    if editable_project.description.is_some() {
        new_project.description = editable_project.description;
    }

    if editable_project.currency.is_some() {
        new_project.currency = editable_project.currency.unwrap();
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
    .await
    .context("Failed to update project")
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    BROADCASTER
        .broadcast(
            Event::default()
                .event::<String>(EventSSE::ProjectModified.to_string())
                .data(EventSSE::ProjectModified.to_string()),
        )
        .await;

    Ok(update_project)
}

#[delete("/api/projects/{project_id}")]
pub async fn delete_project_by_id(project_id: Uuid) -> Result<(), ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;
    tracing::info!("projectid = {:?}", project_id);

    sqlx::query!("DELETE FROM projects WHERE id = $1", project_id)
        .execute(&pool)
        .await
        .context("Failed to delete project with specified id")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    BROADCASTER
        .broadcast(
            Event::default()
                .event::<String>(EventSSE::ProjectDeleted.to_string())
                .data(EventSSE::ProjectDeleted.to_string()),
        )
        .await;

    Ok(())
}
