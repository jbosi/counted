use dioxus::prelude::*;
use shared::{BatchProject, CreatableProject, EditableProject, ProjectDto};
use uuid::Uuid;

#[cfg(feature = "server")]
use crate::db::get_db;
#[cfg(feature = "server")]
use sqlx::{Pool, Postgres};

#[cfg(feature = "server")]
pub async fn get_project(project_id: Uuid) -> Result<ProjectDto, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let project: ProjectDto =
        sqlx::query_as("SELECT id, name, created_at, currency, description, owner_account_id FROM projects WHERE id = $1")
            .bind(project_id)
            .fetch_one(&pool)
            .await
            .context("Failed get project with specified id")
            .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(project)
}

#[cfg(feature = "server")]
pub async fn get_projects(account_id: Option<Uuid>) -> Result<Vec<ProjectDto>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let projects: Vec<ProjectDto> = match account_id {
        Some(id) => sqlx::query_as(
            "SELECT id, name, created_at, currency, description, owner_account_id FROM projects WHERE owner_account_id = $1",
        )
        .bind(id)
        .fetch_all(&pool)
        .await
        .context("Failed to get projects for account")
        .map_err(|e| ServerFnError::new(e.to_string()))?,

        None => sqlx::query_as(
            "SELECT id, name, created_at, currency, description, owner_account_id FROM projects WHERE owner_account_id IS NULL",
        )
        .fetch_all(&pool)
        .await
        .context("Failed to get anonymous projects")
        .map_err(|e| ServerFnError::new(e.to_string()))?,
    };

    Ok(projects)
}

#[cfg(feature = "server")]
pub async fn get_projects_by_ids(payload: BatchProject) -> Result<Vec<ProjectDto>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let projects: Vec<ProjectDto> = sqlx::query_as(
        "SELECT id, name, created_at, currency, description, owner_account_id FROM projects WHERE id = ANY($1)",
    )
    .bind(&payload.ids[..])
    .fetch_all(&pool)
    .await
    .context("Failed to get projects")
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(projects)
}

#[cfg(feature = "server")]
pub async fn add_project(
    project: CreatableProject,
    owner_account_id: Option<Uuid>,
) -> Result<Uuid, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let project_id: Uuid = sqlx::query_scalar(
        "INSERT INTO projects(name, description, currency, owner_account_id) VALUES ($1, $2, $3, $4) RETURNING id",
    )
    .bind(&project.name)
    .bind(&project.description)
    .bind("EUR")
    .bind(owner_account_id)
    .fetch_one(&pool)
    .await
    .context("Failed to add project")
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(project_id)
}

#[cfg(feature = "server")]
pub async fn update_project_by_id(
    editable_project: EditableProject,
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

    let update_project: ProjectDto = sqlx::query_as(
        "UPDATE projects SET name = $1, description = $2, currency = $3 WHERE id = $4 RETURNING id, name, created_at, currency, description, owner_account_id",
    )
    .bind(&new_project.name)
    .bind(&new_project.description)
    .bind(&new_project.currency)
    .bind(new_project.id)
    .fetch_one(&pool)
    .await
    .context("Failed to update project")
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(update_project)
}

#[cfg(feature = "server")]
pub async fn delete_project_by_id(project_id: Uuid) -> Result<(), ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    // TODO allow to archive projects

    sqlx::query!("DELETE FROM projects WHERE id = $1", project_id)
        .execute(&pool)
        .await
        .context("Failed to delete project with specified id")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}
