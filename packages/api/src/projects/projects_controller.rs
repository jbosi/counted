use chrono::Local;
use dioxus::core::bail;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{BatchProject, CreatableProject, EditableProject, ProjectDto, ProjectStatus};
use uuid::Uuid;

#[cfg(feature = "server")]
use crate::db::get_db;
#[cfg(feature = "server")]
use crate::users::users_repository::{delete_users, get_users_by_project_id};
#[cfg(feature = "server")]
use crate::utils::get_current_account_id;

#[cfg(feature = "server")]
use crate::projects::projects_repository;

#[get("/api/v1/projects/{project_id}")]
pub async fn get_project(project_id: Uuid) -> Result<ProjectDto, ServerFnError> {
    let pool = get_db().await;
    let mut tx = pool.begin().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    let project: ProjectDto = projects_repository::get_project(&mut *tx, project_id).await?;

    #[cfg(feature = "server")]
    if let Some(owner_id) = project.owner_account_id {
        let current = get_current_account_id().await;
        if current != Some(owner_id) {
            return Err(ServerFnError::new("Forbidden"));
        }
    }

    tx.commit().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(project)
}

#[get("/api/v1/projects")]
pub async fn get_projects() -> Result<Vec<ProjectDto>, ServerFnError> {
    #[cfg(feature = "server")]
    let account_id = get_current_account_id().await;
    #[cfg(not(feature = "server"))]
    let account_id: Option<Uuid> = None;

    let pool = get_db().await;
    let mut tx = pool.begin().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    let projects: Vec<ProjectDto> = projects_repository::get_projects(&mut *tx, account_id).await?;

    tx.commit().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(projects)
}

#[post("/api/v1/projects/batch")]
pub async fn get_projects_by_ids(
    Json(payload): Json<BatchProject>,
) -> Result<Vec<ProjectDto>, ServerFnError> {
    #[cfg(feature = "server")]
    let account_id = get_current_account_id().await;
    #[cfg(not(feature = "server"))]
    let account_id: Option<Uuid> = None;

    let pool = get_db().await;
    let mut tx = pool.begin().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    let projects: Vec<ProjectDto> = projects_repository::get_projects_by_ids(&mut *tx, payload).await?;

    #[cfg(feature = "server")]
    for project in &projects {
        if let Some(owner_id) = project.owner_account_id {
            if account_id != Some(owner_id) {
                return Err(ServerFnError::new("Forbidden"));
            }
        }
    }

    tx.commit().await.map_err(|e| ServerFnError::new(e.to_string()))?;

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

    let pool = get_db().await;
    let mut tx = pool.begin().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    let project_id: Uuid =
        projects_repository::add_project(&mut *tx, creatable_project.clone(), owner_account_id).await?;

    tx.commit().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    let new_project = ProjectDto {
        id: project_id,
        name: creatable_project.name,
        description: creatable_project.description,
        created_at: Local::now().naive_local(),
        currency: "EUR".to_string(),
        owner_account_id,
        status: ProjectStatus::Ongoing,
    };

    Ok(new_project)
}

#[put("/api/v1/projects")]
pub async fn update_project_by_id(
    Json(editable_project): Json<EditableProject>,
) -> Result<ProjectDto, ServerFnError> {
    let pool = get_db().await;
    let mut tx = pool.begin().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    #[cfg(feature = "server")]
    {
        let project = projects_repository::get_project(&mut *tx, editable_project.id).await?;
        if let Some(owner_id) = project.owner_account_id {
            let current = get_current_account_id().await;
            if current != Some(owner_id) {
                return Err(ServerFnError::new("Forbidden"));
            }
        }
    }

    let updated_project: ProjectDto =
        projects_repository::update_project_by_id(&mut *tx, editable_project).await?;

    tx.commit().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(updated_project)
}

#[delete("/api/v1/projects/{project_id}")]
pub async fn delete_project_by_id(project_id: Uuid) -> Result<(), ServerFnError> {
    let pool = get_db().await;
    let mut tx = pool.begin().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    #[cfg(feature = "server")]
    {
        let project = projects_repository::get_project(&mut *tx, project_id).await?;
        if let Some(owner_id) = project.owner_account_id {
            let current = get_current_account_id().await;
            if current != Some(owner_id) {
                return Err(ServerFnError::new("Forbidden"));
            }
        }
    }

    let users_bound_to_project = get_users_by_project_id(&mut *tx, project_id)
        .await
        .context("failed to get users bound to project")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    projects_repository::delete_project_by_id(&mut *tx, project_id).await?;

    delete_users(&mut *tx, users_bound_to_project.iter().map(|user| user.id).collect())
        .await
        .context("failed to deleted bound users")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    tx.commit().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}
