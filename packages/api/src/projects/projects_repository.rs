use dioxus::prelude::*;
use shared::{BatchProject, CreatableProject, EditableProject, ProjectDto, ProjectStatus};
use uuid::Uuid;

#[cfg(feature = "server")]
use sqlx::PgConnection;

#[cfg(feature = "server")]
pub async fn get_project(
    executor: &mut PgConnection,
    project_id: Uuid,
) -> Result<ProjectDto, ServerFnError> {
    let project: ProjectDto = sqlx::query_as!(
        ProjectDto,
        r#"SELECT id, name, created_at, currency, description, status as "status: ProjectStatus", owner_account_id FROM projects WHERE id = $1"#,
        project_id
    )
    .fetch_one(&mut *executor)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to get project: {}", e)))?;

    Ok(project)
}

#[cfg(feature = "server")]
pub async fn get_projects(
    executor: &mut PgConnection,
    account_id: Option<Uuid>,
) -> Result<Vec<ProjectDto>, ServerFnError> {
    let projects: Vec<ProjectDto> = match account_id {
        Some(id) => sqlx::query_as!(
            ProjectDto,
            r#"SELECT DISTINCT p.id, p.name, p.created_at, p.currency, p.description, p.owner_account_id,
                      p.status as "status: ProjectStatus"
               FROM projects p
               LEFT JOIN account_projects ap ON ap.project_id = p.id AND ap.account_id = $1
               WHERE p.owner_account_id = $1 OR ap.account_id = $1"#,
            id
        )
        .fetch_all(&mut *executor)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to get projects by account: {}", e)))?,

        None => sqlx::query_as!(
            ProjectDto,
            r#"SELECT id, name, created_at, currency, description, owner_account_id, status as "status: ProjectStatus" FROM projects WHERE owner_account_id IS NULL"#
        )
        .fetch_all(&mut *executor)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to get unowned projects: {}", e)))?,
    };

    Ok(projects)
}

#[cfg(feature = "server")]
pub async fn has_account_access(
    executor: &mut PgConnection,
    account_id: Uuid,
    project_id: Uuid,
) -> Result<bool, ServerFnError> {
    let exists: bool = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM account_projects WHERE account_id = $1 AND project_id = $2)",
        account_id,
        project_id
    )
    .fetch_one(&mut *executor)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to check account access: {}", e)))?
    .unwrap_or(false);

    Ok(exists)
}

#[cfg(feature = "server")]
pub async fn get_projects_by_ids(
    executor: &mut PgConnection,
    payload: BatchProject,
) -> Result<Vec<ProjectDto>, ServerFnError> {
    let projects: Vec<ProjectDto> = sqlx::query_as!(
        ProjectDto,
        r#"SELECT id, name, created_at, currency, description, owner_account_id, status as "status: ProjectStatus" FROM projects WHERE id = ANY($1)"#,
        &payload.ids[..]
    )
    .fetch_all(&mut *executor)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to get projects by ids: {}", e)))?;

    Ok(projects)
}

#[cfg(feature = "server")]
pub async fn add_project(
    executor: &mut PgConnection,
    project: CreatableProject,
    owner_account_id: Option<Uuid>,
) -> Result<Uuid, ServerFnError> {
    let project_id: Uuid = sqlx::query_scalar!(
        "INSERT INTO projects(name, description, currency, owner_account_id) VALUES ($1, $2, $3, $4) RETURNING id",
        project.name,
        project.description,
        "EUR",
        owner_account_id
    )
    .fetch_one(&mut *executor)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to add project: {}", e)))?;

    Ok(project_id)
}

#[cfg(feature = "server")]
pub async fn update_project_by_id(
    executor: &mut PgConnection,
    editable_project: EditableProject,
) -> Result<ProjectDto, ServerFnError> {
    let mut new_project =
        get_project(&mut *executor, editable_project.id).await.expect("Unable to find requested project_id");

    if editable_project.name.is_some() {
        new_project.name = editable_project.name.unwrap();
    }

    if editable_project.description.is_some() {
        new_project.description = editable_project.description;
    }

    if editable_project.currency.is_some() {
        new_project.currency = editable_project.currency.unwrap();
    }

    if editable_project.status.is_some() {
        new_project.status = editable_project.status.unwrap();
    }

    let update_project: ProjectDto = sqlx::query_as!(
        ProjectDto,
        r#"UPDATE projects SET name = $1, description = $2, currency = $3, status = $4 WHERE id = $5 RETURNING id, name, created_at, currency, description, status as "status: ProjectStatus", owner_account_id"#,
        new_project.name,
        new_project.description,
        new_project.currency,
        new_project.status as ProjectStatus,
        new_project.id
    )
    .fetch_one(&mut *executor)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to update project: {}", e)))?;

    Ok(update_project)
}

#[cfg(feature = "server")]
pub async fn delete_project_by_id(
    executor: &mut PgConnection,
    project_id: Uuid,
) -> Result<(), ServerFnError> {
    // TODO allow to archive projects

    sqlx::query!("DELETE FROM projects WHERE id = $1", project_id)
        .execute(&mut *executor)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to delete project: {}", e)))?;

    Ok(())
}
