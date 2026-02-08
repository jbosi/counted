use chrono::Local;
use dioxus::core::bail;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{BatchProject, CreatableProject, EditableProject, ProjectDto};
use uuid::Uuid;

#[cfg(feature = "server")]
use crate::users::users_repository::{delete_users, get_users_by_project_id};

#[cfg(feature = "server")]
use crate::projects::projects_repository;

#[get("/api/projects/{project_id}")]
pub async fn get_project(project_id: Uuid) -> Result<ProjectDto, ServerFnError> {
    let projects: ProjectDto = projects_repository::get_project(project_id).await?;

    Ok(projects)
}

#[get("/api/projects")]
pub async fn get_projects() -> Result<Vec<ProjectDto>, ServerFnError> {
    let projects: Vec<ProjectDto> = projects_repository::get_projects().await?;

    Ok(projects)
}

#[post("/api/projects/batch")]
pub async fn get_projects_by_ids(
    Json(payload): Json<BatchProject>,
) -> Result<Vec<ProjectDto>, ServerFnError> {
    let projects: Vec<ProjectDto> = projects_repository::get_projects_by_ids(payload).await?;

    Ok(projects)
}

#[post("/api/projects")]
pub async fn add_project(
    Json(creatable_project): Json<CreatableProject>,
) -> Result<ProjectDto, ServerFnError> {
    let project_id: Uuid = projects_repository::add_project(creatable_project.clone()).await?;

    let new_project = ProjectDto {
        id: project_id,
        name: creatable_project.name,
        description: creatable_project.description,
        created_at: Local::now().naive_local(),
        currency: "EUR".to_string(),
    };

    Ok(new_project)
}

#[put("/api/projects")]
pub async fn update_project_by_id(
    Json(editable_project): Json<EditableProject>,
) -> Result<ProjectDto, ServerFnError> {
    let updated_project: ProjectDto =
        projects_repository::update_project_by_id(editable_project).await?;

    Ok(updated_project)
}

#[delete("/api/projects/{project_id}")]
pub async fn delete_project_by_id(project_id: Uuid) -> Result<(), ServerFnError> {
    let users_bound_to_project = get_users_by_project_id(project_id)
        .await
        .context("failed to get users bound to project")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    projects_repository::delete_project_by_id(project_id).await?;

    delete_users(users_bound_to_project.iter().map(|user| user.id).collect())
        .await
        .context("failed to deleted bound users")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}
