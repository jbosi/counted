use chrono::Local;
use dioxus::core::bail;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{BatchProject, CreatableProject, EditableProject, ProjectDto};
use uuid::Uuid;

#[cfg(feature = "server")]
use crate::users::users_repository::{delete_users, get_users_by_project_id};
#[cfg(feature = "server")]
use crate::utils::get_current_account_id;

#[cfg(feature = "server")]
use crate::projects::projects_repository;

#[get("/api/v1/projects/{project_id}")]
pub async fn get_project(project_id: Uuid) -> Result<ProjectDto, ServerFnError> {
    let project: ProjectDto = projects_repository::get_project(project_id).await?;

    #[cfg(feature = "server")]
    if let Some(owner_id) = project.owner_account_id {
        let current = get_current_account_id().await;
        if current != Some(owner_id) {
            return Err(ServerFnError::new("Forbidden"));
        }
    }

    Ok(project)
}

#[get("/api/v1/projects")]
pub async fn get_projects() -> Result<Vec<ProjectDto>, ServerFnError> {
    #[cfg(feature = "server")]
    let account_id = get_current_account_id().await;
    #[cfg(not(feature = "server"))]
    let account_id: Option<Uuid> = None;

    let projects: Vec<ProjectDto> = projects_repository::get_projects(account_id).await?;

    Ok(projects)
}

#[post("/api/v1/projects/batch")]
pub async fn get_projects_by_ids(
    Json(payload): Json<BatchProject>,
) -> Result<Vec<ProjectDto>, ServerFnError> {
    let projects: Vec<ProjectDto> = projects_repository::get_projects_by_ids(payload).await?;

    Ok(projects)
}

#[post("/api/v1/projects")]
pub async fn add_project(
    Json(creatable_project): Json<CreatableProject>,
) -> Result<ProjectDto, ServerFnError> {
    #[cfg(feature = "server")]
    let owner_account_id = get_current_account_id().await;
    #[cfg(not(feature = "server"))]
    let owner_account_id: Option<Uuid> = None;

    let project_id: Uuid =
        projects_repository::add_project(creatable_project.clone(), owner_account_id).await?;

    let new_project = ProjectDto {
        id: project_id,
        name: creatable_project.name,
        description: creatable_project.description,
        created_at: Local::now().naive_local(),
        currency: "EUR".to_string(),
        owner_account_id,
    };

    Ok(new_project)
}

#[put("/api/v1/projects")]
pub async fn update_project_by_id(
    Json(editable_project): Json<EditableProject>,
) -> Result<ProjectDto, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let project = projects_repository::get_project(editable_project.id).await?;
        if let Some(owner_id) = project.owner_account_id {
            let current = get_current_account_id().await;
            if current != Some(owner_id) {
                return Err(ServerFnError::new("Forbidden"));
            }
        }
    }

    let updated_project: ProjectDto =
        projects_repository::update_project_by_id(editable_project).await?;

    Ok(updated_project)
}

#[delete("/api/v1/projects/{project_id}")]
pub async fn delete_project_by_id(project_id: Uuid) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        let project = projects_repository::get_project(project_id).await?;
        if let Some(owner_id) = project.owner_account_id {
            let current = get_current_account_id().await;
            if current != Some(owner_id) {
                return Err(ServerFnError::new("Forbidden"));
            }
        }
    }

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
